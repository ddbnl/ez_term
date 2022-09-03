use std::time::Duration;
use ez_term::*;

fn main() {

    let (root_widget, mut state_tree, mut scheduler) = load_ui();

    fn mock_app(mut context: ThreadedContext) {

        for x in 1..=5 {
            let state = context.state_tree.get_mut("my_progress_bar").as_progress_bar_mut();
            state.set_value(x*20);
            state.update(&mut context.scheduler);

            let state = context.state_tree.get_mut("my_progress_label").as_label_mut();
            state.set_text(format!("{}%", x*20));
            state.update(&mut context.scheduler);

            std::thread::sleep(Duration::from_secs(1)) };
    }


    let start_button_callback = |context: Context| {

        let on_finish = |context: Context| {

            let state = context.state_tree.get_mut("my_progress_label").as_label_mut();
            state.set_text("Finished!".to_string());
            state.update(context.scheduler);
        };
        context.scheduler.schedule_threaded(Box::new(mock_app), Some(Box::new(on_finish)));
        true
    };

    let new_callback_config = CallbackConfig::from_on_press(Box::new(start_button_callback));
    scheduler.update_callback_config("my_button", new_callback_config);

    run(root_widget, state_tree, scheduler);
}