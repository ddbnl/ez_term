use ez_term::*;

fn main() {
    let (root_widget, mut state_tree, mut scheduler, mut custom_data) = load_ui();
    run(root_widget, state_tree, scheduler, custom_data);
}
