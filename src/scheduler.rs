use std::time::{Duration, Instant};
use crate::common::{EzContext, GenericEzTask, StateTree, ViewTree, WidgetTree};

#[derive(Default)]
pub struct Scheduler {
    pub tasks: Vec<Task>,
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

    pub fn run_tasks(&mut self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                     widget_tree: &WidgetTree) {

        let mut remaining_tasks = Vec::new();
        while !self.tasks.is_empty() {
            let mut task = self.tasks.pop().unwrap();
            let context = EzContext::new(task.widget.clone(), view_tree,
                                         state_tree, widget_tree,self);

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
}

impl Task {

    pub fn new(widget: String, func: GenericEzTask, recurring: bool, interval: Duration,
               last_execution: Option<Instant>)
        -> Self {
        Task { widget, func, recurring, interval, canceled: false, last_execution }
    }

    pub fn cancel(&mut self) { self.canceled = true; }

}