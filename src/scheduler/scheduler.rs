use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::thread::{JoinHandle};
use std::time::{Duration, Instant};
use crossterm::style::Color;
use crate::{CallbackConfig, EzContext};
use crate::run::run::{open_popup, stop};
use crate::property::ez_properties::EzProperties;
use crate::property::ez_values::EzValues;
use crate::property::ez_property::EzProperty;
use crate::run::definitions::StateTree;
use crate::scheduler::definitions::{EzPropertyUpdater, EzThread, GenericEzTask};
use crate::states::definitions::{HorizontalAlignment, VerticalAlignment};


#[derive(Default)]
pub struct Scheduler {
    pub widgets_to_update: Vec<String>,
    pub force_redraw: bool,
    pub properties: HashMap<String, EzProperties>,
    pub tasks: Vec<Task>,
    pub threads_to_start: Vec<(EzThread, Option<GenericEzTask>)>,
    pub thread_handles: Vec<(JoinHandle<()>, Option<GenericEzTask>)>,
    pub new_callback_configs: Vec<(String, CallbackConfig)>,
    pub updated_callback_configs: Vec<(String, CallbackConfig)>,
    pub property_receivers: HashMap<String, Receiver<EzValues>>,
    pub property_subscribers: HashMap<String, Vec<EzPropertyUpdater>>,
    pub property_callbacks: HashMap<String, Vec<Box<dyn FnMut(EzContext)>>>,
}


pub struct Task {
    pub widget: String,
    pub func: GenericEzTask,
    pub recurring: bool,
    pub canceled: bool,
    pub interval: Duration,
    pub last_execution: Option<Instant>,
}


impl Scheduler {

    /// Schedule a [GenericEzTask] to be executed once, after the passed duration. The duration can
    /// be 0. A [GenericEzTask] is any FnMut that accepts an [EzContext] parameter. The [EzContext]
    /// allows you to access the [StateTree] and [Scheduler] from within the function, so you can
    /// make changes to the UI. Your closure should look like this:
    /// ```
    /// use ez_term::EzContext;
    /// let my_closure = |context: EzContext| { };
    /// ```
    /// Or if you prefer an explicit function:
    /// ```
    /// fn my_func(context: EzContext) { };
    /// ```
    /// Closures are recommended for most use cases, as they allow capturing variables. You can
    /// use a [GenericEzTask] in many ways:
    /// # Changing a widget state
    /// ```
    /// let my_closure = |context: EzContext| {
    ///     let state = context.state_tree.get_by_id_mut("my_label").as_label_mut();
    ///     state.set_text("New text!".to_string());
    ///     state.update(context.scheduler);
    /// };
    /// scheduler.schedule_once(state.get_id(), Box::new(my_closure), Duration::from_secs(0));
    /// ```
    /// # Starting a scheduled task after a delay
    /// Note that we will now use the 'move' keyword to move our counting variable into our task.
    /// ```
    /// let increment: usize = 1;
    /// let my_counter = move |context: EzContext| {
    ///     let state = context.state_tree.get_by_id_mut("my_label").as_label_mut();
    ///     state.set_text(format!("Counting {}", increment));
    ///     state.update(context.scheduler);
    /// };
    /// let my_scheduler = |context: EzContext| {
    ///     context.scheduler.schedule_interval("my_label".to_string(), Box::new(my_counter),
    ///     Duration::from_secs(1))
    /// };
    /// scheduler.schedule_once(state.get_id(), Box::new(my_scheduler), Duration::from_secs(5));
    /// ```
    pub fn schedule_once(&mut self, widget: String, func: GenericEzTask, after: Duration) {

        let task = Task::new(widget, func, false, after,
                             Some(Instant::now()));
        self.tasks.push(task);
    }

    pub fn schedule_interval(&mut self, widget: String,  func: GenericEzTask, interval: Duration)
        -> &mut Task {
        let task = Task::new(widget, func, true, interval, None);
        self.tasks.push(task);
        self.tasks.last_mut().unwrap()
    }

    pub fn schedule_threaded(&mut self,
                             threaded_func: Box<dyn FnOnce(HashMap<String, EzProperties>) + Send>,
                             on_finish: Option<GenericEzTask>) {

        self.threads_to_start.push((threaded_func, on_finish));
    }

    pub fn open_popup(&mut self, template: String, state_tree: &mut StateTree) -> String {
        open_popup(template, state_tree, self)
    }

    /// Pass a callback config that will be set verbatim on the object on the next frame.
    pub fn set_callback_config(&mut self, widget_path: &str, callback_config: CallbackConfig) {
        self.new_callback_configs.push((widget_path.to_string(), callback_config));
    }

    /// Pass a callback config that will update the current callback config for the object on the
    /// next frame. Only sets new callbacks, cannot remove old ones.
    pub fn update_callback_config(&mut self, widget_path: &str, callback_config: CallbackConfig) {
        self.updated_callback_configs.push((widget_path.to_string(), callback_config));

    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribed to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to the property will update automatically.
    pub fn new_usize_property(&mut self, name: &str, value: usize) -> EzProperty<usize> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.properties.insert(name.to_string(), EzProperties::Usize(property.clone()));
        self.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribed to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to the property will update automatically.
    pub fn new_string_property(&mut self, name: &str, value: String) -> EzProperty<String> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.properties.insert(name.to_string(), EzProperties::String(property.clone()));
        self.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribed to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to the property will update automatically.
    pub fn new_bool_property(&mut self, name: &str, value: bool) -> EzProperty<bool> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.properties.insert(name.to_string(), EzProperties::Bool(property.clone()));
        self.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribed to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to the property will update automatically.
    pub fn new_color_property(&mut self, name: &str, value: Color) -> EzProperty<Color> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.properties.insert(name.to_string(), EzProperties::Color(property.clone()));
        self.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribed to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to the property will update automatically.
    pub fn new_vertical_alignment_property(&mut self, name: &str, value: VerticalAlignment)
        -> EzProperty<VerticalAlignment> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.properties.insert(name.to_string(), EzProperties::VerticalAlignment(property.clone()));
        self.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribed to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to the property will update automatically.
    pub fn new_horizontal_alignment_property(&mut self, name: &str, value: HorizontalAlignment)
        -> EzProperty<HorizontalAlignment> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.properties.insert(name.to_string(), EzProperties::HorizontalAlignment(property.clone()));
        self.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribed to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to the property will update automatically.
    pub fn new_horizontal_pos_hint_property(
        &mut self, name: &str, value: Option<(HorizontalAlignment, f64)>)
        -> EzProperty<Option<(HorizontalAlignment, f64)>> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.properties.insert(name.to_string(), EzProperties::HorizontalPosHint(property.clone()));
        self.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribed to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to the property will update automatically.
    pub fn new_vertical_pos_hint_property(
        &mut self, name: &str, value: Option<(VerticalAlignment, f64)>)
        -> EzProperty<Option<(VerticalAlignment, f64)>> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.properties.insert(name.to_string(), EzProperties::VerticalPosHint(property.clone()));
        self.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribed to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to the property will update automatically.
    pub fn new_size_hint_property(&mut self, name: &str, value: Option<f64>)
        -> EzProperty<Option<f64>> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.properties.insert(name.to_string(), EzProperties::SizeHint(property.clone()));
        self.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Subscribe one property to another, ensuring the subscriber will always have the value of the
    /// property it is subscribed to on the next frame. An update func is required which will be
    /// called when the property subscribed to changes. The update func receives the new value and
    /// is responsible for setting the appropriate field on the subscriber.
    pub fn subscribe_to_ez_property(&mut self, name: &str, update_func: EzPropertyUpdater) {

        if !self.property_subscribers.contains_key(name) {
            self.property_subscribers.insert(name.to_string(), Vec::new());
        }
        self.property_subscribers.get_mut(name).unwrap().push(update_func);
    }

    /// Schedule a widget to be updated on the next frame. Can also be called from the widget itself
    /// as ```[widget.update(scheduler)]``` (for convenience).
    pub fn update_widget(&mut self, path: String) {
        if path.starts_with("/modal") {
            self.force_redraw = true;
            return
        }
        if !self.widgets_to_update.contains(&path) {
            self.widgets_to_update.push(path);
        }
    }

    /// Schedule a full screen redraw on the next frame. [get_contents] will be called on the root
    /// widget and drawn to screen. Only changed pixels are actually drawn as an optimization.
    pub fn force_redraw(&mut self) { self.force_redraw = true; }

    /// Bind a callback function to the changing of an EzProperty. Make sure that the function you
    /// create has the right signature, e.g.:
    /// ```
    /// |context: EzContext| {
    ///     let state  = context.state_tree.get_by_id("my_label");
    ///     state.set_text("Value changed");
    ///     state.update(context.scheduler);
    /// }
    /// ```
    pub fn bind_ez_property_callback(&mut self, name: String, callback: Box<dyn FnMut(EzContext)>) {

        let callbacks =
            self.property_callbacks.entry(name).or_insert(Vec::new());
        callbacks.push(callback);
    }

    /// Gracefully exit the app.
    pub fn exit(&self) { stop(); }
}

impl Task {

    pub fn new(widget: String, func: GenericEzTask, recurring: bool,
               interval: Duration, last_execution: Option<Instant>)
        -> Self { Task { widget, func, recurring, interval, canceled: false, last_execution } }

    pub fn cancel(&mut self) { self.canceled = true; }

}