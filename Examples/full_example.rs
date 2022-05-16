use ez_term::{ez_parser, widgets, run, common};
use ez_term::widgets::widget::EzObject;

fn main() {

    // Initialize root widget from config
    let mut root_widget = ez_parser::load_ez_ui("./Examples/full_example.ez");

    // Do whatever to initialize and customize root widget directly from code
    let map_widget =
        root_widget.get_child_by_path_mut("/root_layout/map_layout/map_widget").unwrap();
    let mut map = Vec::new();
    for x in 0..map_widget.get_width() {
        map.push(Vec::new());
        for _ in 0..map_widget.get_height() {
            map[x].push(widgets::widget::Pixel::from_symbol('.'.to_string()));
        }
    }
    map_widget.set_contents(map);

    // Test a checkbox on value callback
    root_widget.get_child_by_path_mut(
        "/root_layout/map_layout/player_details_layout/target_layout/target_checkbox_widget")
        .unwrap().set_bind_on_value_change(test_checkbox_on_value_change);

    // Test a radio button group on value callback
    root_widget.get_child_by_path_mut(
        "/root_layout/map_layout/player_details_layout/target_layout/option1")
        .unwrap().set_bind_on_value_change(test_radio_button_on_value_change);
    root_widget.get_child_by_path_mut(
        "/root_layout/map_layout/player_details_layout/target_layout/option2")
        .unwrap().set_bind_on_value_change(test_radio_button_on_value_change);
    root_widget.get_child_by_path_mut(
        "/root_layout/map_layout/player_details_layout/target_layout/option3")
        .unwrap().set_bind_on_value_change(test_radio_button_on_value_change);


    // Run app. Now everything must happen from bindings as root widget is passed over
    run::run(root_widget);
}


// As an example we will change the label next to a checkbox to say "enabled" or
// "disabled" depending on the state of a checkbox.
fn test_checkbox_on_value_change(widget_path: String, view_tree: &mut common::ViewTree,
                                 state_tree: &mut common::StateTree, widget_tree: &common::WidgetTree) {

    // First we get the widget state object of the widget that changed value, using the 'widget_path'
    // parameter as a key. The state contains the current value. Then we cast the generic widget
    // state as a CheckboxState, so we can access all its fields. Then we check the 'active'
    // field.
    let enabled = if state_tree
        .get(&widget_path)
        .unwrap()
        .as_checkbox().active
        {"Enabled"} else {"Disabled"};
    // Next we will retrieve a label widget and change the 'text' field of its' state. This will
    // cause the text to change on the next frame.
    state_tree
        .get_mut("/root_layout/map_layout/player_details_layout/target_layout/target_checkbox_title_widget")
        .unwrap()
        .as_label_mut().text = enabled.to_string();
}


// As an example we will change the label next to a radio button group to display the id of the
// selected radio button
fn test_radio_button_on_value_change(widget_path: String, view_tree: &mut common::ViewTree,
                                     state_tree: &mut common::StateTree,
                                     widget_tree: &common::WidgetTree) {

    // First we get the EzObjects enum of the widget that changed value, using the 'widget_path'
    // parameter as a key. Then we cast it into a radio button object. We already know the state of
    // the widget is active, because a radio button only calls on value change if it become active,
    // so we don't do anything with the WidgetState. We will use the widget object to get the ID.
    let name = widget_tree
        .get(&widget_path)
        .unwrap()
        .as_radio_button()
        .get_id();
    // Next we will retrieve a label widget and change the 'text' field of its' state. This will
    // cause the text to change on the next frame.
    state_tree
        .get_mut("/root_layout/map_layout/player_details_layout/target_layout/target_radio_title_widget")
        .unwrap()
        .as_label_mut().text = name;
}
