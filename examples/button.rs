use ez_term::*;

fn main() {
    let (root_widget, mut state_tree, mut scheduler) = load_ui();

    let change_label_callback = |context: Context| {
        let state = context.state_tree.get_mut("my_label").as_label_mut();
        state.set_text("Button was clicked!".to_string());
        state.update(context.scheduler);
        false
    };
    let callback_config = CallbackConfig::from_on_press(Box::new(change_label_callback));
    scheduler.update_callback_config("change_label_button", callback_config);

    run(root_widget, state_tree, scheduler);
}
