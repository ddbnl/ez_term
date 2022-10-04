use ez_term::*;
use std::time::Duration;

fn main() {
    let (root_widget, mut state_tree, mut scheduler, mut custom_data) = load_ui();

    let my_callback = |context: Context| {
        let my_task = |context: Context| {
            let state = context.state_tree.get_mut("my_label").as_label_mut();
            state.set_text("Button was pressed 3 seconds ago!".to_string());
            state.update(context.scheduler);
        };

        let state = context.state_tree.get_mut("my_label").as_label_mut();
        state.set_text("Waiting...".to_string());
        state.update(context.scheduler);

        context
            .scheduler
            .schedule_once("my_task", Box::new(my_task), Duration::from_secs(3));
        true
    };
    let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
    scheduler.update_callback_config("my_button", new_callback_config);

    run(root_widget, state_tree, scheduler, custom_data);
}
