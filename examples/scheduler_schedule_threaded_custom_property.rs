use ez_term::*;
use std::time::Duration;

fn main() {
    let (root_widget, mut state_tree, mut scheduler, mut custom_data) = load_ui();

    // We must register our custom property!
    scheduler.new_usize_property("my_progress", 0);

    fn mock_app(mut context: ThreadedContext) {
        for x in 1..=5 {
            let my_progress = context.scheduler.get_property_mut("my_progress");
            my_progress.as_usize_mut().set(x * 20);
            std::thread::sleep(Duration::from_secs(1))
        }
    }

    let start_button_callback = |context: Context| {
        context
            .scheduler
            .schedule_threaded(Box::new(mock_app), None);
        true
    };

    let new_callback_config = CallbackConfig::from_on_press(Box::new(start_button_callback));
    scheduler.update_callback_config("my_button", new_callback_config);
}
