use std::time::Duration;
use ez_term::*;

fn main() {

    let (root_widget, mut state_tree, mut scheduler) = load_ui();

    let my_task = |context: Context| {

        let state = context.state_tree.get_mut("my_label").as_label_mut();
        state.set_text("3 seconds have passed!".to_string());
        state.update(context.scheduler);
    };

    scheduler.schedule_once("my_task", Box::new(my_task), Duration::from_secs(3));
    run(root_widget, state_tree, scheduler);
}