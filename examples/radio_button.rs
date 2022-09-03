use ez_term::*;

fn main () {
    let (root_widget, mut state_tree, mut scheduler) = load_ui();

    for x in 1..=3 {

        let radio_id = format!("my_radio_{}", x);
        let change_label_callback = move |context: Context| {

            let label_state = context.state_tree.get_mut("my_label").as_label_mut();
            label_state.set_text(format!("Choice: Option {}", x));
            label_state.update(context.scheduler);
            false
        };

        let callback_config =
            CallbackConfig::from_on_value_change(Box::new(change_label_callback));
        scheduler.update_callback_config(&radio_id, callback_config);
    }


    run(root_widget, state_tree, scheduler);
}