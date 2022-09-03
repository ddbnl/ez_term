use ez_term::*;

fn main() {
    // We load the UI from the .ez files
    let (root_layout, mut state_tree, mut scheduler) = load_ui();

    // We update the callbacks for the buttons, using the functions defined below
    scheduler.update_callback_config("to_screen_2_btn",
                                     CallbackConfig::from_on_press(Box::new(to_screen_two_callback)));
    scheduler.update_callback_config("to_screen_1_btn",
                                     CallbackConfig::from_on_press(Box::new(to_screen_one_callback)));
    // We run the UI
    run(root_layout, state_tree, scheduler);
}

// We define the callback functions. We could also use closures if we wanted to.
fn to_screen_one_callback(context: Context) -> bool {
    let state = context.state_tree.get_mut("root").as_layout_mut();
    state.set_active_screen("screen_1");
    state.update(context.scheduler);
    true
}

fn to_screen_two_callback(context: Context) -> bool {
    let state = context.state_tree.get_mut("root").as_layout_mut();
    state.set_active_screen("screen_2");
    state.update(context.scheduler);
    true
}