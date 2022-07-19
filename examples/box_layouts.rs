use ez_term::*;

fn main () {
    let (root_widget, state_tree, mut scheduler) = load_ui();
    run(root_widget, state_tree, scheduler);
}