fn main () {
    let (root_layout, scheduler) = ez_term::load_ez_file("./Examples/BoxLayouts/box_layouts.ez");
    ez_term::run(root_layout, scheduler);
}