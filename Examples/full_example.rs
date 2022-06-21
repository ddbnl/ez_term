use std::collections::HashMap;
use std::time::Duration;
use crossterm::style::Color;
use ez_term::{self, GenericState, EzObject, EzContext, EzProperty};


fn main() {

    // Step 1: Initialize root widget from config

    // We will get a root widget and a scheduler. We can use the root widget to access all our
    // subwidgets and make any changes we need before starting the app.
    // We can use the scheduler to schedule any (recurring) functions we need to before starting
    // the app.
    let (root_widget, mut scheduler) =
        ez_term::load_ez_ui("./Examples/full_example.ez");

    // Step 2: Customize widgets where needed. Here are some examples:

    // We will set up the menu screen buttons with closures.
    scheduler.update_callback_config(
        "/root/menu_screen/menu_box/start_button".to_string(),
        ez_term::CallbackConfig::from_on_press(Box::new(
            |context: ez_term::EzContext| {
                let root_state = context.state_tree.get_mut("/root")
                    .unwrap().as_layout_mut();
                root_state.set_active_screen("main_screen".to_string());
                root_state.update(context.scheduler);
                false
            } )));

    scheduler.update_callback_config("/root/menu_screen/menu_box/quit_button".to_string(),
        ez_term::CallbackConfig::from_on_press(Box::new(
                |context: ez_term::EzContext| {
                    context.scheduler.exit();
                    true
                } )));

    // Now we'll set up the main screen callbacks using functions defined at the buttom of this file

    // Set a checkbox on value callback
    scheduler.update_callback_config(
        "/root/main_screen/left_box/bottom_box/small_box_2/checkbox_section_box/checkbox_box/checkbox".to_string(),
        ez_term::CallbackConfig::from_on_value_change(
            Box::new(test_checkbox_on_value_change)));

    // Set a slider on value callback
    scheduler.update_callback_config(
        "/root/main_screen/left_box/bottom_box/small_box_2/slider_section_box/slider_float/slider".to_string(),
        ez_term::CallbackConfig::from_on_value_change(
            Box::new(test_slider_on_value_change)));

    // Set a slider on value callback
    let value_property = scheduler.new_usize_property("progress_property".to_string(), 0);
    let value_property_callback = |context: EzContext| {
        let val = context.state_tree.get("/root/main_screen/left_box/bottom_box/small_box_2/progress_bar_section_box/progress_bar")
            .unwrap().as_progress_bar().value.clone();
        let state = context.state_tree.get_mut("/root/main_screen/left_box/bottom_box/small_box_2/progress_bar_section_box/progress_label")
            .unwrap().as_label_mut();
        state.text.set(format!("{}%", val.to_string()));
        state.update(context.scheduler);
        true
    };
    value_property.bind(Box::new(value_property_callback), &mut scheduler);
    scheduler.update_callback_config(
        "/root/main_screen/left_box/bottom_box/small_box_2/progress_bar_section_box/progress_button".to_string(),
        ez_term::CallbackConfig::from_on_press(
            Box::new(progress_bar_button)));

    // Set a radio button group on value callback
    scheduler.update_callback_config(
        "/root/main_screen/left_box/bottom_box/small_box_2/radio_section_box/radio_box/radio1".to_string(),
        ez_term::CallbackConfig::from_on_value_change(
            Box::new(test_radio_button_on_value_change)));
    scheduler.update_callback_config(
        "/root/main_screen/left_box/bottom_box/small_box_2/radio_section_box/radio_box/radio2".to_string(),
        ez_term::CallbackConfig::from_on_value_change(
            Box::new(test_radio_button_on_value_change)));
    scheduler.update_callback_config(
        "/root/main_screen/left_box/bottom_box/small_box_2/radio_section_box/radio_box/radio3".to_string(),
        ez_term::CallbackConfig::from_on_value_change(
            Box::new(test_radio_button_on_value_change)));

    // Set a dropdown on value change callback
    scheduler.update_callback_config(
        "/root/main_screen/left_box/bottom_box/small_box_3/dropdown_section_box/dropdown_box/dropdown".to_string(),
        ez_term::CallbackConfig::from_on_value_change(
            Box::new(test_dropdown_on_value_change)));

    // Set a button callback to create a popup
    scheduler.update_callback_config(
        "/root/main_screen/left_box/bottom_box/small_box_3/popup_section_box/popup_button".to_string(),
        ez_term::CallbackConfig::from_on_press(
            Box::new(test_popup_button_on_press)));

    // Set a text input on keyboard enter
    scheduler.update_callback_config(
        "/root/main_screen/left_box/bottom_box/small_box_1/input_3_box/input_3".to_string(),
        ez_term::CallbackConfig::from_on_keyboard_enter(
            Box::new(test_text_input_on_keyboard_enter)));

    // Set a button on press
    scheduler.update_callback_config(
        "/root/main_screen/left_box/bottom_box/small_box_2/button_section_box/button_box/button".to_string(),
        ez_term::CallbackConfig::from_on_press(
            Box::new(test_on_button_press)));

    let mut neon = (0, 255, 0);
    let mut switch: u8 = 0;
    let neon_banner = move | context: ez_term::EzContext | {
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
        let state = context.state_tree.get_mut(&context.widget_path).unwrap().as_canvas_mut();
        state.get_colors_config_mut().foreground = color;
        state.update(context.scheduler);
        true
    };

    scheduler.schedule_interval("/root/main_screen/left_box/canvas_box/canvas".to_string(),
                                Box::new(neon_banner), Duration::from_millis(200));

    // Step 3: Run app
    // Now everything must happen from bindings as root widget is passed over
    ez_term::run(root_widget, scheduler);
}


// As an example we will change the label next to a checkbox to say "enabled" or
// "disabled" depending on the state of a checkbox.
fn test_checkbox_on_value_change(context: ez_term::EzContext) -> bool {

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
        .get_mut("/root/main_screen/left_box/bottom_box/small_box_2/checkbox_section_box/checkbox_box/checkbox_label")
        .unwrap()
        .as_label_mut();
    label_state.get_text_mut().set(text.to_string());
    label_state.get_colors_config_mut().foreground = color;
    false
}


// As an example we will change the label next to a slider to reflect its' value. We will also
// change the color from red>yellow>green depending on the value
fn test_slider_on_value_change(context: ez_term::EzContext) -> bool {

    // First we get the widget state object of the widget that changed value, using the 'widget_path'
    // parameter as a key. The state contains the current value.
    let state = context.state_tree
        .get(&context.widget_path)
        .unwrap()
        .as_slider();
    let value = state.get_value();
    // Now we will create a text and a color depending on whether the checkbox was turned on or off
    let text = value.to_string();
    let color =
        if state.value.get() as f32 / state.maximum as f32 <= 1.0/3.0 {Color::Red}
        else if state.value.get() as f32 / state.maximum as f32 <= 2.0/3.0 {Color::Yellow}
        else {Color::Green};
    // Next we will retrieve a label widget state and change the text and color field. This will
    // cause the text to change on the next frame.
    let label_state = context.state_tree
        .get_mut("/root/main_screen/left_box/bottom_box/small_box_2/slider_section_box/slider_float/slider_label")
        .unwrap()
        .as_label_mut();
    label_state.get_text_mut().set(text);
    label_state.get_colors_config_mut().foreground = color;
    label_state.update(context.scheduler);
    false
}


// As an example we will change the label next to a radio button group to display the id of the
// selected radio button. We will also change the label to be the color of the radio button that
// became active.
fn test_radio_button_on_value_change(context: ez_term::EzContext) -> bool {

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
        .get_mut("/root/main_screen/left_box/bottom_box/small_box_2/radio_section_box/radio_box/radio_label")
        .unwrap()
        .as_label_mut();
    label_state.get_text_mut().set(name);
    label_state.get_colors_config_mut().foreground = color;
    false
}

// As an example we will change the label next to a dropdown display the active dropdown choice.
fn test_dropdown_on_value_change(context: ez_term::EzContext) -> bool {

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
        .get_mut("/root/main_screen/left_box/bottom_box/small_box_3/dropdown_section_box/dropdown_box/dropdown_label")
        .unwrap()
        .as_label_mut().get_text_mut().set(value);
    false
}


// As an example we will change the label below a text input to add 'confirmed' to its' text after
// an enter on the text input. We will also deselect the widget.
fn test_text_input_on_keyboard_enter(context: EzContext) -> bool {

    // First we get the widget state object of the widget that changed value, using the 'widget_path'
    // parameter as a key. The state contains the current value. Then we cast the generic widget
    // state as a TextInputState, so we can access all its fields.
    let text_input_state = context.state_tree
        .get_mut(&context.widget_path)
        .unwrap()
        .as_text_input_mut();
    let value = text_input_state.get_text().value.clone();
    // Now we will set the selected field of the text input state to false. This will deselect the
    // widget on the next frame.
    text_input_state.set_selected(false);
    // Next we will retrieve a label widget and change the 'text' field of its' state. This will
    // cause the text to change on the next frame.
    context.state_tree
        .get_mut("/root/main_screen/left_box/bottom_box/small_box_1/input_3_box/input_3_label")
        .unwrap()
        .as_label_mut().get_text_mut().set(format!("{} CONFIRMED", value));
    false
}


// As an example we will change a label after a button is pressed.
fn test_on_button_press(context: ez_term::EzContext) -> bool {

    // We will retrieve a label widget and change the 'text' field of its' state. This will
    // cause the text to change on the next frame.
    let label_state = context.state_tree
        .get_mut("/root/main_screen/left_box/bottom_box/small_box_2/button_section_box/button_box/button_label")
        .unwrap()
        .as_label_mut();
    let number: usize =
        label_state.get_text().value.split_once(':')
            .unwrap().1.trim().split_once("times")
            .unwrap().0.trim().parse().unwrap();
    label_state.get_text_mut().set(format!("Clicked: {} times", number + 1));
    false
}


// As an example we will change a label after a button is pressed.
fn test_popup_button_on_press(context: ez_term::EzContext) -> bool {

    // We will open a popup in this callback. We open a popup by defining a template in the
    // Ez file, and then using the template name with the [common::open_popup] function to
    // spawn the template.
    let popup_path = ez_term::open_popup("TestPopup".to_string(),
                                        context.state_tree, context.scheduler);

    // We want to bind a callback to the dismiss button that dismisses the popup. In order to allow
    // allow the button to finish its click animation we will create two closures. One closure
    // actually dismisses the popup. The second closure simply schedules the first one to run with
    // a delay. We will bind the delaying function to the dismiss button.
    let dismiss =
        move |context: ez_term::EzContext| {
            let state = context.state_tree.get_mut("/root").unwrap().as_layout_mut();
            state.dismiss_modal();
            state.update(context.scheduler);
            false
        };
    let path_clone = popup_path.clone();
    let dismiss_delay =
        move |context: ez_term::EzContext| {
            context.scheduler.schedule_once(path_clone.clone(), Box::new(dismiss),
                                            Duration::from_millis(50));
            true
        };

    context.scheduler.update_callback_config(
        format!("{}/dismiss_button", popup_path),
        ez_term::CallbackConfig::from_on_press(Box::new(dismiss_delay)));
    false
}


// As an example we will change a label after a button is pressed.
fn progress_bar_button(context: ez_term::EzContext) -> bool {

    // We disable the progress bar button first so it cannot be pressed twice.
    let state = context.state_tree.get_mut(&context.widget_path).unwrap().as_generic_mut();
    state.set_disabled(true);
    state.update(context.scheduler);
    let progress_button_path = "/root/main_screen/left_box/bottom_box/small_box_2/progress_bar_section_box/progress_button".to_string();
    context.scheduler.schedule_threaded(Box::new(progress_example_app),
        Some(Box::new(move |context: EzContext| {
            let state = context.state_tree.get_mut(&progress_button_path)
                .unwrap().as_generic_mut();
            state.set_disabled(false);
            state.update(context.scheduler);
            true
        })));
    false
}


fn progress_example_app(mut properties: HashMap<String, ez_term::EzProperties>) {

    let value_property = properties.get_mut("progress_property").unwrap();
    for x in 1..6 {
        value_property.as_usize_mut().set(x*20);
        std::thread::sleep(Duration::from_secs(1))
    };
}
