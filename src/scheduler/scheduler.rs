//! # Scheduler
//!
//! A module implementing the Scheduler struct.
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::thread::{JoinHandle};
use std::time::{Duration, Instant};

use crossterm::style::Color;

use crate::{CallbackConfig, EzContext, EzPropertiesMap};
use crate::parser::ez_definition::Templates;
use crate::property::ez_properties::EzProperties;
use crate::property::ez_property::EzProperty;
use crate::property::ez_values::EzValues;
use crate::run::definitions::{Coordinates, StateTree};
use crate::run::run::{open_popup, stop};
use crate::scheduler::definitions::{EzPropertyUpdater, EzThread, GenericEzTask};
use crate::states::definitions::{HorizontalAlignment, VerticalAlignment};
use crate::states::ez_state::EzState;


/// The Scheduler is a key component of the framework. It, along with the [StateTree], gives
/// you control over the UI at runtime.
/// # Features of the scheduler
/// - Scheduling to redraw a widget on the next frame, or force a global screen redraw
/// - scheduling functions/closures to run once or on an interval
/// - run functions or closures in a background thread
/// - set a new [CallbackConfig] for a widget, or update the existing one
/// - Bind a callback to an [EzProperty] (to be called on value change)
/// - Creating custom [EzProperty]s
/// - Set the selected widget
/// - Stop the app
pub struct SchedulerFrontend {

    /// Backend of the scheduler. Do not use. Use the public funcs instead.
    pub backend: Scheduler,
}



impl SchedulerFrontend {

    /// Schedule a [GenericEzTask] to be executed once, after the passed duration. The duration can
    /// be 0 to start this on the next frame. This should only be used to manipulate the UI; to run
    /// any functions that will not return immediately, use [schedule_threaded].
    /// # The GenericEzTask function
    /// A [GenericEzTask] is any FnMut that accepts an [EzContext] parameter. The [EzContext]
    /// allows you to access the [StateTree] and [Scheduler] from within the function, so you can
    /// make changes to the UI. Your closure should look like this:
    /// ```
    /// use ez_term::*;
    /// let my_closure = |context: EzContext| { };
    /// ```
    /// Or if you prefer an explicit function:
    /// ```
    /// use ez_term::*;
    /// fn my_func(context: EzContext) { };
    /// ```
    /// Closures are recommended for most use cases, as they allow capturing variables. You can
    /// use a run-once [GenericEzTask] in many ways:
    /// # Changing a widget state
    /// ```
    /// use ez_term::*;
    /// use std::time::Duration;
    /// let my_closure = |context: EzContext| {
    ///     let state = context.state_tree.get_by_id_mut("my_label").as_label_mut();
    ///     state.set_text("New text!".to_string());
    ///     state.update(context.scheduler);
    ///     true
    /// };
    /// scheduler.schedule_once(state.get_id(), Box::new(my_closure), Duration::from_secs(0));
    /// ```
    /// Note that we used the 'move' keyword to move our counting variable into our task.
    pub fn schedule_once(&mut self, widget: String, func: GenericEzTask, after: Duration) {

        let task = Task::new(widget, func, false, after,
                             Some(Instant::now()));
        self.backend.tasks.push(task);
    }

    /// Schedule a [GenericEzTask] to be executed repeatedly on an interval. As long as the passed
    /// function keeps returning true, it will be scheduled to run again after the interval.
    /// This should only be used to manipulate the UI; to run any functions that will not return
    /// immediately, use [schedule_threaded].
    /// # The GenericEzTask function
    /// A [GenericEzTask] is any FnMut that accepts an [EzContext] parameter. The [EzContext]
    /// allows you to access the [StateTree] and [Scheduler] from within the function, so you can
    /// make changes to the UI. Your closure should look like this:
    /// ```
    /// use ez_term::*;
    /// let my_closure = |context: EzContext| { };
    /// ```
    /// Or if you prefer an explicit function:
    /// ```
    /// use ez_term::*;
    /// fn my_func(context: EzContext) { };
    /// ```
    /// Closures are recommended for most use cases, as they allow capturing variables. You can
    /// use an interval [GenericEzTask] in many ways:
    /// # Changing a widget state
    /// We will make a counter that counts from 1 to 10. Each time it increments, it will update
    /// a label with the new value. At 10 it will stop the recurring task by returning 'false'.
    /// ```
    /// use ez_term::*;
    /// use std::time::Duration;
    /// let increment: usize = 1;
    /// let my_counter_func = move |context: EzContext| {
    ///     let state = context.state_tree.get_by_id_mut("my_label").as_label_mut();
    ///     state.set_text(format!("Counting {}", increment));
    ///     state.update(context.scheduler);
    ///     return if increment <= 10 {
    ///         true
    ///     } else {
    ///         false
    ///     }
    /// };
    /// scheduler.schedule_interval("my_label".to_string(), Box::new(my_counter), Duration::from_secs(1))
    /// ```
    pub fn schedule_interval(&mut self, widget: String,  func: GenericEzTask, interval: Duration)
        -> &mut Task {
        let task = Task::new(widget, func, true, interval, None);
        self.backend.tasks.push(task);
        self.backend.tasks.last_mut().unwrap()
    }

    /// Schedule to immediately run a function or closure in a background thread. Use this for
    /// any kind of task that is longer running. If you are making a UI for your app, this is where
    /// you would start running your app. The first argument is the function to run in a background
    /// thread, the second argument is an optional function to execute when the thread finishes.
    /// # The background thread function:
    /// The closure to run in the background should have the following signature:
    /// ```
    /// use ez_term::*;
    /// let my_closure = |properties: EzPropertiesMap| { };
    /// ```
    /// Or if you prefer an explicit function:
    /// ```
    /// use ez_term::*;
    /// fn my_func(properties: EzPropertiesMap) { };
    /// ```
    /// # The on_finish callback function:
    /// The second on_finish argument can be None, or a closure with the following signature:
    /// ```
    /// use ez_term::*;
    /// let my_closure = |context: EzContext| { };
    /// ```
    /// Or if you prefer an explicit function:
    /// ```
    /// use ez_term::*;
    /// fn my_func(context: EzContext) { };
    /// ```
    /// # Example:
    /// ```
    /// // Let's define a mock app. This represents our app that we're building a UI for.
    /// use ez_term::*;
    /// use std::time::Duration;
    /// fn progress_example_app(mut properties: EzPropertiesMap) {
    ///
    ///     for x in 1..10 {
    ///         std::thread::sleep(Duration::from_secs(1))
    ///     };
    /// }
    /// // Now we will run it in a background thread.
    /// scheduler.schedule_threaded(Box::new(progress_example_app), None);
    /// ```
    ///
    pub fn schedule_threaded(&mut self, threaded_func: Box<dyn FnOnce(EzPropertiesMap) + Send>,
                             on_finish: Option<GenericEzTask>) {

        self.backend.threads_to_start.push((threaded_func, on_finish));
    }

    /// Open a template defined in an .ez file as a popup.
    /// # Example
    /// We will first define a popup template in the .ez file, and then spawn it in the UI.
    /// ```
    /// - <MyPopup@Layout>:
    ///     id: my_popup
    ///     mode: float
    ///     size_hint: 0.5, 0.5
    ///     border: true
    ///     pos_hint: center, middle
    ///     - Label:
    ///         text: This is a test popup.
    ///         auto_scale: true, true
    ///         pos_hint: center, top
    ///     - Button:
    ///         id: dismiss_button
    ///         selection_order: 1
    ///         text: Dismiss
    ///         size_hint_x: 1
    ///         auto_scale_height: true
    ///         pos_hint: center, bottom
    /// ```
    /// ```
    /// use ez_term::*;
    /// // We will open the popup, which gives us the path of the spawned popup
    /// let popup_path = scheduler.open_popup("TestPopup".to_string(), context.state_tree);
    /// // Now we will bind the dismiss button we defined to a dismiss callback
    /// let dismiss =
    /// move |context: EzContext| {
    ///     let state =
    ///         context.state_tree.get_by_path_mut("/root").as_layout_mut();
    ///     state.dismiss_modal(context.scheduler);
    ///     state.update(context.scheduler);
    ///     false
    /// };
    /// scheduler.update_callback_config(format!("{}/dismiss_button", popup_path).as_str(),
    ///                                  CallbackConfig::from_on_press(Box::new(dismiss_delay)));
    /// // The popup will open on the next frame!
    /// ```
    pub fn open_popup(&mut self, template: String, state_tree: &mut StateTree) -> String {
        open_popup(template, state_tree, self)
    }

    /// Replace the entire [CallbackConfig] of a widget with a new one. Unless you want to erase
    /// all current callbacks for a widget, use [update_callback_config] instead.
    /// The first argument is the ID or the Path of the widget, the second argument is the new
    /// [CallbackConfig].
    /// # Example
    /// We will write a callback for a button which disabled the button when pressed. We then
    /// replace the callback config for the button with a new one containing the new callback.
    /// ```
    /// use ez_term::*;
    /// let my_callback = |context: EzContext| {
    ///     let state = context.state_tree.get_by_id_mut("my_button").as_button_mut();
    ///     state.set_disabled(true);
    ///     state.update(context.scheduler);
    ///     true
    /// };
    /// scheduler.set_callback_config("my_button",
    ///                               CallbackConfig::from_on_press(Box::new(dismiss_delay)));
    /// ```
    pub fn set_callback_config(&mut self, for_widget: &str, callback_config: CallbackConfig) {
        self.backend.new_callback_configs.push((for_widget.to_string(), callback_config));
    }

    /// Update the [CallbackConfig] of a widget. The first argument is the ID or the Path of the
    /// widget, the second argument is the [CallbackConfig] used to update. Any callback set in the
    /// passed [CallbackConfig] will be set on the existing one, overwriting the previous callback
    /// if necessary. Callbacks left empty in the passed [CallbackConfig] will be ignored, they will
    /// *not* cause exising callbacks to be removed.
    /// # Example
    /// We will write a callback for a button which disabled the button when pressed. We then
    /// update the callback config for the button with the new callback.
    /// ```
    /// use ez_term::*;
    /// let my_callback = |context: EzContext| {
    ///     let state = context.state_tree.get_by_id_mut("my_button").as_button_mut();
    ///     state.set_disabled(true);
    ///     state.update(context.scheduler);
    ///     true
    /// };
    /// scheduler.update_callback_config("my_button",
    ///                                  CallbackConfig::from_on_press(Box::new(dismiss_delay)));
    /// ```
    pub fn update_callback_config(&mut self, for_widget: &str, callback_config: CallbackConfig) {
        self.backend.updated_callback_configs.push((for_widget.to_string(), callback_config));

    }

    pub fn create_widget(&mut self, widget_type: &str, id: &str, path: &str) -> &mut EzState {

        let new_path = format!("{}/{}", path, id);
        let base_type;
        let new_state;
        if self.backend.templates.contains_key(widget_type) {
            base_type = self.backend.templates
                .get(widget_type).unwrap().resolve_base_type(&self.backend.templates);
            new_state = self.backend.templates.get_mut(widget_type).unwrap()
                .clone().parse(self, path.to_string(), 0,
                               Some(vec!(format!("id: {}", id))))
                .as_ez_object_mut().get_state();
        } else {
            base_type = widget_type.to_string();
            new_state = EzState::from_string(
                &base_type, path.to_string(), self);
        };

        self.backend.widgets_to_create.push((new_path, base_type, new_state));
        &mut self.backend.widgets_to_create.last_mut().unwrap().2

    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribe to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to this property will update automatically.
    pub fn new_usize_property(&mut self, name: &str, value: usize) -> EzProperty<usize> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(name.to_string(),
                                       EzProperties::Usize(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribe to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to this property will update automatically.
    pub fn new_string_property(&mut self, name: &str, value: String) -> EzProperty<String> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(name.to_string(),
                                       EzProperties::String(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribe to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to this property will update automatically.
    pub fn new_bool_property(&mut self, name: &str, value: bool) -> EzProperty<bool> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(name.to_string(),
                                       EzProperties::Bool(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribe to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to this property will update automatically.
    pub fn new_color_property(&mut self, name: &str, value: Color) -> EzProperty<Color> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(name.to_string(),
                                       EzProperties::Color(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribe to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to this property will update automatically.
    pub fn new_vertical_alignment_property(&mut self, name: &str, value: VerticalAlignment)
        -> EzProperty<VerticalAlignment> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(name.to_string(),
                                       EzProperties::VerticalAlignment(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribe to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to this property will update automatically.
    pub fn new_horizontal_alignment_property(&mut self, name: &str, value: HorizontalAlignment)
        -> EzProperty<HorizontalAlignment> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(name.to_string(),
                                       EzProperties::HorizontalAlignment(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribe to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to this property will update automatically.
    pub fn new_horizontal_pos_hint_property(
        &mut self, name: &str, value: Option<(HorizontalAlignment, f64)>)
        -> EzProperty<Option<(HorizontalAlignment, f64)>> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(name.to_string(),
                                       EzProperties::HorizontalPosHint(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribe to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to this property will update automatically.
    pub fn new_vertical_pos_hint_property(
        &mut self, name: &str, value: Option<(VerticalAlignment, f64)>)
        -> EzProperty<Option<(VerticalAlignment, f64)>> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(name.to_string(),
                                       EzProperties::VerticalPosHint(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribe to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to this property will update automatically.
    pub fn new_size_hint_property(&mut self, name: &str, value: Option<f64>)
        -> EzProperty<Option<f64>> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(name.to_string(),
                                       EzProperties::SizeHint(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        property
    }

    /// Subscribe one property to another, ensuring the subscriber will always have the value of the
    /// property it is subscribed to on the next frame. An update func is required which will be
    /// called when the property subscribed to changes. The update func receives the new value and
    /// is responsible for setting the appropriate field on the subscriber.
    pub fn subscribe_to_ez_property(&mut self, name: &str, update_func: EzPropertyUpdater) {

        if !self.backend.property_subscribers.contains_key(name) {
            self.backend.property_subscribers.insert(name.to_string(), Vec::new());
        }
        self.backend.property_subscribers.get_mut(name).unwrap().push(update_func);
    }

    /// Schedule a widget to be updated on the next frame. Can also be called from the widget itself
    /// as ```[widget.update(scheduler)]``` (for convenience).
    pub fn update_widget(&mut self, path: String) {
        if path.starts_with("/modal") {
            self.backend.force_redraw = true;
            return
        }
        if !self.backend.widgets_to_update.contains(&path) {
            self.backend.widgets_to_update.push(path);
        }
    }

    /// Schedule a full screen redraw on the next frame. [get_contents] will be called on the root
    /// widget and drawn to screen. Only changed pixels are actually drawn as an optimization.
    pub fn force_redraw(&mut self) { self.backend.force_redraw = true; }

    /// Bind a callback function to the changing of an EzProperty. Make sure that the function you
    /// create has the right signature, e.g.:
    /// ```
    /// use ez_term::*;
    /// let my_property = scheduler.new_usize_property("my_property".to_string(), 0);
    ///
    /// let my_callback = |context: EzContext| {
    ///     let state  = context.state_tree.get_by_id("my_label");
    ///     state.set_text("Value changed");
    ///     state.update(context.scheduler);
    /// };
    ///
    /// scheduler.bind_ez_property_callback("my_property", Box::new(my_callback));
    /// ```
    pub fn bind_ez_property_callback(&mut self, name: &str, callback: Box<dyn FnMut(EzContext)>) {

        let callbacks =
            self.backend.property_callbacks.entry(name.to_string()).or_insert(Vec::new());
        callbacks.push(callback);
    }
    
    /// Set the passed widget (can be ID or path) as selected. Automatically deselects the current
    /// selection if any, and calls the appropriate callbacks.
    pub fn set_selected_widget(&mut self, widget: &str, mouse_pos: Option<Coordinates>) {
        self.backend.next_selection = Some((widget.to_string(), mouse_pos));
    }

    /// Set the passed widget (can be ID or path) as selected. Automatically deselects the current
    /// selection if any, and calls the appropriate callbacks.
    pub fn deselect_widget(&mut self) { self.backend.deselect = true }

    /// Gracefully exit the app.
    pub fn exit(&self) { stop(); }
}


/// See [SchedulerFrontend] for more info.
#[derive(Default)]
pub struct Scheduler {

    /// List of widgets that will be redrawn on the next frame. Don't use this directly, use
    /// [update_widget] or EzState.update instead.
    pub widgets_to_update: Vec<String>,

    /// If true, the entire screen will be redrawn on the next frame. Only differences compared to
    /// the previous frame. Use [force_redraw] to set this.
    pub force_redraw: bool,

    /// A <Name, [EzProperty]> HashMap to keep track of all properties at runtime. Also passed to
    /// closures that are spawned in background threads, as [EzProperty] is thread-safe.
    pub properties: HashMap<String, EzProperties>,

    /// A list of scheduled tasks. Can be run-once or interval tasks. These are checked on every
    /// frame, and ran if the 'after' Duration has passed for run-once tasks, or after the 'interval'
    /// duration has passed for interval tasks.
    pub tasks: Vec<Task>,

    /// List of <Function, Optional on_finish callback>. This list is checked every frame. If
    /// theres an item in here, it will be used to spawn a background thread based on the passed
    /// function. Once the function is finished running, the optional callback will be executed if
    /// there was one.
    pub threads_to_start: Vec<(EzThread, Option<GenericEzTask>)>,

    /// List of thread handles with optional callbacks. If a thread is finished running, the
    /// JoinHandle is joined and the callback executed if there was any.
    pub thread_handles: Vec<(JoinHandle<()>, Option<GenericEzTask>)>,

    /// Templates defined in the .ez files. Used by [create_widget]
    pub templates: Templates,

    /// List of new widgets that will be created on the next frame. Use [create_widget] for this.
    pub widgets_to_create: Vec<(String, String, EzState)>,

    /// List of <Widget path, [CallbackConfig]. Every frame this list is checked, and the widget
    /// belonging to the widget path will have its' [CallbackConfig] replaced with the new one.
    pub new_callback_configs: Vec<(String, CallbackConfig)>,

    /// List of <Widget path, [CallbackConfig]. Every frame this list is checked, and the widget
    /// belonging to the widget path will have its' [CallbackConfig] updated with the new one,
    /// meaning any callbacks that are unset in the old config, but set in the new one, will be
    /// updated. Callbacks are never deleted, but will be overwritten.
    pub updated_callback_configs: Vec<(String, CallbackConfig)>,

    /// A <Widget path, Receiver> HashMap, used to get the receiver of an EzProprerty channel.
    /// New values are received on this receiver and then synced to any subsribed properties.
    pub property_receivers: HashMap<String, Receiver<EzValues>>,

    /// A <Widget path, update_callback> HashMap, used to get the updater callback EzProprerty.
    /// When a property subsribes to another, it must provide an updater callback. When the value
    /// changes, the callback will be called with the new value, and is responsible for syncing the
    /// subscribing property to the new value.
    pub property_subscribers: HashMap<String, Vec<EzPropertyUpdater>>,

    /// A <Widget path, user_callback> HashMap, used to store callbacks the user has registered to
    /// the changing of a value of an [EzProperty]. When the value changes, the callback is called.
    pub property_callbacks: HashMap<String, Vec<Box<dyn FnMut(EzContext)>>>,

    /// The widget (ID or path) set here will be selected on the next frame, deselecting the current
    /// selection if any and calling the appropriate callbacks. The optional mouse_position will be
    /// passed to the on_select callback.
    pub next_selection: Option<(String, Option<Coordinates>)>,

    /// If true, deselect the currently selection widget (if any) on the next frame.
    pub deselect: bool,
}

/// A struct representing a run-once- or recurring task. This struct is not directly used by the
/// end-user, but created when they schedule a task through the [Scheduler].
pub struct Task {

    /// The widget for which this task was scheduled. Convenience property as this is passed to the
    /// task callback func, allowed the user to easily retrieve the state of the widget.
    pub widget: String,

    /// Function that will be called when this task is executed.
    pub func: GenericEzTask,

    /// This task is run-once if this is false, in which case it will be executed if the 'after'
    /// Duration has passed. This task is recurring if this is true, in which case it will be
    /// executed after the 'interval' Duration has passed, and scheduled again as long as the
    /// function keeps returning true.
    pub recurring: bool,

    /// Task will be dropped if this is true. Mainly used to cancel a run-once task before it is
    /// executed.
    pub canceled: bool,

    /// The schedule on which a recurring task must run.
    pub interval: Duration,

    /// Last time this task was executed, used to keep track of when it should run next.
    pub last_execution: Option<Instant>,
}

impl Task {

    pub fn new(widget: String, func: GenericEzTask, recurring: bool,
               interval: Duration, last_execution: Option<Instant>)
        -> Self { Task { widget, func, recurring, interval, canceled: false, last_execution } }

    pub fn cancel(&mut self) { self.canceled = true; }

}