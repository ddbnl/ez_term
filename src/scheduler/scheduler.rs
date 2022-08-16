//! # Scheduler
//!
//! A module implementing the Scheduler struct.
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::thread::{JoinHandle};
use std::time::{Duration, Instant};
use crossterm::event::{KeyCode, KeyModifiers};

use crossterm::style::Color;

use crate::{CallbackConfig, EzPropertiesMap};
use crate::parser::ez_definition::Templates;
use crate::property::ez_properties::EzProperties;
use crate::property::ez_property::EzProperty;
use crate::property::ez_values::EzValues;
use crate::run::definitions::{Coordinates, StateTree};
use crate::run::run::{open_and_register_modal, stop};
use crate::scheduler::definitions::{EzPropertyUpdater, EzThread, GenericFunction, GenericRecurringTask, GenericTask,
                                    KeyboardCallbackFunction};
use crate::states::definitions::{create_keymap_modifiers, HorizontalAlignment, HorizontalPosHint, LayoutMode, LayoutOrientation, VerticalAlignment, VerticalPosHint};
use crate::states::ez_state::EzState;
use crate::widgets::ez_object::EzObjects;


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
    /// any functions that will not return immediately use [schedule_threaded].
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
    ///     let state = context.state_tree.get_mut("my_label").as_label_mut();
    ///     state.set_text("New text!".to_string());
    ///     state.update(context.scheduler);
    /// };
    /// scheduler.schedule_once("my_task", Box::new(my_closure), Duration::from_secs(0));
    /// ```
    /// Note that we used the 'move' keyword to move our counting variable into our task.
    pub fn schedule_once(&mut self, name: &str, func: GenericTask, after: Duration) {

        let task = Task::new(name.to_string(), func,after);
        self.backend.tasks.push(task);
    }

    /// Cancel a run-once task by name.
    /// ```
    /// scheduler.cancel_task("my_task")
    /// ```
    pub fn cancel_task(&mut self, name: &str) {

        let mut to_cancel = None;
        for (i, task) in self.backend.tasks.iter().enumerate() {
            if &task.name == name {
                to_cancel = Some(i);
            }
        }
        if let Some(i) = to_cancel {
            self.backend.tasks.remove(i);
        }
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
    ///
    /// let mut counter: usize = 1;
    /// let my_counter_func = move |context: EzContext| {
    ///     let state = context.state_tree.get_mut("my_label").as_label_mut();
    ///     state.set_text(format!("Counting {}", counter));
    ///     state.update(context.scheduler);
    ///     counter += 1;
    ///     return if counter <= 10 {
    ///         true
    ///     } else {
    ///         false
    ///     };
    /// };
    /// scheduler.schedule_recurring("my_task", Box::new(my_counter), Duration::from_secs(1))
    /// ```
    pub fn schedule_recurring(&mut self, name: &str, func: GenericRecurringTask, interval: Duration) {
        let task = RecurringTask::new(name.to_string(), func, interval);
        self.backend.recurring_tasks.push(task);
    }

    /// Cancel a recurring task by name.
    /// ```
    /// scheduler.cancel_recurring_task("my_task")
    /// ```
    pub fn cancel_recurring_task(&mut self, name: &str) {

        let mut to_cancel = None;
        for (i, task) in self.backend.recurring_tasks.iter().enumerate() {
            if &task.name == name {
                to_cancel = Some(i);
            }
        }
        if let Some(i) = to_cancel {
            self.backend.recurring_tasks.remove(i);
        }
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
    /// fn progress_example_app(mut properties: EzPropertiesMap, state_tree: StateTree) {
    ///
    ///     for x in 1..10 {
    ///         std::thread::sleep(Duration::from_secs(1))
    ///     };
    /// }
    /// // Now we will run it in a background thread.
    /// scheduler.schedule_threaded(Box::new(progress_example_app), None);
    /// ```
    ///
    pub fn schedule_threaded(
        &mut self, threaded_func: Box<dyn FnOnce(EzPropertiesMap, StateTree) + Send>,
        on_finish: Option<GenericTask>) {

        self.backend.threads_to_start.push((threaded_func, on_finish));
    }

    /// Open a template defined in an .ez file as a popup.
    /// # Example
    /// We will first define a popup template in the .ez file, and then spawn it in the UI.
    /// ```
    /// - <MyPopup@Layout>:
    ///     mode: float
    ///     size_hint: 0.5, 0.5
    ///     pos_hint: center, middle
    ///     border: true
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
    /// scheduler.open_popup("TestPopup".to_string(), context.state_tree);
    /// // Now we will bind the dismiss button we defined to a dismiss callback
    /// let dismiss = |context: EzContext| {
    ///
    ///     context.scheduler.dismiss_modal(context.state_tree);
    ///     false
    /// };
    /// scheduler.update_callback_config("dismiss_button",
    ///                                  CallbackConfig::from_on_press(Box::new(dismiss)));
    /// // The popup will open on the next frame!
    /// ```
    pub fn open_modal(&mut self, template: &str, state_tree: &mut StateTree) {
        open_and_register_modal(template.to_string(), state_tree, self);
    }

    /// Dismiss the current modal.
    pub fn dismiss_modal(&mut self, state_tree: &mut StateTree) {
        state_tree.as_layout_mut().dismiss_modal(self);
        let removed = state_tree.remove_node("/modal".to_string());
        for state in removed.get_all() {
            state.as_generic().clean_up_properties(self);
        }
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
    ///     let state = context.state_tree.get_mut("my_button").as_button_mut();
    ///     state.set_disabled(true);
    ///     state.update(context.scheduler);
    ///     true
    /// };
    /// scheduler.overwrite_callback_config("my_button",
    ///                               CallbackConfig::from_on_press(Box::new(my_callback)));
    /// ```
    pub fn overwrite_callback_config(&mut self, for_widget: &str, callback_config: CallbackConfig) {
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
    ///     let state = context.state_tree.get_mut("my_button").as_button_mut();
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

    /// Create a new widget from code. After calling this method, the widget will appear on the
    /// next frame. The state of the new widget will be available in the state tree immediately
    /// after calling this method. The parameters are:
    /// Widget_type (&str): name of the widget or template to create
    /// Id (&str): ID of the new widget
    /// Parent (&str): Path or ID of the parent layout to create this widget in
    /// State_tree (&mut StateTree): the state tree root.
    ///
    /// # Example
    ///
    /// Here is an example spawning mock Sql records in a layout. First the .ez file:
    /// ```
    /// - Layout:
    ///     mode: box
    ///     orientation: vertical
    ///     - Label:
    ///         text: Retrieved SQL records:
    ///         auto_scale: true, true
    ///     - Layout:
    ///         id: sql_records_layout
    ///         mode: box
    ///         orientation: vertical
    ///
    /// - <SqlRecord@Layout>:
    ///     mode: box
    ///     orientation: horizontal
    ///     - Label:
    ///         id: record_id
    ///         auto_scale: true, true
    ///     - Label:
    ///         id: record_name
    ///         auto_scale: true, true
    ///     - Label:
    ///         id: record_date
    ///         auto_scale: true, true
    /// ```
    /// The code:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let sql_records = get_sql_records();
    /// let parent_id = "sql_records_layout";
    /// let template_name = "SqlRecord";
    ///
    /// for (i, sql_record) in sql_records.iter().enumerate() {
    ///
    ///     let new_id = format!("record_{}", i).as_str();
    ///     scheduler.create_widget(template_name, new_id, parent_id, &mut state_tree);
    ///
    ///     let new_record_widget = state_tree.get_mut(new_id);
    ///     new_record_widget.get("record_id").as_label_mut().set_text(sql_record.id);
    ///     new_record_widget.get("record_name").as_label_mut().set_text(sql_record.name);
    ///     new_record_widget.get("record_date").as_label_mut().set_text(sql_record.date);
    ///
    /// }
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn create_widget(&mut self, widget_type: &str, id: &str, parent: &str,
                         state_tree: &mut StateTree)  {

        let path = if !parent.contains('/') { // parent is an ID, resolve it
            state_tree.get(parent).as_generic().get_path().clone()
        } else {
            parent.to_string()
        };
        let new_path = format!("{}/{}", path, id);
        let base_type;
        let new_widget;
        if self.backend.templates.contains_key(widget_type) {
            new_widget = self.backend.templates.get_mut(widget_type).unwrap()
                .clone().parse(self, path.to_string(), 0,
                               Some(vec!(format!("id: {}", id))));
            state_tree.add_node(new_path, new_widget.as_ez_object().get_state());
            if let EzObjects::Layout(ref i) = new_widget {
                for child in i.get_widgets_recursive() {
                    state_tree.add_node(child.as_ez_object().get_path(),
                                        child.as_ez_object().get_state());
                }
            }
        } else {
            base_type = widget_type.to_string();
            let new_state = EzState::from_string(&base_type, new_path.to_string(),
                                                 self);
            new_widget = EzObjects::from_string(
                &base_type, new_path.to_string(), id.to_string(), self, new_state);

        };
        self.backend.widgets_to_create.push(new_widget);

    }

    /// Remove a widget on the next frame. Pass the path or ID of the widget to remove.
    pub fn remove_widget(&mut self, name: &str) {

        if name == "root" || name == "/root" {
            panic!("Cannot remove the root layout")
        } else if name == "modal" || name == "/root/modal" {
            panic!("Cannot remove modal widget; use scheduler.dismiss_modal instead.")
        }
        self.backend.widgets_to_remove.push(name.to_string());
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribe to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to this property will update automatically.
    pub fn new_usize_property(&mut self, name: &str, value: usize) -> EzProperty<usize> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(
            name.to_string(),(EzProperties::Usize(property.clone()), receiver));
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribe to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to this property will update automatically.
    pub fn new_f64_property(&mut self, name: &str, value: f64) -> EzProperty<f64> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(
            name.to_string(),(EzProperties::F64(property.clone()), receiver));
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
        self.backend.properties.insert(
            name.to_string(),(EzProperties::String(property.clone()), receiver));
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
        self.backend.properties.insert(
            name.to_string(),(EzProperties::Bool(property.clone()), receiver));
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
        self.backend.properties.insert(
            name.to_string(),(EzProperties::Color(property.clone()), receiver));
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribe to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to this property will update automatically.
    pub fn new_layout_mode_property(&mut self, name: &str, value: LayoutMode)
                                           -> EzProperty<LayoutMode> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(
            name.to_string(),(EzProperties::LayoutMode(property.clone()), receiver));
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribe to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to this property will update automatically.
    pub fn new_layout_orientation_property(&mut self, name: &str, value: LayoutOrientation)
                                    -> EzProperty<LayoutOrientation> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(
            name.to_string(),(EzProperties::LayoutOrientation(property.clone()), receiver));
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
        self.backend.properties.insert(
            name.to_string(),(EzProperties::VerticalAlignment(property.clone()), receiver));
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
        self.backend.properties.insert(
            name.to_string(),(EzProperties::HorizontalAlignment(property.clone()), receiver));
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribe to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to this property will update automatically.
    pub fn new_horizontal_pos_hint_property(
        &mut self, name: &str, value: HorizontalPosHint) -> EzProperty<HorizontalPosHint> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(
            name.to_string(),(EzProperties::HorizontalPosHint(property.clone()), receiver));
        property
    }

    /// Register a new property and return it. After a property has been registered it's possible
    /// for widget properties to subscribe to it, meaning their values will be kept in sync. If
    /// you want to bind a value in your app to a widget property, use this func to get a property
    /// and pass it to your app. Then use property.set() to update it. Any widget properties bound
    /// to this property will update automatically.
    pub fn new_vertical_pos_hint_property(
        &mut self, name: &str, value: VerticalPosHint) -> EzProperty<VerticalPosHint> {

        let (property, receiver) =
            EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(
            name.to_string(),(EzProperties::VerticalPosHint(property.clone()), receiver));
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
        self.backend.properties.insert(
            name.to_string(),(EzProperties::SizeHint(property.clone()), receiver));
        property
    }

    /// Retrieve a property. Should be used to retrieve custom properties; access widget properties
    /// through the state_tree.
    pub fn get_property(&self, name: &str) -> &EzProperties {
        &self.backend.properties.get(name).unwrap_or_else(
            || panic!("Could not find property: {}", name)
        ).0
    }

    /// Retrieve a property. Should be used to retrieve custom properties; access widget properties
    /// through the state_tree.
    pub fn get_property_mut(&mut self, name: &str) -> &mut EzProperties {
        &mut self.backend.properties.get_mut(name).unwrap_or_else(
            || panic!("Could not find property: {}", name)
        ).0
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
    pub fn update_widget(&mut self, path: &str) {
        if path.starts_with("/root/modal") {
            self.backend.force_redraw = true;
            return
        }
        let path = path.to_string();
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
    ///     let state  = context.state_tree.get("my_label").as_label_mut();
    ///     state.set_text("Value changed".to_string());
    ///     state.update(context.scheduler);
    /// };
    ///
    /// scheduler.bind_ez_property_callback("my_property", Box::new(my_callback));
    /// ```
    pub fn bind_property_callback(&mut self, name: &str, callback: GenericFunction) {

        if self.backend.property_callbacks.contains(&name.to_string()) {
            self.backend.new_property_callbacks.push((name.to_string(), callback));
        } else {
            let mut config = CallbackConfig::default();
            config.property_callbacks.push(callback);
            let name = if !name.contains('/') { format!("/root/{}", name)
            } else { name.to_string() };
            self.overwrite_callback_config(&name, config);
            self.backend.property_callbacks.push(name);
        }
    }

    /// Globally bind a keyboard key (CrossTerm [KeyCode]). These keybinds take priority over all
    /// others and work in all contexts.
    pub fn bind_global_key(&mut self, key: KeyCode, modifiers: Option<Vec<KeyModifiers>>,
                           callback: KeyboardCallbackFunction) {

        let modifiers = create_keymap_modifiers(modifiers);
        self.backend.update_global_keymap.insert((key, modifiers), callback);
    }

    /// Remove a single global keybind by keycode.
    pub fn remove_global_key(&mut self, key: KeyCode, modifiers: Option<Vec<KeyModifiers>>) {

        let modifiers = create_keymap_modifiers(modifiers);
        self.backend.remove_global_keymap.push((key, modifiers));
    }

    /// Remove all custom global keybinds currently active.
    pub fn clear_global_keys(&mut self) {
        self.backend.clear_global_keymap = true;
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
    pub properties: HashMap<String, (EzProperties, Receiver<EzValues>)>,

    /// A list of scheduled tasks. These are checked on every frame, and ran if the 'after'
    /// delay has passed.
    pub tasks: Vec<Task>,

    /// A list of scheduled recurring tasks.These are checked on every frame, and ran after the
    /// 'interval' duration has passed.
    pub recurring_tasks: Vec<RecurringTask>,

    /// List of <Function, Optional on_finish callback>. This list is checked every frame. If
    /// theres an item in here, it will be used to spawn a background thread based on the passed
    /// function. Once the function is finished running, the optional callback will be executed if
    /// there was one.
    pub threads_to_start: Vec<(EzThread, Option<GenericTask>)>,

    /// List of thread handles with optional callbacks. If a thread is finished running, the
    /// JoinHandle is joined and the callback executed if there was any.
    pub thread_handles: Vec<(JoinHandle<()>, Option<GenericTask>)>,

    /// Templates defined in the .ez files. Used by [create_widget]
    pub templates: Templates,

    /// List of new widgets that will be created on the next frame. Use [create_widget] for this.
    pub widgets_to_create: Vec<EzObjects>,

    /// List of new widgets that will be removed on the next frame. Use [remove_widget] for this.
    pub widgets_to_remove: Vec<String>,

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
    pub property_callbacks: Vec<String>,

    /// Every frame these property callbacks are registered. Allows user to add property callbacks
    /// without having access to the CallbackTree.
    pub new_property_callbacks: Vec<(String, GenericFunction)>,

    /// The widget (ID or path) set here will be selected on the next frame, deselecting the current
    /// selection if any and calling the appropriate callbacks. The optional mouse_position will be
    /// passed to the on_select callback.
    pub next_selection: Option<(String, Option<Coordinates>)>,

    /// If true, deselect the currently selection widget (if any) on the next frame.
    pub deselect: bool,

    /// KeyMap containing global keybinds. Keys bound here take priority over widget keymaps, and
    /// work in all contexts.
    pub update_global_keymap: HashMap<(KeyCode, KeyModifiers), KeyboardCallbackFunction>,
    
    /// KeyCodes in this vec will be removed from the global keymap on the next frame.
    pub remove_global_keymap: Vec<(KeyCode, KeyModifiers)>,
    
    /// Entire global keymap will be cleared on the next frame.
    pub clear_global_keymap: bool,
}

/// A struct representing a run-once. This struct is not directly used by the
/// end-user, but created when they schedule a task through the [Scheduler].
pub struct Task {

    /// Name which can be used to cancel the task
    pub name: String,

    /// Function that will be called when this task is executed.
    pub func: GenericTask,

    /// Task will be dropped if this is true. Mainly used to cancel a run-once task before it is
    /// executed.
    pub canceled: bool,

    /// The schedule on which a recurring task must run.
    pub delay: Duration,

    pub created: Instant,
}

impl Task {

    pub fn new(name: String, func: GenericTask, delay: Duration)
               -> Self { Task { name, func, delay, canceled: false, created: Instant::now() } }

    pub fn cancel(&mut self) { self.canceled = true; }

}
/// A struct representing a run-once- or recurring task. This struct is not directly used by the
/// end-user, but created when they schedule a task through the [Scheduler].
pub struct RecurringTask {

    /// Name which can be used to cancel the task
    pub name: String,

    /// Function that will be called when this task is executed.
    pub func: GenericRecurringTask,

    /// Task will be dropped if this is true. Mainly used to cancel a run-once task before it is
    /// executed.
    pub canceled: bool,

    /// The schedule on which a recurring task must run.
    pub interval: Duration,

    /// Last time this task was executed, used to keep track of when it should run next.
    pub last_execution: Option<Instant>,
}

impl RecurringTask {

    pub fn new(name: String, func: GenericRecurringTask, interval: Duration)
               -> Self { RecurringTask { name, func, interval, canceled: false,
        last_execution: None } }

    pub fn cancel(&mut self) { self.canceled = true; }

}