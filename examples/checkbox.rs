use ez_term::*;

fn main() {
    let (root_widget, mut state_tree, mut scheduler, mut custom_data) = load_ui();

    let change_label_callback = |context: Context| {
        let checkbox_state = context
            .state_tree
            .get_mut(&context.widget_path)
            .as_checkbox_mut();
        let active = checkbox_state.get_active();
        let (text, color) = if active {
            ("Active".to_string(), Color::Green)
        } else {
            ("Inactive".to_string(), Color::Red)
        };

        let label_state = context.state_tree.get_mut("my_label").as_label_mut();
        label_state.set_text(text);
        label_state.get_color_config_mut().set_fg_color(color);
        label_state.update(context.scheduler);
        false
    };

    let callback_config = CallbackConfig::from_on_value_change(Box::new(change_label_callback));
    scheduler.update_callback_config("change_label_checkbox", callback_config);

    run(root_widget, state_tree, scheduler, custom_data);
}
