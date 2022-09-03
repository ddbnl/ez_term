use ez_term::*;

fn main () {
    let (root_widget, mut state_tree, mut scheduler) = load_ui();



    // Let's write the callback that syncs a new text input value to the slider
    let sync_slider_callback = |context: Context| {

        let input_state = context.state_tree.get_mut("my_input")
            .as_text_input_mut();
        let mut value = input_state.get_text();
        if value.is_empty() {
            return false
        }

        let slider_state = context.state_tree.get_mut("input_slider")
            .as_slider_mut();
        match value.trim().parse() {
            Ok(i) => {
                if i != slider_state.get_value() {
                    slider_state.set_value(i);
                    slider_state.update(context.scheduler);
                }
            },
            Err(_) => {
                let old_value = slider_state.get_value();
                let input_state = context.state_tree.get_mut("my_input")
                    .as_text_input_mut();
                input_state.set_text(old_value.to_string());
                input_state.update(context.scheduler);
            }
        }
        false
    };
    let callback_config =
        CallbackConfig::from_on_value_change(Box::new(sync_slider_callback));
    scheduler.update_callback_config("my_input", callback_config);

    run(root_widget, state_tree, scheduler);
}