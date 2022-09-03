use std::time::Duration;
use ez_term::*;

fn main () {
    let (root_widget, mut state_tree, mut scheduler) = load_ui();

    // Manual progress bar callback
    let manual_progress_callback = |context: Context| {
        let state = context.state_tree.get_mut("manual_progress_bar")
            .as_progress_bar_mut();
        let progress = state.get_value();
        if progress < state.get_max() {
            state.set_value(progress + 20);
            state.update(context.scheduler);
            if state.get_value() == state.get_max() {
                let state = context.state_tree.get_mut("manual_button").as_button_mut();
                state.set_disabled(true);
            }
        }
        false

    };
    let callback_config =
        CallbackConfig::from_on_press(Box::new(manual_progress_callback));
    scheduler.update_callback_config("manual_button", callback_config);

    // Recurring task progress bar callback
    let task_progress_callback = |context: Context| {

        let task_progress_task = |context: Context| {
            let state = context.state_tree.get_mut("task_progress_bar")
                .as_progress_bar_mut();
            let progress = state.get_value();
            if progress < state.get_max() {
                state.set_value(progress + 20);
                state.update(context.scheduler);
                true
            } else {
                let state = context.state_tree.get_mut("task_button").as_button_mut();
                state.set_disabled(false);
                state.update(context.scheduler);
                false
            }
        };
        let state = context.state_tree.get_mut("task_progress_bar")
            .as_progress_bar_mut();
        state.set_value(0);
        let state = context.state_tree.get_mut("task_button").as_button_mut();
        state.set_disabled(true);
        state.update(context.scheduler);
        context.scheduler.schedule_recurring("update_progress_bar",
                                     Box::new(task_progress_task),
                                     Duration::from_secs(1));
        false
    };
    let callback_config =
        CallbackConfig::from_on_press(Box::new(task_progress_callback));
    scheduler.update_callback_config("task_button", callback_config);

    // Threaded task progress bar callback
    let thread_progress_callback = |context: Context| {

        let progress_thread = |mut context: ThreadedContext| {
            let state = context.state_tree.get_mut("thread_progress_bar")
                .as_progress_bar_mut();
            state.set_value(0);
            state.update(&mut context.scheduler);
            for x in 1..=5 {
                std::thread::sleep(Duration::from_secs(1));
                state.set_value(x * 20);
                state.update(&mut context.scheduler);
            }
        };
        let on_finish = |context: Context| {
            let state = context.state_tree.get_mut("thread_button").as_button_mut();
            state.set_disabled(false);
            state.update(context.scheduler);
        };
        let state = context.state_tree.get_mut("thread_button").as_button_mut();
        state.set_disabled(true);
        state.update(context.scheduler);
        context.scheduler.schedule_threaded(Box::new(progress_thread),
                                     Some(Box::new(on_finish)));
        false
    };
    let callback_config =
        CallbackConfig::from_on_press(Box::new(thread_progress_callback));
    scheduler.update_callback_config("thread_button", callback_config);

    run(root_widget, state_tree, scheduler);
}