use std::time::{Duration, Instant};
use crate::common::{EzContext, GenericEzFunction, StateTree, ViewTree, WidgetTree};

pub struct Scheduler {
    pub tasks: Vec<Task>,
}

pub struct Task {
    pub widget: String,
    pub func: GenericEzFunction,
    pub recurring: bool,
    pub canceled: bool,
    pub interval: Duration,
    pub last_execution: Instant
}


impl Scheduler {

    pub fn new() -> Self {
        Scheduler { tasks: Vec::new() }
    }

    pub fn schedule_once(&mut self, widget: String, func: GenericEzFunction, after: Duration) {
        let task = Task::new(widget, func, false, after);
        self.tasks.push(task);
    }

    pub fn schedule_interval(&mut self, widget: String,  func: GenericEzFunction, interval: Duration)
        -> &mut Task {
        let task = Task::new(widget, func, true, interval);
        self.tasks.push(task);
        self.tasks.last_mut().unwrap()
    }

    pub fn run_tasks(&mut self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                     widget_tree: &WidgetTree) {

        let mut remaining_tasks = Vec::new();
        while !self.tasks.is_empty() {
            let mut task = self.tasks.pop().unwrap();
            let elapsed = task.last_execution.elapsed();
            let context = EzContext::new(task.widget.clone(), view_tree,
                                         state_tree, widget_tree,self);
            if elapsed >= task.interval {
                (task.func)(context);
                task.last_execution = Instant::now();
                if task.recurring {
                    remaining_tasks.push(task);
                }
            } else if !task.canceled {
                remaining_tasks.push(task);
            }
        }
        self.tasks = remaining_tasks;
    }
}

impl Task {

    pub fn new(widget: String, func: GenericEzFunction, recurring: bool, interval: Duration)
        -> Self {
        Task { widget, func, recurring, interval, canceled: false, last_execution: Instant::now() }
    }

    pub fn cancel(&mut self) {
        self.canceled = true;
    }

}