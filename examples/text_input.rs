use ez_term::*;

fn main () {
    let (root_widget, mut state_tree, mut scheduler) = load_ui();

    // First we write the mirroring on_value_change callback
    let change_label_callback = |context: Context| {

        let input_state = context.state_tree.get_mut("my_input")
            .as_text_input_mut();
        let text = input_state.get_text();
        let label_state = context.state_tree.get_mut("my_label").as_label_mut();
        label_state.set_text(text);
        label_state.update(context.scheduler);
        false
    };
    let callback_config =
        CallbackConfig::from_on_value_change(Box::new(change_label_callback));
    scheduler.update_callback_config("my_input", callback_config);

    // Now we write the confirming on_keyboard_enter callback
    let confirm_label_callback = |context: Context| {

        let input_state = context.state_tree.get_mut("my_input")
            .as_text_input_mut();
        let text = format!("{} CONFIRMED!", input_state.get_text());
        let label_state = context.state_tree.get_mut("my_label").as_label_mut();
        label_state.set_text(text);
        label_state.update(context.scheduler);
        false
    };
    let callback_config =
        CallbackConfig::from_on_keyboard_enter(Box::new(confirm_label_callback));
    scheduler.update_callback_config("my_input", callback_config);


    run(root_widget, state_tree, scheduler);
}