use std::time::Duration;
use crossterm::style::Color;
use ez_term::{ez_parser, run, states};
use ez_term::common;
use ez_term::widgets::widget::EzObject;
use ez_term::states::state::{GenericState};


fn main() {

    // Step 1: Initialize root widget from config

    // We will get a root widget and a scheduler. We can use the root widget to access all our
    // subwidgets and make any changes we need before starting the app.
    // We can use the scheduler to schedule any (recurring) functions we need to before starting
    // the app.
    let (root_widget, mut scheduler) =
        ez_parser::load_ez_ui("./Examples/full_example.ez");

    // Step 2: Customize widgets where needed. Here are some examples:

    // Set a checkbox on value callback
    scheduler.update_callback_config(
        "/root/left_box/bottom_box/small_box_2/checkbox_section_box/checkbox_box/checkbox".to_string(),
        states::definitions::CallbackConfig::from_on_value_change(Box::new(test_checkbox_on_value_change)));

    // Set a radio button group on value callback
    scheduler.update_callback_config(
        "/root/left_box/bottom_box/small_box_2/radio_section_box/radio_box/radio1".to_string(),
        states::definitions::CallbackConfig::from_on_value_change(Box::new(test_radio_button_on_value_change)));
    scheduler.update_callback_config(
        "/root/left_box/bottom_box/small_box_2/radio_section_box/radio_box/radio2".to_string(),
        states::definitions::CallbackConfig::from_on_value_change(Box::new(test_radio_button_on_value_change)));
    scheduler.update_callback_config(
        "/root/left_box/bottom_box/small_box_2/radio_section_box/radio_box/radio3".to_string(),
        states::definitions::CallbackConfig::from_on_value_change(Box::new(test_radio_button_on_value_change)));

    // Set a dropdown on value change callback
    scheduler.update_callback_config(
        "/root/left_box/bottom_box/small_box_3/dropdown_section_box/dropdown_box/dropdown".to_string(),
        states::definitions::CallbackConfig::from_on_value_change(Box::new(test_dropdown_on_value_change)));

    // Set a button callback to create a popup
    scheduler.update_callback_config(
        "/root/left_box/bottom_box/small_box_3/popup_section_box/popup_button".to_string(),
        states::definitions::CallbackConfig::from_on_press(Box::new(test_popup_button_on_press)));

    // Set a text input on value change callback
    scheduler.update_callback_config(
        "/root/left_box/bottom_box/small_box_1/input_3_box/input_3".to_string(),
        states::definitions::CallbackConfig::from_on_value_change(Box::new(test_text_input_on_value_change)));
    // Set a text input on keyboard enter
    scheduler.update_callback_config(
        "/root/left_box/bottom_box/small_box_1/input_3_box/input_3".to_string(),
        states::definitions::CallbackConfig::from_on_keyboard_enter(Box::new(test_text_input_on_keyboard_enter)));

    // Set a button on press
    scheduler.update_callback_config(
        "/root/left_box/bottom_box/small_box_2/button_section_box/button_box/button".to_string(),
        states::definitions::CallbackConfig::from_on_press(Box::new(test_on_button_press)));

    let mut neon = (0, 255, 0);
    let mut switch: u8 = 0;
    let neon_banner = move | context: common::definitions::EzContext | {
        let color = Color::from(neon);
        if switch == 0 {
            if neon.0 > 245 {
                neon.0 = 20;
            }
            neon = (neon.0 + 2, neon.1, neon.2);
            switch += 1;
        } else if switch == 1 {
            if neon.1 < 20 {
                neon.1 = 255
            }
            neon = (neon.0, neon.1 - 2, neon.2);
            switch += 1;
        } else {
            if neon.2 > 200 {
                neon.2 = 50;
            }
            neon = (neon.0, neon.1, neon.2 + 2);
        }
        context.state_tree.get_mut(&context.widget_path).unwrap().as_canvas_mut()
            .get_colors_config_mut().foreground = color;
        true
    };

    scheduler.schedule_interval("/root/left_box/canvas_box/canvas".to_string(),
                                Box::new(neon_banner), Duration::from_millis(200));

    // Step 3: Run app
    // Now everything must happen from bindings as root widget is passed over
    run::run(root_widget, scheduler);
}


// As an example we will change the label next to a checkbox to say "enabled" or
// "disabled" depending on the state of a checkbox.
fn test_checkbox_on_value_change(context: common::definitions::EzContext) {

    // First we get the widget state object of the widget that changed value, using the 'widget_path'
    // parameter as a key. The state contains the current value. Then we cast the generic widget
    // state as a CheckboxState, so we can access all its fields. Then we check the 'active' field.
    let enabled = context.state_tree
        .get(&context.widget_path)
        .unwrap()
        .as_checkbox().get_active();
    // Now we will create a text and a color depending on whether the checkbox was turned on or off
    let text = if enabled {"Enabled"} else {"Disabled"};
    let color = if enabled {Color::Green} else {Color::Red};
    // Next we will retrieve a label widget state and change the text and color field. This will
    // cause the text to change on the next frame.
    let label_state = context.state_tree
        .get_mut("/root/left_box/bottom_box/small_box_2/checkbox_section_box/checkbox_box/checkbox_label")
        .unwrap()
        .as_label_mut();
    label_state.set_text(text.to_string());
    label_state.get_colors_config_mut().foreground = color;
}


// As an example we will change the label next to a radio button group to display the id of the
// selected radio button. We will also change the label to be the color of the radio button that
// became active.
fn test_radio_button_on_value_change(context: common::definitions::EzContext) {

    // First we get the EzObjects enum of the widget that changed value, using the 'widget_path'
    // parameter as a key. Then we cast it into a radio button object. We will use this object to
    // get the ID of the widget, because we want to print it to a label.
    // We don't use the state of the widget in this callback to check whether it is active,
    // because a radio button only calls on_value_change if it became active.
    let name = context.widget_tree
        .get(&context.widget_path)
        .unwrap()
        .as_radio_button()
        .get_id();
    // Now we will get the radio button state because we need to know its' color.
    let color = context.state_tree
        .get(&context.widget_path)
        .unwrap()
        .as_radio_button()
        .get_color_config()
        .foreground;
    // Next we will retrieve a label widget and change the 'text' field of its' state to the ID of
    // the radio button that became active. This will cause the text to change on the next frame.
    let label_state = context.state_tree
        .get_mut("/root/left_box/bottom_box/small_box_2/radio_section_box/radio_box/radio_label")
        .unwrap()
        .as_label_mut();
    label_state.set_text(name);
    label_state.get_colors_config_mut().foreground = color;
}

// As an example we will change the label next to a dropdown display the active dropdown choice.
fn test_dropdown_on_value_change(context: common::definitions::EzContext) {

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
        .get_mut("/root/left_box/bottom_box/small_box_3/dropdown_section_box/dropdown_box/dropdown_label")
        .unwrap()
        .as_label_mut().set_text(value);
}


// As an example we will change the label below a text input to mirror any typed text.
fn test_text_input_on_value_change(context: common::definitions::EzContext) {

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
        .get_mut("/root/left_box/bottom_box/small_box_1/input_3_box/input_3_label")
        .unwrap()
        .as_label_mut().set_text(value);
}


// As an example we will change the label below a text input to add 'confirmed' to its' text after
// an enter on the text input. We will also deselect the widget.
fn test_text_input_on_keyboard_enter(context: common::definitions::EzContext) {

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
        .get_mut("/root/left_box/bottom_box/small_box_1/input_3_box/input_3_label")
        .unwrap()
        .as_label_mut().set_text(format!("{} CONFIRMED", value));
}


// As an example we will change a label after a button is pressed.
fn test_on_button_press(context: common::definitions::EzContext) {

    // We will retrieve a label widget and change the 'text' field of its' state. This will
    // cause the text to change on the next frame.
    let label_state = context.state_tree
        .get_mut("/root/left_box/bottom_box/small_box_2/button_section_box/button_box/button_label")
        .unwrap()
        .as_label_mut();
    let number: usize =
        label_state.get_text().split_once(':').unwrap().1.trim().split_once("times")
            .unwrap().0.trim().parse().unwrap();
    label_state.set_text(format!("Clicked: {} times", number + 1));
}


// As an example we will change a label after a button is pressed.
fn test_popup_button_on_press(context: common::definitions::EzContext) {

    // We will open a popup in this callback. We open a popup by defining a template in the
    // Ez file, and then using the template name with the [common::open_popup] function to
    // spawn the template.
    let popup_path = common::widget_functions::open_popup("TestPopup".to_string(),
                                        context.state_tree, context.scheduler);

    // We want to bind a callback to the dismiss button that dismisses the popup. In order to allow
    // allow the button to finish its click animation we will create two closures. One closure
    // actually dismisses the popup. The second closure simply schedules the first one to run with
    // a delay. We will bind the delaying function to the dismiss button.
    let dismiss =
        move |context: common::definitions::EzContext| {
            context.state_tree.get_mut("/root").unwrap().as_layout_mut().dismiss_modal();
            false
        };
    let path_clone = popup_path.clone();
    let dismiss_delay =
        move |context: common::definitions::EzContext| {
            context.scheduler.schedule_once(path_clone.clone(), Box::new(dismiss),
                                            Duration::from_millis(50));
        };

    context.scheduler.update_callback_config(
        format!("{}/dismiss_button", popup_path.clone()),
        states::definitions::CallbackConfig::from_on_press(Box::new(dismiss_delay)));


}
