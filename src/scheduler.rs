use std::time::{Duration, Instant};
use crate::{CallbackConfig, EzContext, run};
use crate::common::definitions::{CallbackTree, GenericEzTask, StateTree, ViewTree, WidgetTree, EzThread, EzUsizeProperties, EzUsizePropertySubscribers, EzUsizePropertyUpdater, EzStringPropertySubscribers, EzStringProperties, EzStringPropertyUpdater};
use crate::property::{StringProperty, UsizeProperty};


#[derive(Default)]
pub struct Scheduler {
    tasks: Vec<Task>,
    pub widgets_to_update: Vec<String>,
    pub force_redraw: bool,
    threads_to_start: EzThread,
    thread_handles: Vec<(std::thread::JoinHandle<()>, Option<GenericEzTask>)>,
    new_callback_configs: Vec<(String, CallbackConfig)>,
    updated_callback_configs: Vec<(String, CallbackConfig)>,
    usize_properties: EzUsizeProperties,
    usize_property_subscribers: EzUsizePropertySubscribers,
    string_properties: EzStringProperties,
    string_property_subscribers: EzStringPropertySubscribers,
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

    pub fn schedule_threaded(&mut self, threaded_func: Box<dyn FnOnce(Vec<UsizeProperty>) + Send>,
                             on_finish: Option<GenericEzTask>) {
        self.threads_to_start.push((threaded_func, on_finish));
    }

    pub fn update_threads(&mut self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                          widget_tree: &WidgetTree, _callback_tree: &mut CallbackTree) {

        while !self.threads_to_start.is_empty() {
            let (thread_func, on_finish) = self.threads_to_start.pop().unwrap();
            let mut properties = Vec::new();
            for (property, _) in  self.usize_properties.values() {
                properties.push(property.clone());
            }
            let handle: std::thread::JoinHandle<()> = std::thread::spawn(move || thread_func(properties));
            self.thread_handles.push((handle, on_finish))
        }
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
            handle.join();
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
    pub fn set_callback_config(&mut self, widget_path: String, callback_config: CallbackConfig) {
        self.new_callback_configs.push((widget_path, callback_config));
    }

    /// Pass a callback config that will update the current callback config for the object on the
    /// next frame. Only sets new callbacks, cannot remove old ones.
    pub fn update_callback_config(&mut self, widget_path: String, callback_config: CallbackConfig) {
        self.updated_callback_configs.push((widget_path, callback_config));

    }

    pub fn update_callback_configs(&mut self, callback_tree: &mut CallbackTree) {

        while !self.new_callback_configs.is_empty() {
            let (path, callback_config) =
                self.new_callback_configs.pop().unwrap();
            callback_tree.insert(path, callback_config);
        }
        while !self.updated_callback_configs.is_empty() {
            let (path, callback_config) =
                self.updated_callback_configs.pop().unwrap();
            callback_tree.get_mut(&path).unwrap_or_else(
                || panic!("Cannot set new callback config for path \"{}\" as it cannot be resolved",
                path)).update_from(callback_config);
        }
    }

    pub fn exit(&self) { run::stop(); }

    pub fn new_usize_property(&mut self, name: String, value: usize) -> UsizeProperty {

        let (property, receiver) =
            UsizeProperty::new(name.clone(), value);
        self.usize_properties.insert(name, (property.clone(), receiver));
        property
    }

    pub fn new_string_property(&mut self, name: String, value: String) -> StringProperty {

        let (property, receiver) =
            StringProperty::new(name.clone(), value);
        self.string_properties.insert(name, (property.clone(), receiver));
        property
    }

    pub fn update_properties(&mut self, state_tree: &mut StateTree) {

        let mut to_update = Vec::new();

        for (name, update_funcs) in
                self.usize_property_subscribers.iter_mut() {
            let (_, receiver) = self.usize_properties.get(name).unwrap();
            if let Ok(new) = receiver.try_recv() {
                for update_func in update_funcs {
                    to_update.push(update_func(state_tree, new));
                }
            }
        }

        for (name, update_funcs) in
        self.string_property_subscribers.iter_mut() {
            let (_, receiver) = self.string_properties.get(name).unwrap();
            if let Ok(new) = receiver.try_recv() {
                for update_func in update_funcs {
                    to_update.push(update_func(state_tree, new.clone()));
                }
            }
        }

        self.widgets_to_update.extend(to_update);
    }

    pub fn subscribe_to_usize_callback(&mut self, name: String, update_func: EzUsizePropertyUpdater) {

        if !self.usize_property_subscribers.contains_key(&name) {
            self.usize_property_subscribers.insert(name.clone(), Vec::new());
        }
        self.usize_property_subscribers.get_mut(&name).unwrap().push(update_func);
    }

    pub fn subscribe_to_string_callback(&mut self, name: String, update_func: EzStringPropertyUpdater) {

        if !self.string_property_subscribers.contains_key(&name) {
            self.string_property_subscribers.insert(name.clone(), Vec::new());
        }
        self.string_property_subscribers.get_mut(&name).unwrap().push(update_func);
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
}

impl Task {

    pub fn new(widget: String, func: GenericEzTask, recurring: bool,
               interval: Duration, last_execution: Option<Instant>)
        -> Self { Task { widget, func, recurring, interval, canceled: false, last_execution } }

    pub fn cancel(&mut self) { self.canceled = true; }

}