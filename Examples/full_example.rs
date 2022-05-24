use ez_term::{ez_parser, run};
use ez_term::common::{EzContext};
use ez_term::states::state::{SelectableState};
use ez_term::widgets::widget::EzObject;

fn main() {

    // Step 1: Initialize root widget from config

    // We will get a root widget and a scheduler. We can use the root widget to access all our
    // subwidgets and make any changes we need before starting the app.
    // We can use the scheduler to schedule any (recurring) functions we need to before starting
    // the app.
    let (mut root_widget, mut scheduler) =
        ez_parser::load_ez_ui("./Examples/full_example.ez");

    // Step 3: Run app
    // Now everything must happen from bindings as root widget is passed over
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
        .as_checkbox().get_active()
        {"Enabled"} else {"Disabled"};
    // Next we will retrieve a label widget and change the 'text' field of its' state. This will
    // cause the text to change on the next frame.
    context.state_tree
        .get_mut("/root_layout/left_layout/bottom_layout/small_layout_2/layout2_checkbox_title_widget")
        .unwrap()
        .as_label_mut().set_text(enabled.to_string());
}


// As an example we will change the label next to a radio button group to display the id of the
// selected radio button
fn test_radio_button_on_value_change(context: EzContext) {

    // First we get the EzObjects enum of the widget that changed value, using the 'widget_path'
    // parameter as a key. Then we cast it into a radio button object. We already know the state of
    // the widget is active, because a radio button only calls on value change if it become active,
    // so we don't do anything with the State. We will use the widget object to get the ID.
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
        .as_label_mut().set_text(name);
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
        .get_choice();
    // Next we will retrieve a label widget and change the 'text' field of its' state. This will
    // cause the text to change on the next frame.
    context.state_tree
        .get_mut("/root_layout/left_layout/bottom_layout/small_layout_2/layout2_dropdown_title_widget")
        .unwrap()
        .as_label_mut().set_text(value);
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
        .get_text();
    // Next we will retrieve a label widget and change the 'text' field of its' state. This will
    // cause the text to change on the next frame.
    context.state_tree
        .get_mut("/root_layout/left_layout/bottom_layout/small_layout_3/layout3_text_input_title_widget")
        .unwrap()
        .as_label_mut().set_text(value);
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
    let value = text_input_state.get_text();
    // Now we will set the selected field of the text input state to false. This will deselect the
    // widget on the next frame.
    text_input_state.set_selected(false);
    // Next we will retrieve a label widget and change the 'text' field of its' state. This will
    // cause the text to change on the next frame.
    context.state_tree
        .get_mut("/root_layout/left_layout/bottom_layout/small_layout_3/layout3_text_input_title_widget")
        .unwrap()
        .as_label_mut().set_text(format!("{} CONFIRMED", value));
}


// As an example we will change the label below a text input to add 'confirmed' to its' text after
// an enter on the text input. We will also deselect the widget.
fn test_on_button_keyboard_enter(context: EzContext) {

    // We will retrieve a label widget and change the 'text' field of its' state. This will
    // cause the text to change on the next frame.
    let label_state = context.state_tree
        .get_mut("/root_layout/left_layout/bottom_layout/small_layout_3/layout3_button_label")
        .unwrap()
        .as_label_mut();
    let number: usize =
        label_state.get_text().split_once(':').unwrap().1.trim().split_once("times")
            .unwrap().0.trim().parse().unwrap();
    label_state.set_text(format!("Button clicked: {} times", number + 1));
}