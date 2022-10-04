use ez_term::*;

fn main() {
    let (root_widget, mut state_tree, mut scheduler, mut custom_data) = load_ui();

    for x in 1..=30 {
        let new_id = format!("label_{}", x);
        let (new_widget, mut new_states) =
            scheduler.prepare_create_widget("MyLabel", new_id.as_str(), "root", &mut state_tree);
        new_states.as_label_mut().set_width(x + 2); // +2 width for border
        new_states.as_label_mut().set_text(x.to_string());
        scheduler.create_widget(new_widget, new_states, &mut state_tree);
    }

    run(root_widget, state_tree, scheduler, custom_data);
}
