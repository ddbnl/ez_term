//! A widget that displays text non-interactively.
use std::io::{Error, ErrorKind};
use std::time::Duration;

use crate::parser::load_base_properties;
use crate::parser::load_common_properties::load_common_property;
use crate::run::definitions::{CallbackTree, Coordinates, Pixel, PixelMap, StateTree};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::button_state::ButtonState;
use crate::states::definitions::{HorizontalAlignment, VerticalAlignment};
use crate::states::ez_state::{EzState, GenericState};
use crate::widgets::ez_object::{EzObject, EzObjects};
use crate::widgets::helper_functions::{add_border, add_padding, align_content_horizontally, align_content_vertically, format_text, wrap_text};
use crate::Context;
use crate::scheduler::definitions::CustomDataMap;

#[derive(Clone, Debug)]
pub struct Button {
    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [LabelState] and [State]
    pub state: ButtonState,
}

impl Button {
    pub fn new(id: String, path: String, scheduler: &mut SchedulerFrontend) -> Self {
        Button {
            id,
            path: path.clone(),
            state: ButtonState::new(path, scheduler),
        }
    }

    pub fn from_state(
        id: String,
        path: String,
        _scheduler: &mut SchedulerFrontend,
        state: EzState,
    ) -> Self {
        Button {
            id,
            path: path.clone(),
            state: state.as_button().to_owned(),
        }
    }
}

impl EzObject for Button {
    fn load_ez_parameter(
        &mut self,
        parameter_name: String,
        parameter_value: String,
        scheduler: &mut SchedulerFrontend,
    ) -> Result<(), Error> {
        let consumed =
            load_common_property(&parameter_name, parameter_value.clone(), self, scheduler)?;
        if consumed {
            return Ok(());
        }
        match parameter_name.as_str() {
            "text" => load_base_properties::load_string_property(
                parameter_value.trim(),
                scheduler,
                self.path.clone(),
                &parameter_name,
                self.get_state_mut(),
            )?,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid parameter name for button: {}", parameter_name),
                ))
            }
        }
        Ok(())
    }

    fn set_id(&mut self, id: &str) {
        self.id = id.to_string()
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn set_path(&mut self, id: &str) {
        self.id = id.to_string()
    }

    fn get_path(&self) -> String {
        self.path.clone()
    }

    fn get_state(&self) -> EzState {
        EzState::Button(self.state.clone())
    }

    fn get_state_mut(&mut self) -> &mut dyn GenericState {
        &mut self.state
    }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree.get_mut(&self.get_path()).as_button_mut();

        let (fg_color, bg_color) = if state.get_flashing() {
            (
                state.get_color_config().get_flash_fg_color(),
                state.get_color_config().get_flash_bg_color(),
            )
        } else {
            state.get_context_colors()
        };

        let text = state.get_text();

        let write_width =
            if state.get_infinite_size().width || state.get_auto_scale().get_auto_scale_width() {
                text.len() + 1
            } else {
                state.get_effective_size().width
            };

        let default_pixel = Pixel::new(" ".to_string(),
                                       fg_color,bg_color);
        let (text, pixels) = format_text(text, default_pixel.clone());
        let content_lines = wrap_text(text, write_width, pixels);
        let write_height =
            if state.get_infinite_size().height || state.get_auto_scale().get_auto_scale_height() {
                content_lines.len()
            } else {
                state.get_effective_size().height
            };

        let longest_line = content_lines.iter().map(|x| x.len()).max();
        let longest_line = if let Some(i) = longest_line { i } else { 0 };
        let mut contents = Vec::new();
        for x in 0..longest_line {
            let mut new_y: Vec<Pixel> = Vec::new();
            for y in 0..write_height {
                if y < content_lines.len() && x < content_lines[y].len() {
                    new_y.push(content_lines[y][x].clone());
                } else {
                    new_y.push(default_pixel.clone());
                }
            }
            contents.push(new_y);
        }
        if state.get_auto_scale().get_auto_scale_width() {
            state.set_effective_width(contents.len());
        }
        if state.get_auto_scale().get_auto_scale_height() {
            let height = if !contents.is_empty() {
                contents[0].len()
            } else {
                0
            };
            state.set_effective_height(height);
        }
        (contents, _) = align_content_horizontally(
            contents,
            HorizontalAlignment::Center,
            state.get_effective_size().width,
            " ".to_string(),
            fg_color,
            bg_color,
        );
        (contents, _) = align_content_vertically(
            contents,
            VerticalAlignment::Middle,
            state.get_effective_size().height,
            " ".to_string(),
            fg_color,
            bg_color,
        );
        contents = add_border(
            contents,
            state.get_border_config(),
            state.get_color_config(),
        );
        let state = state_tree.get(&self.get_path()).as_button();
        let parent_colors = state_tree
            .get(self.get_path().rsplit_once('/').unwrap().0)
            .as_generic()
            .get_color_config();
        contents = add_padding(
            contents,
            state.get_padding(),
            parent_colors.get_bg_color(),
            parent_colors.get_fg_color(),
        );
        contents
    }

    fn on_press(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        custom_data: &mut CustomDataMap,
    ) -> bool {
        let consumed = self.on_press_callback(state_tree, callback_tree, scheduler, custom_data);
        if consumed {
            return consumed;
        }
        self.handle_on_press(state_tree, scheduler);
        return true;
    }

    fn on_hover(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        mouse_pos: Coordinates,
        custom_data: &mut CustomDataMap,
    ) -> bool {
        let consumed = self.on_hover_callback(state_tree, callback_tree, scheduler, mouse_pos,
                                              custom_data);
        if consumed {
            return consumed;
        }
        scheduler.set_selected_widget(&self.path, Some(mouse_pos));
        true
    }

    fn get_clone(&self, scheduler: &mut SchedulerFrontend) -> EzObjects {
        let mut clone = self.clone();
        let mut new_state = ButtonState::new(self.path.clone(), scheduler);
        new_state.copy_state_values(self.get_state());
        clone.state = new_state;
        EzObjects::Button(clone)
    }
}
impl Button {
    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(
        config: Vec<String>,
        id: String,
        path: String,
        scheduler: &mut SchedulerFrontend,
        file: String,
        line: usize,
    ) -> Self {
        let mut obj = Button::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler, file, line).unwrap();
        obj
    }

    pub fn handle_on_press(&self, state_tree: &mut StateTree, scheduler: &mut SchedulerFrontend) {
        let state = state_tree.get_mut(&self.get_path()).as_button_mut();
        state.set_flashing(true);
        state.update(scheduler);
        let path = self.path.clone();
        let scheduled_func = move |context: Context| {
            if !context.state_tree.contains(path.as_str()) {
                return;
            }
            let state = context.state_tree.get_mut(path.as_str()).as_button_mut();
            state.set_flashing(false);
            state.update(context.scheduler);
        };
        scheduler.schedule_once(
            self.get_path().as_str(),
            Box::new(scheduled_func),
            Duration::from_millis(50),
        );
    }
}
