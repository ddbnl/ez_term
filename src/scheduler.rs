use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::thread::{JoinHandle, spawn};
use std::time::{Duration, Instant};
use crate::{CallbackConfig, EzContext, run};
use crate::common::definitions::{CallbackTree, GenericEzTask, StateTree, ViewTree, WidgetTree,
                                 EzThread, EzPropertyUpdater};
use crate::property::{EzProperties, EzValues, EzProperty};


#[derive(Default)]
pub struct Scheduler {
    tasks: Vec<Task>,
    pub widgets_to_update: Vec<String>,
    pub force_redraw: bool,
    threads_to_start: Vec<(EzThread, Option<GenericEzTask>)>,
    thread_handles: Vec<(JoinHandle<()>, Option<GenericEzTask>)>,
    new_callback_configs: Vec<(String, CallbackConfig)>,
    updated_callback_configs: Vec<(String, CallbackConfig)>,
    pub properties: HashMap<String, EzProperties>,
    property_receivers: HashMap<String, Receiver<EzValues>>,
    property_subscribers: HashMap<String, Vec<EzPropertyUpdater>>,
    property_callbacks: HashMap<String, Vec<Box<dyn FnMut(EzContext)>>>,
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

    pub fn schedule_once(&mut self, widget: String, func: GenericEzTask,
                         after: Duration) {
        let task = Task::new(widget, func, false, after,
                             Some(Instant::now()));
        self.tasks.push(task);
    }

    pub fn schedule_interval(&mut self, widget: String,  func: GenericEzTask,
                             interval: Duration)
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

    pub fn update_threads(&mut self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                          widget_tree: &WidgetTree) {

        self.start_new_threads();
        self.check_finished_threads(view_tree, state_tree, widget_tree)
    }

    pub fn start_new_threads(&mut self) {

        while !self.threads_to_start.is_empty() {
            let (thread_func, on_finish) =
                self.threads_to_start.pop().unwrap();

            let properties = self.properties.clone();
            let handle: JoinHandle<()> = spawn(move || thread_func(properties));
            self.thread_handles.push((handle, on_finish))
        }
    }

    pub fn check_finished_threads(&mut self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                                  widget_tree: &WidgetTree) {

        let mut finished = Vec::new();
        for (i, (handle, _)) in self.thread_handles.iter_mut().enumerate() {
            if handle.is_finished() {
                finished.push(i);
            }
        }
        for i in finished {
            let (handle, on_finish) = self.thread_handles.remove(i);
            if let Some(mut func) = on_finish {
                let context = EzContext::new(String::new(), view_tree,
                                             state_tree, widget_tree, self);
                func(context);
            }
            handle.join().unwrap();
        }
    }

    pub fn run_tasks(&mut self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                     widget_tree: &WidgetTree, _callback_tree: &mut CallbackTree) {

        let mut remaining_tasks = Vec::new();
        while !self.tasks.is_empty() {
            let mut task = self.tasks.pop().unwrap();
            let context = EzContext::new(task.widget.clone(), view_tree,
                                         state_tree, widget_tree, self);

            if let Some(time) = task.last_execution {
                let elapsed = time.elapsed();
                if elapsed >= task.interval && !task.canceled {
                    let result = (task.func)(context);
                    task.last_execution = Some(Instant::now());
                    if task.recurring && result {
                        remaining_tasks.push(task);
                    }
                } else if !task.canceled {
                    remaining_tasks.push(task);
                }
            } else if !task.canceled {
                let result = (task.func)(context);
                task.last_execution = Some(Instant::now());
                if task.recurring && result {
                    remaining_tasks.push(task);
                }
            }
        }
        self.tasks = remaining_tasks;
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

    pub fn update_callback_configs(&mut self, callback_tree: &mut CallbackTree) {

        while !self.new_callback_configs.is_empty() {
            let (path, callback_config) =
                self.new_callback_configs.pop().unwrap();
            callback_tree.insert(path, callback_config);
        }
        while !self.updated_callback_configs.is_empty() {
            let (path_or_id, callback_config) =
                self.updated_callback_configs.pop().unwrap();
            if path_or_id.contains('/') {
                callback_tree.get_by_path_mut(&path_or_id).update_from(callback_config);
            } else {
                callback_tree.get_by_id_mut(&path_or_id).update_from(callback_config);
            }
        }
    }

    pub fn exit(&self) { run::stop(); }

    pub fn new_usize_property(&mut self, name: String, value: usize) -> EzProperty<usize> {

        let (property, receiver) =
            EzProperty::new(name.clone(), value);
        self.properties.insert(name.clone(), EzProperties::Usize(property.clone()));
        self.property_receivers.insert(name, receiver);
        property
    }

    pub fn new_string_property(&mut self, name: String, value: String) -> EzProperty<String> {

        let (property, receiver) =
            EzProperty::new(name.clone(), value);
        self.properties.insert(name.clone(), EzProperties::String(property.clone()));
        self.property_receivers.insert(name, receiver);
        property
    }

    pub fn update_properties(&mut self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                             widget_tree: &WidgetTree, callback_tree: &mut CallbackTree) {

        let mut to_update = Vec::new();
        let mut to_callback = Vec::new();

        for (name, update_funcs) in
                self.property_subscribers.iter_mut() {
            let receiver = self.property_receivers.get(name).unwrap();
            let mut new_val = None;
            // Drain all new values if any, we only care about the latest.
            while let Ok(new) = receiver.try_recv() {
                new_val = Some(new);
            }
            if let Some(val) = new_val {
                to_callback.push(name.clone());
                for update_func in update_funcs {
                    to_update.push(update_func(state_tree, val.clone()));
                }
            }
        }
        for name in to_callback {
            if callback_tree.objects.contains_key(&name) {
                let context =
                    EzContext::new(name.clone(), view_tree, state_tree,
                                   widget_tree,self);
                if let Some(ref mut callback) =
                        callback_tree.get_by_path_mut(&name).on_value_change {
                    callback(context);
                }
            }
        }
        self.widgets_to_update.extend(to_update);
    }

    pub fn subscribe_to_ez_property(&mut self, name: String, update_func: EzPropertyUpdater) {

        if !self.property_subscribers.contains_key(&name) {
            self.property_subscribers.insert(name.clone(), Vec::new());
        }
        self.property_subscribers.get_mut(&name).unwrap().push(update_func);
    }

    pub fn update_widget(&mut self, path: String) {
        if path.starts_with("/modal") {
            self.force_redraw = true;
            return
        }
        if !self.widgets_to_update.contains(&path) {
            self.widgets_to_update.push(path);
        }
    }

    pub fn force_redraw(&mut self) { self.force_redraw = true; }

    pub fn bind_ez_property_callback(&mut self, name: String, callback: Box<dyn FnMut(EzContext)>) {

        let callbacks =
            self.property_callbacks.entry(name).or_insert(Vec::new());
        callbacks.push(callback);
    }
}

impl Task {

    pub fn new(widget: String, func: GenericEzTask, recurring: bool,
               interval: Duration, last_execution: Option<Instant>)
        -> Self { Task { widget, func, recurring, interval, canceled: false, last_execution } }

    pub fn cancel(&mut self) { self.canceled = true; }

}