use std::time::Duration;
use ez_term::*;

fn main() {

    let (root_widget, mut state_tree, mut scheduler) = load_ui();

    let my_callback = |context: Context| {

        let mut counter: usize = 1;

        let my_task = move |context: Context| {

            let state = context.state_tree.get_mut("my_label").as_label_mut();
            state.set_text(format!("Counting {}", counter));
            state.update(context.scheduler);
            counter += 1;
            return if counter <= 5 {
                true
            } else {
                false
            };
        };
        context.scheduler.schedule_recurring("my_task", Box::new(my_task), Duration::from_secs(1));
        true
    };
    let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
    scheduler.update_callback_config("my_button", new_callback_config);

    run(root_widget, state_tree, scheduler);

}