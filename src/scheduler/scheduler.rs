//! # Scheduler
//!
//! A module implementing the Scheduler struct.
use crossterm::event::{KeyCode, KeyModifiers};
use std::collections::HashMap;
use std::mem::swap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crossterm::style::Color;

use crate::parser::ez_definition::Templates;
use crate::property::ez_properties::EzProperties;
use crate::property::ez_property::EzProperty;
use crate::property::ez_values::EzValues;
use crate::run::definitions::{Coordinates, StateTree};
use crate::run::run::{open_and_register_modal, stop};
use crate::scheduler::definitions::{
    EzPropertyUpdater, EzThread, GenericFunction, GenericRecurringTask, GenericTask,
    KeyboardCallbackFunction,
};
use crate::states::definitions::{
    create_keymap_modifiers, HorizontalAlignment, HorizontalPosHint, LayoutMode, LayoutOrientation,
    VerticalAlignment, VerticalPosHint,
};
use crate::states::ez_state::EzState;
use crate::widgets::ez_object::EzObjects;
use crate::{CallbackConfig, EzPropertiesMap};

/// The Scheduler is a key component of the framework. It, along with the [StateTree], gives
/// you control over the UI at runtime.
#[derive(Default)]
pub struct SchedulerFrontend {
    /// Backend of the scheduler. Do not use. Use the exposed methods instead.
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

    bind_global_key_sender:
        Option<Sender<(KeyCode, Option<Vec<KeyModifiers>>, KeyboardCallbackFunction)>>,
    bind_global_key_receiver:
        Option<Receiver<(KeyCode, Option<Vec<KeyModifiers>>, KeyboardCallbackFunction)>>,

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
    /// Method that allows you to schedule a closure or function for single execution after a delay
    /// (which can be 0).
    /// Only intended for code that returns immediately (like manipulating the UI); to run blocking
    /// app-code, use schedule threaded.
    /// For a tutorial see: [Single execution](#scheduler_tasks_single).
    ///
    /// # Parameters:
    ///
    /// - Scheduled task name: String
    /// - Function: Box<FnMut(Context)>
    /// - Delay: std::Duration;
    ///
    /// # Example:
    ///
    /// As an example, we'll create a scheduled task that changes a label text:
    /// ```
    /// use std::time::Duration;
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let my_closure = |context: Context| {
    ///     let state = context.state_tree.get_mut("my_label").as_label_mut();
    ///     state.set_text("New text!".to_string());
    ///     state.update(context.scheduler);
    /// };
    /// scheduler.schedule_once("my_task", Box::new(my_closure), Duration::from_secs(0));
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn schedule_once(&mut self, name: &str, func: GenericTask, after: Duration) {
        if !self.synced {
            let task = Task::new(name.to_string(), func, after);
            self.backend.tasks.push(task);
        } else {
            self.schedule_once_sender
                .as_ref()
                .unwrap()
                .send((name.to_string(), func, after))
                .unwrap();
        }
    }

    /// Cancel a run-once task. This of course only works if called before the task was executed (possible
    /// if it had a delay). This function is always safe to call, if there's no task to cancel it will
    /// not panic.
    ///
    /// # Parameters:
    ///
    /// - Scheduled task name: String
    ///
    /// # Example:
    ///
    /// We'll schedule a run-once task and then cancel it:
    ///
    /// ```
    /// use std::time::Duration;
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let my_closure = |context: Context| {
    ///     let state = context.state_tree.get_mut("my_label").as_label_mut();
    ///     state.set_text("New text!".to_string());
    ///     state.update(context.scheduler);
    /// };
    /// scheduler.schedule_once("my_task", Box::new(my_closure), Duration::from_secs(0));
    ///
    /// scheduler.cancel_task("my_task");
    /// run(root_widget, state_tree, scheduler);
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
            self.cancel_task_sender
                .as_ref()
                .unwrap()
                .send(name.to_string())
                .unwrap()
        }
    }

    /// Method that allows you to schedule a closure or function for recurring execution; it will be
    /// executed on an interval as long as the function keeps returning true.
    /// Only intended for code that returns immediately (like manipulating the UI); to run blocking
    /// app-code, use schedule threaded.
    ///
    /// # Parameters:
    ///
    /// - Scheduled task name: String
    /// - Function: Box<FnMut(Context) -> bool>
    /// - Interval: std::Duration;
    ///
    /// # Example:
    ///
    /// As an example, we'll create a scheduled task that changes a label text to count time. When 60
    /// seconds have been counted, the function will be cancelled:
    /// ```
    /// use std::time::Duration;
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let counter: usize = 0;
    /// let my_closure = move |context: Context| {
    ///     let state = context.state_tree.get_mut("my_label").as_label_mut();
    ///     state.set_text(format!("Time passed {}", counter));
    ///     state.update(context.scheduler);
    ///     return if counter == 60 {
    ///         false
    ///     } else {
    ///         true
    ///    }
    /// };
    /// scheduler.schedule_recurring("my_task", Box::new(my_closure), Duration::from_secs(1));
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn schedule_recurring(
        &mut self,
        name: &str,
        func: GenericRecurringTask,
        interval: Duration,
    ) {
        if !self.synced {
            let task = RecurringTask::new(name.to_string(), func, interval);
            self.backend.recurring_tasks.push(task);
        } else {
            self.schedule_recurring_sender
                .as_ref()
                .unwrap()
                .send((name.to_string(), func, interval))
                .unwrap();
        }
    }

    /// Cancel a recurring task. This function is always safe to call, if there's no task to cancel it
    /// will not panic. An alternative way to cancel a recurring task is to return false from the
    /// scheduled function (which is recommended for most use cases. This function lets you cancel a
    /// recurring task 'from the outside'.
    ///
    /// # Parameters:
    ///
    /// - Scheduled task name: String
    ///
    /// # Example:
    ///
    /// We'll schedule a recurring task and then cancel it:
    /// ```
    /// use std::time::Duration;
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let counter: usize = 0;
    /// let my_closure = move |context: Context| {
    ///     let state = context.state_tree.get_mut("my_label").as_label_mut();
    ///     state.set_text(format!("Time passed {}", counter));
    ///     state.update(context.scheduler);
    ///     return if counter == 60 {
    ///         false
    ///     } else {
    ///         true
    ///    }
    /// };
    /// scheduler.schedule_recurring("my_task", Box::new(my_closure), Duration::from_secs(1));
    ///
    /// scheduler.cancel_recurring_task("my_task");
    /// run(root_widget, state_tree, scheduler);
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
            self.cancel_recurring_sender
                .as_ref()
                .unwrap()
                .send(name.to_string())
                .unwrap()
        }
    }

    /// Method that allows you to schedule a closure or function for threaded execution. This allows
    /// you to run code that does not return immediately (like your app code). You can use the
    /// state tree from the threaded function to manipulate the UI, but the scheduler will not be
    /// available from a thread. A hashmap with custom properties is also available.
    /// For more a tutorial see: [Threaded execution](#scheduler_tasks_threaded)
    ///
    /// # Parameters:
    ///
    /// - Threaded function: Box<dyn FnOnce(HashMap<String, EzProperty>, StateTree) + Send>
    /// - On_finish callback function: Option<Box<FnMut(Context)>>>
    ///
    /// # Example:
    ///
    /// We'll schedule a counter function that does not return immediately:
    /// ```
    /// use std::time::Duration;
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// fn example_app(mut properties: EzPropertiesMap, mut state_tree: StateTree) {
    ///
    ///     let state = state_tree.get_mut("my_label").as_label_mut();
    ///     for x in 1..10 {
    ///         state.set_text(format!("Time passed {}", x));
    ///         std::thread::sleep(Duration::from_secs(1))
    ///     };
    /// }
    ///
    /// fn on_finish_callback(context: Context) {
    ///     let state = state_tree.get_mut("my_label").as_label_mut();
    ///     state.set_text("Finished!".to_string());
    /// }
    ///
    /// scheduler.schedule_threaded(Box::new(example_app), Some(Box::new(on_finish_callback)));
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn schedule_threaded(&mut self, threaded_func: EzThread, on_finish: Option<GenericTask>) {
        if !self.synced {
            self.backend
                .threads_to_start
                .push((threaded_func, on_finish));
        } else {
            self.schedule_threaded_sender
                .as_ref()
                .unwrap()
                .send((threaded_func, on_finish))
                .unwrap();
        }
    }

    /// Method that allows you to open a modal (e.g. a popup). To open a modal you need to define a
    /// Layout template in an .ez file. You can then spawn an instance of the template as a modal using
    /// this method. The ID of the layout spawned as a modal will be 'modal', its full path will be
    /// '/root/modal'.
    ///
    /// # Parameters:
    ///
    /// - Layout template name: &str
    /// - State tree: &mut StateTree
    ///
    /// # Example:
    ///
    /// We'll create a popup and open it. The popup will have some text and a dismiss button.
    /// First we need to define a Layout template in an .ez file:
    /// ```
    /// - <MyPopupTemplate@Layout>:
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
    ///         text: Dismiss
    ///         size_hint_x: 1
    ///         auto_scale_height: true
    ///         pos_hint: center, bottom
    ///         selection_order: 1
    /// ```
    /// Now we'll spawn the popup from code based on this template.
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let dismiss = |context: Context| {
    ///
    ///     context.scheduler.dismiss_modal(context.state_tree);
    ///     false
    /// };
    ///
    /// scheduler.update_callback_config("dismiss_button",
    ///                                  CallbackConfig::from_on_press(Box::new(dismiss)));
    ///
    /// scheduler.open_modal("MyPopupTemplate", &mut state_tree);
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn open_modal(&mut self, template: &str, state_tree: &mut StateTree) {
        if !self.synced {
            open_and_register_modal(template.to_string(), state_tree, self);
        } else {
            self.open_modal_sender
                .as_ref()
                .unwrap()
                .send(template.to_string())
                .unwrap();
        }
    }

    /// Dismiss the open modal. Can always be called safely even if one no longer exists (though this
    /// does trigger a screen redraw so try to avoid that).
    /// For a tutorial on modals see: [Managing popups](#scheduler_modals)
    ///
    /// # Parameters:
    ///
    /// - State tree: &mut StateTree
    ///
    /// # Example:
    ///
    /// We'll dismiss any open modal:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.dismiss_modal(&mut state_tree);
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn dismiss_modal(&mut self, state_tree: &mut StateTree) {
        if !self.synced {
            state_tree.as_layout_mut().dismiss_modal(self);

            let mut removed_paths = Vec::new();
            let removed = state_tree.remove_node("/modal".to_string());
            for state in removed.get_all() {
                removed_paths.push(state.as_generic().get_path());
                state.as_generic().clean_up_properties(self);
            }
            self.backend
                .widgets_to_update
                .retain(|x| !removed_paths.contains(&x));
        } else {
            self.dismiss_modal_sender
                .as_ref()
                .unwrap()
                .send(true)
                .unwrap();
        }
    }

    /// Replace the entire CallbackConfig of a widget on the next frame. You can pass an empty
    /// CallbackConfig to remove all callbacks for a widget.
    ///
    /// # Parameters:
    ///
    /// - Widget ID or path: &str
    /// - New CallbackConfig: CallbackConfig
    ///
    /// # Example:
    ///
    /// We'll remove any existing callbacks by overwriting with an empty CallbackConfig for a button
    /// with id "my_button".
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let callback_config = CallbackConfig::default();
    /// scheduler.overwrite_callback_config("my_button", callback_config);
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn overwrite_callback_config(&mut self, for_widget: &str, callback_config: CallbackConfig) {
        if !self.synced {
            self.backend
                .new_callback_configs
                .push((for_widget.to_string(), callback_config));
        } else {
            self.overwrite_callback_config_sender
                .as_ref()
                .unwrap()
                .send((for_widget.to_string(), callback_config))
                .unwrap()
        }
    }

    /// Update the CallbackConfig of a widget on the next frame. Any callback set on the new callback
    /// config will be set on the existing one (overwriting if one already exists). Any existing
    /// callbacks that are *not* set on the new config are left intact. In other words, this allows
    /// you to set new callbacks without removing the existing ones.
    ///
    /// # Parameters:
    ///
    /// - Widget ID or path: &str
    /// - New CallbackConfig: CallbackConfig
    ///
    /// # Example:
    ///
    /// We'll add a callback for a button with id "my_button".
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let my_callback = |context: Context| {
    ///     let state = context.state_tree.get_mut("my_button").as_button_mut();
    ///     state.set_disabled(true);
    ///     state.update(context.scheduler);
    /// };
    ///
    /// let callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
    /// scheduler.update_callback_config("my_button", callback_config);
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn update_callback_config(&mut self, for_widget: &str, callback_config: CallbackConfig) {
        if !self.synced {
            self.backend
                .updated_callback_configs
                .push((for_widget.to_string(), callback_config));
        } else {
            self.update_callback_config_sender
                .as_ref()
                .unwrap()
                .send((for_widget.to_string(), callback_config))
                .unwrap();
        }
    }

    /// Create a widget from a template or base widget type and add it to a layout. This allows you to
    /// create widgets from code.
    ///
    ///
    /// # Parameters:
    ///
    /// - Widget type or template: &str
    /// - ID of new widget: &str
    /// - ID or path of parent layout: &str
    /// - State tree: &mut StateTree
    ///
    /// # Example:
    ///
    /// We'll add labels to a layout from a for loop; after creating the labels we'll alter the text of
    /// each label based on the for loop number:
    ///
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let parent_id = "my_layout";
    ///
    /// for x in 1..=10 {
    ///
    ///     let new_id = format!("label_{}", x).as_str();
    ///     let (new_widget, mut new_states) =
    ///             scheduler.prepare_create_widget("Label", new_id, parent_id, &mut state_tree);
    ///
    ///     new_states.as_label_mut().set_text(format!("Hello world {}!", x));
    ///
    ///     scheduler.create_widget(new_widget, new_states, &mut state_tree);
    ///
    /// }
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn prepare_create_widget(
        &mut self,
        widget_type: &str,
        id: &str,
        parent: &str,
        state_tree: &mut StateTree,
    ) -> (EzObjects, StateTree) {
        let path = if !parent.contains('/') {
            /// parent is an ID, resolve it
            state_tree.get(parent).as_generic().get_path().clone()
        } else {
            parent.to_string()
        };
        let new_path = format!("{}/{}", path, id);
        let base_type;
        let new_widget;
        if self.backend.templates.contains_key(widget_type) {
            new_widget = self
                .backend
                .templates
                .get_mut(widget_type)
                .unwrap()
                .clone()
                .parse(self, path.to_string(), 0, Some(vec![format!("id: {}", id)]));
        } else {
            base_type = widget_type.to_string();
            let new_state = EzState::from_string(&base_type, new_path.to_string(), self);
            new_widget = EzObjects::from_string(
                &base_type,
                new_path.to_string(),
                id.to_string(),
                self,
                new_state,
            );
        };
        let mut new_states = StateTree::new(id.to_string(), new_widget.as_ez_object().get_state());
        if let EzObjects::Layout(ref i) = new_widget {
            for child in i.get_widgets_recursive() {
                let relative_path = child
                    .as_ez_object()
                    .get_path()
                    .split_once(new_widget.as_ez_object().get_id().as_str())
                    .unwrap()
                    .1
                    .to_string();
                let widget_path =
                    format!("{}{}", new_widget.as_ez_object().get_id(), relative_path);
                new_states.add_node(widget_path, child.as_ez_object().get_state());
            }
        }
        (new_widget, new_states)
    }

    /// Finish creating a widget from a template or base widget type and add it to a layout.
    /// This allows you to create widgets from code.
    ///
    /// # Parameters:
    ///
    /// - new_widget: EzObject (get it from the 'prepare_create_widget' method)
    /// - new_states: StateTree (get it from the 'prepare_create_widget' method)
    /// - State tree: &mut StateTree
    ///
    /// # Example:
    ///
    /// We'll add labels to a layout from a for loop; after creating the labels we'll alter the text of
    /// each label based on the for loop number:
    ///
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let parent_id = "my_layout";
    ///
    /// for x in 1..=10 {
    ///
    ///     let new_id = format!("label_{}", x).as_str();
    ///     let (new_widget, mut new_states) =
    ///             scheduler.prepare_create_widget("Label", new_id, parent_id, &mut state_tree);
    ///
    ///     new_states.as_label_mut().set_text(format!("Hello world {}!", x));
    ///
    ///     scheduler.create_widget(new_widget, new_states, &mut state_tree);
    ///
    /// }
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn create_widget(
        &mut self,
        new_widget: EzObjects,
        new_states: StateTree,
        state_tree: &mut StateTree,
    ) {
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
            self.create_widget_sender
                .as_ref()
                .unwrap()
                .send((new_widget, new_states))
                .unwrap();
            self.new_receivers_sender
                .as_ref()
                .unwrap()
                .send(receivers)
                .unwrap();
            self.new_properties_sender
                .as_ref()
                .unwrap()
                .send(self.backend.properties.clone())
                .unwrap();
        }
    }

    /// Remove a widget from a layout. Removing a layout will also remove all children of that layout.
    /// You cannot remove the root layout. You cannot remove a modal root, use scheduler.dismiss_modal
    /// instead.
    ///
    ///
    /// # Parameters:
    ///
    /// - ID or path of widget to remove: &str
    ///
    /// # Example:
    ///
    /// We'll remove labels from a layout from a for loop:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// for x in 1..=10 {
    ///
    ///     let label_id = format!("label_{}", x).as_str();
    ///     scheduler.remove_widget(label_id);
    /// }
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn remove_widget(&mut self, name: &str) {
        if !self.synced {
            if name == "root" || name == "/root" {
                panic!("Cannot remove the root layout")
            } else if name == "modal" || name == "/root/modal" {
                panic!("Cannot remove modal widget; use scheduler.dismiss_modal instead.")
            }
            self.backend.widgets_to_remove.push(name.to_string());
        } else {
            self.remove_widget_sender
                .as_ref()
                .unwrap()
                .send(name.to_string())
                .unwrap();
        }
    }

    fn get_update_func(&mut self, name: &str) {
        if name.contains('/') {
            let (widget, property_name) = name.rsplit_once('/').unwrap();
            let (widget, property_name) = (widget.to_string(), property_name.to_string());
            let updater = move |state_tree: &mut StateTree, val: EzValues| {
                state_tree
                    .get_mut(&widget)
                    .as_generic_mut()
                    .update_property(&property_name, val);
            };
            if !self.backend.property_updaters.contains_key(name) {
                self.backend
                    .property_updaters
                    .insert(name.to_string(), Box::new(updater));
            }
        }
    }

    /// Create a custom property. You can bind this property to widget properties of the same type;
    /// then when you update the custom property, the widget will update automatically as well.
    /// The name of custom properties may not contain any '/'.
    /// For a tutorial on this see: [Creating custom properties](#scheduler_properties).
    ///
    /// ## Parameters:
    ///
    /// - Name of the new property: &str
    /// - Value of the new property: usize
    ///
    /// ## Example:
    ///
    /// We'll create a custom property and bind it to a widget in an .ez file.
    /// First the code:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.new_usize_property("my_property", 1);
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    ///
    /// Now the .ez file:
    ///
    /// ```
    /// - Layout:
    ///     - ProgressBar:
    ///         value: properties.my_property
    /// ```
    pub fn new_usize_property(&mut self, name: &str, value: usize) -> EzProperty<usize> {
        let (property, receiver) = EzProperty::new(name.to_string(), value);
        self.backend
            .properties
            .insert(name.to_string(), EzProperties::Usize(property.clone()));
        self.backend
            .property_receivers
            .insert(name.to_string(), receiver);
        self.get_update_func(name);
        property
    }

    /// Create a custom property. You can bind this property to widget properties of the same type;
    /// then when you update the custom property, the widget will update automatically as well.
    /// The name of custom properties may not contain any '/'.
    /// For a tutorial on this see: [Creating custom properties](#scheduler_properties).
    ///
    /// ## Parameters:
    ///
    /// - Name of the new property: &str
    /// - Value of the new property: f64
    ///
    /// ## Example:
    ///
    /// We'll create a custom property and bind it to a widget in an .ez file.
    /// First the code:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.new_f64_property("my_property", 0.5);
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    /// Now the .ez file:
    /// ```
    /// - Layout:
    ///     scroll_y: true
    ///     scroll_start_y: properties.my_property
    /// ```
    pub fn new_f64_property(&mut self, name: &str, value: f64) -> EzProperty<f64> {
        let (property, receiver) = EzProperty::new(name.to_string(), value);
        self.backend
            .properties
            .insert(name.to_string(), EzProperties::F64(property.clone()));
        self.backend
            .property_receivers
            .insert(name.to_string(), receiver);
        self.get_update_func(name);
        property
    }

    /// Create a custom property. You can bind this property to widget properties of the same type;
    /// then when you update the custom property, the widget will update automatically as well.
    /// The name of custom properties may not contain any '/'.
    /// For a tutorial on this see: [Creating custom properties](#scheduler_properties).
    ///
    /// ## Parameters:
    ///
    /// - Name of the new property: &str
    /// - Value of the new property: String
    ///
    /// ## Example:
    ///
    /// We'll create a custom property and bind it to a widget in an .ez file.
    /// First the code:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.new_string_property("my_property", "Hello world!".to_string());
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    /// Now the .ez file:
    /// ```
    /// - Layout:
    ///     - Label:
    ///         text: properties.my_property
    /// ```
    pub fn new_string_property(&mut self, name: &str, value: String) -> EzProperty<String> {
        let (property, receiver) = EzProperty::new(name.to_string(), value);
        self.backend
            .properties
            .insert(name.to_string(), EzProperties::String(property.clone()));
        self.backend
            .property_receivers
            .insert(name.to_string(), receiver);
        self.get_update_func(name);
        property
    }

    /// Create a custom property. You can bind this property to widget properties of the same type;
    /// then when you update the custom property, the widget will update automatically as well.
    /// The name of custom properties may not contain any '/'.
    /// For a tutorial on this see: [Creating custom properties](#scheduler_properties).
    ///
    /// ## Parameters:
    ///
    /// - Name of the new property: &str
    /// - Value of the new property: bool
    ///
    /// ## Example:
    ///
    /// We'll create a custom property and bind it to a widget in an .ez file.
    /// First the code:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.new_bool_property("my_property", true);
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    /// Now the .ez file:
    /// ```
    /// - Layout:
    ///     - Button:
    ///         disabled: properties.my_property
    /// ```
    pub fn new_bool_property(&mut self, name: &str, value: bool) -> EzProperty<bool> {
        let (property, receiver) = EzProperty::new(name.to_string(), value);
        self.backend
            .properties
            .insert(name.to_string(), EzProperties::Bool(property.clone()));
        self.backend
            .property_receivers
            .insert(name.to_string(), receiver);
        self.get_update_func(name);
        property
    }

    /// Create a custom property. You can bind this property to widget properties of the same type;
    /// then when you update the custom property, the widget will update automatically as well.
    /// The name of custom properties may not contain any '/'.
    /// For a tutorial on this see: [Creating custom properties](#scheduler_properties).
    ///
    /// ## Parameters:
    ///
    /// - Name of the new property: &str
    /// - Value of the new property: Color
    ///
    /// ## Example:
    ///
    /// We'll create a custom property and bind it to a widget in an .ez file.
    /// First the code:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.new_color_property("my_property", Color::Yellow);
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    /// Now the .ez file:
    /// ```
    /// - Layout:
    ///     - Label:
    ///         fg_color: properties.my_property
    /// ```
    pub fn new_color_property(&mut self, name: &str, value: Color) -> EzProperty<Color> {
        let (property, receiver) = EzProperty::new(name.to_string(), value);
        self.backend
            .properties
            .insert(name.to_string(), EzProperties::Color(property.clone()));
        self.backend
            .property_receivers
            .insert(name.to_string(), receiver);
        self.get_update_func(name);
        property
    }

    /// Create a custom property. You can bind this property to widget properties of the same type;
    /// then when you update the custom property, the widget will update automatically as well.
    /// The name of custom properties may not contain any '/'.
    /// For a tutorial on this see: [Creating custom properties](#scheduler_properties).
    ///
    /// ## Parameters:
    ///
    /// - Name of the new property: &str
    /// - Value of the new property: LayoutMode
    ///
    /// ## Example:
    ///
    /// We'll create a custom property and bind it to a widget in an .ez file.
    /// First the code:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.new_layout_mode_property("my_property", LayoutMode::Table);
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    /// Now the .ez file:
    /// ```
    /// - Layout:
    ///     mode: properties.my_property
    /// ```
    pub fn new_layout_mode_property(
        &mut self,
        name: &str,
        value: LayoutMode,
    ) -> EzProperty<LayoutMode> {
        let (property, receiver) = EzProperty::new(name.to_string(), value);
        self.backend
            .properties
            .insert(name.to_string(), EzProperties::LayoutMode(property.clone()));
        self.backend
            .property_receivers
            .insert(name.to_string(), receiver);
        self.get_update_func(name);
        property
    }

    /// Create a custom property. You can bind this property to widget properties of the same type;
    /// then when you update the custom property, the widget will update automatically as well.
    /// The name of custom properties may not contain any '/'.
    /// For a tutorial on this see: [Creating custom properties](#scheduler_properties).
    ///
    /// ## Parameters:
    ///
    /// - Name of the new property: &str
    /// - Value of the new property: LayoutOrientation
    ///
    /// ## Example:
    ///
    /// We'll create a custom property and bind it to a widget in an .ez file.
    /// First the code:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.new_layout_orientation_property("my_property", LayoutOrientation::Vertical);
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    /// Now the .ez file:
    /// ```
    /// - Layout:
    ///     orientation: properties.my_property
    /// ```
    pub fn new_layout_orientation_property(
        &mut self,
        name: &str,
        value: LayoutOrientation,
    ) -> EzProperty<LayoutOrientation> {
        let (property, receiver) = EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(
            name.to_string(),
            EzProperties::LayoutOrientation(property.clone()),
        );
        self.backend
            .property_receivers
            .insert(name.to_string(), receiver);
        self.get_update_func(name);
        property
    }

    /// Create a custom property. You can bind this property to widget properties of the same type;
    /// then when you update the custom property, the widget will update automatically as well.
    /// The name of custom properties may not contain any '/'.
    /// For a tutorial on this see: [Creating custom properties](#scheduler_properties).
    ///
    /// ## Parameters:
    ///
    /// - Name of the new property: &str
    /// - Value of the new property: HorizontalAlignment
    ///
    /// ## Example:
    ///
    /// We'll create a custom property and bind it to a widget in an .ez file.
    /// First the code:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.new_horizontal_alignment_property("my_property", HorizontalAlignment::Center);
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    /// Now the .ez file:
    /// ```
    /// - Layout:
    ///     - Label:
    ///         halign: properties.my_property
    /// ```
    pub fn new_vertical_alignment_property(
        &mut self,
        name: &str,
        value: VerticalAlignment,
    ) -> EzProperty<VerticalAlignment> {
        let (property, receiver) = EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(
            name.to_string(),
            EzProperties::VerticalAlignment(property.clone()),
        );
        self.backend
            .property_receivers
            .insert(name.to_string(), receiver);
        self.get_update_func(name);
        property
    }

    /// Create a custom property. You can bind this property to widget properties of the same type;
    /// then when you update the custom property, the widget will update automatically as well.
    /// The name of custom properties may not contain any '/'.
    /// For a tutorial on this see: [Creating custom properties](#scheduler_properties).
    ///
    /// ## Parameters:
    ///
    /// - Name of the new property: &str
    /// - Value of the new property: VerticalAlignment
    ///
    /// ## Example:
    ///
    /// We'll create a custom property and bind it to a widget in an .ez file.
    /// First the code:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.new_vertical_alignment_property("my_property", VerticalAlignment::Middle);
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    /// Now the .ez file:
    /// ```
    /// - Layout:
    ///     - Label:
    ///         valign: properties.my_property
    /// ```
    pub fn new_horizontal_alignment_property(
        &mut self,
        name: &str,
        value: HorizontalAlignment,
    ) -> EzProperty<HorizontalAlignment> {
        let (property, receiver) = EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(
            name.to_string(),
            EzProperties::HorizontalAlignment(property.clone()),
        );
        self.backend
            .property_receivers
            .insert(name.to_string(), receiver);
        self.get_update_func(name);
        property
    }

    /// Create a custom property. You can bind this property to widget properties of the same type;
    /// then when you update the custom property, the widget will update automatically as well.
    /// The name of custom properties may not contain any '/'.
    /// For a tutorial on this see: [Creating custom properties](#scheduler_properties).
    ///
    /// ## Parameters:
    ///
    /// - Name of the new property: &str
    /// - Value of the new property: HorizontalPosHint
    ///
    /// ## Example:
    ///
    /// We'll create a custom property and bind it to a widget in an .ez file.
    /// First the code:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.new_horizontal_pos_hint_property("my_property",
    ///                                            Some((HorizontalAlignment::Right, 0.75)));
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    /// Now the .ez file:
    /// ```
    /// - Layout:
    ///     mode: float
    ///     - Label:
    ///         pos_hint_x: properties.my_property
    /// ```
    pub fn new_horizontal_pos_hint_property(
        &mut self,
        name: &str,
        value: HorizontalPosHint,
    ) -> EzProperty<HorizontalPosHint> {
        let (property, receiver) = EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(
            name.to_string(),
            EzProperties::HorizontalPosHint(property.clone()),
        );
        self.backend
            .property_receivers
            .insert(name.to_string(), receiver);
        self.get_update_func(name);
        property
    }

    /// Create a custom property. You can bind this property to widget properties of the same type;
    /// then when you update the custom property, the widget will update automatically as well.
    /// The name of custom properties may not contain any '/'.
    /// For a tutorial on this see: [Creating custom properties](#scheduler_properties).
    ///
    /// ## Parameters:
    ///
    /// - Name of the new property: &str
    /// - Value of the new property: VerticalPosHint
    ///
    /// ## Example:
    ///
    /// We'll create a custom property and bind it to a widget in an .ez file.
    /// First the code:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.new_vertical_pos_hint_property("my_property",
    ///                                            Some((VerticalAlignment::Middle, 0.75)));
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    /// Now the .ez file:
    /// ```
    /// - Layout:
    ///     mode: float
    ///     - Label:
    ///         pos_hint_y: properties.my_property
    /// ```
    pub fn new_vertical_pos_hint_property(
        &mut self,
        name: &str,
        value: VerticalPosHint,
    ) -> EzProperty<VerticalPosHint> {
        let (property, receiver) = EzProperty::new(name.to_string(), value);
        self.backend.properties.insert(
            name.to_string(),
            EzProperties::VerticalPosHint(property.clone()),
        );
        self.backend
            .property_receivers
            .insert(name.to_string(), receiver);
        self.get_update_func(name);
        property
    }
    /// Create a custom property. You can bind this property to widget properties of the same type;
    /// then when you update the custom property, the widget will update automatically as well.
    /// The name of custom properties may not contain any '/'.
    /// For a tutorial on this see: [Creating custom properties](#scheduler_properties).
    ///
    /// ## Parameters:
    ///
    /// - Name of the new property: &str
    /// - Value of the new property: SizeHint
    ///
    /// ## Example:
    ///
    /// We'll create a custom property and bind it to a widget in an .ez file.
    /// First the code:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.new_size_hint_property("my_property", Some(0.75));
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    /// Now the .ez file:
    /// ```
    /// - Layout:
    ///     mode: float
    ///     - Label:
    ///         size_hint_x: properties.my_property
    /// ```
    pub fn new_size_hint_property(
        &mut self,
        name: &str,
        value: Option<f64>,
    ) -> EzProperty<Option<f64>> {
        let (property, receiver) = EzProperty::new(name.to_string(), value);
        self.backend
            .properties
            .insert(name.to_string(), EzProperties::SizeHint(property.clone()));
        self.backend
            .property_receivers
            .insert(name.to_string(), receiver);
        self.get_update_func(name);
        property
    }

    /// Get a reference to a custom property.
    ///
    /// # Parameters:
    ///
    /// - Name of the custom property: &str
    ///
    /// # Example:
    ///
    /// We'll retrieve a custom property:
    ///
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.new_usize_property("my_property", 10);
    /// let property = scheduler.get_property("my_property");
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn get_property(&self, name: &str) -> &EzProperties {
        self.backend
            .properties
            .get(name)
            .unwrap_or_else(|| panic!("Could not find property: {}", name))
    }

    /// Get a mutable reference to a  custom property created earlier.
    ///
    /// # Parameters:
    ///
    /// - Name of the custom property: &str
    ///
    /// # Example:
    ///
    /// We'll retrieve a custom property and modify it:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.new_usize_property("my_property", 10);
    /// let property = scheduler.get_property_mut("my_property");
    /// property.set(20);
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn get_property_mut(&mut self, name: &str) -> &mut EzProperties {
        self.backend
            .properties
            .get_mut(name)
            .unwrap_or_else(|| panic!("Could not find property: {}", name))
    }

    /// As an end-user, use 'property.bind' or 'scheduler.bind_property.
    /// Subscribe one property to another, ensuring the subscriber will always have the value of the
    /// property it is subscribed to on the next frame. An update func is required which will be
    /// called when the property subscribed to changes. The update func receives the new value and
    /// is responsible for setting the appropriate field on the subscriber.
    pub fn subscribe_to_property(&mut self, name: &str, subscriber: String) {
        if !self.synced {
            if !self.backend.property_subscribers.contains_key(name) {
                self.backend
                    .property_subscribers
                    .insert(name.to_string(), Vec::new());
            }
            self.backend
                .property_subscribers
                .get_mut(name)
                .unwrap()
                .push(subscriber);
        } else {
            self.subscribe_to_property_sender
                .as_ref()
                .unwrap()
                .send((name.to_string(), subscriber))
                .unwrap();
        }
    }

    /// Schedule a widget to be redrawn on the next frame. If you are working with a widget state,
    /// it is usually more convenient to call "state.update" instead of this method. This method
    /// only accepts a full widget path, not an ID.
    ///
    /// # Parameters:
    ///
    /// - Path of the widget: &str
    ///
    /// # Example:
    ///
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.update_widget("/root/my_layout/my_label");
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn update_widget(&mut self, path: &str) {
        if !self.synced {
            if path.starts_with("/root/modal") {
                self.backend.force_redraw = true;
                return;
            }
            let path = path.to_string();
            if !self.backend.widgets_to_update.contains(&path) {
                self.backend.widgets_to_update.push(path);
            }
        } else {
            self.update_widget_sender
                .as_ref()
                .unwrap()
                .send(path.to_string())
                .unwrap();
        }
    }

    /// Forces a global screen redraw (though only changed pixels will actually be redrawn). While this
    /// method if exposed to give you the option to use it, this is generally not recommended for
    /// performance reasons. It's preferred to call updates on changed widgets, rather than global
    /// redraws.
    ///
    /// # Parameters:
    ///
    /// This method takes no parameters.
    ///
    /// # Example:
    ///
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.force_redraw();
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn force_redraw(&mut self) {
        if !self.synced {
            self.backend.force_redraw = true;
        } else {
            self.force_redraw_sender
                .as_ref()
                .unwrap()
                .send(true)
                .unwrap();
        }
    }

    /// Bind a callback to a property (can be a widget property or a custom property). Whenever the
    /// property value changes, the callback is called. This method only takes a full property path
    /// (e.g. "/root/layout/my_label/width"); it is usually more convenient to get the property from
    /// the widget state, and then call "property.bind".
    ///
    /// # Parameters:
    ///
    /// - property path: &str
    /// - callback function: Box<dyn FnMut(Context)>
    ///
    /// # Example:
    ///
    /// Let's bind a property; we'll show how to do it from the scheduler, and how to do it from the
    /// property itself. We'll create a callback that displays the width of a label in its text:
    ///
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// fn my_callback(context: Context) {
    ///     let state = context.state_tree.get_mut("my_label").as_label_mut();
    ///     let width = state.get_size().get_width();
    ///     state.set_text(format!("My width is: {}", width));
    /// }
    ///
    /// // bind from scheduler
    /// scheduler.bind_property_callback("/root/layout/my_label/width", Box::new(my_callback));
    ///
    /// // bind from property
    ///     let state = state_tree.get_mut("my_label").as_label_mut();
    ///     state.size.width.bind(Box::new(my_callback), &mut scheduler);
    ///
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn bind_property_callback(&mut self, name: &str, callback: GenericFunction) {
        if !self.synced {
            if self.backend.property_callbacks.contains(&name.to_string()) {
                self.backend
                    .new_property_callbacks
                    .push((name.to_string(), callback));
            } else {
                let mut config = CallbackConfig::default();
                config.property_callbacks.push(callback);
                let name = if !name.contains('/') {
                    format!("/root/{}", name)
                } else {
                    name.to_string()
                };
                self.overwrite_callback_config(&name, config);
                self.backend.property_callbacks.push(name);
            }
        } else {
            self.bind_property_sender
                .as_ref()
                .unwrap()
                .send((name.to_string(), callback))
                .unwrap();
        }
    }

    /// Bind a callback to a custom key being pressed anywhere in the UI. Global key binds take
    /// priority over widget key binds.
    ///
    /// # Parameters:
    ///
    /// - key: KeyCode
    /// - modifiers: Option<Vec<KeyModifiers>>
    /// - callback function: Box<dyn FnMut(Context, KeyCode, KeyModifiers)>
    ///
    /// # Example:
    ///
    /// We'll bind the key combination "shift + A" to change a label text:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// fn my_callback(context: Context) {
    ///     let state = context.state_tree.get_mut("my_label").as_label_mut();
    ///     state.set_text("Shift A was pressed!".to_string());
    /// }
    ///
    /// scheduler.bind_global_key(KeyCode::Char('a'), Some(vec!(KeyModifiers::SHIFT)),
    ///                           Box::new(my_callback));
    ///
    /// run(root_widget, state_tree, scheduler);
    pub fn bind_global_key(
        &mut self,
        key: KeyCode,
        modifiers: Option<Vec<KeyModifiers>>,
        callback: KeyboardCallbackFunction,
    ) {
        if !self.synced {
            let modifiers = create_keymap_modifiers(modifiers);
            self.backend
                .update_global_keymap
                .insert((key, modifiers), callback);
        } else {
            self.bind_global_key_sender
                .as_ref()
                .unwrap()
                .send((key, modifiers, callback))
                .unwrap();
        }
    }

    /// Remove one specific global key bind.
    ///
    /// # Parameters:
    ///
    /// - key: KeyCode
    /// - modifiers: Option<Vec<KeyModifiers>>
    ///
    /// # Example:
    ///
    /// We'll remove the key combination "shift + A" from the global key binds:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.remove_global_key(KeyCode::Char('a'), Some(vec!(KeyModifiers::SHIFT)));
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn remove_global_key(&mut self, key: KeyCode, modifiers: Option<Vec<KeyModifiers>>) {
        if !self.synced {
            let modifiers = create_keymap_modifiers(modifiers);
            self.backend.remove_global_keymap.push((key, modifiers));
        } else {
            self.remove_global_key_sender
                .as_ref()
                .unwrap()
                .send((key, modifiers))
                .unwrap();
        }
    }

    /// Remove all global key binds.
    /// For a tutorial on this see: [Managing callbacks](#scheduler_callbacks)
    ///
    /// # Parameters:
    ///
    /// This method takes no parameters.
    ///
    /// # Example:
    ///
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    ///
    /// scheduler.clear_global_keys();
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn clear_global_keys(&mut self) {
        if !self.synced {
            self.backend.clear_global_keymap = true;
        } else {
            self.clear_global_keys_sender
                .as_ref()
                .unwrap()
                .send(true)
                .unwrap();
        }
    }

    /// Users can select certain widget types through mouse or keyboard. This method allows selecting
    /// widgets programmatically.
    ///
    ///
    /// # Parameters:
    ///
    /// - Widget ID or path: &str
    /// - Optional mouse coordinates for selection: Option<Coordinates>
    ///
    /// # Example:
    ///
    /// We'll select a widget once with no mouse_pos, and once with mouse_pos:
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.set_selected_widget("my_button", None);
    /// scheduler.set_selected_widget("my_button", Some(Coordinates::new(3, 1)));
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn set_selected_widget(&mut self, widget: &str, mouse_pos: Option<Coordinates>) {
        if !self.synced {
            self.backend.next_selection = Some((widget.to_string(), mouse_pos));
        } else {
            self.set_selected_widget_sender
                .as_ref()
                .unwrap()
                .send((widget.to_string(), mouse_pos))
                .unwrap();
        }
    }

    /// Users can select certain widget types through mouse or keyboard. This method allows deselecting
    /// the current widget programmatically.
    /// For a tutorial on this see: [Managing widget selection](#scheduler_selection)
    ///
    /// # Parameters:
    ///
    /// This method takes no parameters.
    ///
    /// # Example:
    ///
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.deselect_widget();
    ///
    /// run(root_widget, state_tree, scheduler);
    /// ```
    pub fn deselect_widget(&mut self) {
        if !self.synced {
            self.backend.deselect = true
        } else {
            self.deselect_widget_sender
                .as_ref()
                .unwrap()
                .send(true)
                .unwrap();
        }
    }

    /// Exit the program gracefully. EzTerm makes several changes to the terminal to display the UI,
    /// so if you do not exit gracefully it may leave the terminal in an unusable state.
    ///
    /// # Parameters:
    ///
    /// This method takes no parameters.
    ///
    /// # Example:
    ///
    /// ```
    /// use ez_term::*;
    ///
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// scheduler.exit();
    /// ```
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
                    Ok(i) => received = Some(i),
                    Err(_) => break,
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
                    Ok(i) => received = Some(i),
                    Err(_) => break,
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
        self.ask_sync_state_tree_sender
            .as_ref()
            .unwrap()
            .send(true)
            .unwrap();
        self.sync_state_tree_receiver
            .as_ref()
            .unwrap()
            .recv()
            .unwrap()
    }

    /// Sync (custom) properties from the main thread. The properties cannot be shared across threads;
    /// therefore each thread has its own properties. Changes made from a thread will be synced
    /// to the main thread, however changes made in the main thread will not be synced to the
    /// other thread(s). Often it is not important to have up to date properties in your thread,
    /// but if it is necessary at any point, call this method first.
    pub fn sync_properties(&mut self) {
        self.ask_sync_properties_sender
            .as_ref()
            .unwrap()
            .send(true)
            .unwrap();
        self.backend.properties = self
            .sync_properties_receiver
            .as_ref()
            .unwrap()
            .recv()
            .unwrap();
    }

    pub fn _check_method_channels(&mut self, state_tree: &mut StateTree) {
        if self.syncing == 0 {
            return;
        }

        while let Ok((name, func, after)) = self.schedule_once_receiver.as_ref().unwrap().try_recv()
        {
            self.schedule_once(&name, func, after);
        }
        while let Ok((name, func, interval)) = self
            .schedule_recurring_receiver
            .as_ref()
            .unwrap()
            .try_recv()
        {
            self.schedule_recurring(&name, func, interval);
        }
        while let Ok((func, on_finish)) =
            self.schedule_threaded_receiver.as_ref().unwrap().try_recv()
        {
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
        while let Ok((for_widget, callback_config)) = self
            .overwrite_callback_config_receiver
            .as_ref()
            .unwrap()
            .try_recv()
        {
            self.overwrite_callback_config(for_widget.as_str(), callback_config);
        }
        while let Ok((for_widget, callback_config)) = self
            .update_callback_config_receiver
            .as_ref()
            .unwrap()
            .try_recv()
        {
            self.update_callback_config(for_widget.as_str(), callback_config);
        }
        while let Ok((new_widget, new_states)) =
            self.create_widget_receiver.as_ref().unwrap().try_recv()
        {
            self.create_widget(new_widget, new_states, state_tree);
        }
        while let Ok(name) = self.remove_widget_receiver.as_ref().unwrap().try_recv() {
            self.remove_widget(name.as_str());
        }
        while let Ok((name, value)) = self
            .new_usize_property_receiver
            .as_ref()
            .unwrap()
            .try_recv()
        {
            self.new_usize_property(name.as_str(), value);
        }
        while let Ok((name, value)) = self.new_f64_property_receiver.as_ref().unwrap().try_recv() {
            self.new_f64_property(name.as_str(), value);
        }
        while let Ok((name, value)) = self
            .new_string_property_receiver
            .as_ref()
            .unwrap()
            .try_recv()
        {
            self.new_string_property(name.as_str(), value);
        }
        while let Ok((name, value)) = self
            .new_color_property_receiver
            .as_ref()
            .unwrap()
            .try_recv()
        {
            self.new_color_property(name.as_str(), value);
        }
        while let Ok((name, value)) = self.new_bool_property_receiver.as_ref().unwrap().try_recv() {
            self.new_bool_property(name.as_str(), value);
        }
        while let Ok((name, value)) = self
            .new_layout_mode_property_receiver
            .as_ref()
            .unwrap()
            .try_recv()
        {
            self.new_layout_mode_property(name.as_str(), value);
        }
        while let Ok((name, value)) = self
            .new_layout_orientation_property_receiver
            .as_ref()
            .unwrap()
            .try_recv()
        {
            self.new_layout_orientation_property(name.as_str(), value);
        }
        while let Ok((name, value)) = self
            .new_horizontal_alignment_property_receiver
            .as_ref()
            .unwrap()
            .try_recv()
        {
            self.new_horizontal_alignment_property(name.as_str(), value);
        }
        while let Ok((name, value)) = self
            .new_vertical_alignment_property_receiver
            .as_ref()
            .unwrap()
            .try_recv()
        {
            self.new_vertical_alignment_property(name.as_str(), value);
        }
        while let Ok((name, value)) = self
            .new_horizontal_pos_hint_property_receiver
            .as_ref()
            .unwrap()
            .try_recv()
        {
            self.new_horizontal_pos_hint_property(name.as_str(), value);
        }
        while let Ok((name, value)) = self
            .new_vertical_pos_hint_property_receiver
            .as_ref()
            .unwrap()
            .try_recv()
        {
            self.new_vertical_pos_hint_property(name.as_str(), value);
        }
        while let Ok((name, value)) = self
            .new_size_hint_property_receiver
            .as_ref()
            .unwrap()
            .try_recv()
        {
            self.new_size_hint_property(name.as_str(), value);
        }
        while let Ok((name, update_func)) = self
            .subscribe_to_property_receiver
            .as_ref()
            .unwrap()
            .try_recv()
        {
            self.subscribe_to_property(name.as_str(), update_func);
        }
        while let Ok(name) = self.update_widget_receiver.as_ref().unwrap().try_recv() {
            self.update_widget(name.as_str());
        }
        while let Ok(_) = self.force_redraw_receiver.as_ref().unwrap().try_recv() {
            self.force_redraw();
        }
        while let Ok((name, func)) = self.bind_property_receiver.as_ref().unwrap().try_recv() {
            self.bind_property_callback(name.as_str(), func);
        }
        while let Ok((key, modifier, func)) =
            self.bind_global_key_receiver.as_ref().unwrap().try_recv()
        {
            self.bind_global_key(key, modifier, func);
        }
        while let Ok((key, modifier)) = self.remove_global_key_receiver.as_ref().unwrap().try_recv()
        {
            self.remove_global_key(key, modifier);
        }
        while let Ok(_) = self.clear_global_keys_receiver.as_ref().unwrap().try_recv() {
            self.clear_global_keys();
        }
        while let Ok((name, mouse_pos)) = self
            .set_selected_widget_receiver
            .as_ref()
            .unwrap()
            .try_recv()
        {
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
        synced_frontend.overwrite_callback_config_sender =
            self.overwrite_callback_config_sender.clone();

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
        synced_frontend.new_layout_mode_property_sender =
            self.new_layout_mode_property_sender.clone();

        if self.new_layout_orientation_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_layout_orientation_property_receiver = Some(receiver);
            self.new_layout_orientation_property_sender = Some(sender.clone());
        }
        synced_frontend.new_layout_orientation_property_sender =
            self.new_layout_orientation_property_sender.clone();

        if self.new_horizontal_alignment_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_horizontal_alignment_property_receiver = Some(receiver);
            self.new_horizontal_alignment_property_sender = Some(sender.clone());
        }
        synced_frontend.new_horizontal_alignment_property_sender =
            self.new_horizontal_alignment_property_sender.clone();

        if self.new_vertical_alignment_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_vertical_alignment_property_receiver = Some(receiver);
            self.new_vertical_alignment_property_sender = Some(sender.clone());
        }
        synced_frontend.new_vertical_alignment_property_sender =
            self.new_vertical_alignment_property_sender.clone();

        if self.new_horizontal_pos_hint_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_horizontal_pos_hint_property_receiver = Some(receiver);
            self.new_horizontal_pos_hint_property_sender = Some(sender.clone());
        }
        synced_frontend.new_horizontal_pos_hint_property_sender =
            self.new_horizontal_pos_hint_property_sender.clone();

        if self.new_vertical_pos_hint_property_receiver.is_none() {
            let (sender, receiver) = channel();
            self.new_vertical_pos_hint_property_receiver = Some(receiver);
            self.new_vertical_pos_hint_property_sender = Some(sender.clone());
        }
        synced_frontend.new_vertical_pos_hint_property_sender =
            self.new_vertical_pos_hint_property_sender.clone();

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
    pub fn new(name: String, func: GenericTask, delay: Duration) -> Self {
        Task {
            name,
            func,
            delay,
            canceled: false,
            created: Instant::now(),
        }
    }

    pub fn cancel(&mut self) {
        self.canceled = true;
    }
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
    pub fn new(name: String, func: GenericRecurringTask, interval: Duration) -> Self {
        RecurringTask {
            name,
            func,
            interval,
            canceled: false,
            last_execution: None,
        }
    }

    pub fn cancel(&mut self) {
        self.canceled = true;
    }
}
