//! # Scheduler
//!
//! A module implementing the Scheduler struct.
use std::collections::HashMap;
use std::mem::swap;
use std::sync::mpsc::{channel, Receiver, Sender};
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
use crate::scheduler::definitions::{EzPropertyUpdater, EzThread, GenericFunction,
                                    GenericRecurringTask, GenericTask,
                                    KeyboardCallbackFunction};
use crate::states::definitions::{create_keymap_modifiers, HorizontalAlignment, HorizontalPosHint,
                                 LayoutMode, LayoutOrientation, VerticalAlignment, VerticalPosHint};
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
#[derive(Default)]
pub struct SchedulerFrontend {

    /// Backend of the scheduler. Do not use. Use the public funcs instead.
    pub backend: Scheduler,

    /// How many synced frontends are running
    syncing: usize,

    /// Whether this is a synced frontend (i.e. running in a thread)
    synced: bool,

    schedule_once_sender: Option<Sender<(String, GenericTask, Duration)>>,
    schedule_once_receiver: Option<Receiver<(String, GenericTask, Duration)>>,

    schedule_recurring_sender: Option<Sender<(String, GenericRecurringTask, Duration)>>,
    schedule_recurring_receiver: Option<Receiver<(String, GenericRecurringTask, Duration)>>,

    schedule_threaded_sender: Option<Sender<(EzThread, Option<GenericTask>)>>,
    schedule_threaded_receiver: Option<Receiver<(EzThread, Option<GenericTask>)>>,

    cancel_task_sender: Option<Sender<String>>,
    cancel_task_receiver: Option<Receiver<String>>,

    cancel_recurring_sender: Option<Sender<String>>,
    cancel_recurring_receiver: Option<Receiver<String>>,

    open_modal_sender: Option<Sender<String>>,
    open_modal_receiver: Option<Receiver<String>>,

    dismiss_modal_sender: Option<Sender<bool>>,
    dismiss_modal_receiver: Option<Receiver<bool>>,

    overwrite_callback_config_sender: Option<Sender<(String, CallbackConfig)>>,
    overwrite_callback_config_receiver: Option<Receiver<(String, CallbackConfig)>>,

    update_callback_config_sender: Option<Sender<(String, CallbackConfig)>>,
    update_callback_config_receiver: Option<Receiver<(String, CallbackConfig)>>,

    create_widget_sender: Option<Sender<(EzObjects, StateTree)>>,
    create_widget_receiver: Option<Receiver<(EzObjects, StateTree)>>,
    new_properties_sender: Option<Sender<HashMap<String, EzProperties>>>,
    new_properties_receiver: Option<Receiver<HashMap<String, EzProperties>>>,
    new_receivers_sender: Option<Sender<HashMap<String, Receiver<EzValues>>>>,
    new_receivers_receiver: Option<Receiver<HashMap<String, Receiver<EzValues>>>>,

    remove_widget_sender: Option<Sender<String>>,
    remove_widget_receiver: Option<Receiver<String>>,

    new_usize_property_sender: Option<Sender<(String, usize)>>,
    new_usize_property_receiver: Option<Receiver<(String, usize)>>,

    new_f64_property_sender: Option<Sender<(String, f64)>>,
    new_f64_property_receiver: Option<Receiver<(String, f64)>>,

    new_string_property_sender: Option<Sender<(String, String)>>,
    new_string_property_receiver: Option<Receiver<(String, String)>>,

    new_bool_property_sender: Option<Sender<(String, bool)>>,
    new_bool_property_receiver: Option<Receiver<(String, bool)>>,

    new_color_property_sender: Option<Sender<(String, Color)>>,
    new_color_property_receiver: Option<Receiver<(String, Color)>>,

    new_layout_mode_property_sender: Option<Sender<(String, LayoutMode)>>,
    new_layout_mode_property_receiver: Option<Receiver<(String, LayoutMode)>>,

    new_layout_orientation_property_sender: Option<Sender<(String, LayoutOrientation)>>,
    new_layout_orientation_property_receiver: Option<Receiver<(String, LayoutOrientation)>>,

    new_horizontal_alignment_property_sender: Option<Sender<(String, HorizontalAlignment)>>,
    new_horizontal_alignment_property_receiver: Option<Receiver<(String, HorizontalAlignment)>>,

    new_vertical_alignment_property_sender: Option<Sender<(String, VerticalAlignment)>>,
    new_vertical_alignment_property_receiver: Option<Receiver<(String, VerticalAlignment)>>,

    new_horizontal_pos_hint_property_sender: Option<Sender<(String, HorizontalPosHint)>>,
    new_horizontal_pos_hint_property_receiver: Option<Receiver<(String, HorizontalPosHint)>>,

    new_vertical_pos_hint_property_sender: Option<Sender<(String, VerticalPosHint)>>,
    new_vertical_pos_hint_property_receiver: Option<Receiver<(String, VerticalPosHint)>>,

    new_size_hint_property_sender: Option<Sender<(String, Option<f64>)>>,
    new_size_hint_property_receiver: Option<Receiver<(String, Option<f64>)>>,

    subscribe_to_property_sender: Option<Sender<(String, String)>>,
    subscribe_to_property_receiver: Option<Receiver<(String, String)>>,

    update_widget_sender: Option<Sender<String>>,
    update_widget_receiver: Option<Receiver<String>>,

    force_redraw_sender: Option<Sender<bool>>,
    force_redraw_receiver: Option<Receiver<bool>>,

    bind_property_sender: Option<Sender<(String, GenericFunction)>>,
    bind_property_receiver: Option<Receiver<(String, GenericFunction)>>,

    bind_global_key_sender: Option<Sender<(KeyCode, Option<Vec<KeyModifiers>>,
                                           KeyboardCallbackFunction)>>,
    bind_global_key_receiver: Option<Receiver<(KeyCode, Option<Vec<KeyModifiers>>,
                                               KeyboardCallbackFunction)>>,

    remove_global_key_sender: Option<Sender<(KeyCode, Option<Vec<KeyModifiers>>)>>,
    remove_global_key_receiver: Option<Receiver<(KeyCode, Option<Vec<KeyModifiers>>)>>,

    clear_global_keys_sender: Option<Sender<bool>>,
    clear_global_keys_receiver: Option<Receiver<bool>>,

    set_selected_widget_sender: Option<Sender<(String, Option<Coordinates>)>>,
    set_selected_widget_receiver: Option<Receiver<(String, Option<Coordinates>)>>,

    deselect_widget_sender: Option<Sender<bool>>,
    deselect_widget_receiver: Option<Receiver<bool>>,

    exit_sender: Option<Sender<bool>>,
    exit_receiver: Option<Receiver<bool>>,

    ask_sync_state_tree_sender: Option<Sender<bool>>,
    sync_state_tree_receiver: Option<Receiver<StateTree>>,
    sync_state_tree_main: Vec<(Receiver<bool>, Sender<StateTree>)>,

    ask_sync_properties_sender: Option<Sender<bool>>,
    sync_properties_receiver: Option<Receiver<EzPropertiesMap>>,
    sync_properties_main: Vec<(Receiver<bool>, Sender<EzPropertiesMap>)>,
}


impl SchedulerFrontend {

    /// Schedule a [GenericEzTask] to be executed once, after the passed duration. The duration can
    /// be 0 to start this on the next frame. This should only be used to manipulate the UI; to run
    /// any functions that will not return immediately use [schedule_threaded].
    /// # The GenericEzTask function
    /// A [GenericEzTask] is any FnMut that accepts an [Context] parameter. The [Context]
    /// allows you to access the [StateTree] and [Scheduler] from within the function, so you can
    /// make changes to the UI. Your closure should look like this:
    /// ```
    /// use ez_term::*;
    /// let my_closure = |context: Context| { };
    /// ```
    /// Or if you prefer an explicit function:
    /// ```
    /// use ez_term::*;
    /// fn my_func(context: Context) { };
    /// ```
    /// Closures are recommended for most use cases, as they allow capturing variables. You can
    /// use a run-once [GenericEzTask] in many ways:
    /// # Changing a widget state
    /// ```
    /// use ez_term::*;
    /// use std::time::Duration;
    /// let my_closure = |context: Context| {
    ///     let state = context.state_tree.get_mut("my_label").as_label_mut();
    ///     state.set_text("New text!".to_string());
    ///     state.update(context.scheduler);
    /// };
    /// scheduler.schedule_once("my_task", Box::new(my_closure), Duration::from_secs(0));
    /// ```
    /// Note that we used the 'move' keyword to move our counting variable into our task.
    pub fn schedule_once(&mut self, name: &str, func: GenericTask, after: Duration) {

        if !self.synced {
            let task = Task::new(name.to_string(), func, after);
            self.backend.tasks.push(task);
        } else {
            self.schedule_once_sender.as_ref().unwrap().send((name.to_string(), func, after)).unwrap();
        }
    }

    /// Cancel a run-once task by name.
    /// ```
    /// scheduler.cancel_task("my_task")
    /// ```
    pub fn cancel_task(&mut self, name: &str) {

        if !self.synced {
            let mut to_cancel = None;
            for (i, task) in self.backend.tasks.iter().enumerate() {
                if &task.name == name {
                    to_cancel = Some(i);
                }
            }
            if let Some(i) = to_cancel {
                self.backend.tasks.remove(i);
            }
        } else {
            self.cancel_task_sender.as_ref().unwrap().send(name.to_string()).unwrap()
        }
    }

    /// Schedule a [GenericEzTask] to be executed repeatedly on an interval. As long as the passed
    /// function keeps returning true, it will be scheduled to run again after the interval.
    /// This should only be used to manipulate the UI; to run any functions that will not return
    /// immediately, use [schedule_threaded].
    /// # The GenericEzTask function
    /// A [GenericEzTask] is any FnMut that accepts an [Context] parameter. The [Context]
    /// allows you to access the [StateTree] and [Scheduler] from within the function, so you can
    /// make changes to the UI. Your closure should look like this:
    /// ```
    /// use ez_term::*;
    /// let my_closure = |context: Context| { };
    /// ```
    /// Or if you prefer an explicit function:
    /// ```
    /// use ez_term::*;
    /// fn my_func(context: Context) { };
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
    /// let my_counter_func = move |context: Context| {
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

        if !self.synced {
            let task = RecurringTask::new(name.to_string(), func, interval);
            self.backend.recurring_tasks.push(task);
        } else {
            self.schedule_recurring_sender.as_ref().unwrap()
                .send((name.to_string(), func, interval)).unwrap();
        }
    }

    /// Cancel a recurring task by name.
    /// ```
    /// scheduler.cancel_recurring_task("my_task")
    /// ```
    pub fn cancel_recurring_task(&mut self, name: &str) {

        if !self.synced {
            let mut to_cancel = None;
            for (i, task) in self.backend.recurring_tasks.iter().enumerate() {
                if &task.name == name {
                    to_cancel = Some(i);
                }
            }
            if let Some(i) = to_cancel {
                self.backend.recurring_tasks.remove(i);
            }
        } else {
            self.cancel_recurring_sender.as_ref().unwrap().send(name.to_string()).unwrap()
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
    /// let my_closure = |context: Context| { };
    /// ```
    /// Or if you prefer an explicit function:
    /// ```
    /// use ez_term::*;
    /// fn my_func(context: Context) { };
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
        &mut self, threaded_func: EzThread,on_finish: Option<GenericTask>) {

        if !self.synced {
            self.backend.threads_to_start.push((threaded_func, on_finish));
        } else {
            self.schedule_threaded_sender.as_ref().unwrap()
                .send((threaded_func, on_finish)).unwrap();
        }
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
    /// let dismiss = |context: Context| {
    ///
    ///     context.scheduler.dismiss_modal(context.state_tree);
    ///     false
    /// };
    /// scheduler.update_callback_config("dismiss_button",
    ///                                  CallbackConfig::from_on_press(Box::new(dismiss)));
    /// // The popup will open on the next frame!
    /// ```
    pub fn open_modal(&mut self, template: &str, state_tree: &mut StateTree) {

        if !self.synced {
            open_and_register_modal(template.to_string(), state_tree, self);
        } else {
            self.open_modal_sender.as_ref().unwrap().send(template.to_string()).unwrap();
        }
    }

    /// Dismiss the current modal.
    pub fn dismiss_modal(&mut self, state_tree: &mut StateTree) {

        if !self.synced {
            state_tree.as_layout_mut().dismiss_modal(self);

            let mut removed_paths = Vec::new();
            let removed = state_tree.remove_node("/modal".to_string());
            for state in removed.get_all() {
                removed_paths.push(state.as_generic().get_path());
                state.as_generic().clean_up_properties(self);
            }
            self.backend.widgets_to_update.retain(|x| !removed_paths.contains(&x));
        } else {
            self.dismiss_modal_sender.as_ref().unwrap().send(true).unwrap();
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
    /// let my_callback = |context: Context| {
    ///     let state = context.state_tree.get_mut("my_button").as_button_mut();
    ///     state.set_disabled(true);
    ///     state.update(context.scheduler);
    ///     true
    /// };
    /// scheduler.overwrite_callback_config("my_button",
    ///                               CallbackConfig::from_on_press(Box::new(my_callback)));
    /// ```
    pub fn overwrite_callback_config(&mut self, for_widget: &str, callback_config: CallbackConfig) {

        if !self.synced {
            self.backend.new_callback_configs.push((for_widget.to_string(), callback_config));
        } else {
            self.overwrite_callback_config_sender.as_ref().unwrap()
                .send((for_widget.to_string(), callback_config)).unwrap()
        }
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
    /// let my_callback = |context: Context| {
    ///     let state = context.state_tree.get_mut("my_button").as_button_mut();
    ///     state.set_disabled(true);
    ///     state.update(context.scheduler);
    ///     true
    /// };
    /// scheduler.update_callback_config("my_button",
    ///                                  CallbackConfig::from_on_press(Box::new(dismiss_delay)));
    /// ```
    pub fn update_callback_config(&mut self, for_widget: &str, callback_config: CallbackConfig) {

        if !self.synced {
            self.backend.updated_callback_configs
                .push((for_widget.to_string(), callback_config));
        } else {
            self.update_callback_config_sender.as_ref().unwrap()
                .send((for_widget.to_string(), callback_config)).unwrap();
        }

    }

    pub fn prepare_create_widget(&mut self, widget_type: &str, id: &str, parent: &str,
                                 state_tree: &mut StateTree) -> (EzObjects, StateTree) {

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
        } else {
            base_type = widget_type.to_string();
            let new_state = EzState::from_string(&base_type,
                                                 new_path.to_string(),
                                                 self);
            new_widget = EzObjects::from_string(
                &base_type, new_path.to_string(), id.to_string(),
                self, new_state);
        };
        let mut new_states =
            StateTree::new(id.to_string(), new_widget.as_ez_object().get_state());
        if let EzObjects::Layout(ref i) = new_widget {
            for child in i.get_widgets_recursive() {
                let relative_path = child.as_ez_object().get_path().split_once(
                    new_widget.as_ez_object().get_id().as_str()).unwrap().1.to_string();
                let widget_path  = format!("{}{}", new_widget.as_ez_object().get_id(), relative_path);
                new_states.add_node(widget_path,child.as_ez_object().get_state());
            }
        }
        (new_widget, new_states)
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
    pub fn create_widget(&mut self, new_widget: EzObjects, new_states: StateTree,
                         state_tree: &mut StateTree)  {

        let path = new_states.obj.as_generic().get_path().clone();
        if !self.synced {
            state_tree.extend(path, new_states);
            self.backend.widgets_to_create.push(new_widget);
            if let Some(ref recv) = self.new_properties_receiver {
                if let Ok(i) = recv.try_recv() {
                    self.backend.properties.extend(i);
                }
            }
            if let Some(ref recv) = self.new_receivers_receiver {
                if let Ok(i) = recv.try_recv() {
                    self.backend.property_receivers.extend(i);
                }
            }

        } else {
            let mut receivers = HashMap::new();
            swap(&mut receivers, &mut self.backend.property_receivers);
            state_tree.extend(path, new_states.clone());
            self.create_widget_sender.as_ref().unwrap()
                .send((new_widget, new_states)).unwrap();
            self.new_receivers_sender.as_ref().unwrap().send(receivers).unwrap();
            self.new_properties_sender.as_ref().unwrap()
                .send(self.backend.properties.clone()).unwrap();
        }
    }

    /// Remove a widget on the next frame. Pass the path or ID of the widget to remove.
    pub fn remove_widget(&mut self, name: &str) {

        if !self.synced {
            if name == "root" || name == "/root" {
                panic!("Cannot remove the root layout")
            } else if name == "modal" || name == "/root/modal" {
                panic!("Cannot remove modal widget; use scheduler.dismiss_modal instead.")
            }
            self.backend.widgets_to_remove.push(name.to_string());
        } else {
            self.remove_widget_sender.as_ref().unwrap().send(name.to_string()).unwrap();
        }
    }

    fn get_update_func(&mut self, name: &str) {

        if name.contains('/') {
            let (widget, property_name) = name.rsplit_once('/').unwrap();
            let (widget, property_name) = (widget.to_string(), property_name.to_string());
            let updater =
                move | state_tree: &mut StateTree, val: EzValues | {
                    state_tree.get_mut(&widget).as_generic_mut()
                        .update_property(&property_name, val);
                };
            if !self.backend.property_updaters.contains_key(name) {
                self.backend.property_updaters.insert(name.to_string(), Box::new(updater));
            }
        }
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
            name.to_string(), EzProperties::Usize(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        self.get_update_func(name);
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
            name.to_string(), EzProperties::F64(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        self.get_update_func(name);
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
            name.to_string(), EzProperties::String(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        self.get_update_func(name);
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
            name.to_string(), EzProperties::Bool(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        self.get_update_func(name);
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
            name.to_string(), EzProperties::Color(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        self.get_update_func(name);
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
            name.to_string(), EzProperties::LayoutMode(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        self.get_update_func(name);
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
            name.to_string(), EzProperties::LayoutOrientation(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        self.get_update_func(name);
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
            name.to_string(), EzProperties::VerticalAlignment(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        self.get_update_func(name);
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
            name.to_string(), EzProperties::HorizontalAlignment(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        self.get_update_func(name);
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
            name.to_string(), EzProperties::HorizontalPosHint(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        self.get_update_func(name);
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
            name.to_string(), EzProperties::VerticalPosHint(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        self.get_update_func(name);
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
            name.to_string(), EzProperties::SizeHint(property.clone()));
        self.backend.property_receivers.insert(name.to_string(), receiver);
        self.get_update_func(name);
        property
    }

    /// Retrieve a property. Should be used to retrieve custom properties; access widget properties
    /// through the state_tree.
    pub fn get_property(&self, name: &str) -> &EzProperties {

        self.backend.properties.get(name).unwrap_or_else(
            || panic!("Could not find property: {}", name)
        )
    }

    /// Retrieve a property. Should be used to retrieve custom properties; access widget properties
    /// through the state_tree.
    pub fn get_property_mut(&mut self, name: &str) -> &mut EzProperties {
        self.backend.properties.get_mut(name).unwrap_or_else(
            || panic!("Could not find property: {}", name)
        )
    }

    /// Subscribe one property to another, ensuring the subscriber will always have the value of the
    /// property it is subscribed to on the next frame. An update func is required which will be
    /// called when the property subscribed to changes. The update func receives the new value and
    /// is responsible for setting the appropriate field on the subscriber.
    pub fn subscribe_to_property(&mut self, name: &str, subscriber: String) {

        if !self.synced {
            if !self.backend.property_subscribers.contains_key(name) {
                self.backend.property_subscribers.insert(name.to_string(), Vec::new());
            }
            self.backend.property_subscribers.get_mut(name).unwrap().push(subscriber);
        } else {
            self.subscribe_to_property_sender.as_ref().unwrap()
                .send((name.to_string(), subscriber)).unwrap();
        }
    }

    /// Schedule a widget to be updated on the next frame. Can also be called from the widget itself
    /// as ```[widget.update(scheduler)]``` (for convenience).
    pub fn update_widget(&mut self, path: &str) {

        if !self.synced {
            if path.starts_with("/root/modal") {
                self.backend.force_redraw = true;
                return
            }
            let path = path.to_string();
            if !self.backend.widgets_to_update.contains(&path) {
                self.backend.widgets_to_update.push(path);
            }
        } else {
            self.update_widget_sender.as_ref().unwrap().send(path.to_string()).unwrap();
        }
    }

    /// Schedule a full screen redraw on the next frame. [get_contents] will be called on the root
    /// widget and drawn to screen. Only changed pixels are actually drawn as an optimization.
    pub fn force_redraw(&mut self) {

        if !self.synced {
            self.backend.force_redraw = true;
        } else {
            self.force_redraw_sender.as_ref().unwrap().send(true).unwrap();
        }
    }

    /// Bind a callback function to the changing of an EzProperty. Make sure that the function you
    /// create has the right signature, e.g.:
    /// ```
    /// use ez_term::*;
    /// let my_property = scheduler.new_usize_property("my_property".to_string(), 0);
    ///
    /// let my_callback = |context: Context| {
    ///     let state  = context.state_tree.get("my_label").as_label_mut();
    ///     state.set_text("Value changed".to_string());
    ///     state.update(context.scheduler);
    /// };
    ///
    /// scheduler.bind_ez_property_callback("my_property", Box::new(my_callback));
    /// ```
    pub fn bind_property_callback(&mut self, name: &str, callback: GenericFunction) {

        if !self.synced {
            if self.backend.property_callbacks.contains(&name.to_string()) {
                self.backend.new_property_callbacks.push((name.to_string(), callback));
            } else {
                let mut config = CallbackConfig::default();
                config.property_callbacks.push(callback);
                let name = if !name.contains('/') {
                    format!("/root/{}", name)
                } else { name.to_string() };
                self.overwrite_callback_config(&name, config);
                self.backend.property_callbacks.push(name);
            }
        } else {
            self.bind_property_sender.as_ref().unwrap().send((name.to_string(), callback)).unwrap();
        }
    }

    /// Globally bind a keyboard key (CrossTerm [KeyCode]). These keybinds take priority over all
    /// others and work in all contexts.
    pub fn bind_global_key(&mut self, key: KeyCode, modifiers: Option<Vec<KeyModifiers>>,
                           callback: KeyboardCallbackFunction) {

        if !self.synced {
            let modifiers = create_keymap_modifiers(modifiers);
            self.backend.update_global_keymap.insert((key, modifiers), callback);
        } else {
            self.bind_global_key_sender.as_ref().unwrap().send((key, modifiers, callback)).unwrap();
        }
    }

    /// Remove a single global keybind by keycode.
    pub fn remove_global_key(&mut self, key: KeyCode, modifiers: Option<Vec<KeyModifiers>>) {

        if !self.synced {
            let modifiers = create_keymap_modifiers(modifiers);
            self.backend.remove_global_keymap.push((key, modifiers));
        } else {
            self.remove_global_key_sender.as_ref().unwrap().send((key, modifiers)).unwrap();
        }

    }

    /// Remove all custom global keybinds currently active.
    pub fn clear_global_keys(&mut self) {

        if !self.synced {
            self.backend.clear_global_keymap = true;
        } else {
            self.clear_global_keys_sender.as_ref().unwrap().send(true).unwrap();
        }
    }

    /// Set the passed widget (can be ID or path) as selected. Automatically deselects the current
    /// selection if any, and calls the appropriate callbacks.
    pub fn set_selected_widget(&mut self, widget: &str, mouse_pos: Option<Coordinates>) {

        if !self.synced {
            self.backend.next_selection = Some((widget.to_string(), mouse_pos));
        } else {
            self.set_selected_widget_sender.as_ref().unwrap()
                .send((widget.to_string(), mouse_pos)).unwrap();
        }
    }

    /// Set the passed widget (can be ID or path) as selected. Automatically deselects the current
    /// selection if any, and calls the appropriate callbacks.
    pub fn deselect_widget(&mut self) {

        if !self.synced {
            self.backend.deselect = true
        } else {
            self.deselect_widget_sender.as_ref().unwrap().send(true).unwrap();
        }
    }


    /// Gracefully exit the app.
    pub fn exit(&self) {

        if !self.synced {
            stop();
        } else {
            self.exit_sender.as_ref().unwrap().send(true).unwrap();
        }
    }

    pub fn is_syncing(&self) -> bool {
        self.syncing != 0
    }

    fn check_sync_state_tree(&self, state_tree: &mut StateTree) {

        for (ask_receiver, reply_sender) in self.sync_state_tree_main.iter() {
            let mut received = None;
            loop {
                match ask_receiver.try_recv() {
                    Ok(i) => { received = Some(i) },
                    Err(_) => { break }
                }
            }
            if received.is_some() {
                reply_sender.send(state_tree.clone()).unwrap();
            }
        }
    }

    fn check_sync_properties(&self) {

        for (ask_receiver, reply_sender) in self.sync_properties_main.iter() {
            let mut received = None;
            loop {
                match ask_receiver.try_recv() {
                    Ok(i) => { received = Some(i) },
                    Err(_) => { break }
                }
            }
            if received.is_some() {
                reply_sender.send(self.backend.properties.clone()).unwrap();
            }
        }
    }

    /// Sync state tree from the main thread. The state tree cannot be shared across threads;
    /// therefore each thread has its own state tree. Changes made from a thread will be synced
    /// to the main thread, however changes made in the main thread will not be synced to the
    /// other thread(s). Often it is not important to have an up to date state tree in your thread,
    /// but if it is necessary at any point, call this method first.
    pub fn sync_state_tree(&mut self) -> StateTree {

        self.ask_sync_state_tree_sender.as_ref().unwrap().send(true).unwrap();
        self.sync_state_tree_receiver.as_ref().unwrap().recv().unwrap()

    }

    /// Sync (custom) properties from the main thread. The properties cannot be shared across threads;
    /// therefore each thread has its own properties. Changes made from a thread will be synced
    /// to the main thread, however changes made in the main thread will not be synced to the
    /// other thread(s). Often it is not important to have up to date properties in your thread,
    /// but if it is necessary at any point, call this method first.
    pub fn sync_properties(&mut self) {

        self.ask_sync_properties_sender.as_ref().unwrap().send(true).unwrap();
        self.backend.properties =
            self.sync_properties_receiver.as_ref().unwrap()
                .recv().unwrap();
    }

    pub fn _check_method_channels(&mut self, state_tree: &mut StateTree) {
        if self.syncing == 0 { return }

        while let Ok((name, func, after)) = self.schedule_once_receiver.as_ref().unwrap().try_recv() {
            self.schedule_once(&name, func, after);
        }
        while let Ok((name, func, interval)) = self.schedule_recurring_receiver.as_ref().unwrap().try_recv() {
            self.schedule_recurring(&name, func, interval);
        }
        while let Ok((func, on_finish)) = self.schedule_threaded_receiver.as_ref().unwrap().try_recv() {
            self.schedule_threaded(func, on_finish);
        }
        while let Ok(name) = self.cancel_task_receiver.as_ref().unwrap().try_recv() {
                self.cancel_task(name.as_str());
        }
        while let Ok(name) = self.cancel_recurring_receiver.as_ref().unwrap().try_recv() {
            self.cancel_recurring_task(name.as_str());
        }
        while let Ok(template) = self.open_modal_receiver.as_ref().unwrap().try_recv() {
            self.open_modal(template.as_str(), state_tree);
        }
        while let Ok(_) = self.dismiss_modal_receiver.as_ref().unwrap().try_recv() {
            self.dismiss_modal(state_tree);
        }
        while let Ok((for_widget, callback_config)) =
                self.overwrite_callback_config_receiver.as_ref().unwrap().try_recv() {
            self.overwrite_callback_config(for_widget.as_str(), callback_config);
        }
        while let Ok((for_widget, callback_config))
                = self.update_callback_config_receiver.as_ref().unwrap().try_recv() {
            self.update_callback_config(for_widget.as_str(), callback_config);
        }
        while let Ok((new_widget, new_states)) =
                self.create_widget_receiver.as_ref().unwrap().try_recv() {

            self.create_widget(new_widget, new_states, state_tree);
        }
        while let Ok(name) = self.remove_widget_receiver.as_ref().unwrap().try_recv() {
            self.remove_widget(name.as_str());
        }
        while let Ok((name, value)) =
                self.new_usize_property_receiver.as_ref().unwrap().try_recv() {
            self.new_usize_property(name.as_str(), value);
        }
        while let Ok((name, value)) =
                self.new_f64_property_receiver.as_ref().unwrap().try_recv() {
            self.new_f64_property(name.as_str(), value);
        }
        while let Ok((name, value)) =
                self.new_string_property_receiver.as_ref().unwrap().try_recv() {
            self.new_string_property(name.as_str(), value);
        }
        while let Ok((name, value)) =
                self.new_color_property_receiver.as_ref().unwrap().try_recv() {
            self.new_color_property(name.as_str(), value);
        }
        while let Ok((name, value)) =
                self.new_bool_property_receiver.as_ref().unwrap().try_recv() {
            self.new_bool_property(name.as_str(), value);
        }
        while let Ok((name, value)) =
                self.new_layout_mode_property_receiver.as_ref().unwrap().try_recv() {
            self.new_layout_mode_property(name.as_str(), value);
        }
        while let Ok((name, value)) =
                self.new_layout_orientation_property_receiver.as_ref().unwrap().try_recv() {
            self.new_layout_orientation_property(name.as_str(), value);
        }
        while let Ok((name, value)) =
                self.new_horizontal_alignment_property_receiver.as_ref().unwrap().try_recv() {
            self.new_horizontal_alignment_property(name.as_str(), value);
        }
        while let Ok((name, value)) =
                self.new_vertical_alignment_property_receiver.as_ref().unwrap().try_recv() {
            self.new_vertical_alignment_property(name.as_str(), value);
        }
        while let Ok((name, value)) =
                self.new_horizontal_pos_hint_property_receiver.as_ref().unwrap().try_recv() {
            self.new_horizontal_pos_hint_property(name.as_str(), value);
        }
        while let Ok((name, value)) =
                self.new_vertical_pos_hint_property_receiver.as_ref().unwrap().try_recv() {
            self.new_vertical_pos_hint_property(name.as_str(), value);
        }
        while let Ok((name, value)) =
                self.new_size_hint_property_receiver.as_ref().unwrap().try_recv() {
            self.new_size_hint_property(name.as_str(), value);
        }
        while let Ok((name, update_func)) =
                self.subscribe_to_property_receiver.as_ref().unwrap().try_recv() {
            self.subscribe_to_property(name.as_str(), update_func);
        }
        while let Ok(name) = self.update_widget_receiver.as_ref().unwrap().try_recv() {
            self.update_widget(name.as_str());
        }
        while let Ok(_) = self.force_redraw_receiver.as_ref().unwrap().try_recv() {
            self.force_redraw();
        }
        while let Ok((name, func)) =
                self.bind_property_receiver.as_ref().unwrap().try_recv() {
            self.bind_property_callback(name.as_str(), func);
        }
        while let Ok((key, modifier, func)) =
                self.bind_global_key_receiver.as_ref().unwrap().try_recv() {
            self.bind_global_key(key, modifier, func);
        }
        while let Ok((key, modifier)) =
                self.remove_global_key_receiver.as_ref().unwrap().try_recv() {
            self.remove_global_key(key, modifier);
        }
        while let Ok(_) = self.clear_global_keys_receiver.as_ref().unwrap().try_recv() {
            self.clear_global_keys();
        }
        while let Ok((name, mouse_pos)) =
                    self.set_selected_widget_receiver.as_ref().unwrap().try_recv() {
            self.set_selected_widget(name.as_str(), mouse_pos);
        }
        while let Ok(_) = self.deselect_widget_receiver.as_ref().unwrap().try_recv() {
            self.deselect_widget();
        }
        while let Ok(_) = self.exit_receiver.as_ref().unwrap().try_recv() {
            self.exit();
        }
        self.check_sync_properties();
        self.check_sync_state_tree(state_tree);
    }

    /// Method to stop a synced scheduler. No need to use this as an end-user; schedule_threaded
    /// will do the work for you.
    pub fn _stop_sync_to_thread(&mut self) {
        self.syncing -= 1;
    }

    /// Method to set up a synced scheduler to use in a thread. No need to use this as an
    /// end-user; schedule_threaded will do the work for you.
    pub fn _sync_to_thread(&mut self) -> SchedulerFrontend {

        let mut synced_frontend = SchedulerFrontend::default();
        self.syncing += 1;
        synced_frontend.synced = true;

        if self.schedule_once_receiver.is_none() {
            let (sender, receiver) = channel();
            self.schedule_once_receiver = Some(receiver);
            self.schedule_once_sender = Some(sender.clone());
        }
        synced_frontend.schedule_once_sender = self.schedule_once_sender.clone();

        if self.schedule_recurring_receiver.is_none() {
            let (sender, receiver) = channel();
            self.schedule_recurring_receiver = Some(receiver);
            self.schedule_recurring_sender = Some(sender.clone());
        }
        synced_frontend.schedule_recurring_sender = self.schedule_recurring_sender.clone();

        if self.schedule_threaded_receiver.is_none() {
            let (sender, receiver) = channel();
            self.schedule_threaded_receiver = Some(receiver);
            self.schedule_threaded_sender = Some(sender.clone());
        }
        synced_frontend.schedule_threaded_sender = self.schedule_threaded_sender.clone();

        if self.cancel_task_receiver.is_none() {
            let (sender, receiver) = channel();
            self.cancel_task_receiver = Some(receiver);
            self.cancel_task_sender = Some(sender.clone());
        }
        synced_frontend.cancel_task_sender = self.cancel_task_sender.clone();

        if self.cancel_recurring_receiver.is_none() {
            let (sender, receiver) = channel();
            self.cancel_recurring_receiver = Some(receiver);
            self.cancel_recurring_sender = Some(sender.clone());
        }
        synced_frontend.cancel_recurring_sender = self.cancel_recurring_sender.clone();

        if self.open_modal_receiver.is_none() {
            let (sender, receiver) = channel();
            self.open_modal_receiver = Some(receiver);
            self.open_modal_sender = Some(sender.clone());
        }
        synced_frontend.open_modal_sender = self.open_modal_sender.clone();

        if self.dismiss_modal_receiver.is_none() {
            let (sender, receiver) = channel();
            self.dismiss_modal_receiver = Some(receiver);
            self.dismiss_modal_sender = Some(sender.clone());
        }
        synced_frontend.dismiss_modal_sender = self.dismiss_modal_sender.clone();

        if self.overwrite_callback_config_receiver.is_none() {
            let (sender, receiver) = channel();
            self.overwrite_callback_config_receiver = Some(receiver);
            self.overwrite_callback_config_sender = Some(sender.clone());
        }
        synced_frontend.overwrite_callback_config_sender = self.overwrite_callback_config_sender.clone();

        if self.update_callback_config_receiver.is_none() {
            let (sender, receiver) = channel();
            self.update_callback_config_receiver = Some(receiver);
            self.update_callback_config_sender = Some(sender.clone());
        }
        synced_frontend.update_callback_config_sender = self.update_callback_config_sender.clone();

        if self.create_widget_receiver.is_none() {
            let (sender, receiver) = channel();
            self.create_widget_receiver = Some(receiver);
            self.create_widget_sender = Some(sender.clone());
        }
        synced_frontend.create_widget_sender = self.create_widget_sender.clone();

        if self.new_properties_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_properties_receiver = Some(receiver);
            self.new_properties_sender = Some(sender);
        }
        synced_frontend.new_properties_sender = self.new_properties_sender.clone();

        if self.new_receivers_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_receivers_receiver = Some(receiver);
            self.new_receivers_sender = Some(sender);
        }
        synced_frontend.new_receivers_sender = self.new_receivers_sender.clone();

        if self.remove_widget_receiver.is_none() {
            let (sender, receiver) = channel();
            self.remove_widget_receiver = Some(receiver);
            self.remove_widget_sender = Some(sender.clone());
        }
        synced_frontend.remove_widget_sender = self.remove_widget_sender.clone();

        if self.new_usize_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_usize_property_receiver = Some(receiver);
            self.new_usize_property_sender = Some(sender.clone());
        }
        synced_frontend.new_usize_property_sender = self.new_usize_property_sender.clone();

        if self.new_f64_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_f64_property_receiver = Some(receiver);
            self.new_f64_property_sender = Some(sender.clone());
        }
        synced_frontend.new_f64_property_sender = self.new_f64_property_sender.clone();

        if self.new_bool_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_bool_property_receiver = Some(receiver);
            self.new_bool_property_sender = Some(sender.clone());
        }
        synced_frontend.new_bool_property_sender = self.new_bool_property_sender.clone();

        if self.new_string_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_string_property_receiver = Some(receiver);
            self.new_string_property_sender = Some(sender.clone());
        }
        synced_frontend.new_string_property_sender = self.new_string_property_sender.clone();

        if self.new_color_property_sender.is_none() {
            let (sender, receiver) = channel();
            self.new_color_property_receiver = Some(receiver);
            self.new_color_property_sender = Some(sender.clone());
        }
        synced_frontend.new_color_property_sender = self.new_color_property_sender.clone();

        if self.new_layout_mode_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_layout_mode_property_receiver = Some(receiver);
            self.new_layout_mode_property_sender = Some(sender.clone());
        }
        synced_frontend.new_layout_mode_property_sender = self.new_layout_mode_property_sender.clone();

        if self.new_layout_orientation_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_layout_orientation_property_receiver = Some(receiver);
            self.new_layout_orientation_property_sender = Some(sender.clone());
        }
        synced_frontend.new_layout_orientation_property_sender = self.new_layout_orientation_property_sender.clone();

        if self.new_horizontal_alignment_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_horizontal_alignment_property_receiver = Some(receiver);
            self.new_horizontal_alignment_property_sender = Some(sender.clone());
        }
        synced_frontend.new_horizontal_alignment_property_sender = self.new_horizontal_alignment_property_sender.clone();

        if self.new_vertical_alignment_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_vertical_alignment_property_receiver = Some(receiver);
            self.new_vertical_alignment_property_sender = Some(sender.clone());
        }
        synced_frontend.new_vertical_alignment_property_sender = self.new_vertical_alignment_property_sender.clone();

        if self.new_horizontal_pos_hint_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_horizontal_pos_hint_property_receiver = Some(receiver);
            self.new_horizontal_pos_hint_property_sender = Some(sender.clone());
        }
        synced_frontend.new_horizontal_pos_hint_property_sender = self.new_horizontal_pos_hint_property_sender.clone();

        if self.new_vertical_pos_hint_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_vertical_pos_hint_property_receiver = Some(receiver);
            self.new_vertical_pos_hint_property_sender = Some(sender.clone());
        }
        synced_frontend.new_vertical_pos_hint_property_sender = self.new_vertical_pos_hint_property_sender.clone();

        if self.new_size_hint_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_size_hint_property_receiver = Some(receiver);
            self.new_size_hint_property_sender = Some(sender.clone());
        }
        synced_frontend.new_size_hint_property_sender = self.new_size_hint_property_sender.clone();

        if self.subscribe_to_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.subscribe_to_property_receiver = Some(receiver);
            self.subscribe_to_property_sender = Some(sender.clone());
        }
        synced_frontend.subscribe_to_property_sender = self.subscribe_to_property_sender.clone();

        if self.update_widget_receiver.is_none() {
            let (sender, receiver) = channel();
            self.update_widget_receiver = Some(receiver);
            self.update_widget_sender = Some(sender.clone());
        }
        synced_frontend.update_widget_sender = self.update_widget_sender.clone();

        if self.force_redraw_receiver.is_none() {
            let (sender, receiver) = channel();
            self.force_redraw_receiver = Some(receiver);
            self.force_redraw_sender = Some(sender.clone());
        }
        synced_frontend.force_redraw_sender = self.force_redraw_sender.clone();

        if self.bind_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.bind_property_receiver = Some(receiver);
            self.bind_property_sender = Some(sender.clone());
        }
        synced_frontend.bind_property_sender = self.bind_property_sender.clone();

        if self.bind_global_key_receiver.is_none() {
            let (sender, receiver) = channel();
            self.bind_global_key_receiver = Some(receiver);
            self.bind_global_key_sender = Some(sender.clone());
        }
        synced_frontend.bind_global_key_sender = self.bind_global_key_sender.clone();

        if self.remove_global_key_receiver.is_none() {
            let (sender, receiver) = channel();
            self.remove_global_key_receiver = Some(receiver);
            self.remove_global_key_sender = Some(sender.clone());
        }
        synced_frontend.remove_global_key_sender = self.remove_global_key_sender.clone();

        if self.clear_global_keys_receiver.is_none() {
            let (sender, receiver) = channel();
            self.clear_global_keys_receiver = Some(receiver);
            self.clear_global_keys_sender = Some(sender.clone());
        }
        synced_frontend.clear_global_keys_sender = self.clear_global_keys_sender.clone();

        if self.set_selected_widget_receiver.is_none() {
            let (sender, receiver) = channel();
            self.set_selected_widget_receiver = Some(receiver);
            self.set_selected_widget_sender = Some(sender.clone());
        }
        synced_frontend.set_selected_widget_sender = self.set_selected_widget_sender.clone();

        if self.deselect_widget_receiver.is_none() {
            let (sender, receiver) = channel();
            self.deselect_widget_receiver = Some(receiver);
            self.deselect_widget_sender = Some(sender.clone());
        }
        synced_frontend.deselect_widget_sender = self.deselect_widget_sender.clone();

        if self.exit_receiver.is_none() {
            let (sender, receiver) = channel();
            self.exit_receiver = Some(receiver);
            self.exit_sender = Some(sender.clone());
        }
        synced_frontend.exit_sender = self.exit_sender.clone();

        let (ask_sender, ask_receiver) = channel();
        let (reply_sender, reply_receiver) = channel();
        self.sync_state_tree_main.push((ask_receiver, reply_sender));
        synced_frontend.ask_sync_state_tree_sender = Some(ask_sender);
        synced_frontend.sync_state_tree_receiver = Some(reply_receiver);

        let (ask_sender, ask_receiver) = channel();
        let (reply_sender, reply_receiver) = channel();
        self.sync_properties_main.push((ask_receiver, reply_sender));
        synced_frontend.ask_sync_properties_sender = Some(ask_sender);
        synced_frontend.sync_properties_receiver = Some(reply_receiver);

        synced_frontend.backend.properties = self.backend.properties.clone();
        synced_frontend.backend.templates = self.backend.templates.clone();
        synced_frontend
    }
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

    /// A <Widget path, Receiver> HashMap, used to get the receiver of an EzProperty channel.
    /// New values are received on this receiver and then synced to any subscribed properties.
    pub property_receivers: HashMap<String, Receiver<EzValues>>,

    pub property_updaters: HashMap<String, EzPropertyUpdater>,

    /// A <Widget path, update_callback> HashMap, used to get the updater callback EzProperty.
    /// When a property subscribes to another, it must provide an updater callback. When the value
    /// changes, the callback will be called with the new value, and is responsible for syncing the
    /// subscribing property to the new value.
    pub property_subscribers: HashMap<String, Vec<String>>,

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