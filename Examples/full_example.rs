use ez_term::{ez_parser, widgets, run, common};
use ez_term::common::{EzContext};
use ez_term::widgets::widget::EzObject;

fn main() {

    // Step 1: Initialize root widget from config
    // We will get a root widget and a scheduler. We can use the root widget to access all our
    // subwidgets and make any changes we need before starting the app.
    // We can use the scheduler to schedule any (recurring) functions we need to before starting
    // the app.
    let (mut root_widget, mut scheduler) =
        ez_parser::load_ez_ui("./Examples/full_example.ez");

    // Step 2: Customize widgets where needed
    // Manually fill a canvas widget.
    let map_widget =
        root_widget.get_child_by_path_mut("/root_layout/left_layout/canvas_widget").unwrap();
    let mut map = Vec::new();
    for x in 0..map_widget.get_width() {
        map.push(Vec::new());
        for _ in 0..map_widget.get_height() {
            map[x].push(widgets::widget::Pixel::from_symbol('.'.to_string()));
        }
    }
    map_widget.set_contents(map);

    // Set a checkbox on value callback
    root_widget.get_child_by_path_mut(
        "/root_layout/left_layout/bottom_layout/small_layout_2/layout2_checkbox_widget")
        .unwrap().set_bind_on_value_change(test_checkbox_on_value_change);

    // Set a radio button group on value callback
    root_widget.get_child_by_path_mut(
        "/root_layout/left_layout/bottom_layout/small_layout_2/option1")
        .unwrap().set_bind_on_value_change(test_radio_button_on_value_change);
    root_widget.get_child_by_path_mut(
        "/root_layout/left_layout/bottom_layout/small_layout_2/option2")
        .unwrap().set_bind_on_value_change(test_radio_button_on_value_change);
    root_widget.get_child_by_path_mut(
        "/root_layout/left_layout/bottom_layout/small_layout_2/option3")
        .unwrap().set_bind_on_value_change(test_radio_button_on_value_change);

    // Set a dropdown on value change callback
    root_widget.get_child_by_path_mut(
        "/root_layout/left_layout/bottom_layout/small_layout_2/layout2_dropdown_widget")
        .unwrap().set_bind_on_value_change(test_dropdown_on_value_change);

    // Set a text input on value change callback
    root_widget.get_child_by_path_mut(
        "/root_layout/left_layout/bottom_layout/small_layout_3/layout3_input_widget")
        .unwrap().set_bind_on_value_change(test_text_input_on_value_change);
    // Set a text input on keyboard enter
    root_widget.get_child_by_path_mut(
        "/root_layout/left_layout/bottom_layout/small_layout_3/layout3_input_widget")
        .unwrap().set_bind_keyboard_enter(test_text_input_on_keyboard_enter);

    // Set a button on press
    root_widget.get_child_by_path_mut(
        "/root_layout/left_layout/bottom_layout/small_layout_3/layout3_button")
        .unwrap().set_bind_on_press(test_on_button_keyboard_enter);

    /// # Step 3: Run app
    /// Now everything must happen from bindings as root widget is passed over
    run::run(root_widget, scheduler);
}


// As an example we will change the label next to a checkbox to say "enabled" or
// "disabled" depending on the state of a checkbox.
fn test_checkbox_on_value_change(context: EzContext) {

    // First we get the widget state object of the widget that changed value, using the 'widget_path'
    // parameter as a key. The state contains the current value. Then we cast the generic widget
    // state as a CheckboxState, so we can access all its fields. Then we check the 'active'
    // field.
    let enabled = if context.state_tree
        .get(&context.widget_path)
        .unwrap()
        .as_checkbox().active
        {"Enabled"} else {"Disabled"};
    // Next we will retrieve a label widget and change the 'text' field of its' state. This will
    // cause the text to change on the next frame.
    context.state_tree
        .get_mut("/root_layout/left_layout/bottom_layout/small_layout_2/layout2_checkbox_title_widget")
        .unwrap()
        .as_label_mut().text = enabled.to_string();
}


// As an example we will change the label next to a radio button group to display the id of the
// selected radio button
fn test_radio_button_on_value_change(context: EzContext) {

    // First we get the EzObjects enum of the widget that changed value, using the 'widget_path'
    // parameter as a key. Then we cast it into a radio button object. We already know the state of
    // the widget is active, because a radio button only calls on value change if it become active,
    // so we don't do anything with the WidgetState. We will use the widget object to get the ID.
    let name = context.widget_tree
        .get(&context.widget_path)
        .unwrap()
        .as_radio_button()
        .get_id();
    // Next we will retrieve a label widget and change the 'text' field of its' state. This will
    // cause the text to change on the next frame.
    context.state_tree
        .get_mut("/root_layout/left_layout/bottom_layout/small_layout_2/layout2_radio_title_widget")
        .unwrap()
        .as_label_mut().text = name;
}

// As an example we will change the label next to a dropdown display the active dropdown choice.
fn test_dropdown_on_value_change(context: EzContext) {

    // First we get the widget state object of the widget that changed value, using the 'widget_path'
    // parameter as a key. The state contains the current value. Then we cast the generic widget
    // state as a DropdownState, so we can access all its fields.
    let value = context.state_tree
        .get(&context.widget_path)
        .unwrap()
        .as_dropdown()
        .choice.to_string();
    // Next we will retrieve a label widget and change the 'text' field of its' state. This will
    // cause the text to change on the next frame.
    context.state_tree
        .get_mut("/root_layout/left_layout/bottom_layout/small_layout_2/layout2_dropdown_title_widget")
        .unwrap()
        .as_label_mut().text = value;
}


// As an example we will change the label below a text input to mirror any typed text.
fn test_text_input_on_value_change(context: EzContext) {

    // First we get the widget state object of the widget that changed value, using the 'widget_path'
    // parameter as a key. The state contains the current value. Then we cast the generic widget
    // state as a TextInputState, so we can access all its fields.
    let value = context.state_tree
        .get(&context.widget_path)
        .unwrap()
        .as_text_input()
        .text.to_string();
    // Next we will retrieve a label widget and change the 'text' field of its' state. This will
    // cause the text to change on the next frame.
    context.state_tree
        .get_mut("/root_layout/left_layout/bottom_layout/small_layout_3/layout3_text_input_title_widget")
        .unwrap()
        .as_label_mut().text = value;
}


// As an example we will change the label below a text input to add 'confirmed' to its' text after
// an enter on the text input. We will also deselect the widget.
fn test_text_input_on_keyboard_enter(context: EzContext) {

    // First we get the widget state object of the widget that changed value, using the 'widget_path'
    // parameter as a key. The state contains the current value. Then we cast the generic widget
    // state as a TextInputState, so we can access all its fields.
    let text_input_state = context.state_tree
        .get_mut(&context.widget_path)
        .unwrap()
        .as_text_input_mut();
    let value = text_input_state.text.to_string();
    // Now we will set the selected field of the text input state to false. This will deselect the
    // widget on the next frame.
    text_input_state.selected = false;
    // Next we will retrieve a label widget and change the 'text' field of its' state. This will
    // cause the text to change on the next frame.
    context.state_tree
        .get_mut("/root_layout/left_layout/bottom_layout/small_layout_3/layout3_text_input_title_widget")
        .unwrap()
        .as_label_mut().text = format!("{} CONFIRMED", value);
}


// As an example we will change the label below a text input to add 'confirmed' to its' text after
// an enter on the text input. We will also deselect the widget.
fn test_on_button_keyboard_enter(context: EzContext) {

    // First we get the widget state object of the widget that changed value, using the 'widget_path'
    // parameter as a key. The state contains the current value. Then we cast the generic widget
    // state as a TextInputState, so we can access all its fields.
    let button_state = context.state_tree
        .get_mut(&context.widget_path)
        .unwrap()
        .as_button_mut();
    // Next we will retrieve a label widget and change the 'text' field of its' state. This will
    // cause the text to change on the next frame.
    let label_state = context.state_tree
        .get_mut("/root_layout/left_layout/bottom_layout/small_layout_3/layout3_button_label")
        .unwrap()
        .as_label_mut();
    let number: usize =
        label_state.text.split_once(':').unwrap().1.trim().split_once("times")
            .unwrap().0.trim().parse().unwrap();
    label_state.text = format!("Button clicked: {} times", number + 1);
}