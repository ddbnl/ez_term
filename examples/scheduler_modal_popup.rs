use ez_term::*;

fn main() {
    let (root_widget, mut state_tree, mut scheduler, mut custom_data) = load_ui();

    let create_popup_callback = |context: Context| {
        let dismiss_popup_callback = |context: Context| {
            context.scheduler.dismiss_modal(context.state_tree);
            true
        };

        context.scheduler.open_modal("MyPopup", context.state_tree);
        let callback_config = CallbackConfig::from_on_press(Box::new(dismiss_popup_callback));
        context
            .scheduler
            .update_callback_config("dismiss_button", callback_config);
        true
    };

    let callback_config = CallbackConfig::from_on_press(Box::new(create_popup_callback));
    scheduler.update_callback_config("create_button", callback_config);

    run(root_widget, state_tree, scheduler, custom_data);
}
