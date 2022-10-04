use ez_term::*;
use std::time::Duration;

fn main() {
    let (root_widget, mut state_tree, mut scheduler, mut custom_data) = load_ui();

    let hover_cb = |context: Context, mouse_pos: Coordinates| {
        let state = context.state_tree.get_mut("my_label").as_label_mut();
        state.set_text("Hovered!".to_string());
        state.update(context.scheduler);
        true
    };

    let hover_exit_cb = |context: Context| {
        let state = context.state_tree.get_mut("my_label").as_label_mut();
        state.set_text("Not hovered".to_string());
        state.update(context.scheduler);
        true
    };

    let mut callback_config = CallbackConfig::from_on_hover(Box::new(hover_cb));
    callback_config.on_hover_exit = Some(Box::new(hover_exit_cb));
    scheduler.update_callback_config("my_label", callback_config);

    run(root_widget, state_tree, scheduler, custom_data);

}
