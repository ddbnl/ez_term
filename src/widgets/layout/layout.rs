//! # layout
//! Module implementing the layout struct.
use std::collections::HashMap;
use crossterm::event::{Event, KeyCode};
use crate::parser::load_base_properties::{load_ez_bool_property, load_ez_string_property};
use crate::parser::load_properties::load_common_property;
use crate::widgets::ez_object::{EzObject, EzObjects};
use crate::states::layout_state::LayoutState;
use crate::states::ez_state::{EzState, GenericState};
use crate::scheduler::scheduler::Scheduler;
use crate::states::definitions::{LayoutMode, LayoutOrientation};
use crate::property::ez_values::EzValues;
use crate::run::definitions::{CallbackTree, Coordinates, Pixel, PixelMap, StateTree, WidgetTree};
use crate::run::tree::ViewTree;
use crate::widgets::helper_functions::{add_border, add_padding, reposition_with_pos_hint,
                                       resize_with_size_hint};


/// A layout is where widgets live. They implements methods for hardcoding widget placement or
/// placing them automatically in various ways.
#[derive(Clone, Debug)]
pub struct Layout {

    /// ID of the layout, used to construct [path]
    pub id: String,

    /// Full path to this layout, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// List of children widgets and/or layouts
    pub children: Vec<EzObjects>,

    /// Child ID to index in [children] lookup. Used to get widgets by [id] and [path]
    pub child_lookup: HashMap<String, usize>,

    /// Runtime state of this layout, see [LayoutState] and [State]
    pub state: LayoutState,
}


impl Layout {
    pub fn new(id: String, path: String, scheduler: &mut Scheduler) -> Self {
        Layout {
            id,
            path: path.clone(),
            children: Vec::new(),
            child_lookup: HashMap::new(),
            state: LayoutState::new(path, scheduler),
        }
    }

    fn load_fill_property(&mut self, parameter_value: &str, scheduler: &mut Scheduler) {

        let path = self.path.clone();
        self.state.set_fill(load_ez_bool_property(
            parameter_value.trim(), scheduler, path.clone(),
            Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                let state = state_tree.get_by_path_mut(&path)
                    .as_layout_mut();
                state.set_fill(val.as_bool().to_owned());
                path.clone()
            })))
    }

    fn load_filler_symbol_property(&mut self, parameter_value: &str, scheduler: &mut Scheduler) {

        let path = self.path.clone();
        self.state.set_filler_symbol(load_ez_string_property(
            parameter_value.trim(), scheduler, path.clone(),
            Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                let state = state_tree.get_by_path_mut(&path)
                    .as_layout_mut();
                state.set_filler_symbol(val.as_string().to_owned());
                path.clone()
            })))
    }

    fn load_scrolling_enable_x_property(&mut self, parameter_value: &str,
                                        scheduler: &mut Scheduler) {

        let path = self.path.clone();
        self.state.get_scrolling_config_mut().enable_x.set(load_ez_bool_property(
            parameter_value.trim(), scheduler, path.clone(),
            Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                let state = state_tree.get_by_path_mut(&path)
                    .as_layout_mut();
                state.get_scrolling_config_mut().enable_x.set(val.as_bool().to_owned());
                path.clone()
            })))
    }

    fn load_scrolling_enable_y_property(&mut self, parameter_value: &str,
                                        scheduler: &mut Scheduler) {

        let path = self.path.clone();
        self.state.get_scrolling_config_mut().enable_y.set(load_ez_bool_property(
            parameter_value.trim(), scheduler, path.clone(),
            Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                let state = state_tree.get_by_path_mut(&path)
                    .as_layout_mut();
                state.get_scrolling_config_mut().enable_y.set(val.as_bool().to_owned());
                path.clone()
            })))
    }
}


impl EzObject for Layout {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String,
                         scheduler: &mut Scheduler) {

        let consumed = load_common_property(
            &parameter_name, parameter_value.clone(),self, scheduler);
        if consumed { return }
        match parameter_name.as_str() {
            "mode" => {
                match parameter_value.to_lowercase().trim() {
                    "box" => self.state.mode = LayoutMode::Box,
                    "float" => self.state.mode = LayoutMode::Float,
                    "screen" => self.state.mode = LayoutMode::Screen,
                    "tabbed" => self.state.mode = LayoutMode::Tabbed,
                    _ => panic!("Invalid parameter value for mode {}", parameter_value)
                }
            },
            "orientation" => {
                match parameter_value.trim() {
                    "horizontal" =>
                        self.state.orientation = LayoutOrientation::Horizontal,
                    "vertical" =>
                        self.state.orientation = LayoutOrientation::Vertical,
                    _ => panic!("Invalid parameter value for orientation {}",
                                       parameter_value)
                }
            },
            "scroll" => {
                let (x, y) = parameter_value.split_once(',').unwrap();
                self.load_scrolling_enable_x_property(x.trim(), scheduler);
                self.load_scrolling_enable_y_property(y.trim(), scheduler);
            }
            "scroll_x" => self.load_scrolling_enable_x_property(
                parameter_value.trim(), scheduler),
            "scroll_y" => self.load_scrolling_enable_y_property(
                parameter_value.trim(), scheduler),
            "fill" => self.load_fill_property(parameter_value.trim(), scheduler),
            "filler_symbol" =>
                self.load_filler_symbol_property(parameter_value.trim(), scheduler),
            _ => panic!("Invalid parameter name for layout {}", parameter_name)
        }
    }
    fn set_id(&mut self, id: String) { self.id = id; }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::Layout(self.state.clone()) }

    fn get_state_mut(&mut self) -> &mut dyn GenericState { &mut self.state }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let mut merged_content = PixelMap::new();
        let mode = state_tree.get_by_path(&self.path).as_layout().mode.clone();
        let orientation =
            state_tree.get_by_path(&self.path).as_layout().orientation.clone();
        match mode {
            LayoutMode::Box => {
                match orientation {
                    LayoutOrientation::Horizontal => {
                        merged_content =
                            self.get_box_mode_horizontal_orientation_contents(state_tree);
                    },
                    LayoutOrientation::Vertical => {
                        merged_content =
                            self.get_box_mode_vertical_orientation_contents(state_tree);
                    },
                }
            },
            LayoutMode::Float => {
                merged_content =
                    self.get_float_mode_contents(merged_content, state_tree);
            },
            LayoutMode::Screen => {
                merged_content = self.get_screen_mode_contents(state_tree);
            },
            LayoutMode::Tabbed => {
                merged_content = self.get_tabbed_mode_contents(state_tree);
            }
        }

        merged_content = self.add_user_filler(state_tree, merged_content);
        merged_content = self.auto_scale_to_content(state_tree, merged_content);
        merged_content = self.add_empty_filler(state_tree, merged_content);
        merged_content = self.create_horizontal_scroll_box(state_tree, merged_content);
        merged_content = self.create_vertical_scroll_box(state_tree, merged_content);
        let state = state_tree.get_by_path(&self.get_full_path()).as_layout();

        if merged_content.is_empty() { return merged_content } // Empty widget
        // Put border around content if border if set
        if state.get_border_config().enabled.value {
            merged_content = add_border(merged_content, state.get_border_config());
        }
        // Put padding around content if set
        merged_content = add_padding(
            merged_content, state.get_padding(),
            state.get_color_config().background.value,
            state.get_color_config().foreground.value);
        merged_content = self.get_modal_contents(state_tree, merged_content);

        self.propagate_absolute_positions(state_tree);
        merged_content
    }

    fn handle_event(&self, event: Event, _view_tree: &mut ViewTree, state_tree: &mut StateTree,
                    _widget_tree: &WidgetTree, _callback_tree: &mut CallbackTree,
                    scheduler: &mut Scheduler) -> bool {

        let state = state_tree.get_by_path_mut(&self.get_full_path())
            .as_layout_mut();
        if let Event::Key(key) = event {
            if key.code == KeyCode::PageUp {
                self.handle_scroll_up(state_tree, scheduler);
                return true
            } else if key.code == KeyCode::PageDown {
                self.handle_scroll_down(state_tree, scheduler);
                return true
            } else if key.code == KeyCode::Left {
                if state.get_mode() == &LayoutMode::Tabbed {
                    self.handle_tab_left(state_tree, scheduler);
                } else {
                    self.handle_scroll_left(state_tree, scheduler);
                }
                return true
            } else if key.code == KeyCode::Right {
                if state.get_mode() == &LayoutMode::Tabbed {
                    self.handle_tab_right(state_tree, scheduler);
                } else {
                    self.handle_scroll_right(state_tree, scheduler);
                }
                return true
            }
        }
        false
    }

    fn on_keyboard_enter(&self, _view_tree: &mut ViewTree, state_tree: &mut StateTree,
                         _widget_tree: &WidgetTree, _callback_tree: &mut CallbackTree,
                         scheduler: &mut Scheduler) -> bool {
        let state = state_tree.get_by_path_mut(&self.path).as_layout_mut();
        if !state.selected_tab_header.is_empty() {
            state.set_active_tab(state.get_selected_tab_header()
                .strip_suffix("_tab_header").unwrap().to_string());
            state.update(scheduler);
            return true
        }
        false
    }

    fn on_left_mouse_click(&self, _view_tree: &mut ViewTree, state_tree: &mut StateTree,
                           _widget_tree: &WidgetTree, _callback_tree: &mut CallbackTree,
                           scheduler: &mut Scheduler, mouse_pos: Coordinates) -> bool {

        let state = state_tree.get_by_path_mut(&self.path).as_layout_mut();

        if state.scrolling_config.is_scrolling_x &&
            mouse_pos.y == state.get_effective_size().height + 1 {

            let (scrollbar_size, scrollbar_pos) =
                self.get_horizontal_scrollbar_parameters(
                state.get_scrolling_config().original_width,
                state.get_effective_size().width,
                state.get_scrolling_config().view_start_x);

            if mouse_pos.x < scrollbar_pos {
                self.handle_scroll_left(state_tree, scheduler);
                return true
            } else if mouse_pos.x > scrollbar_pos + scrollbar_size {
                self.handle_scroll_right(state_tree, scheduler);
                return true
            }
        }

        if state.scrolling_config.is_scrolling_y &&
            mouse_pos.x == state.get_size().width.value - 1 {

            let (scrollbar_size, scrollbar_pos) = self.get_vertical_scrollbar_parameters(
                state.get_scrolling_config().original_height,
                state.get_effective_size().height,
                state.get_scrolling_config().view_start_y);

            if mouse_pos.y < scrollbar_pos {
                self.handle_scroll_up(state_tree, scheduler);
                return true
            } else if mouse_pos.y > scrollbar_pos + scrollbar_size {
                self.handle_scroll_down(state_tree, scheduler);
                return true
            }
        }
        false
    }

    fn on_scroll_up(&self, _view_tree: &mut ViewTree, state_tree: &mut StateTree,
                    _widget_tree: &WidgetTree, _callback_tree: &mut CallbackTree,
                    scheduler: &mut Scheduler) -> bool {

        let state = state_tree.get_by_path_mut(&self.path).as_layout_mut();
        if state.scrolling_config.is_scrolling_y || state.scrolling_config.is_scrolling_x {
            self.handle_scroll_up(state_tree, scheduler);
            return true
        }
        false
    }

    fn on_scroll_down(&self, _view_tree: &mut ViewTree, state_tree: &mut StateTree,
                      _widget_tree: &WidgetTree, _callback_tree: &mut CallbackTree,
                      scheduler: &mut Scheduler) -> bool {

        let state = state_tree.get_by_path_mut(&self.path).as_layout_mut();
        if state.scrolling_config.is_scrolling_y || state.scrolling_config.is_scrolling_x {
            self.handle_scroll_down(state_tree, scheduler);
            return true
        }
        false
    }

    fn on_select(&self, _view_tree: &mut ViewTree, state_tree: &mut StateTree,
                 _widget_tree: &WidgetTree, _callback_tree: &mut CallbackTree,
                 scheduler: &mut Scheduler, _mouse_pos: Option<Coordinates>) -> bool {

        for child in self.children.iter() {
            if let EzObjects::Button(i) = child {
                let state = state_tree.get_by_path_mut(&self.path).as_layout_mut();
                state.selected_tab_header = i.path.clone();
                state.update(scheduler);
                return true
            }
        }
        true
    }

    fn on_deselect(&self, _view_tree: &mut ViewTree, state_tree: &mut StateTree,
                 _widget_tree: &WidgetTree, _callback_tree: &mut CallbackTree,
                 scheduler: &mut Scheduler) -> bool {

        let state = state_tree.get_by_path_mut(&self.path).as_layout_mut();
        state.selected_tab_header.clear();
        state.update(scheduler);
        true
    }
}

impl Layout {
    /// Initialize an instance of a layout with the passed config parsed by [ez_parser]
    pub fn from_config(config: Vec<String>, id: String, path: String, scheduler: &mut Scheduler)
                       -> Self {
        let mut obj = Layout::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler).unwrap();
        obj
    }

    /// Scale size down to the size of the actual content of the layout.
    fn auto_scale_to_content(&self, state_tree: &mut StateTree, contents: PixelMap) -> PixelMap {
        let state = state_tree.get_by_path_mut(&self.get_full_path())
            .as_layout_mut();
        // If user wants to autoscale width we set width to length of content
        if state.get_auto_scale().width.value {
            let auto_scale_width = contents.len();
            if auto_scale_width < state.get_effective_size().width {
                state.set_effective_width(auto_scale_width);
            }
        }
        // If user wants to autoscale height we set height to the highest column
        if state.get_auto_scale().height.value {
            let auto_scale_height = contents.iter()
                .map(|x| x.len()).max().unwrap_or(0);
            if auto_scale_height < state.get_effective_size().height {
                state.set_effective_height(auto_scale_height);
            }
        }
        contents
    }

    /// Overwrite a PixelMap of current own content with the content of the active modal. Modals
    /// overlap all content.
    fn get_modal_contents(&self, state_tree: &mut StateTree, mut contents: PixelMap) -> PixelMap {
        if state_tree.get_by_path(&self.get_full_path()).as_layout().get_modals().is_empty() {
            return contents
        }

        // Size modal
        let parent_size = state_tree.get_by_path(&self.get_full_path()).as_layout()
            .get_size().clone();
        let modal = state_tree.get_by_path(&self.get_full_path()).as_layout()
            .get_modals().first().unwrap().clone();
        let state = state_tree
            .get_by_path_mut(&modal.as_ez_object().get_full_path());
        resize_with_size_hint(state, parent_size.width.value,
                              parent_size.height.value);
        reposition_with_pos_hint(
            parent_size.width.value, parent_size.height.value,
            state.as_generic_mut());
        let x = state.as_generic().get_position().x.value;
        let y = state.as_generic().get_position().y.value;
        state.as_generic_mut().set_absolute_position(Coordinates::new(x, y));

        //Get contents
        let modal_content;
        if let EzObjects::Layout(ref i) = modal {
            i.set_child_sizes(state_tree);
            modal_content = i.get_contents(state_tree);
            i.propagate_absolute_positions(state_tree);
        } else {
            modal_content = modal.as_ez_object().get_contents(state_tree);
        }

        // Overwrite own content with modal (modal is always on top)
        let state = state_tree
            .get_by_path_mut(&state_tree.get_by_path(&self.get_full_path()).as_layout()
                .get_modals().first().unwrap().as_ez_object()
                .get_full_path()).as_generic();
        let start_pos = state.get_position();
        for x in 0..modal_content.len() {
            for y in 0..modal_content[x].len() {
                let write_pos = Coordinates::new(start_pos.x.get() + x,
                                                 start_pos.y.get() + y);
                if write_pos.x < parent_size.width.value &&
                    write_pos.y < parent_size.height.value {
                    contents[write_pos.x][write_pos.y] = modal_content[x][y].clone();
                }
            }
        }
        contents
    }
    /// Fill any empty positions with [Pixel] from [get_filler]
    pub fn add_user_filler(&self, state_tree: &mut StateTree, mut contents: PixelMap) -> PixelMap {

        let state = state_tree.get_by_path_mut(&self.get_full_path())
            .as_layout_mut();
        if !state.fill.value { return contents }

        let filler = Pixel::new(state.get_filler_symbol().value.clone(),
                                state.get_color_config().filler_foreground.value,
                                state.get_color_config().filler_background.value);

        for x in 0..state.get_effective_size().width {
            for y in contents[x].iter_mut() {
                if y.symbol.is_empty() || y.symbol == " " {
                    y.symbol = filler.symbol.clone();
                }
            }
            while contents[x].len() < (state.get_effective_size().height) {
                contents[x].push(filler.clone());
            }
        }
        while contents.len() < state.get_effective_size().width {
            let mut new_x = Vec::new();
            for _ in 0..state.get_effective_size().height {
                new_x.push(filler.clone());
            }
            contents.push(new_x);
        }
        contents
    }

    /// Fill any empty positions with empty [Pixel]. Used to fill full size of the layout in case
    /// the user did not define a custom filler.
    pub fn add_empty_filler(&self, state_tree: &mut StateTree, mut contents: PixelMap) -> PixelMap {

        let state = state_tree.get_by_path_mut(&self.get_full_path())
            .as_layout_mut();
        while contents.len() < state.get_effective_size().width {
            contents.push(Vec::new());
        }
        for x in contents.iter_mut() {
            while x.len() < state.get_effective_size().height {
                x.push(Pixel::new(
                    " ".to_string(), state.get_color_config().foreground.value,
                    state.get_color_config().background.value));
            }
        }
        contents
    }
}
