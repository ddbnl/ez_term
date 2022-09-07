//! # Widget:
//! A module containing the base structs and traits for widgets"
//! functions allows starting the app based on a root layout.
use std::io::Error;

use crossterm::event::Event;

use crate::run::definitions::{CallbackTree, Coordinates, IsizeCoordinates, PixelMap, StateTree};
use crate::run::tree::ViewTree;
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::ez_state::{EzState, GenericState};
use crate::widgets::button::Button;
use crate::widgets::canvas::Canvas;
use crate::widgets::checkbox::Checkbox;
use crate::widgets::dropdown::{Dropdown, DroppedDownMenu};
use crate::widgets::label::Label;
use crate::widgets::layout::layout::Layout;
use crate::widgets::progress_bar::ProgressBar;
use crate::widgets::radio_button::RadioButton;
use crate::widgets::slider::Slider;
use crate::widgets::text_input::TextInput;
use crate::Context;

/// Enum with variants representing Layouts and each widget type. A layout is not considered a
/// widget, so this enum gathers widgets and layouts in one place, as they do have methods in
/// common (e.g. both have positions, sizes, etc.). To access common methods, cast this enum
/// into a EzObject (trait for Layouts+Widgets) or EzWidget (Widgets only).
#[derive(Clone, Debug)]
pub enum EzObjects {
    Layout(Layout),
    Label(Label),
    Button(Button),
    Canvas(Canvas),
    Checkbox(Checkbox),
    Dropdown(Dropdown),
    DroppedDownMenu(DroppedDownMenu),
    RadioButton(RadioButton),
    TextInput(TextInput),
    Slider(Slider),
    ProgressBar(ProgressBar),
}
impl EzObjects {
    /// Get an EzObjects based on a string containing the base widget type.
    pub fn from_string(
        type_name: &str,
        path: String,
        id: String,
        scheduler: &mut SchedulerFrontend,
        state: EzState,
    ) -> Self {
        match type_name {
            "Layout" => EzObjects::Layout(Layout::from_state(id, path, scheduler, state)),
            "Canvas" => EzObjects::Canvas(Canvas::from_state(id, path, scheduler, state)),
            "Label" => EzObjects::Label(Label::from_state(id, path, scheduler, state)),
            "Button" => EzObjects::Button(Button::from_state(id, path, scheduler, state)),
            "CheckBox" => EzObjects::Checkbox(Checkbox::from_state(id, path, scheduler, state)),
            "RadioButton" => {
                EzObjects::RadioButton(RadioButton::from_state(id, path, scheduler, state))
            }
            "TextInput" => EzObjects::TextInput(TextInput::from_state(id, path, scheduler, state)),
            "Dropdown" => EzObjects::Dropdown(Dropdown::from_state(id, path, scheduler, state)),
            "Slider" => EzObjects::Slider(Slider::from_state(id, path, scheduler, state)),
            "ProgressBar" => {
                EzObjects::ProgressBar(ProgressBar::from_state(id, path, scheduler, state))
            }
            _ => panic!(
                "Cannot create widget from string \"{}\". This widget type does not exist.",
                type_name
            ),
        }
    }
    /// Cast this enum to a generic [EzObject] trait object. As this trait is implemented by both
    /// [layout] and [widget], it is safe to call on all variants.
    pub fn as_ez_object(&self) -> &dyn EzObject {
        match self {
            EzObjects::Label(i) => i,
            EzObjects::Button(i) => i,
            EzObjects::Layout(i) => i,
            EzObjects::Canvas(i) => i,
            EzObjects::Checkbox(i) => i,
            EzObjects::Dropdown(i) => i,
            EzObjects::DroppedDownMenu(i) => i,
            EzObjects::RadioButton(i) => i,
            EzObjects::TextInput(i) => i,
            EzObjects::Slider(i) => i,
            EzObjects::ProgressBar(i) => i,
        }
    }

    /// Cast this enum to a mutable generic [EzObject] trait object. As this trait is implemented
    /// by both [layout] and [widget], it is safe to call on all variants.
    pub fn as_ez_object_mut(&mut self) -> &mut dyn EzObject {
        match self {
            EzObjects::Layout(i) => i,
            EzObjects::Label(i) => i,
            EzObjects::Button(i) => i,
            EzObjects::Canvas(i) => i,
            EzObjects::Checkbox(i) => i,
            EzObjects::Dropdown(i) => i,
            EzObjects::DroppedDownMenu(i) => i,
            EzObjects::RadioButton(i) => i,
            EzObjects::TextInput(i) => i,
            EzObjects::Slider(i) => i,
            EzObjects::ProgressBar(i) => i,
        }
    }

    /// Cast this as a layout ref, you must be sure you have one.
    pub fn as_layout(&self) -> &Layout {
        if let EzObjects::Layout(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this as a mutable layout ref, you must be sure you have one.
    pub fn as_layout_mut(&mut self) -> &mut Layout {
        if let EzObjects::Layout(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }
    /// Cast this as a Canvas widget ref, you must be sure you have one.
    pub fn as_canvas(&self) -> &Canvas {
        if let EzObjects::Canvas(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this as a mutable Canvas widget ref, you must be sure you have one.
    pub fn as_canvas_mut(&mut self) -> &mut Canvas {
        if let EzObjects::Canvas(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this state as a Label widget ref, you must be sure you have one.
    pub fn as_label(&self) -> &Label {
        if let EzObjects::Label(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this state as a mutable Label widget ref, you must be sure you have one.
    pub fn as_label_mut(&mut self) -> &mut Label {
        if let EzObjects::Label(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this state as a Button widget ref, you must be sure you have one.
    pub fn as_button(&self) -> &Button {
        if let EzObjects::Button(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this state as a mutable Button widget ref, you must be sure you have one.
    pub fn as_button_mut(&mut self) -> &mut Button {
        if let EzObjects::Button(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this state as a Slider widget ref, you must be sure you have one.
    pub fn as_slider(&self) -> &Slider {
        if let EzObjects::Slider(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this state as a mutable slider widget ref, you must be sure you have one.
    pub fn as_slider_mut(&mut self) -> &mut Slider {
        if let EzObjects::Slider(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this state as a Progress bar widget ref, you must be sure you have one.
    pub fn as_progress_bar(&self) -> &ProgressBar {
        if let EzObjects::ProgressBar(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this state as a mutable Progress bar widget ref, you must be sure you have one.
    pub fn as_progress_bar_mut(&mut self) -> &mut ProgressBar {
        if let EzObjects::ProgressBar(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this as a Checkbox widget ref, you must be sure you have one.
    pub fn as_checkbox(&self) -> &Checkbox {
        if let EzObjects::Checkbox(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this as a mutable Checkbox widget ref, you must be sure you have one.
    pub fn as_checkbox_mut(&mut self) -> &mut Checkbox {
        if let EzObjects::Checkbox(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this state as a Dropdown widget ref, you must be sure you have one.
    pub fn as_dropdown(&self) -> &Dropdown {
        if let EzObjects::Dropdown(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this state as a mutable Dropdown widget ref, you must be sure you have one.
    pub fn as_dropdown_mut(&mut self) -> &mut Dropdown {
        if let EzObjects::Dropdown(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this state as a DroppedDownMenu widget ref, you must be sure you have one.
    pub fn as_dropped_down_menu(&self) -> &DroppedDownMenu {
        if let EzObjects::DroppedDownMenu(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this state as a mutable DroppedDownMenu widget ref, you must be sure you have one.
    pub fn as_dropped_down_menu_mut(&mut self) -> &mut DroppedDownMenu {
        if let EzObjects::DroppedDownMenu(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this state as a RadioButton widget ref, you must be sure you have one.
    pub fn as_radio_button(&self) -> &RadioButton {
        if let EzObjects::RadioButton(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this state as a mutable RadioButton widget ref, you must be sure you have one.
    pub fn as_radio_button_mut(&mut self) -> &mut RadioButton {
        if let EzObjects::RadioButton(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this state as a TextInput widget ref, you must be sure you have one.
    pub fn as_text_input(&self) -> &TextInput {
        if let EzObjects::TextInput(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }

    /// Cast this state as a mutable TextInput widget ref, you must be sure you have one.
    pub fn as_text_input_mut(&mut self) -> &mut TextInput {
        if let EzObjects::TextInput(i) = self {
            i
        } else {
            panic!("wrong EzObject.")
        }
    }
}

/// Trait representing both widgets and layouts implementing methods which are common to all UI
/// objects (such as size, position, etc.). If you don't know if an object is a Widget or a layout
/// (or don't care), cast the EzObjects enum into this type using [az_ez_object].
pub trait EzObject {
    /// Accepts config lines from the ez_parser module and prepares them to be loaded by
    /// load_ez_parameter below.
    fn load_ez_config(
        &mut self,
        config: Vec<String>,
        scheduler: &mut SchedulerFrontend,
        file: String,
        line: usize,
    ) -> Result<(), Error> {
        for (i, line_str) in config.iter().enumerate() {
            let total_line = line + i + 1;
            let (parameter_name, parameter_value) = line_str.split_once(':').unwrap_or_else(|| {
                panic!(
                    "Config parameter must contain a \":\", \
                e.g. \"parameter: value\". This does not contain one: \"{}\"",
                    line_str
                )
            });
            self.load_ez_parameter(
                parameter_name.to_string(),
                parameter_value.to_string(),
                scheduler,
            )
            .unwrap_or_else(|e| {
                panic!(
                    "Error on line {} of file: \"{}\".\n. \
                Could not load property \"{}\" with error: \"{}\"",
                    total_line, file, line_str, e
                )
            });
        }
        Ok(())
    }

    /// Load parameters for an object. Overloaded in each Widget/layout module to load parameters
    /// specific to the respective widget definition.
    fn load_ez_parameter(
        &mut self,
        parameter_name: String,
        parameter_value: String,
        scheduler: &mut SchedulerFrontend,
    ) -> Result<(), Error>;

    /// Set ID of the widget. IDs are used to create widgets paths. E.g.
    /// "/root_layout/sub_layout/widget_1".
    fn set_id(&mut self, id: &str);

    /// Get ID of the widget. IDs are used to create widgets paths. E.g.
    /// "/root_layout/sub_layout/widget_1".
    fn get_id(&self) -> String;

    /// Set full path to a widget. E.g. "/root_layout/sub_layout/widget_1". Call "get_by_path"
    /// method on the root layout and pass a full widget pass to retrieve a widget.
    fn set_path(&mut self, path: &str);

    /// Get full path to a widget. E.g. "/root_layout/sub_layout/widget_1". Call "get_by_path"
    /// method on the root layout and pass a full widget pass to retrieve a widget.
    fn get_path(&self) -> String;

    /// Return an [EzState]. Each EzObject must implement this to return the variant state
    /// that belongs to it.
    fn get_state(&self) -> EzState;

    /// Return a mut [EzState]. Each EzObject must implement this to return the variant state
    /// that belongs to it.
    fn get_state_mut(&mut self) -> &mut dyn GenericState;

    /// Redraw the widget on the screen. Using the view tree, only changed content is written to
    /// improve performance.
    fn redraw(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree) {
        let state = state_tree.get(&self.get_path()).as_generic();
        let pos = state.get_absolute_position();
        let content = self.get_contents(state_tree);
        view_tree.write_content(pos.as_coordinates(), content);
    }

    /// Set the content for a widget manually. This is not implemented for most widgets, as they
    /// get their content from their state. E.g. a label gets content from its' current text.
    fn set_contents(&mut self, _contents: PixelMap) {
        panic!(
            "Cannot manually set content color for this widget {}",
            self.get_id()
        );
    }

    /// Gets the visual content for this widget. Overloaded by each widget module. E.g. a label
    /// gets its' content from its' text, a checkbox from whether it has been checked, etc.
    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap;

    /// Optionally consume an event that was passed to this widget. Return true if the event should
    /// be considered consumed. Simply consults the keymap by default, but can be overloaded for
    /// more complex circumstances.
    fn handle_event(
        &self,
        event: Event,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        if let Event::Key(key) = event {
            if callback_tree
                .get(&self.get_path())
                .obj
                .keymap
                .contains(key.code, key.modifiers)
            {
                let func = callback_tree
                    .get_mut(&self.get_path())
                    .obj
                    .keymap
                    .get_mut(key.code, key.modifiers)
                    .unwrap();
                let context = Context::new(self.get_path(), state_tree, scheduler);
                func(context, key.code, key.modifiers);
                return true;
            }
        }
        false
    }

    /// Called on an object when it is selected and the user presses enter on the keyboard. This
    /// default implementation only calls the appropriate callback. Objects can overwrite this
    /// function but must remember to also call the callback.
    fn on_keyboard_enter(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        let consumed = self.on_keyboard_enter_callback(state_tree, callback_tree, scheduler);
        if !consumed {
            return self.on_press(state_tree, callback_tree, scheduler);
        }
        false
    }

    /// Call the bound callback if there is any. This method can always be called safely. Used to
    /// prevent a lot of duplicate ```if let Some(i)``` code.
    fn on_keyboard_enter_callback(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        if let Some(ref mut i) = callback_tree
            .get_mut(&self.get_path())
            .obj
            .on_keyboard_enter
        {
            return i(Context::new(self.get_path(), state_tree, scheduler));
        };
        false
    }

    /// Called on an object when it is left clicked. This default implementation only calls the
    /// appropriate callback. Objects can overwrite this function but must remember to also
    /// call the callback.
    fn on_left_mouse_click(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        mouse_pos: Coordinates,
    ) -> bool {
        let consumed =
            self.on_left_mouse_click_callback(state_tree, callback_tree, scheduler, mouse_pos);
        if !consumed {
            return self.on_press(state_tree, callback_tree, scheduler);
        }
        false
    }

    /// Call the bound callback if there is any. This method can always be called safely. Used to
    /// prevent a lot of duplicate ```if let Some(i)``` code.
    fn on_left_mouse_click_callback(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        mouse_pos: Coordinates,
    ) -> bool {
        if let Some(ref mut i) = callback_tree
            .get_mut(&self.get_path())
            .obj
            .on_left_mouse_click
        {
            return i(
                Context::new(self.get_path(), state_tree, scheduler),
                mouse_pos,
            );
        };
        false
    }

    /// Called on an object when it is selected and the user presses enter on the keyboard or
    /// when an object is left clicked. Default implementation only calls the appropriate callback.
    /// Objects can overwrite this function but must remember to also call the callback.
    fn on_press(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        self.on_press_callback(state_tree, callback_tree, scheduler)
    }

    /// Call the bound callback if there is any. This method can always be called safely. Used to
    /// prevent a lot of duplicate ```if let Some(i)``` code.
    fn on_press_callback(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        if let Some(ref mut i) = callback_tree.get_mut(&self.get_path()).obj.on_press {
            return i(Context::new(self.get_path(), state_tree, scheduler));
        };
        false
    }

    /// Called on an object when it is right clicked. This  default implementation only calls the
    /// appropriate callback. Objects can overwrite this function but must remember to also call
    /// the callback.
    fn on_right_mouse_click(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        mouse_pos: Coordinates,
    ) -> bool {
        self.on_right_mouse_click_callback(state_tree, callback_tree, scheduler, mouse_pos)
    }

    /// Call the bound callback if there is any. This method can always be called safely. Used to
    /// prevent a lot of duplicate ```if let Some(i)``` code.
    fn on_right_mouse_click_callback(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        mouse_pos: Coordinates,
    ) -> bool {
        if let Some(ref mut i) = callback_tree
            .get_mut(&self.get_path())
            .obj
            .on_right_mouse_click
        {
            return i(
                Context::new(self.get_path(), state_tree, scheduler),
                mouse_pos,
            );
        };
        false
    }

    /// Called on an object when it is mouse hovered. This default implementation only calls the
    /// appropriate callback. Objects can overwrite this function but must remember to also call
    /// the callback.
    fn on_hover(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        mouse_pos: Coordinates,
    ) -> bool {
        self.on_hover_callback(state_tree, callback_tree, scheduler, mouse_pos)
    }

    /// Call the bound callback if there is any. This method can always be called safely. Used to
    /// prevent a lot of duplicate ```if let Some(i)``` code.
    fn on_hover_callback(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        mouse_pos: Coordinates,
    ) -> bool {
        if let Some(ref mut i) = callback_tree.get_mut(&self.get_path()).obj.on_hover {
            return i(
                Context::new(self.get_path(), state_tree, scheduler),
                mouse_pos,
            );
        };
        false
    }

    /// Called on an object when it was mouse hovered but now the mouse has left the widget.
    /// This default implementation only calls the
    /// appropriate callback. Objects can overwrite this function but must remember to also call
    /// the callback.
    fn on_hover_exit(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        self.on_hover_exit_callback(state_tree, callback_tree, scheduler)
    }

    /// Call the bound callback if there is any. This method can always be called safely. Used to
    /// prevent a lot of duplicate ```if let Some(i)``` code.
    fn on_hover_exit_callback(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        if let Some(ref mut i) = callback_tree.get_mut(&self.get_path()).obj.on_hover_exit {
            return i(
                Context::new(self.get_path(), state_tree, scheduler));
        };
        false
    }

    /// Called on an object when it is left mouse dragged. This default implementation only calls
    /// the appropriate callback. Objects can overwrite this function but must remember to also
    /// call the callback.
    fn on_drag(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        previous_pos: Option<IsizeCoordinates>,
        mouse_pos: IsizeCoordinates,
    ) -> bool {
        self.on_drag_callback(
            state_tree,
            callback_tree,
            scheduler,
            previous_pos,
            mouse_pos,
        )
    }

    /// Call the bound callback if there is any. This method can always be called safely. Used to
    /// prevent a lot of duplicate ```if let Some(i)``` code.
    fn on_drag_callback(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        previous_pos: Option<IsizeCoordinates>,
        mouse_pos: IsizeCoordinates,
    ) -> bool {
        if let Some(ref mut i) = callback_tree.get_mut(&self.get_path()).obj.on_drag {
            return i(
                Context::new(self.get_path(), state_tree, scheduler),
                previous_pos,
                mouse_pos,
            );
        };
        false
    }

    /// Called on an object when it is mouse scrolled up. This default implementation only calls the
    /// appropriate callback. Objects can overwrite this function but must remember to also call
    /// the callback.
    fn on_scroll_up(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        self.on_scroll_up_callback(state_tree, callback_tree, scheduler)
    }

    /// Call the bound callback if there is any. This method can always be called safely. Used to
    /// prevent a lot of duplicate ```if let Some(i)``` code.
    fn on_scroll_up_callback(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        if let Some(ref mut i) = callback_tree.get_mut(&self.get_path()).obj.on_scroll_up {
            return i(Context::new(self.get_path(), state_tree, scheduler));
        };
        false
    }

    /// Called on an object when it is mouse scrolled down. This default implementation only calls
    /// the appropriate callback. Objects can overwrite this function but must remember to also call
    /// the callback.
    fn on_scroll_down(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        self.on_scroll_down_callback(state_tree, callback_tree, scheduler)
    }

    /// Call the bound callback if there is any. This method can always be called safely. Used to
    /// prevent a lot of duplicate ```if let Some(i)``` code.
    fn on_scroll_down_callback(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        if let Some(ref mut i) = callback_tree.get_mut(&self.get_path()).obj.on_scroll_down {
            return i(Context::new(self.get_path(), state_tree, scheduler));
        };
        false
    }

    /// Called on an object when its' value changes. This default implementation only calls the
    /// appropriate callback. Objects can overwrite this function but must remember to also call
    /// the callback.
    fn on_value_change(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        self.on_value_change_callback(state_tree, callback_tree, scheduler)
    }

    /// Call the bound callback if there is any. This method can always be called safely. Used to
    /// prevent a lot of duplicate ```if let Some(i)``` code.
    fn on_value_change_callback(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        if let Some(ref mut i) = callback_tree.get_mut(&self.get_path()).obj.on_value_change {
            return i(Context::new(self.get_path(), state_tree, scheduler));
        };
        false
    }

    /// Called on an object when it is selected. This default implementation only calls the
    /// appropriate callback. Objects can overwrite this function but must remember to also call
    /// the callback.
    fn on_select(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        mouse_pos: Option<Coordinates>,
    ) -> bool {
        self.on_select_callback(state_tree, callback_tree, scheduler, mouse_pos)
    }

    /// Call the bound callback if there is any. This method can always be called safely. Used to
    /// prevent a lot of duplicate ```if let Some(i)``` code.
    fn on_select_callback(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        mouse_pos: Option<Coordinates>,
    ) -> bool {
        if let Some(ref mut i) = callback_tree.get_mut(&self.get_path()).obj.on_select {
            return i(
                Context::new(self.get_path(), state_tree, scheduler),
                mouse_pos,
            );
        };
        false
    }

    /// Called on an object when it is deselected. This default implementation only calls the
    /// appropriate callback. Objects can overwrite this function but must remember to also call
    /// the callback.
    fn on_deselect(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        self.on_deselect_callback(state_tree, callback_tree, scheduler)
    }

    /// Call the bound callback if there is any. This method can always be called safely. Used to
    /// prevent a lot of duplicate ```if let Some(i)``` code.
    fn on_deselect_callback(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        if let Some(ref mut i) = callback_tree.get_mut(&self.get_path()).obj.on_deselect {
            return i(Context::new(self.get_path(), state_tree, scheduler));
        };
        false
    }

    fn get_clone(&self, scheduler: &mut SchedulerFrontend) -> EzObjects;
}
