fn main () {
    let (root_layout, scheduler) = ez_term::load_ez_file("./Examples/FloatLayout/float_layout_pos_hints.ez");
    ez_term::run(root_layout, scheduler);
}