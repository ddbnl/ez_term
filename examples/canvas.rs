use ez_term::*;

fn main() {
    let (root_widget, mut state_tree, mut scheduler) = load_ui();

    // We will create a square of 'S' symbols like this:
    // ```
    // S S S
    // S S S
    // S S S
    // ```
    let state = state_tree.get_mut("my_canvas").as_canvas_mut();

    // We create a single pixel
    let pixel = Pixel::new("S".to_string(), Color::White, Color::Black);

    // We create a row of three pixels
    let row = vec![pixel.clone(); 3];

    // We create a pixelmap of three rows (we now have a square)
    let content = vec![row.clone(); 3];

    state.set_contents(content);

    run(root_widget, state_tree, scheduler);
}
