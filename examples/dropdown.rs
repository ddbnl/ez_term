use ez_term::*;

fn main() {
    let (root_widget, mut state_tree, mut scheduler, mut custom_data) = load_ui();

    let change_label_callback = |context: Context| {
        let dropdown_state = context
            .state_tree
            .get_mut(&context.widget_path)
            .as_dropdown_mut();
        let choice = dropdown_state.get_choice().clone();

        let label_state = context.state_tree.get_mut("my_label").as_label_mut();
        label_state.set_text(choice);
        label_state.update(context.scheduler);
        false
    };

    let callback_config = CallbackConfig::from_on_value_change(Box::new(change_label_callback));
    scheduler.update_callback_config("change_label_dropdown", callback_config);

    run(root_widget, state_tree, scheduler, custom_data);
}
