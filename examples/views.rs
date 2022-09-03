use ez_term::*;

fn main() {
    let (root_widget, mut state_tree, mut scheduler) = load_ui();

    // We create a callback to navigate backwards through the view
    let navigate_back_callback = |context: Context| {
        let state = context.state_tree.get_mut("view_layout").as_layout_mut();
        let current_page = state.get_view_page();
        if current_page > 1 {
            state.set_view_page(current_page - 1);
            state.update(context.scheduler);
        }
        true
    };
    let callback_config = CallbackConfig::from_on_press(Box::new(navigate_back_callback));
    scheduler.update_callback_config("back_button", callback_config);

    // We create a callback to navigate forwards through the view
    let navigate_forward_callback = |context: Context| {
        let state = context.state_tree.get_mut("view_layout").as_layout_mut();
        let current_page = state.get_view_page();
        state.set_view_page(current_page + 1);
        state.update(context.scheduler);
        true
    };
    let callback_config = CallbackConfig::from_on_press(Box::new(navigate_forward_callback));
    scheduler.update_callback_config("forward_button", callback_config);

    // We programmatically create a lot of widgets to test views
    for x in 0..=40 {
        let new_id = format!("view_test_{}", x);
        let (new_widget, mut new_states) = scheduler.prepare_create_widget(
            "ViewTest",
            new_id.as_str(),
            "view_layout",
            &mut state_tree,
        );
        new_states.as_label_mut().set_text(new_id);
        scheduler.create_widget(new_widget, new_states, &mut state_tree);
    }

    run(root_widget, state_tree, scheduler);
}
