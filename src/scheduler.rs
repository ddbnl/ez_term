use std::time::{Duration, Instant};
use crate::states;
use crate::common;


#[derive(Default)]
pub struct Scheduler {
    tasks: Vec<Task>,
    new_callback_configs: Vec<(String, states::definitions::CallbackConfig)>,
    updated_callback_configs: Vec<(String, states::definitions::CallbackConfig)>,
}


pub struct Task {
    pub widget: String,
    pub func: common::definitions::GenericEzTask,
    pub recurring: bool,
    pub canceled: bool,
    pub interval: Duration,
    pub last_execution: Option<Instant>,
}


impl Scheduler {

    pub fn schedule_once(&mut self, widget: String, func: common::definitions::GenericEzTask,
                         after: Duration) {
        let task = Task::new(widget, func, false, after,
                             Some(Instant::now()));
        self.tasks.push(task);
    }

    pub fn schedule_interval(&mut self, widget: String,  func: common::definitions::GenericEzTask,
                             interval: Duration)
        -> &mut Task {
        let task = Task::new(widget, func, true, interval, None);
        self.tasks.push(task);
        self.tasks.last_mut().unwrap()
    }

    pub fn run_tasks(&mut self, view_tree: &mut common::definitions::ViewTree,
                     state_tree: &mut common::definitions::StateTree,
                     widget_tree: &common::definitions::WidgetTree,
                     _callback_tree: &mut common::definitions::CallbackTree) {

        let mut remaining_tasks = Vec::new();
        while !self.tasks.is_empty() {
            let mut task = self.tasks.pop().unwrap();
            let context = common::definitions::EzContext::new(task.widget.clone(), view_tree,
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
    pub fn set_callback_config(&mut self, widget_path: String,
                                  callback_config: states::definitions::CallbackConfig) {
        self.new_callback_configs.push((widget_path, callback_config));
    }

    /// Pass a callback config that will update the current callback config for the object on the
    /// next frame. Only sets new callbacks, cannot remove old ones.
    pub fn update_callback_config(&mut self, widget_path: String,
                                  callback_config: states::definitions::CallbackConfig) {
        self.updated_callback_configs.push((widget_path, callback_config));

    }

    pub fn update_callback_configs(&mut self,
                                   callback_tree: &mut common::definitions::CallbackTree) {

        while !self.new_callback_configs.is_empty() {
            let (path, callback_config) =
                self.new_callback_configs.pop().unwrap();
            callback_tree.insert(path, callback_config);
        }
        while !self.updated_callback_configs.is_empty() {
            let (path, callback_config) =
                self.updated_callback_configs.pop().unwrap();
            callback_tree.get_mut(&path).unwrap().update_from(callback_config);
        }
    }
}

impl Task {

    pub fn new(widget: String, func: common::definitions::GenericEzTask, recurring: bool,
               interval: Duration, last_execution: Option<Instant>)
        -> Self {
        Task { widget, func, recurring, interval, canceled: false, last_execution }
    }

    pub fn cancel(&mut self) { self.canceled = true; }

}