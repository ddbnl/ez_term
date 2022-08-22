//! # EzTerm
//!
//! A terminal-UI framework focussed on simplicity. Create interfaces through YAML-ish config files
//! using smart-layouts and basic widgets. No need to specify specific sizes or positions (unless
//! you want to) and no need to write code except for callbacks. Focus on coding your app, not the
//! UI.
//!
//! If you are new to EzTerm, it is recommended to at least read the [Project structure](#structure)
//! section below. It takes only a few minutes to read, and explains how to set up a new cargo
//! project for an EzTerm project. After you have a [Minimal example](#minimal_example), you can
//! either continue reading the general tutorial with the [EzLanguage](#ez_lang) section, or use
//! the [Examples](#examples) if you prefer that over reading.
//!
//! Once you are familiar with the basics and are working on your own projects, you can use the
//! [Reference](#reference) section to look up details on available properties, callbacks, etc.
//!
//! **Docs table of contents:**
//! 1. [Tutorial - How to use EzTerm](#how_to_use)
//!     1. [Project structure](#structure)
//!         1. [UI config files](#ui_config_files)
//!         2. [UI Rust modules](#ui_rust_modules)
//!         3. [Your app](#your_app)
//!         4. [Minimal example](#minimal_example)
//!     2. [Ez language](#ez_language)
//!         1. [Basics](#ez_language_basics)
//!         2. [Templates](#ez_language_templates)
//!         3. [Layouts Modes](#ez_language_layout_modes)
//!             1. [Box Mode](#ez_language_box)
//!             2. [Stack Mode](#ez_language_stack)
//!             3. [Table Mode](#ez_language_table)
//!             4. [Float Mode](#ez_language_float)
//!             5. [Tab Mode](#ez_language_tab)
//!             6. [Screen Mode](#ez_language_screen)
//!             7. [Scrolling](#ez_language_scrolling)
//!             8. [Views](#ez_language_views)
//!         4. [Widget overview](#widget_overview)
//!         5. [Sizing](#sizing)
//!             1. [Relative sizing: size hints](#sizing_relative)
//!             2. [Auto-scaling](#sizing_scaling)
//!             3. [Absolute size - Manual](#sizing_absolute)
//!             4. [Absolute size - Math](#sizing_math)
//!         6. [Positioning](#)
//!             1. [Automatic positioning: layout modes](#positioning_automatic)
//!             2. [Relative positioning: position hints](#positioning_relative)
//!             3. [Absolute positioning - Manual](#positioning_absolute)
//!             3. [Absolute positioning - Maths](#positioning_maths)
//!             4. [Adjusting position: aligning and padding](#positioning_adjusting)
//!         7. [Keyboard selection](#keyboard_selection)
//!         8. [Binding properties](#binding_properties)
//!             1. [Referring to other properties](#binding_properties_referring)
//!             2. [Property type overview](#binding_properties_overview)
//!             3. [Using maths on properties](#binding_properties_maths)
//!     3. [Scheduler](#scheduler)
//!         1. [Widget states and the State Tree](#scheduler_states)
//!         2. [Using the scheduler object](#scheduler_object)
//!         3. [Managing callbacks](#scheduler_callbacks)
//!             1. [General callback structure](#scheduler_callbacks_structure)
//!             2. [Callback config](#scheduler_callbacks_config)
//!             3. [On_keyboard_enter](#scheduler_callbacks_enter)
//!             4. [On_left_mouse_click](#scheduler_callbacks_left)
//!             5. [On_press](#scheduler_callbacks_press)
//!             6. [On_select](#scheduler_callbacks_select)
//!             7. [On_deselect](#scheduler_callbacks_deselect)
//!             8. [On_right_mouse_click](#scheduler_callbacks_right)
//!             9. [On_hover](#scheduler_callbacks_hover)
//!             10. [On_drag](#scheduler_callbacks_drag)
//!             11. [On_scroll_up](#scheduler_callbacks_up)
//!             12. [On_scroll_down](#scheduler_callbacks_down)
//!             13. [On_value_change](#scheduler_callbacks_value)
//!             14. [Custom key binds](#scheduler_callbacks_keymap)
//!             15. [Property binds](#scheduler_callbacks_property)
//!         4. [Managing scheduled tasks](#scheduler_tasks)
//!             1. [Single execution](#scheduler_tasks_single)
//!             2. [Recurring execution](#scheduler_tasks_recurring)
//!             3. [Threaded execution](#scheduler_tasks_threaded)
//!                 1. [Using StateTree](#scheduler_tasks_threaded_state)
//!                 2. [Using custom properties](#scheduler_tasks_threaded_property)
//!         5. [Creating custom properties](#scheduler_properties)
//!         6. [Managing popups](#scheduler_modals)
//!         7. [Managing widgets programmatically](#scheduler_widgets_from_code)
//!             1. [Creating widgets from code](#scheduler_programmatic_create)
//!             2. [Removing widgets from code](#scheduler_programmatic_remove)
//!         8. [Updating widgets](#scheduler_updating)
//!         9. [Managing widget selection](#scheduler_selection)
//!     4. [Global (key)bindings](#keybindings)
//! 2. [Reference](#reference)
//!     1. [Common properties](#reference_common)
//!         1. [x](#reference_common_x)
//!         2. [y](#reference_common_y)
//!         3. [pos](#reference_common_pos)
//!         4. [width](#reference_common_width)
//!         5. [height](#reference_common_height)
//!         6. [size](#reference_common_size)
//!         7. [size_hint_x](#reference_common_size_hint_x)
//!         8. [size_hint_y](#reference_common_size_hint_y)
//!         9. [size_hint](#reference_common_size_hint)
//!         10. [pos_hint_x](#reference_common_pos_hint_x)
//!         11. [pos_hint_y](#reference_common_pos_hint_y)
//!         12. [pos_hint](#reference_common_pos_hint)
//!         13. [auto_scale_width](#reference_common_auto_scale_width)
//!         14. [auto_scale_height](#reference_common_auto_scale_height)
//!         15. [auto_scale](#reference_common_auto_scale)
//!         16. [padding_top](#reference_common_padding_top)
//!         17. [padding_bottom](#reference_common_padding_bottom)
//!         18. [padding_left](#reference_common_padding_left)
//!         19. [padding_right](#reference_common_padding_right)
//!         20. [padding_x](#reference_common_padding_x)
//!         21. [padding_y](#reference_common_padding_y)
//!         22. [padding](#reference_common_padding)
//!         23. [halign](#reference_common_halign)
//!         24. [valign](#reference_common_valign)
//!         25. [disabled](#reference_common_disabled)
//!         26. [selection_order](#reference_common_selection_order)
//!         27. [border](#reference_common_border)
//!         28. [horizontal_symbol](#reference_common_horizontal_symbol)
//!         29. [vertical_symbol](#reference_common_vertical_symbol)
//!         30. [top_left_symbol](#reference_common_top_left_symbol)
//!         31. [top_right_symbol](#reference_common_top_right_symbol)
//!         32. [bottom_left_symbol](#reference_common_bottom_left_symbol)
//!         33. [bottom_right_symbol](#reference_common_bottom_right_symbol)
//!         34. [fg_color](#reference_common_fg_color)
//!         35. [bg_color](#reference_common_bg_color)
//!         36. [selection_fg_color](#reference_common_selection_fg_color)
//!         37. [selection_bg_color](#reference_common_selection_bg_color)
//!         38. [disabled_fg_color](#reference_common_disabled_fg_color)
//!         39. [disabled_bg_color](#reference_common_disabled_bg_color)
//!         40. [border_fg_color](#reference_common_border_fg_color)
//!         41. [border_bg_color](#reference_common_border_bg_color)
//!     2. [Layout](#reference_layout)
//!         1. [Properties](#reference_layout_properties)
//!             1. [mode](#reference_layout_properties_mode)
//!             2. [orientation](#reference_layout_properties_orientation)
//!             3. [active_screen](#reference_layout_properties_active_screen)
//!             4. [active_tab](#reference_layout_properties_active_tab)
//!             5. [tab_name](#reference_layout_properties_tab_name)
//!             6. [tab_header_fg_color](#reference_layout_properties_tab_header_fg_color)
//!             7. [tab_header_bg_color](#reference_layout_properties_tab_header_bg_color)
//!             8. [tab_header_border_fg_color](#reference_layout_properties_tab_header_border_fg_color)
//!             9. [tab_header_border_bg_color](#reference_layout_properties_tab_header_border_bg_color)
//!             10. [tab_header_active_fg_color](#reference_layout_properties_tab_header_active_fg_color)
//!             11. [tab_header_active_bg_color](#reference_layout_properties_tab_header_active_bg_color)
//!             12. [can_drag](#reference_layout_properties_can_drag)
//!             13. [fill](#reference_layout_properties_fill)
//!             14. [filler_symbol](#reference_layout_properties_filler_symbol)
//!             15. [filler_fg_color](#reference_layout_properties_filler_fg_color)
//!             16. [filler_bg_color](#reference_layout_properties_filler_bg_color)
//!             17. [scroll_x](#reference_layout_properties_scroll_x)
//!             18. [scroll_y](#reference_layout_properties_scroll_y)
//!             19. [scroll_start_x](#reference_layout_properties_scroll_start_x)
//!             20. [scroll_start_y](#reference_layout_properties_scroll_start_y)
//!             21. [rows](#reference_layout_properties_rows)
//!             22. [cols](#reference_layout_properties_cols)
//!             23. [col_default_width](#reference_layout_properties_col_default_width)
//!             24. [force_default_col_width](#reference_layout_properties_force_default_col_width)
//!             25. [row_default_height](#reference_layout_properties_row_default_height)
//!             26. [force_default_row_height](#reference_layout_properties_force_default_row_height)
//!         2. [Default callback implementations](#reference_layout_default_callbacks)
//!             1. [Scroll up](#reference_layout_default_callbacks_scrollup)
//!             2. [Scroll down](#reference_layout_default_callbacks_scrolldown)
//!         3. [Available callbacks](#reference_layout_available_callbacks)
//!     3. [Label](#reference_label)
//!         1. [Properties](#reference_label_properties)
//!             1. [text](#reference_label_properties_text)
//!             2. [from_file](#reference_label_properties_from_file)
//!         2. [Default callback implementations](#reference_label_default_callbacks)
//!         3. [Available callbacks](#reference_label_available_callbacks)
//!     4. [Button](#reference_button)
//!         1. [Properties](#reference_button_properties)
//!             1. [text](#reference_button_properties_text)
//!             2. [flash_fg_color](#reference_button_properties_flash_fg_color)
//!             3. [flash_bg_color](#reference_button_properties_flash_bg_color)
//!         2. [Default callback implementations](#reference_button_default_callbacks)
//!             1. [Left click](#reference_button_default_callbacks_leftclick)
//!             2. [Keyboard enter](#reference_button_default_callbacks_enter)
//!         3. [Available callbacks](#reference_button_available_callbacks)
//!     5. [Checkbox](#reference_checkbox)
//!         1. [Properties](#reference_checkbox_properties)
//!             1. [active](#reference_checkbox_properties_active)
//!             2. [active_symbol](#reference_checkbox_properties_active_symbol)
//!             3. [inactive_symbol](#reference_checkbox_properties_inactive_symbol)
//!         2. [Default callback implementations](#reference_checkbox_default_callbacks)
//!             1. [Left click](#reference_checkbox_default_callbacks_leftclick)
//!             2. [Keyboard enter](#reference_checkbox_default_callbacks_enter)
//!         3. [Available callbacks](#reference_checkbox_available_callbacks)
//!     6. [Radio button](#reference_radiobutton)
//!         1. [Properties](#reference_radiobutton_properties)
//!             1. [active](#reference_radiobutton_properties_active)
//!             2. [active_symbol](#reference_radiobutton_properties_active_symbol)
//!             3. [inactive_symbol](#reference_radiobutton_properties_inactive_symbol)
//!             1. [group](#reference_radiobutton_properties_group)
//!         2. [Default callback implementations](#reference_radiobutton_default_callbacks)
//!             1. [Left click](#reference_radiobutton_default_callbacks_leftclick)
//!             2. [Keyboard enter](#reference_radiobutton_default_callbacks_enter)
//!         3. [Available callbacks](#reference_radiobutton_available_callbacks)
//!     7. [Dropdown](#reference_dropdown)
//!         1. [Properties](#reference_dropdown_properties)
//!             1. [choice](#reference_dropdown_properties_choice)
//!             2. [options](#reference_dropdown_properties_options)
//!             3. [allow_none](#reference_dropdown_properties_allow_none)
//!         2. [Default callback implementations](#reference_dropdown_default_callbacks)
//!             1. [Left click](#reference_dropdown_default_callbacks_leftclick)
//!             2. [Keyboard enter](#reference_dropdown_default_callbacks_enter)
//!         3. [Available callbacks](#reference_dropdown_available_callbacks)
//!     8. [Slider](#reference_slider)
//!         1. [Properties](#reference_slider_properties)
//!             1. [value](#reference_slider_properties_value)
//!             2. [min](#reference_slider_properties_min)
//!             3. [max](#reference_slider_properties_max)
//!             4. [step](#reference_slider_properties_step)
//!         2. [Default callback implementations](#reference_slider_default_callbacks)
//!             1. [Left click](#reference_slider_default_callbacks_leftclick)
//!             2. [Drag](#reference_slider_default_callbacks_drag)
//!         3. [Available callbacks](#reference_slider_available_callbacks)
//!     9. [Text input](#reference_textinput)
//!         1. [Properties](#reference_textinput_properties)
//!             1. [text](#reference_textinput_properties_text)
//!             1. [max_length](#reference_textinput_properties_max_length)
//!             1. [cursor_color](#reference_textinput_properties_cursor_color)
//!             1. [text](#reference_textinput_properties_text)
//!         2. [Default callback implementations](#reference_textinput_default_callbacks)
//!             1. [Left click](#reference_textinput_default_callbacks_leftclick)
//!             2. [Select](#reference_textinput_default_callbacks_select)
//!         3. [Available callbacks](#reference_textinput_available_callbacks)
//!     10. [Progress bar](#reference_progressbar)
//!         1. [Properties](#reference_progressbar_properties)
//!             1. [value](#reference_progressbar_properties_value)
//!             1. [max](#reference_progressbar_max_value)
//!         2. [Default callback implementations](#reference_progressbar_default_callbacks)
//!         3. [Available callbacks](#reference_progressbar_available_callbacks)
//!     11. [Canvas](#reference_canvas)
//!         1. [Properties](#reference_canvas_properties)
//!             1. [from_file](#reference_canvas_properties_from_file)
//!             1. [Setting content from code](#reference_canvas_properties_from_code)
//!         2. [Default callback implementations](#reference_canvas_default_callbacks)
//!         3. [Available callbacks](#reference_canvas_available_callbacks)
//!     12. [Scheduler](#reference_scheduler)
//!         1. [schedule_once](#reference_scheduler_schedule_once)
//!         2. [cancel_task](#reference_scheduler_cancel_task)
//!         3. [schedule_recurring](#reference_scheduler_schedule_recurring)
//!         4. [cancel_recurring_task](#reference_scheduler_cancel_recurring_task)
//!         5. [schedule_threaded](#reference_scheduler_schedule_threaded)
//!         6. [open_modal](#reference_scheduler_open_modal)
//!         7. [dismiss_modal](#reference_scheduler_dismiss_modal)
//!         8. [overwrite_callback_config](#reference_scheduler_overwrite_callback_config)
//!         9. [update_callback_config](#reference_scheduler_update_callback_config)
//!         10. [bind_property_callback](#reference_scheduler_bind_property_callback)
//!         11. [bind_global_key](#reference_scheduler_bind_global_key)
//!         12. [remove_global_key](#reference_scheduler_remove_global_key)
//!         13. [clear_global_keys](#reference_scheduler_clear_global_keys)
//!         14. [create_widget](#reference_scheduler_create_widget)
//!         15. [remove_widget](#reference_scheduler_remove_widget)
//!         16. [new_usize_property](#reference_scheduler_new_usize_property)
//!         17. [new_f64_property](#reference_scheduler_new_f64_property)
//!         18. [new_string_property](#reference_scheduler_new_string_property)
//!         19. [new_bool_property](#reference_scheduler_new_bool_property)
//!         20. [new_color_property](#reference_scheduler_new_color_property)
//!         21. [new_layout_mode_property](#reference_scheduler_new_layout_mode_property)
//!         22. [new_layout_orientation_property](#reference_scheduler_new_layout_orientation_property)
//!         23. [new_horizontal_alignment_property](#reference_scheduler_new_horizontal_alignment_property)
//!         24. [new_vertical_alignment_property](#reference_scheduler_new_vertical_alignment_property)
//!         25. [new_horizontal_pos_hint_property](#reference_scheduler_new_horizontal_pos_hint_property)
//!         26. [new_vertical_pos_hint_property](#reference_scheduler_new_vertical_pos_hint_property)
//!         27. [new_size_hint_property](#reference_scheduler_new_size_hint_property)
//!         28. [get_property](#reference_scheduler_get_property)
//!         29. [get_property_mut](#reference_scheduler_get_property_mut)
//!         30. [update_widget](#reference_scheduler_update_widget)
//!         31. [force_redraw](#reference_scheduler_force_redraw)
//!         32. [set_selected_widget](#reference_scheduler_set_selected_widget)
//!         33. [deselect_widget](#reference_scheduler_deselect_widget)
//!         34. [exit](#reference_scheduler_exit)
//! 3. Examples
//!
//!
//! <a name="how_to_use"></a>
//! ## 1. Tutorial - How to use EzTerm
//!
//! This section will explain how to use this framework step-by-step. It functions as a general
//! tutorial, explaining all the features with little examples. Depending on your preferences,
//! you could read this first and then go to the [examples], or go to [examples] first and consult
//! this tutorial for more details. Using both this tutorial and the examples you should be able to
//! get started on your own project. Once you are, you can use [reference] for a full API reference.
//!
//! <a name="structure"></a>
//! ### 1.1 The structure on an EzTerm project
//!
//! First we will learn how to prepare an EzTerm project; it consists of three parts:
//! - UI config files (files with the '.ez' extension)
//! - UI Rust module(s)
//! - Your actual app (also Rust modules)
//!
//! <a name="ui_config_files"></a>
//! #### 1.1.1 UI config files
//!
//! UI config files have the '.ez' extension. They define what your UI will look like using layouts
//! and widgets. You can have as many .ez files as you like, so you can split up your UI along
//! multiple files. The language syntax will be explained below in [Ez Language](#ez_language).
//!
//! When you compile your project, the .ez files are automatically merged into your binary, so you
//! do not have to ship them alongside your executable. In order to merge the .ez files into your
//! binary, cargo needs to know where they are. You declare this in an environment variable called
//! "EZ_FOLDER" before you compile. Let's say you put the .ez files in your project root in a folder
//! called 'ui':
//! ```
//! /project_root
//!   /cargo.toml
//!   /src
//!   /ui
//!     /my_ui.ez
//! ```
//! Then you would declare the environment variable like this:
//! - On Linux:
//! ```
//! export EZ_FOLDER="/path/to/project/ui"
//! ```
//! - On Windows:
//! ```
//! $env:EZ_FOLDER="C:\path\to\project\ui"
//! ```
//! Note that the path should be a full path, not a relative one. Once we have one or more .ez files
//! and we set the environment variable, we can move on to the Rust code.

//! <a name="ui_rust_modules"></a>
//!  #### 1.1.2 UI Rust module(s)
//!
//! We have our .ez files describing what our UI should look like. Now we need a rust module
//! that will initialize the UI and start it. It makes sense for this to be main.rs, but it does
//! not have to be. Here is what the the module should contain at an absolute minimum:
//! ```
//! use ez_term::*;
//!
//! fn main() {
//!
//!     let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!     run(root_widget, state_tree, scheduler);
//! }
//! ```
//!
//! Initializing- and starting the UI are separate steps, because you might want to make some
//! changes to the UI from code before it starts (we'll cover this in de Scheduler chapter).
//! We will discuss callbacks later, but for now we will note that callbacks (either as closures
//! or full functions) could also be defined in this same module, or a separate mode as you like.
//!
//! To summarize, we now have a folder with our .ez files, a module to initialize- and start our UI,
//! and perhaps another module containing callbacks:
//! ```
//! /project_root
//!   /cargo.toml
//!   /src
//!     /main.rs
//!     /callbacks.rs
//!   /ui
//!     /my_ui.ez
//! ```
//!
//! <a name="your_app"></a>
//! #### 1.1.3 Your app
//!
//! Finally your project will obviously contain the Rust modules of your actual app (that you are
//! building the UI for). The UI will run in the main thread and will call (parts of) your App to
//! run in a background thread through callbacks (for example, when a button is pushed), or through
//! a scheduled task (e.g. run every 10 seconds without user input). We'll discuss how to run your
//! app, and how your app can manipulate the UI, later in the Scheduler section.
//! With your actual app included, the full project structure could look like this:
//! ```
//! /project_root
//!   /cargo.toml
//!   /src
//!     /main.rs
//!     /callbacks.rs
//!     /my_app.rs
//!   /ui
//!     /my_ui.ez
//! ```
//!
//! <a name="minimal_example"></a>
//! #### 1.1.4 Minimal example
//!
//! Now that we know the structure of an EzTerm project, we'll create the smallest working example
//! possible to get the structure into our fingers. After that we will move on to explain
//! how to create the actual UI in detail (for which we can use the project we are now creating).
//!
//! **Step 1: Create a new cargo project:**
//!
//! We'll create a new Rust project first using cargo. Feel free to choose another name.
//! ```
//! cargo-new ez_term_test
//! ```
//! In cargo.toml include the framework as a dependency:
//! ```
//! [dependencies]
//! ez_term = "0.1.0"
//! ```
//!
//! **Step 2: Define the UI:**
//!
//! Create a folder named 'ui' in the root of the project. Create a file named 'ui.ez' in the new
//! folder. These names are not mandatory, you can call the folder and file whatever you like. If
//! you choose the default names your project folder now looks like this:
//! ```
//! /ez_term_test
//!   /cargo.toml
//!   /src
//!     /main.rs
//!   /ui
//!     /ui.ez
//! ```
//!
//! In the 'ui.ez' file write or copy the below config to create a small 'hello world'
//! UI (don't worry if the syntax of the .ez file is still unfamiliar, we'll dive into it in the
//! next chapter):
//! ```
//! - Layout:
//!     mode: box
//!     orientation: horizontal
//!     - Label:
//!         text: Hello,
//!         border: true
//!     - Label:
//!         text: World!
//!         border: true
//! ```
//!
//! **Step 3: Create the UI rust module**
//!
//! We now have a UI definiton in the .ez file. We will need to initialize it in a rust module.
//! We will use the existing 'main.rs' to initialize and run the UI. Modify 'main.rs' to look like
//! this:
//! ```
//! use ez_term::*;
//!
//! fn main() {
//!
//!     let (root_widget, state_tree, mut scheduler) = load_ui();
//!     run(root_widget, state_tree, scheduler);
//! }
//! ```
//!
//! **Step 4: Compile and run the project**
//!
//! First we let cargo know where our .ez files can be found through an environment variable:
//! - On Linux:
//! ```
//! export EZ_FOLDER="/path/to/ez_term_test/ui"
//! ```
//! - On Windows:
//! ```
//! $env:EZ_FOLDER="C:\path\to\ez_term_test\ui"
//! ```
//! Make sure you use a full path. Cargo needs to know the location of our .ez files so it can merge
//! them into the binary. Now run the following cargo command in any OS terminal:
//! ```
//! cargo run
//! ```
//! You should you be able to see the 'hello world' UI! Press Escape to quit.
//! Now that you know how to create a basic UI, we'll dive into the specifics of the framework.
//!
//! <a name="ez_language"></a>
//! ### 1.2 Ez language
//!
//! <a name="ez_basics"></a>
//! #### 1.2.1 Basics
//!
//! With EzTerm, the UI is defined in the .ez files, using a YAML(ish) type syntax called EzLang.
//! Like everything in EzTerm, this language is designed to be simple to use. There are only two
//! things you can do in EzLang: define widgets (or widget templates) and set properties on those
//! widgets. Here is an example defining a label widget and setting its' text property:
//! ```
//! - Label:
//!     text: Hello world!
//! ```
//! As you can see in the above example, widget definitions start with a "-" dash. This makes it
//! easier to read the .ez files. After a widget definition we can define the properties of that
//! widget by indenting four spaces on the next line and using the "property: value" syntax. You
//! can find every possible property of each widget in [reference].
//!
//! Every widget must defined inside of a layout. A layout may also be defined inside of another
//! layout, or it can be the root layout. Every EzTerm project must contain exactly one root layout:
//! ```
//! - Layout:
//!     mode: box
//!     - Label:
//!         text: Hello World!
//! ```
//! This example contained only one Layout (the root). Here is an example of nested layouts
//! creating multiple screens (note we still have only one root layout):
//! ```
//! - Layout:
//!     mode: screen
//!     - Layout:
//!         id: screen_1
//!         mode: box
//!         - Label:
//!             text: Hello screen one!
//!     - Layout:
//!         id: screen_2
//!         mode: box
//!         - Label:
//!             text: Hello screen two!
//! ```
//! In the above example we gave the screen layouts an ID through the 'id' property; the ID is
//! optional but becomes necessary when you want to refer to your layout/widget (either from code
//! or from EzLang). It also makes the config file more readable. We will learn how to use the ID
//! in a later section. Don't worry if the properties look unfamiliar, we'll get into them later;
//! for now we are
//! just discussing the basics of the syntax.
//!
//! <a name="ez_language_templates"></a>
//! #### 1.2.2 Templates
//!
//! We'll now take a look at templates, because we'll often use them in tutorial examples. Don't
//! worry about the unfamiliar properties we'll use, they will be explained in sections to come,
//! for now just get an impression of what EzLang looks like.
//!
//! When you start writing your own .ez files, you may notice yourself writing the same types of
//! widgets over and over again. To make your .ez files more readable and more ergonomic, you can
//! use templates in these situations. Let's say for example that your interface has many
//! auto-scaling labels of a certain color:
//! ```
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - Label:
//!         text: Label 1
//!         fg_color: yellow
//!         bg_color: blue
//!         auto_scale: true, true
//!     - Label:
//!         text: Label 2
//!         fg_color: yellow
//!         bg_color: blue
//!         auto_scale: true, true
//! ```
//! This can get quite verbose with two labels, let alone more. Instead we can define a template.
//! A template is defined in the following format:
//! ```
//! - <TemplateName@BaseWidget>:
//! ```
//! The name of the template can be anything you like. The BaseWidget is the widget the template
//! inherits from. This can be a basic widget type (Label, Layout, Checkbox, etc.) or another
//! template. It's possible for templates to inherit from other templates, but in the end it must
//! always inherit from a basic widget. Here is the template for our label:
//! ```
//! - <MyCustomLabel@Label>:
//!     fg_color: yellow
//!     bg_color: blue
//!     auto_scale: true, true
//!
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - MyCustomLabel:
//!         text: Label 1
//!     - MyCustomLabel:
//!         text: Label 2
//! ```
//! This looks much cleaner! We no longer have to define common properties over and over again. They
//! will instead be inherited from the template. It is possible to overwrite properties of a
//! template; properties defined when using a template always overwrite the properties of the
//! template definition:
//! ```
//! - <MyCustomLabel@Label>:
//!     fg_color: yellow
//!     bg_color: blue
//!     auto_scale: true, true
//!
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - MyCustomLabel:
//!         text: Label 1
//!     - MyCustomLabel:
//!         text: Label 2
//!         fg_color: red
//! ```
//! The text of the second label will be red, because the fg_color defined when using a template
//! overwrites the fg_color of the template definition.
//!
//! Templates are not just useful for reusing widgets. They can also be used for widgets used only
//! once, usually to make your .ez file easier to read. This is especially true for layouts. There
//! can be only one root layout, but you can define as many layouts templates as you like on the
//! root level. Consider our earlier example creating multiple screens:
//! ```
//! - Layout:
//!     mode: screen
//!     - Layout:
//!         id: screen_1
//!         mode: box
//!         - Label:
//!             text: Hello screen one!
//!     - Layout:
//!         id: screen_2
//!         mode: box
//!         - Label:
//!             text: Hello screen two!
//! ```
//! This is still readable, but if we keep adding more screens (and more widgets to the screens) it
//! will become hard to read. Here is an alternative using templates:
//! ```
//! - <ScreenOne@Layout>:
//!     mode: box
//!     - Label:
//!         text: Hello screen one!
//!
//! - <ScreenTwo@Layout>:
//!     mode: box
//!     - Label:
//!         text: Hello screen two!
//!
//! - Layout:
//!     mode: screen
//!     - ScreenOne:
//!         id: screen_1
//!     - ScreenTwo:
//!         id: screen_2
//! ```
//! By using layout templates we can segment the definitions into meaningful blocks and keep the
//! indentation levels under control. You can nest templates in templates, so even if you have a
//! very complex UI, your config files can remain flat and readable:
//! ```
//! - <CustomButton@Button>:
//!     fg_color: yellow
//!     bg_color: blue
//!     auto_scale: true, true
//!
//! - <Menu@Layout>:
//!     mode: box
//!     orientation: vertical
//!     - CustomButton:
//!         text: Option 1
//!     - CustomButton:
//!         text: Option 2
//!
//! - <ScreenOne@Layout>:
//!     mode: box
//!     - Menu:
//!
//! - Layout:
//!     mode: screen
//!     - ScreenOne:
//!         id: screen_1
//! ```
//! <a name="ez_language_layout_modes"></a>
//! #### 1.2.3 Layout modes
//!
//! You may have noted the "mode" property of the Layouts; this is one of the most important
//! properties to learn about, because it does most of the heavy lifting in the framework. One of
//! the advantages of EzTerm is that you don't have to hardcode your widget positions and sizes and
//! you don't have to handle UI scaling. Instead, smart layouts do the work for you unless you
//! specify that you want manual positions. To give you control over the way in which objects are
//! placed on the screen, you can choose between layout modes and layout orientations. Here is a
//! short overview of the layout modes:
//!
//!
//! <a name="ez_language_box"></a>
//! ##### 1.2.3.1 Box mode
//!
//! In Box mode objects are placed from left to right (orientation: horizontal) or top to bottom
//! (orientation: vertical). This is the simplest layout mode and is useful in many scenarios. An
//! example of a vertical box mode layout could be a menu (a set of buttons placed under one
//! another):
//! ```
//! - <CustomButton@Button>:
//!     fg_color: yellow
//!     bg_color: blue
//!     auto_scale: true, true
//!
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - CustomButton:
//!         text: Option 1
//!     - CustomButton:
//!         text: Option 2
//!     - CustomButton:
//!         text: Option 3
//! ```
//! It can often be useful to nest Box layouts in other Box layouts to divide the screen into
//! rectangles. Let's say for example we want two menus, one on the left side of the screen and one
//! on the right. We could divide the screen horizontally with a Box layout, and then add two
//! vertical Box layouts (one menu for each side of the screen):
//! ```
//! - <CustomButton@Button>:
//!     fg_color: yellow
//!     bg_color: blue
//!     auto_scale: true, true
//!
//! - Layout:
//!     mode: box
//!     orientation: horizontal
//!     - Layout:
//!         mode: box
//!         orientation: vertical
//!         border: true
//!         - CumstomButton:
//!             text: Left option 1
//!         - CumstomButton:
//!             text: Left option 2
//!         - CumstomButton:
//!             text: Left option 3
//!     - Layout:
//!         mode: box
//!         orientation: vertical
//!         border: true
//!         - CumstomButton:
//!             text: Right option 1
//!         - CumstomButton:
//!             text: Right option 2
//!         - CumstomButton:
//!             text: Right option 3
//! ```
//!
//! <a name="ez_language_stack"></a>
//! ##### 1.2.3.2 Stack mode
//!
//! Stack mode stacks widgets inside of the layout according to the primary and secondary
//! orientation. This can be useful if you want a bunch of widgets to fit into a layout
//! efficiently (especially useful if the widgets are many different sizes). Unlike box mode
//! which can either be horizontal or vertical, stack mode has 8 possible orientations.
//! The default orientation is "top-bottom, left-right". This means that objects will be placed
//! from top to bottom until there is no more vertical space to place the next widget. The next
//! widget will then be placed at the top again, to the right of the first widget.
//! In short: widgets are placed in the primary direction until space runs out; then they
//! will be shifted towards the secondary direction. The possible orientations are:
//! - Top-bottom, left-right ('tb-lr')
//! - Top-bottom, right-left ('tb-rl')
//! - Bottom-top, left-right ('bt-lr')
//! - Bottom-top, right-left ('bt-rl')
//! - Left-right, top-bottom ('lr-tb')
//! - Left-right, bottom-top ('lr-bt')
//! - Right-left, top-bottom ('rl-tb')
//! - Right-left, bottom-top ('rl-bt')
//!
//! Here is an example: we have labels of different sizes and we want to place them efficiently
//! in a layout, trying not to waste space. Instead of coming up with a complex solution, we just
//! let the stack layout do the work for us. We want the widgets stacked left-to-right,
//! top-to-bottom:
//! ```
//! - <CustomLabel@Label>:
//!     auto_scale: true, true
//!     border: true
//!
//! - Layout:
//!     mode: stack
//!     orientation: lr-tb
//!     border: true
//!     size_hint_x: none
//!     width: 31
//!     - CustomLabel:
//!         text: Hi,
//!     - CustomLabel:
//!         text: The size
//!     - CustomLabel:
//!         text: Of these labels
//!     - CustomLabel:
//!         text: Keeps increasing in length!
//! ```
//! The widgets will be stacked automatically. If you resize the window, the row and column each
//! widget occupies may change, as the stack layout rearranges the widgets, but the orientation
//! will always be respected.
//!
//!
//!As you can see, stack layouts are an easy way to make sure your
//! widgets will remain visible even when the window resizes (as long as there ís enough is enough
//! space in general of course).
//!
//! <a name="ez_language_table"></a>
//! ##### 1.2.3.3 Table mode
//!
//! Table mode divides widgets into rows and columns. You must specify how many rows or columns you
//! want, or both. If you specify the amount of columns, the table will grow the amount of rows to
//! fit all the widgets. If you specify the amount of rows, then the amount of columns will grow.
//! If you specify both, the table will be fixed in size.
//! Like the stack layout, the table layout has 8 possible orientations which dictate in what order
//! the cells of the table are filled. The default orientation is top-bottom, left right ('tb-lr'),
//! so if there are 3 columns and 3 rows (and 9 widgets), they would be filled like this:
//! ```
//! 1 4 7
//! 2 5 8
//! 3 6 9
//! ```
//! If you would set the orientation to left-right, top-bottom ('lr-tb') it would look like this:
//! ```
//! 1 2 3
//! 4 5 6
//! 7 8 9
//! ```
//! Each row in the table is sized to the highest widget in that row. Each column is sized to the
//! widest widget in that column. This behavior can be overwritten using the following properties:
//! - row_default_height: 50 (set default height of rows to 50, they can still grow)
//! - force_default_row_height: true (force all rows to always be row_default_height)
//! - col_default_width: 50 (set default width of columns to 50, they can still grow)
//! - force_default_col_width: true (force all columns to always be col_default_width)
//!
//! Here is an example of creating a Sudoku cell using a table layout. We will recreate this cell:
//! ```
//! - - -
//! 3 5 -
//! 9 - 4
//! ```
//!
//! ```
//! - <SudokuLabel@Label>:
//!     auto_scale: true, true
//!     text: -
//!
//! - Layout:
//!     mode: table
//!     border: true
//!     orientation: lr-tb
//!     col_default_width: 3
//!     force_default_col_width: true
//!     row_default_height: 3
//!     force_default_row_height: true
//!     - SudokuLabel:
//!     - SudokuLabel:
//!     - SudokuLabel:
//!     - SudokuLabel:
//!         text: 3
//!     - SudokuLabel:
//!         text: 5
//!     - SudokuLabel:
//!     - SudokuLabel:
//!         text: 9
//!     - SudokuLabel:
//!     - SudokuLabel:
//!         text: 4
//! ```
//!
//! <a name="ez_language_float"></a>
//! ##### 1.2.3.4 Float mode
//!
//! In float mode, widgets can be placed freely anywhere in the layout. This mode does not have any
//! orientations. Placing widgets can be done using hardcoded positions, or using position hints.
//! Positions hints are suggestions for where you want to place your widgets; the framework will
//! then do the work for you. Let's say we want to put a label in each corner of the screen and one
//! label in the middle of the screen, like this:
//! ```
//! 1   2
//!   3
//! 4   5
//! ```
//! We could most easily accomplish this using a float layout with position hints:
//! ```
//! - <FloatLabel@Label>:
//!     auto_scale: true, true
//!
//! - Layout:
//!     mode: float
//!     - FloatLabel:
//!         text: 1
//!         pos_hint: left, top
//!     - FloatLabel:
//!         text: 2
//!         pos_hint: right, top
//!     - FloatLabel:
//!         text: 3
//!         pos_hint: center, middle
//!     - FloatLabel:
//!         text: 4
//!         pos_hint: left, bottom
//!     - FloatLabel:
//!         text: 5
//!         pos_hint: right, bottom
//! ```
//! You can also specify an offset with positions hints, for example 'right: 0.2' means 20% of
//! the right side of the parent. So if the parent is 10 wide, 'right: 0.2' resolves to the x
//! position '2'. If the parent is 10 wide and you use 'center: 0.2' (20% of center of the parent,
//! which is 5) it will resolve to the x position '1':
//! ```
//! - Layout:
//!     mode: float
//!     - Label:
//!         text: Hello world
//!         pos_hint: right: 0.4, bottom: 0.6
//! ```
//!
//! Float mode also allows you to hardcode positions:
//! ```
//! - Layout:
//!     mode: float
//!     Label:
//!         text: Hello world
//!         auto_scale: true, true
//!         pos: 10, 20
//! ```
//! There are use cases for hard coded positions, but keep in mind these positions won't change
//! if the terminal is resized. In most cases position hints are the better choice as they will
//! scale with the terminal.
//!
//! <a name="ez_language_tab"></a>
//! ##### 1.2.3.5 Tab mode
//!
//! Tab mode creates tabs for you based on child layouts. This means that in tab mode, you can only
//! add other layouts, not individual widgets. A tab header button will automatically be created for
//! each child layout, using the tab_name property of the child layout as a tab name. If the tab_name
//! property is not set, the child ID is used. Here is an example with three tabs:
//! ```
//! - Layout:
//!     mode: tab
//!     active_tab: tab_one
//!     tab_header_fg_color: white
//!     tab_header_bg_color: black
//!     tab_header_border_fg_color: white
//!     tab_header_border_bg_color: black
//!     tab_header_active_fg_color: yellow
//!     tab_header_active_bg_color: black
//!     border: true
//!     - Layout:
//!         id: tab_one
//!         tab_name: Tab one
//!         mode: box
//!         border: true
//!         - Label:
//!             text: Hello tab one!
//!     - Layout:
//!         id: tab_two
//!         tab_name: Tab two
//!         mode: box
//!         border: true
//!         - Label:
//!             text: Hello tab two!
//!     - Layout:
//!         id: tab_three
//!         tab_name: Tab three
//!         mode: box
//!         border: true
//!         - Label:
//!             text: Hello tab three!
//! ```
//! Note that the active_tab property is optional, by default the first tab is active. Users can
//! then switch tabs using the tab header buttons that are created automatically. This example also
//! showed all tab related color properties, but because all default foreground colors are white
//! and all default background colors are black, most could have been left out in this example.
//!
//! Keep in mind that tab header buttons are three pixels high, so the effective height of your
//! layout will be three pixels smaller. We also used the 'tab_header_fg_color' and
//! 'tab_header_bg_color' properties to change the colors of the tab headers, and the
//! 'tab_header_border_fg_color' and 'tab_header_border_bg_color' to color the inactive tab headers,
//! and 'tab_header_active_fg_color' and 'tab_header_active_bg_color' to color the active tab.
//! borders.
//!
//! It is possible to change the active tab from code. Although callbacks will be discussed in a
//! later chapter, we'll look at an example just for reference:
//!```
//! use ez_term::*;
//! fn change_tab_callback(context: Context) {
//!     let state = context.state_tree.get_mut("my_tab_layout").as_layout_mut();
//!     state.set_active_tab("tab_two");
//!     state.update(context.scheduler);
//! }
//! ```
//!
//! <a name="ez_language_screen"></a>
//! ##### 1.2.3.6 Screen mode
//!
//! Screen mode creates screens for you based on child layouts. This means that in screen mode,
//! you can only add other layouts, not individual widgets. Furthermore, only the root layout is
//! allowed to be in screen mode. Only the content of the active screen will be shown at any one
//! time. In effect, screen mode allows you to have multiple root widgets, where each screen is a
//! root. Of course, in reality we still have only one root. Here is an example of multiple screens:
//! ```
//! - Layout:
//!     id: my_screen_layout
//!     mode: screen
//!     active_screen: screen_1
//!     - Layout:
//!         id: screen_1
//!         mode: box
//!         - Label:
//!             text: Hello screen one!
//!     - Layout:
//!         id: screen_2
//!         mode: box
//!         - Label:
//!             text: Hello screen two!
//! ```
//! Note that the active_screen property is optional, by default the first screen is active.
//! Unlike tabs, there is no default way for users to switch between screens. You will have to
//! write callbacks for this. An obvious example would be switching screen after clicking a button
//! (for example in a main menu). Callbacks will be discussed later in the Scheduler chapter, but
//! for reference we'll look at an example of switching screens through a callback:
//!
//! **EzLang**
//! ```
//! - Layout:
//!     mode: screen
//!     - Layout:
//!         id: screen_1
//!         mode: box
//!         orientation: vertical
//!         - Label:
//!             text: Hello screen one!
//!         - Button:
//!             id: to_screen_2_btn
//!             text: Go to screen 2
//!     - Layout:
//!         id: screen_2
//!         mode: box
//!         orientation: vertical
//!         - Label:
//!             text: Hello screen two!
//!         - Button:
//!             id: to_screen_1_btn
//!             text: Go to screen 1
//! ```
//! **Rust code**
//! ```
//! use ez_term::*;
//!
//! fn main() {
//!
//! // We load the UI from the .ez files
//! let (root_layout, mut state_tree, mut scheduler) = load_ui();
//!
//! // We update the callbacks for the buttons, using the functions defined below
//! scheduler.update_callback_config("to_screen_2_btn",
//!                                  CallbackConfig::from_on_press(Box::new(to_screen_two_callback)));
//! scheduler.update_callback_config("to_screen_1_btn",
//!                                  CallbackConfig::from_on_press(Box::new(to_screen_one_callback)));
//! // We run the UI
//! run(root_layout, state_tree, scheduler);
//! }
//!
//! // We define the callback functions. We could also use closures if we wanted to.
//! fn to_screen_one_callback(context: Context) -> bool {
//!     let state = context.state_tree.get_mut("root").as_layout_mut();
//!     state.set_active_screen("screen_1");
//!     state.update(context.scheduler);
//!     true
//! }
//!
//! fn to_screen_two_callback(context: Context) -> bool {
//!     let state = context.state_tree.get_mut("root").as_layout_mut();
//!     state.set_active_screen("screen_2");
//!     state.update(context.scheduler);
//!     true
//! }
//! ```
//! This example used a button callback, but it could of course be any kind of callback or
//! scheduled task.
//!
//! <a name="ez_language_scrolling"></a>
//! ##### 1.2.3.7 Scrolling
//!
//! Scrolling is *not* a dedicated layout mode. Instead, it is a property that can be enabled for
//! Box, Stack, Table and Float layouts. If vertical scrolling is enabled and the content height
//! is bigger than the layout height, a vertical scrolling bar will be created automatically. The
//! same is true if horizontal scrolling is enabled and the width of the content is larger than the
//! width of the layout. Ther user can scroll by hovering the layout and using the scroll wheel,
//! clicking the scrollbar, dragging the scrollbar, or selecting the layout and pressing left/right
//! arrow or page-up/page-down (for horizontal- and vertical scrolling respectively).
//!
//! Scrolling can be enabled with the following properties:
//! ```
//! - Layout:
//!     scroll_x: true
//! ```
//! ```
//! - Layout:
//!     scroll_y: true
//! ```
//!
//! Keep in mind that when you enable scrolling for an axis, that axis is then essentially infinite
//! size (because the user can just keep scrolling to reveal more content). This means that you
//! cannot use relative sizes for a scrolled axis. For example, if you enable vertical scrolling,
//! you cannot use 'size_hint_y" for widgets in that layout, because "size_hint_y" means you define
//! the height of a widget relative to the height of the layout, and when scrolling the height is
//! infinite.
//!
//! Here is an example of scrolling a large amount of text in a label:
//! ```
//! - Layout:
//!     mode: box
//!     scroll_y: true
//!     - Label:
//!         auto_scale_height: true
//!         from_file: ./examples/lorem_ipsum.txt
//! ```
//! Note we enable auto_scale_height for the label. We want the label to be as big as it needs to be
//! to display all the text. No matter how large it will grow, we can view the content by scrolling.
//!
//! <a name="ez_language_views"></a>
//! ##### 1.2.3.8 Views
//!
//! Layout view is also not a dedicated mode, but worth mentioning. It is a property that can be
//! used with box, float, stack and table layouts. It allows you to set a view size, and only that amount
//! of child widgets will be displayed at a time. So if you have for example a table layout that
//! has to display a database with hundreds of records, you could set the view size to 25 to limit
//! the amount of widgets displayed at one time. The view_size property determines how many children
//! are displayed, and the view_page property determines where we are in the view. The first page is
//! '1' (0 is not allowed). If the view_page value exceeds the max number of pages it will not panic,
//! but automatically be set to the max value (so you don't have to worry about it).
//!
//! Let's look at an example. We'll create a parent layout with views enabled in an .ez file.
//! Then we'll generate a lot of widgets in that layout programmatically. We'll also give the parent
//! layout two buttons so the user can navigate through the views.
//!
//! First the .ez file:
//! ```
//! Layout:
//!     mode: box
//!     orientation: vertical
//!     Layout:
//!         id: view_layout
//!         mode: box
//!         orientation: vertical
//!         view_size: 10
//!         view_page: 1
//!     Layout:
//!         mode: box
//!         orientation: horizontal
//!         auto_scale: true, true
//!         Button:
//!             id: back_button
//!             text: <
//!             auto_scale: true, true
//!         Button:
//!             id: forward_button
//!             text: >
//!             auto_scale: true, true
//! ```
//! We now have a view layout and navigation buttons. Let's look at the code:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let navigate_back_callback = |context: Context| {
//!     let state = context.state_tree.get_mut("view_layout").as_layout_mut();
//!     let current_page = state.get_view_page();
//!     if current_page > 1 {
//!         state.set_view_page( current_page - 1);
//!         state.update(context.scheduler);
//!     }
//!     true
//! };
//! let callback_config = CallbackConfig::from_on_press(Box::new(navigate_back_callback));
//! scheduler.update_callback_config("back_button", callback_config);
//!
//! let navigate_forward_callback = |context: Context| {
//!     let state = context.state_tree.get_mut("view_layout").as_layout_mut();
//!     let current_page = state.get_view_page();
//!     state.set_view_page( current_page + 1);
//!     state.update(context.scheduler);
//!     true
//! };
//! let callback_config = CallbackConfig::from_on_press(Box::new(navigate_forward_callback));
//! scheduler.update_callback_config("forward_button", callback_config);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="widget_overview"></a>
//! ### 2.4 Widget overview
//!
//! Widgets are the actual content of the UI and are always placed inside Layouts. It is not
//! possible to place widgets in other widgets. Widgets are defined in the same way as layouts:
//! a definition followed by 'property: value' lines:
//! ```
//! - Label:
//!     text: Hello world
//! ```
//! There are properties common to all widgets (such as size, position and color) and properties
//! specific to one or several widgets. For a detailed explanation of each widget,
//! check the table of contents for the specific widget doc entry; the following is only a short
//! overview of all available widgets:
//!
//! **Label:**
//! A textbox. Can be used to display (colored) text. If the label has a height higher than one,
//! text will be automatically wrapped to respect word boundaries. The text of a label comes either
//! from its 'text' property, or from a text file using the 'from_file' property.
//! Formatted text and justify
//! options are on the roadmap with priority.
//! ```
//! - Label:
//! ```
//!
//! **Text input:**
//! The text input is essentially an interactive Label. The user can select the input through mouse or
//! keyboard, and then type content into it. Selecting the widget will spawn a cursor that the user
//! can control with the left/right buttons. Backspace and delete will remove content as expected.
//! If the text of the input grows larger than the widget, the view will automatically move with
//! the cursor.
//! ```
//! - TextInput:
//! ```
//!
//! **Button:**
//! Clickable button; displays a small animation when clicked. Bind an on_press callback to a button
//! to make it functional. This will be explained in the callback section of the scheduler (see
//! table of contents).
//! ```
//! - Button:
//! ```
//!
//! **Checkbox:**
//! A clickable switch; has two states: on or off. Bind an on_value_change callback to a checkbox
//! to make it functional. This will be explained in the callback section of the scheduler (see
//! table of contents).
//! ```
//! - Checkbox:
//! ```
//!
//! **Radio button:**
//! A radio button is also a clickable switch that can be either on or off. The difference with the
//! checkbox is that the radio button is meant to be part of a group (which can be set using the
//! 'group' EzLang property). Only one radio button can be active in a group, so when the user
//! clicks one, the others will be off. To make the radio buttons functional, bind on_value_change
//! callbacks to each button in the group. Only the radio button that became active will receive
//! an on_value_change event.
//! ```
//! - RadioButton:
//! ```
//!
//! **Dropdown:**
//! A dropdown is a list of items (including an optional empty choice). Initially, only the active
//! choice is displayed. When clicked, a dropdown list becomes visible, from which the user can
//! choose a new option. Use the 'options' EzLang property to set the possible choices. Use the
//! 'allow_none' EzLang property to enable or disable an empty choice. Use the 'choice' EzLang
//! property to set the initial choice; if you don't do this, the empty choice will be active if
//! 'allow_none' is true; if not, then the first option will be active by default.
//! ```
//! - Dropdown:
//! ```
//!
//! **Slider:**
//! The slider allows a user to choose a numerical value by dragging the slider to the left or
//! the right (using keyboard or mouse). A slider has a value, a minimum value, a maximum value,
//! and a step value. The step value determines the minimum amount by which the value can be
//! adjusted. A slider with min 0, max 20, and step 5, has 5 possible values (0, 5, 10
//! , 15, 20).
//! ```
//! - Slider:
//! ```
//!
//! **Canvas**:
//! The canvas is a widget that you either 'paint' yourself, or that gets its' content from a text
//! file. You can set the content of a canvas programmatically through its' "set_content" function.
//! You can load the content from a text file using the EzLang property 'from_file'.
//! ```
//! - Canvas:
//! ```
//!
//!
//! <a name="sizing"></a>
//! #### 1.2.5 Sizing
//!
//! Now we'll learn about the different ways to size widgets and layouts.
//!
//! There are three ways to size widgets and layouts:
//! - Size relative to parent layout;
//! - Auto scale to content;
//! - Absolute size.
//!
//! <a name="sizing_relative"></a>
//! ##### 1.2.5.1 Relative sizing: size hints
//!
//! Size hints can be used to size a widget relative to its parent layout. This is the default way
//! widgets are sized across the framework; this is important to keep in mind! Size hints are
//! controlled through the EzLang 'size_hint_x' and 'size_hint_y' properties. These can be set to a
//! value between 0 and 1. A value of 1 means the size of the parent; 0.5 half the size of the
//! parent, etc. By default size hints are set to 1, so widgets are always as large as their parent
//! by default. If a layout has multiple widgets, and they **all** have default size hints, their
//! size hints will be se to "1 / number_of_widgets". So four widgets with default size hints will
//! receive 0.25 size hints. This gives all layout children equal size by default.
//!
//! As an example of using size hints, lets say we have two labels; we want one label to be 75% of
//! the layout height and the other one 25%. They can both be 100% width:
//! ```
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - Label:
//!         text: Big label
//!         size_hint_y: 0.75
//!         size_hint_x: 1
//!     - Label:
//!         text: Small label
//!         size_hint_y: 0.25
//!         size_hint_x: 1
//! ```
//! The widgets will always respect their size hints, even when the window resizes. We can make the
//! above example shorter by removing the 'size_hint_x' properties, because size hints are already set to
//! '1' by default. As a convenience, there is also a 'size_hint' property which allows you to
//! specify both size hints on one line in the format 'size_hint: x, y':
//! ```
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - Label:
//!         text: Big label
//!         size_hint: 1, 0.75
//!     - Label:
//!         text: Small label
//!         size_hint: 1, 0.25
//! ```
//!
//! <a name="sizing_scaling"></a>
//! ##### 1.2.5.2 Auto scaling
//!
//! All widgets support auto-scaling; when enabled, they will automatically size themselves to their
//! contents. Auto-scaling is turned off by default, and overwrites size_hint if enabled.
//! A widget with auto-scaling enabled for one or both axes (auto_scale_height and/or
//! auto_scale_width) will initially be given infinite size on those axes to create their content.
//! Once they have created their content, their size is then set to the size of their content.
//! For example, a label with text "Hello world" and "auto_scale_width" enabled will have infinite
//! width to create its' content. After creating the label, the width of the content will be 11
//! pixels; the size of the label will then be set to 11. Let's see how auto_scaling works in
//! practice; first we look at a label without auto_scaling:
//! ```
//! - Layout:
//!     mode: box
//!     - Label:
//!         text: Hello world
//!         border: true
//! ```
//! Since the default for widgets is "size_hint: 1, 1", this label now takes up the entire screen.
//! Let's enable auto_scale_width:
//! ```
//! - Layout:
//!     mode: box
//!     - Label:
//!         text: Hello world
//!         border: true
//!         auto_scale_width: true
//! ```
//! The label still takes up the entire height of the screen, but the width is now cropped to the
//! content of the label. We could enable scaling on both axes, using the convenience "auto_scale"
//! property that allows us to set both at the same time in the format "auto_scale: width, height":
//! ```
//! - Layout:
//!     mode: box
//!     - Label:
//!         text: Hello world
//!         border: true
//!         auto_scale: true, true
//! ```
//! The label is now cropped entirely to its' content. Another good use of auto_scale is to allow
//! a widget to grow on one axis. Let's say for example we have a Label with a large amount of text;
//! we could set "auto_scale_height: true" for it. Since the default sizing is "size_hint: 1, 1",
//! and since auto_scale overwrites size_hint, this means the Label will have the width of the
//! parent layout, but its height will now be auto-scaled to its content. In other words: the
//! label is horizontally fixed in size, but can grow vertically:
//! ```
//! - Layout:
//!     mode: box
//!     - Label:
//!         auto_scale_height: true
//!         from_file: ./ui/lorem_ipsum.txt
//!         border: true
//! ```
//! The Label is now growing vertically.
//!
//! <a name="sizing_absolute"></a>
//! ##### 1.2.5.3 Absolute size - Manual:
//!
//! It is possible to set an absolute size for widgets manually. Keep in mind that size_hint will
//! overwrite any manual sizes, so it has to be turned off in those cases. Let's say you want a
//! button to always have 10 width and 3 height:
//! ```
//! - Layout:
//!     mode: box
//!     - Button:
//!         text: Click me
//!         size_hint: none, none
//!         size: 10, 3
//! ```
//! The button will now always be fixed in size. It is of course possible to use absolute size
//! for only one axis, while the other axis uses size_hint or auto_scale:
//! ```
//! - Layout:
//!     mode: box
//!     - Button:
//!         text: Click me
//!         size_hint_y: none
//!         height: 3
//! ```
//! The button will now be fixed to height 3, but its width will scale to the width of the parent
//! layout (because the default is "size_hint: 1, 1").
//!
//! <a name="sizing_maths"></a>
//! ##### 1.2.5.4 Absolute size - Maths:
//!
//! When setting size manually it is possible to refer to other widgets' properties and even
//! perform maths on them (e.g. "widget1.height = widget2.height - widget3.height"). First, here is
//! an example of simply binding the height of one widget to another:
//!
//! ```
//! - <MyLabel@Label>:
//!     text: My text
//!     size_hint: none, none
//!     border: true
//!
//! - Layout:
//!     mode: box
//!     - MyLabel:
//!         id: label_1
//!         size: 10, 3
//!     - MyLabel:
//!         width: 10
//!         height: parent.label_1.height
//!
//! ```
//!
//! Now let's use maths to make the label the height of the parent layout minus the size of the
//! other label:
//! ```
//! - <MyLabel@Label>:
//!     text: My text
//!     size_hint: none, none
//!     border: true
//!
//! - Layout:
//!     mode: box
//!     - MyLabel:
//!         id: label_1
//!         size: 10, 3
//!     - MyLabel:
//!         width: 10
//!         height: parent.height - parent.label_1.height
//! ```
//! Now the label height will always be the remaining height in the layout. It is possible to use
//! any arithmetic operator, as well as regular numbers:
//! ```
//! - <MyLabel@Label>:
//!     text: My text
//!     size_hint: none, none
//!     border: true
//!
//! - Layout:
//!     mode: box
//!     - MyLabel:
//!         id: label_1
//!         size: 10, 3
//!     - MyLabel:
//!         width: 10
//!         height: ((parent.height - parent.label_1.height) / 2) - 10
//! ```
//!
//! <a name="positioning"></a>
//! #### 1.2.6 Positioning:
//!
//! There a multiple ways to control the positioning of widgets; which ways are available depends
//! on the mode a layout is in. There are four ways to control positioning:
//! - Automatic positioning with layout modes
//! - Relative positioning with position hints
//! - Absolute positioning with manual positions
//! - Adjust position through padding and aligning
//!
//! <a name="positioning_automatic"></a>
//! ##### 1.2.6.1 Automatic positioning: layout modes:
//!
//! Most layout modes do not support manual positioning or relative positioning. This is because
//! the point of these layouts is that they do the work for you. Only the float layout, which exists
//! specifically to give you manual control over position, supports position hints or the manual
//! position property. The other widgets, such as box mode, stack mode and table mode, will handle
//! the positioning for you (see their docs for more info). It is however possible to adjust the
//! position of widgets in these modes; see the entry on padding and aligning below for more on that.
//!
//! <a name="positioning_relative"></a>
//! ##### 1.2.6.2 Relative positioning: position hints
//!
//! Position hints can only be used for widgets that are in a layout in float mode. With position
//! hints you give the relative position you want the widget to be in, and it will be handled for
//! you. There are horizontal position hints (pos_hint_x) and vertical position hints (pos_hint_y).
//!
//! The available settings for pos_hint_x are:
//! - Left
//! - Center
//! - Right
//!
//! The available settings for pos_hint_y are:
//! - Top
//! - Middle
//! - Bottom
//!
//! If you want a widget to be in the top left of the screen, you would give a widget:
//! "pos_hint_x: left" and "pos_hint_y: top". For convenience you could use the "pos_hint" property
//! to set both at the same time in the format: "pos_hint: x, y":
//! ```
//! - Layout:
//!     mode: float
//!     - Label:
//!         text: Hello world
//!         auto_scale: true, true
//!         pos_hint: left, top
//! ```
//! Note that in the example we auto_scale the label. If we don't, it would take up the entire
//! screen and positioning would be pointless.
//!
//! It is possible to be more specific with position hints. Instead of just specifying "bottom" for
//! example, we could use "bottom: 0.9". This would position the widget 90% towards the bottom of
//! the layout. The number goes from 0 to 1. This method can be used for 'center', 'right', 'middle'
//! and 'bottom'. It is not useful with 'left' and 'top', because they represent position (0, 0) and
//! cannot be scaled. Let's say we want a label to be 90% towards the right of the layout and 90%
//! towards the bottom:
//! to set both at the same time in the format: "pos_hint: x, y":
//! ```
//! - Layout:
//!     mode: float
//!     - Label:
//!         text: Hello world
//!         auto_scale: true, true
//!         pos_hint: right: 0.9, bottom: 0.9
//! ```
//!
//! <a name="positioning_absolute"></a>
//! ##### 1.2.6.3 Absolute positioning: Manual
//!
//! Manual positions can only be used with widgets in a layout in float mode. The properties "x" and
//! "y" can be used to control one or both positions. It's also possible to use the "pos"
//! convenience property to set both at the same time in the format "pos: x, y". Let's set a Label
//! with absolute position:
//! ```
//! - Layout:
//!     mode: float
//!     - Label:
//!         text: Hello world
//!         pos: 10, 10
//! ```
//!
//! <a name="positioning_maths"></a>
//! ##### 1.2.6.4 Absolute positioning - Maths:
//!
//! When setting position manually it is possible to refer to other widgets' properties and even
//! perform maths on them (e.g. "widget1.x = widget2.x - widget3.x"). First, here is
//! an example of simply binding the y of one widget to another:
//!
//! ```
//! - <MyLabel@Label>:
//!     text: My text
//!     auto_scale: true, true
//!     border: true
//!
//! - Layout:
//!     mode: float
//!     - MyLabel:
//!         id: label_1
//!         pos: 5, 5
//!     - MyLabel:
//!         x: 10
//!         y: parent.label_1.x
//!
//! ```
//!
//! Now let's use maths to make the label y position a fixed amount below the other label y:
//! ```
//! - <MyLabel@Label>:
//!     text: My text
//!     auto_scale: true, true
//!     border: true
//!
//! - Layout:
//!     mode: float
//!     - MyLabel:
//!         id: label_1
//!         pos: 5, 5
//!     - MyLabel:
//!         x: parent.label_1.x
//!         y: parent.label_1.y + 5
//!
//! ```
//! You can use as many numbers or references to other properties as you want in these formulas. The
//! only restriction is that the other properties should also be usize.
//!
//! <a name="positioning_adjusting"></a>
//! ##### 1.2.6.5 Adjusting position: padding and aligning
//!
//! It is not possible to control position in layout modes other that float. In the fixed layout
//! modes it is still possible to adjust position. This can be done with padding and aligning.
//!
//! **Aligning:**
//!
//! Aligning can be done horizontally (halign) or vertically (valign). Aligning is only useful if
//! the widget is smaller than the layout.
//!
//! Halign supports:
//! - Left
//! - Center
//! - Right
//!
//! Valign supports:
//! - Top
//! - Middle
//! - Bottom
//!
//! If a widget has "halign: center" and it is less wide than its parent layout, it will be centered
//! horizontally. This can be useful for example in a box mode layout with vertical orientation. In
//! that case, widgets will be stacked vertically and you have no control over horizontal position.
//! By using halign, you can still control whether the widgets go left, right, or in the center.
//! Here is an example of this with a label:
//! ```
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - Label:
//!         text: Hello world
//!         auto_scale: true, true
//!         halign: center
//! ```
//!
//! **Padding:**
//!
//! Padding adds empty space to the left, right, top, and/or bottom of a widget. It allows you to
//! create some space between widgets, create a margin between the border of a layout and a widget,
//! etc. There are 7 different padding properties:
//! - padding: left, right, top, bottom (e.g. "padding: 1, 1, 1, 1")
//! - padding_x: left, right (e.g. "padding_x: 1, 1")
//! - padding_y: top, bottom (e.g. "padding_y: 1, 1")
//! - padding_left: left (e.g. "padding_left: 1")
//! - padding_right: left (e.g. "padding_right: 1")
//! - padding_top: left (e.g. "padding_top: 1")
//! - padding_bottom: left (e.g. "padding_bottom: 1")
//!
//! Here is an example layout with 2 labels without padding:
//! ```
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - Label:
//!         text: Hello world
//!         auto_scale: true, true
//!     - Label:
//!         text: Hello world
//!         auto_scale: true, true
//! ```
//! Here is the same example with padding:
//! ```
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - Label:
//!         text: Hello world
//!         auto_scale: true, true
//!         border: true
//!         padding_left: 1
//!         padding_bottom: 1
//!     - Label:
//!         text: Hello world
//!         auto_scale: true, true
//!         border: true
//!         padding_left: 1
//! ```
//!
//! <a name="keyboard_selection"></a>
//! #### 1.2.7 Keyboard selection:
//!
//! Keyboard selection, unlike mouse selection, requires the configuration of a property. You need
//! to configure the selection order of each widget that should be selectable through keyboard.
//! This selection order is global over the active screen or popup. The 'down arrow' button on the
//! keyboard cycles down through the selection order (1 > 2 > 3, etc.). The 'up arrow' button on the
//! keyboard cycles up (3 > 2 > 1, etc.). If the highest or lowest widget is reached, the selection
//! cycles back around.
//!
//! For example, if you have a menu layout and you want to select buttons from top-to-bottom, you
//! would use:
//! ```
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - Button:
//!         text: Option 1
//!         auto_scale_height: true
//!         selection_order: 1
//!     - Button:
//!         text: Option 2
//!         auto_scale_height: true
//!         selection_order: 5
//!     - Button:
//!         text: Option 3
//!         auto_scale_height: true
//!         selection_order: 10
//! ```
//! Note that we did not use consecutive numbers. Instead we increased the order by 5 each time, so
//! we can leave some space for possible future widgets.
//!
//! If we have multiple layouts, we do not reset the selection order. The order is global. So if
//! we have two layouts:
//! ```
//! - Layout:
//!     mode: box
//!     - Layout:
//!         mode: box
//!         orientation: vertical
//!         - Button:
//!             text: Left Option 1
//!             auto_scale_height: true
//!             selection_order: 1
//!         - Button:
//!             text: Left Option 2
//!             auto_scale_height: true
//!             selection_order: 5
//!         - Button:
//!             text: Left Option 3
//!             auto_scale_height: true
//!             selection_order: 10
//!     - Layout:
//!         mode: box
//!         orientation: vertical
//!         - Button:
//!             text: Right Option 1
//!             auto_scale_height: true
//!             selection_order: 15
//!         - Button:
//!             text: Right Option 2
//!             auto_scale_height: true
//!             selection_order: 20
//!         - Button:
//!             text: Right Option 3
//!             auto_scale_height: true
//!             selection_order: 25
//! ```
//!
//! <a name="binding_properties"></a>
//! #### 1.2.8 Binding properties:

//! <a name="binding_properties_referring"></a>
//! ##### 1.2.8.1 Referring to other properties:
//!
//! It is possible to bind one property to another in EzLang, as long as the properties are of the
//! same type (you can find the type of each property in [Reference]). The exception is the String
//! type property (for example the 'text' property of a label); you can bind any type of property
//! to a String property, as every property can be converted to a String. Here is an example of
//! binding the width of one widget to another:
//! ```
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - Label:
//!         id: longer_label
//!         text: Long label text
//!         auto_scale: true, true
//!     - Label:
//!         text: Short text
//!         auto_scale_height: true
//!         size_hint_x: none
//!         width: parent.longer_label.width
//! ```
//!
//! To bind one property to another, simply refer to that property instead of providing a value.
//! The syntax to do this is a full or relative path to the property you want to bind to.
//! To provide an absolute path start with "root.", e.g.:
//! ```
//! width: root.layout_1.layout_2.label_1.width
//! ```
//!
//! To provide a relative path there are two possibilities: "self" or "parent". "self" refers to the
//! widget itself; it can be used to bind one property of a widget to another. E.g. to bind the
//! width of a widget to its height:
//! ```
//! width: self.height
//! ```
//!
//! "parent" refers to the parent layout and can be used recursively, so "parent.parent" is also
//! valid. This is an easy way to refer to another widget in the same layout:
//! ```
//! width: parent.other_label.width
//! ```
//!
//! There is one other piece of syntax: "property". This can be used to refer to custom properties
//! that you have created with the scheduler. We will discuss these in the coming [Scheduler]
//! chapter. We'll just note for now that we can refer to custom properties from EzLang in the
//! following format:
//! ```
//! width: properties.my_custom_property
//! ```
//!
//! So far we have only used numerical (usize) properties in our examples, but any property can be
//! bound:
//! ```
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - Label:
//!         id: longer_label
//!         text: Long label text
//!         auto_scale: true, true
//!         halign: center
//!     - Label:
//!         text: Short text
//!         size_hint_x: none
//!         width: parent.longer_label.width
//!         halign: parent.longer_label.halign
//!         auto_scale_height: parent.longer_label.auto_scale_height
//! ```
//!
//! <a name="binding_properties_overview"></a>
//! ##### 1.2.8.2 Property type overview:
//! Here is an overview of the property types used by EzTerm (the type of each property can be
//! found in [reference]):
//!
//! - usize
//! - f64
//! - String
//! - bool
//! - Color
//! - LayoutMode
//! - LayoutOrientation
//! - VerticalAlignment
//! - HorizontalAlignment
//! - VerticalPosHint
//! - HorizontalPosHint
//! - SizeHint
//!
//!
//! <a name="binding_properties_maths"></a>
//! ##### 1.2.8.3 Using maths on properties:
//!
//! When referring to usize properties (such as size and position for example) it is possible to
//! use maths with multiple numbers and properties (refer to properties using the syntax
//! we saw in the previous chapters). Whenever any of the properties referred to change, the
//! formula will be reevaluated. Let's look at an example of setting the height of a widget to the
//! height of the parent minus the height of the other widget:
//!
//! ```
//! - <MyLabel@Label>:
//!     text: My text
//!     size_hint: none, none
//!     border: true
//!
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - MyLabel:
//!         id: label_1
//!         size: 10, 3
//!     - MyLabel:
//!         width: 10
//!         height: parent.height - parent.label_1.height
//!
//! ```
//!
//! <a name="scheduler"></a>
//! ### 1.3 Scheduler:
//!
//! The scheduler is an object that allows you to configure callbacks, scheduled tasks, widget- and
//! full screen updates, etc. It is a core part of managing the user interface from code. In this
//! chapter we will learn about all the features of the scheduler and how to use them.
//!
//! <a name="scheduler_states"></a>
//! #### 1.3.1 Widget states and the State Tree:
//!
//! Before we go on to describe the scheduler, we have to look at widget states first. We said that
//! the scheduler was how we manage the UI from code, and we do that by manipulating widget states.
//! Every property of a widget (such as those we set from the .ez files) is contained in its widget
//! state. If we change a property in the widget state and call the '.update' method on it, the
//! widget will be redrawn on the next frame to reflect the new state. So if we wanted to change
//! the text of a label from code it would look like this:
//! ```
//! label_state.set_text("new text".to_string());
//! ```
//! The state of every widget active in our UI is contained in the "State tree". The state tree is
//! available to us when we initialize the UI, and is given to us in every callback. We can use the
//! state tree to get a widget state using the "get" or "get_mut" methods. So if we wanted
//! to change the text of a label with the id "my_label" when initializing the UI, we would do this:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let label_state = state_tree.get_mut("my_label").as_label_mut();
//! label_state.set_text("new text".to_string());
//! ```
//! Note that after getting the state of the label, we have to call the "to_label_mut" method to
//! change the state into a label state. This is because the state tree contains generic states (due
//! to Rusts strict type requirements), and so we have to cast the state into the right type before
//! we can actually use it. This will become second nature quickly when working with EzTerm. Just
//! keep in mind: if you want to actually alter the property of a state, call "to_x" or "to_x_mut"
//! on it first. Here are all available casts:
//!
//! - as_layout(_mut)
//! - as_label(_mut)
//! - as_text_input(_mut)
//! - as_button(_mut)
//! - as_checkbox(_mut)
//! - as_radio_button(_mut)
//! - as_slider(_mut)
//! - as_dropdown(_mut)
//! - as_progress_bar(_mut)
//! - as_canvas(_mut)
//!
//! The 'state_tree' object available to us when initializing the UI, in callbacks and in scheduled
//! tasks, is actually the root layout state. So if you ever need the root layout state, you would get it
//! like this:
//! ```
//! let root_layout_state = state_tree.as_layout();
//! ```
//! If we want any other states, we have to use the "get" or "get_mut" methods to find them. In the
//! earlier example we used a widget ID for this. There are in fact three ways to find states in the
//! state tree: by ID, by path or by chaining 'get' calls. Let's look at examples of all three methods:
//!
//! **By id**:
//! ```
//! let label_state = state_tree.get("my_label").as_label();
//! ```
//! Important note: to find widgets by ID, the ID must be unique from that point in the tree. That
//! means that if you search an ID from the root of the state tree, that the ID must be globally
//! unique! As a general rule, make all IDs in your .ez files globally unique if at all possible,
//! because finding states by ID is the most comfortable way to do it, and incurs no performance
//! penalty due to caching.
//!
//! **By path**:
//! ```
//! let label_state = state_tree.get("/root/layout/sub_layout/my_label").as_label();
//! ```
//! Here we used the full path to find a widget. A true full path always starts with "/root" (the
//! static name of the root layout), but since "state_tree" is in fact the root itself, we do not
//! necessarily have to start our paths with "/root", so we could use the shorter version:
//! ```
//! let label_state = state_tree.get("/layout/sub_layout/my_label").as_label();
//! ```
//! This is still a bit verbose, so it's usually more convenient to make IDs globally unique and
//! search by ID.
//!
//! **By chaining get calls**:
//!
//! The last method is to chain get calls. The 'get' method returns another part of the state tree, so
//! we could just call 'get' again:
//! ```
//! let label_state = state_tree.get("layout").get("sub_layout").get("my_label").as_label();
//! ```
//! This is very verbose; so when is this useful? Mostly you want to avoid it, but it comes in handy
//! when you want to manipulate multiple child states with non-unique IDs. Perhaps you spawned some
//! widgets from code, and non-unique IDs were unavoidable. Let's say you have layouts with three
//! labels, and you want to update the text of each label. You could retrieve the layout state first,
//! and then access each child state from there:
//! ```
//! let layout_1 = state_tree.get("sub_layout_1");
//! layout_1.get("my_label_1").as_label_mut().set_text("Some".to_string());
//! layout_1.get("my_label_2").as_label_mut().set_text("new".to_string());
//! layout_1.get("my_label_3").as_label_mut().set_text("Text".to_string());
//! let layout_2 = state_tree.get("sub_layout_2");
//! layout_2.get("my_label_1").as_label_mut().set_text("Some".to_string());
//! layout_2.get("my_label_2").as_label_mut().set_text("new".to_string());
//! layout_2.get("my_label_3").as_label_mut().set_text("Text".to_string());
//! ```
//!
//! We will describe callbacks in detail below, but we will note for now that the state tree is
//! available in callbacks through the "context" parameter. So if we wanted to change the text of
//! our label from a callback it would look like this:
//! ```
//! use ez_term::*;
//! fn my_callback(context: Context) -> bool {
//!
//!     let label_state = context.state_tree.get_mut("my_label").as_label_mut();
//!     label_state.set_text("new text".to_string());
//!     label_state.update(context.scheduler);
//!     true
//! }
//! ```
//! Don't worry about the callback syntax for now, just note that we control widgets from code by
//! manipulating the state, which we get from the state tree available in the callback context. We
//! could also see in the example that each state has an ".update" method. When we call update, the
//! widget will be redrawn on the next frame. You will want to call this when changing a state from
//! a callback most of the time.
//!
//! Now we'll look at the scheduler object. After that, we will start putting our new knowledge
//! about states to use when discussing the actual features of the scheduler.
//!
//! <a name="scheduler_object"></a>
//! #### 1.3.2 Using the scheduler object:
//!
//! The scheduler is an object which you can use in three places: when initializing the UI, when
//! inside of a callback, and when inside a scheduled function.
//! Here is an example of using the scheduler when initializing the UI:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! scheduler.set_selected_widget("my_widget", None);
//! run(root_widget, state_tree, scheduler);
//! ```
//! Don't worry about the 'set_selected_widget' method for now, we are just looking at accessing the
//! scheduler. When we are inside of a callback, the scheduler is available to us from the context
//! parameter:
//! ```
//! use ez_term::*;
//! fn my_callback(context: Context) -> bool {
//!
//!     context.scheduler.set_selected_widget("my_widget", None);
//!     true
//! }
//! ```
//! The same is true from inside a scheduled task:
//! ```
//! use ez_term::*;
//! fn my_task(context: Context) {
//!
//!     context.scheduler.set_selected_widget("my_widget", None);
//! }
//! ```
//!
//! Now that we know how to use the scheduler object when initializing the UI, from callbacks, and
//! from scheduled tasks, we will look at the many things we can do with the scheduler. Here is a
//! short overview of the features we will look at:
//! - Managing callbacks
//! - Scheduling tasks
//! - Managing popups
//! - Creating/removing widgets from code
//! - Creating custom properties
//! - Updating widgets
//! - Managing widget selection
//!
//!
//! <a name="scheduler_callbacks"></a>
//! #### 1.3.3 Managing callbacks
//!
//! There are many types of callbacks. We will discuss each of them later in this chapter.
//! There is one import thing to mention right away: callbacks in general are executed in the main
//! thread. This means that callbacks are expected to return immediately. Callbacks that only manage
//! the UI will return immediately and can be used normally. If you want to use a callback to run your
//! app, you can use a callback to spawn a [Threaded scheduled task]. We will learn more about this
//! in the sections coming up, but this is important to keep in mind.
//!
//! Now, let's look at the general structure of callbacks:
//!
//! <a name="scheduler_callbacks_structure"></a>
//! ##### 1.3.3.1 General callback structure
//!
//! Callbacks can be created from a closure or from a function. We will see examples of both below.
//!
//! All callbacks take a "context: Context" parameter. The Context object contains the
//! StateTree object (context.state_tree) and the Scheduler object (context.scheduler). We can use
//! these to manage the UI, as is being explained in this chapter. The Context also contains the
//! path of the widget for which the callback was called (context.widget_path). Some callbacks have
//! more parameters (for example, mouse callbacks have a mouse_pos parameter), but we will discuss
//! these separately for each callback when relevant.
//!
//! Finally: each callback returns a bool. The bool indicates whether the event should be consumed.
//! If the event is not consumed, the widget is allowed to execute its default behavior if it has
//! any. For example, the checkbox widget has default "on_press" behavior: when pressed, it will toggle
//! on/off. If you bind a custom "on_press" callback for a checkbox, you control whether the default
//! behavior will be executed by returning 'false' (allowed to run) or 'true' (not allowed to run).
//! This gives you the option to overwrite default widget behavior, or supplement it. If you want to
//! know if returning true for a widget callback would overwrite default behavior, see the
//! [reference] entry for that widget and check the callback chapter. For mouse callbacks (such as
//! on_left_click, on_hover, etc.), it is also important to think about whether you want to consume
//! the event. A mouse click will always hit multiple widgets; if you click a button, you also click
//! the layout that contains the button, the layout that contains the layout, etc. If a widget
//! callback returns true, all the other widgets will not receive the event. The root layout is the
//! first to receive an event, and the widget the last (i.e. events move along the widget path). For
//! performance reasons you should return true for mouse callbacks with no default behavior, unless
//! you have a reason not to do so.
//!
//! To summarize, here are two examples of the default callback structure (one closure and one
//! function):
//!
//! **Callback from closure**
//! ```
//! use ez_term::*;
//! let my_callback = |context: Context| {
//!     true
//! };
//! ```
//! **Callback from function**
//! ```
//! use ez_term::*;
//! fn my_callback(context: Context) -> bool {
//!     true
//! };
//! ```
//!
//! Now that we know what a callback should look like, let's see how to bind callbacks.
//!
//! <a name="scheduler_callbacks_config"></a>
//! ##### 1.3.3.2 Callback config
//!
//! Each widget active in your UI has an associated callback config. This config contains all
//! callbacks that are active for that widget. Initially, the callback config for each widget is
//! empty. To manage the callbacks for a widget, we create a new CallbackConfig object and load
//! our callbacks into it. We then either overwrite the current callback config of a widget, or
//! update it. Overwriting it will delete the current config. If we update it, any callbacks
//! configured in the new config will be set on the current config (while leaving the the others
//! intact).
//!
//! Let's say we want to set an "on_press" callback on a button with the ID: "my_button".
//! We want the callback to change the text on a label. This is how we would do it:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = |context: Context| {
//!
//!     let label_state = context.state_tree.get_mut("my_label").as_label_mut();
//!     label_state.set_text("Button was clicked!".to_string());
//!     label_state.update(context.scheduler);
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
//! scheduler.update_callback_config("my_button", new_callback_config);
//! ```
//! We created a new callback config using the "from_on_press" method. There is a "from_" method for
//! each type of callback to make it easier to initialize a new CallbackConfig.
//! Note that if you want to update a callback config by widget ID like we do in the above example
//! (or find a widget state using ID), the ID must be globally unique. If the ID is not globally
//! unique, you can use the full widget path. Since ID is more convenient, it is recommended to make
//! all your widget IDs unique.
//!
//! We will go through another example accomplishing the same thing as above, but this time we will
//! use a function instead of a closure, and we will overwrite the callback config instead of
//! updating it:
//! ```
//! use ez_term::*;
//! fn my_callback(context: Context) -> bool {
//!
//!     let label_state = context.state_tree.get_mut("my_label").as_label_mut();
//!     label_state.set_text("Button was clicked!".to_string());
//!     label_state.update(context.scheduler);
//!     true
//! };
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
//! scheduler.overwrite_callback_config("my_button", new_callback_config);
//! ```
//!
//! As you can see, both closures and functions can be used to write callbacks. The advantage of
//! closures however is that we can capture variables with the "move" keyword. Let's repeat the
//! first example, but this time we want to update the label with a counter each time the button is
//! pressed:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let mut counter: usize = 0;
//! let my_callback = move |context: Context| {
//!
//!     counter += 1;
//!     let label_state = context.state_tree.get_mut("my_label").as_label_mut();
//!     label_state.set_text(format!("Button was clicked {} times!", counter));
//!     label_state.update(context.scheduler);
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
//! scheduler.update_callback_config("my_button", new_callback_config);
//! ```
//! Here we created a variable and moved it into the callback closure. This can be very useful, and
//! it's a good pattern to keep in mind.
//!
//! Now that we know how to create a callback and bind it to an object, we will go over all callback
//! types. Not all callbacks are available for all widgets. To see which callbacks are available for
//! a widget, check the widget entry under [Reference]. Here is a quick overview of all callbacks:
//!
//! - on_keyboard_enter
//! - on_left_mouse_click
//! - on_press
//! - on_select
//! - on_deselect
//! - on_right_mouse_click
//! - on_hover
//! - on_drag
//! - on_scroll_up
//! - on_scroll_down
//! - on_value_change
//! - Custom key binds
//! - Property binds
//!
//! <a name="scheduler_callbacks_enter"></a>
//! ##### 1.3.3.3 On_keyboard_enter
//!
//! This callback is activated when a widget is selected and the 'enter' key is pressed on the
//! keyboard.
//! To set this callback with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: Context| {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
//! scheduler.update_callback_config("my_button", new_callback_config);
//! ```
//! To set this callback with a function:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: Context) -> bool {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
//! scheduler.update_callback_config("my_button", new_callback_config);
//! ```
//!
//! <a name="scheduler_callbacks_left"></a>
//! ##### 1.3.3.4 On_left_mouse_click
//!
//! This callback is activated when a widget is clicked by the left mouse button. Keep in mind that
//! when a widget is clicked, any layouts underneath it are also clicked. The root layout is the
//! first to receive the mouse click event, followed by sub layouts, and finally the widget. If any
//! layout has a callback that returns true, the event is consumed and does not reach further
//! layouts or widgets.
//! Note that the mouse_pos parameter is available to you; it contains the
//! coordinates of the mouse click relative to the widget. So if the coordinates are (3, 2), it
//! means the click was located in the widget on the third pixel from the left and the second pixel
//! from the top. You can access the coordinates through 'mouse_pos.x' and 'mouse_pos.y'.
//! To set this callback with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: Context, mouse_pos: Coordinates| {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_left_mouse_click(Box::new(my_callback));
//! scheduler.update_callback_config("my_label", new_callback_config);
//! ```
//! To set this callback with a function:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: Context, mouse_pos: Coordinates) -> bool {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_left_mouse_click(Box::new(my_callback));
//! scheduler.update_callback_config("my_label", new_callback_config);
//! ```
//! <a name="scheduler_callbacks_press"></a>
//! ##### 1.3.3.5 On_press
//!
//! This callback is activated when a widget is either clicked by the left mouse button, or
//! keyboard entered when it is selected. In other words, it is a composite callback containing both
//! on_keyboard_enter and on_left_mouse_click. This can be useful for example with buttons, where
//! you want something done regardless of whether the user used his mouse or keyboard to press the
//! button. To set this callback with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: Context| {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
//! scheduler.update_callback_config("my_button", new_callback_config);
//! ```
//! To set this callback with a function:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: Context) -> bool {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
//! scheduler.update_callback_config("my_button", new_callback_config);
//! ```
//!
//! <a name="scheduler_callbacks_select"></a>
//! ##### 1.3.3.6 On_select
//!
//! This callback is activated when a widget is selected. A selection can occur when the user uses
//! the keyboard up/down buttons (and the widget has a selection_order) or when the widget is
//! hovered. Selectable widgets are: buttons, checkboxes, dropdowns, radio buttons and sliders.
//! Text inputs are selectable by keyboard, but not by mouse hovering; instead they have to be
//! clicked to be selected. The second argument in a on_select callback is an Option<Coordinates>.
//! If a widget was selected by keyboard, this argument will be None. If it was selected by mouse,
//! it will contains the coordinates the mouse click relative to the widget. So if the coordinates
//! are (3, 2), it means the click was located in the widget on the third pixel from the left and
//! the second pixel from the top. You can access the coordinates through 'mouse_pos.x' and
//! 'mouse_pos.y'.
//! To set this callback with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: Context, mouse_pos: Option<Coordinates>| {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_select(Box::new(my_callback));
//! scheduler.update_callback_config("my_button", new_callback_config);
//! ```
//! To set this callback with a function:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: Context, mouse_pos: Option<Coordinates>) -> bool {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_select(Box::new(my_callback));
//! scheduler.update_callback_config("my_button", new_callback_config);
//! ```
//! <a name="scheduler_callbacks_deselect"></a>
//! ##### 1.3.3.7 On_deselect
//!
//! This callback is activated when a widget is deselected. A deselection occurs when the mouse
//! cursor leaves the selection widget, or when the user uses the keyboard up/down buttons to move
//! on from the selected widget. To set this callback with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: Context| {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_deselect(Box::new(my_callback));
//! scheduler.update_callback_config("my_button", new_callback_config);
//! ```
//! To set this callback with a function:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: Context) -> bool {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_deselect(Box::new(my_callback));
//! scheduler.update_callback_config("my_button", new_callback_config);
//! ```
//!
//! <a name="scheduler_callbacks_right"></a>
//! ##### 1.3.3.8 On_right_mouse_click
//!
//! This callback is activated when a widget is clicked by the right mouse button. Keep in mind that
//! when a widget is clicked, any layouts underneath it are also clicked. The root layout is the
//! first to receive the mouse click event, followed by sub layouts, and finally the widget. If any
//! layout has a callback that returns true, the event is consumed and does not reach further
//! layouts or widgets.
//! Note that the mouse_pos parameter is available to you; it contains the
//! coordinates of the mouse click relative to the widget. So if the coordinates are (3, 2), it
//! means the click was located in the widget on the third pixel from the left and the second pixel
//! from the top. You can access the coordinates through 'mouse_pos.x' and 'mouse_pos.y'.
//! To set this callback with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: Context, mouse_pos: Coordinates| {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_right_mouse_click(Box::new(my_callback));
//! scheduler.update_callback_config("my_button", new_callback_config);
//! ```
//! To set this callback with a function:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: Context, mouse_pos: Coordinates) -> bool {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_right_mouse_click(Box::new(my_callback));
//! scheduler.update_callback_config("my_button", new_callback_config);
//! ```
//! <a name="scheduler_callbacks_hover"></a>
//! ##### 1.3.3.9 On_hover
//!
//! This callback is activated when a widget is hovered by the mouse. Keep in mind that
//! when a widget is hovered, any layouts underneath it are also hovered. The root layout is the
//! first to receive the hover event, followed by sub layouts, and finally the widget. If any
//! layout has a callback that returns true, the event is consumed and does not reach further
//! layouts or widgets.
//! Note that the mouse_pos parameter is available to you; it contains the
//! coordinates of the mouse click relative to the widget. So if the coordinates are (3, 2), it
//! means the click was located in the widget on the third pixel from the left and the second pixel
//! from the top. You can access the coordinates through 'mouse_pos.x' and 'mouse_pos.y'.
//! To set this callback with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: Context, mouse_pos: Coordinates| {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_hover(Box::new(my_callback));
//! scheduler.update_callback_config("my_label", new_callback_config);
//! ```
//! To set this callback with a function:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: Context, mouse_pos: Coordinates) -> bool {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_hover(Box::new(my_callback));
//! scheduler.update_callback_config("my_label", new_callback_config);
//! ```
//! <a name="scheduler_callbacks_drag"></a>
//! ##### 1.3.3.10 On_drag
//!
//! This callback is activated when a widget is left mouse clicked and the click is not released.
//! As long as the click is not released, the widget will receive a new event every time the mouse
//! cursor changes position, as long as the mouse cursor stays on that widget. The callback receives
//! two extra arguments: one is the previous drag position, and one is the current drag position.
//! The previous drag position argument is an Option<Coordinates>; on the very first drag event,
//! the previous drag position will be None. This is how you know the drag is new. Subsequently,
//! the previous drag position will contain Coordinates. Because you have both the current and the
//! previous coodinates, you know which direction the drag is going.
//! The coordinates are relative to the widget. So if the coordinates are (3, 2), it means the
//! click was located in the widget on the third pixel from the left and the second pixel from the
//! top. You can access the coordinates through 'mouse_pos.x' and 'mouse_pos.y'.
//! To set this callback with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: Context, previous_mouse_pos: Option<Coordinates>,
//!                         mouse_pos: Coordinates| {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_drag(Box::new(my_callback));
//! scheduler.update_callback_config("my_label", new_callback_config);
//! ```
//! To set this callback with a function:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: Context, previous_mouse_pos: Option<Coordinates>,
//!                mouse_pos: Coordinates) -> bool {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_drag(Box::new(my_callback));
//! scheduler.update_callback_config("my_label", new_callback_config);
//! ```
//!
//! <a name="scheduler_callbacks_up"></a>
//! ##### 1.3.3.11 On_scroll_up
//!
//! This callback is activated when a widget is scrolled up by the mouse. Keep in mind that
//! when a widget is scrolled, any layouts underneath it are also scrolled. The root layout is the
//! first to receive the scroll event, followed by sub layouts, and finally the widget. If any
//! layout has a callback that returns true, the event is consumed and does not reach further
//! layouts or widgets.
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: Context| {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_scroll_up(Box::new(my_callback));
//! scheduler.update_callback_config("my_label", new_callback_config);
//! ```
//! To set this callback with a function:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: Context) -> bool {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_scroll_up(Box::new(my_callback));
//! scheduler.update_callback_config("my_label", new_callback_config);
//! ```
//!
//! <a name="scheduler_callbacks_down"></a>
//! ##### 1.3.3.12 On_scroll_down
//!
//! This callback is activated when a widget is scrolled down by the mouse. Keep in mind that
//! when a widget is scrolled, any layouts underneath it are also scrolled. The root layout is the
//! first to receive the scroll event, followed by sub layouts, and finally the widget. If any
//! layout has a callback that returns true, the event is consumed and does not reach further
//! layouts or widgets. To set this callback with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: Context| {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_scroll_down(Box::new(my_callback));
//! scheduler.update_callback_config("my_label", new_callback_config);
//! ```
//! To set this callback with a function:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: Context) -> bool {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_scroll_down(Box::new(my_callback));
//! scheduler.update_callback_config("my_label", new_callback_config);
//! ```
//!
//! <a name="scheduler_callbacks_value"></a>
//! ##### 1.3.3.13 On_value_change
//!
//! This callback is activated when the value of a widget has changed. Only widgets with values
//! support this, which are: checkbox, dropdown, radio button, text input and slider. The only
//! special case is the radio button; when a radio button is activated, all other radio buttons in
//! that group are deactivated (because they're mutually exclusive). For radio buttons,
//! on_value_change is only called when a button becomes *active*.
//! To set this callback with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: Context| {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_value_change(Box::new(my_callback));
//! scheduler.update_callback_config("my_checkbox", new_callback_config);
//! ```
//! To set this callback with a function:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: Context) -> bool {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_value_change(Box::new(my_callback));
//! scheduler.update_callback_config("my_checkbox", new_callback_config);
//! ```
//!
//! <a name="scheduler_callbacks_keymap"></a>
//! ##### 1.3.3.14 Custom key binds
//!
//! Custom keymaps allow you to bind keyboard keys to a callback. Keep in mind that for this to work,
//! a widget must already be selected; only then will it receive the keyboard event.
//! To bind a key, we will use the CallbackConfig again, and use its 'bind_key' method to bind keys.
//! The first parameter of bind_key is the KeyCode we want to bind; the second parameter the
//! modifiers (which can be None, or one of the modifiers CTRL, SHIFT or ALT), and the third
//! parameter is the callback. We can call this method as many times as we like to bind different
//! keys.
//!
//! Let's bind a couple of different keys to a callback closure, to demonstrate simple keys
//! and modified keys:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: Context, keycode: KeyCode| {
//!
//!     true
//! };
//!
//! let mut new_callback_config = CallbackConfig::default();
//!
//! // Bind a simple character
//! new_callback_config.bind_key(KeyCode::Char('a'), None, Box::new(my_callback));
//!
//! // Bind a functional key like the Tab key
//! new_callback_config.bind_key(KeyCode::Tab, None, Box::new(my_callback));
//!
//! // Bind a modified character
//! new_callback_config.bind_key(KeyCode::Char('a'), Some(vec!(KeyModifiers::CONTROL)),
//!                              Box::new(my_callback));
//!
//! // Bind a double modifier
//! new_callback_config.bind_key(KeyCode::Char('a'),
//!                              Some(vec!(KeyModifiers::CONTROL, KeyModifiers::ALT)),
//!                              Box::new(my_callback));
//!
//! scheduler.update_callback_config("my_checkbox", new_callback_config);
//! ```
//! To do the same with a function:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: Context, keycode: KeyCode) -> bool {
//!
//!     true
//! };
//!
//! let mut new_callback_config = CallbackConfig::default();
//! new_callback_config.bind_key(KeyCode::Char('a'), None, Box::new(my_callback));
//! scheduler.update_callback_config("my_checkbox", new_callback_config);
//! ```
//!
//! <a name="scheduler_callbacks_global"></a>
//! ##### 1.3.3.14 Global key binds
//!
//! There is a global keymap. It functions like the custom keymap, except keys from the global
//! keymap always have priority in consuming events, and are available in all situations. The
//! 'bind_global_key' function of the scheduler allows you to bind global keys to callbacks. The
//! 'remove_global_key' and 'clear_global_keys' methods allow you to remove one or all global
//! keybinds.
//!
//! Let's look at some examples: we want to bind various keys to a callback toggling a checkbox on
//! or off:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: Context, keycode: KeyCode) -> bool {
//!
//!     let checkbox_state = context.state_tree.get_mut("my_checkbox").as_checkbox_mut();
//!     checkbox_state.set_active(!checkbox_state.get_active());
//!     checkbox_state.update(context.scheduler);
//!     true
//! };
//!
//!
//! // Bind a simple character
//! scheduler.bind_global_key(KeyCode::Char('a'), None, Box::new(my_callback));
//!
//! // Bind a functional key like the Tab key
//! scheduler.bind_global_key(KeyCode::Tab, None, Box::new(my_callback));
//!
//! // Bind a modified character
//! scheduler.bind_global_key(KeyCode::Char('a'), Some(vec!(KeyModifiers::CONTROL)),
//!                           Box::new(my_callback));
//!
//! // Bind a double modifier
//! scheduler.bind_global_key(KeyCode::Char('a'),
//!                           Some(vec!(KeyModifiers::CONTROL, KeyModifiers::ALT)),
//!                           Box::new(my_callback));
//! ```
//!
//! If we now wanted to remove the 'a' keybind specifically:
//! ```
//! use ez_term::*;
//! scheduler.remove_global_key(KeyCode::Char('a'), None);
//! scheduler.remove_global_key(KeyCode::Tab, None);
//! scheduler.remove_global_key(KeyCode::Char('a'), vec!(KeyModifiers::CONTROL));
//! scheduler.remove_global_key(KeyCode::Char('a'), vec!(KeyModifiers::CONTROL, KeyModifiers::ALT));
//! ```
//!
//! If we wanted to clear all keys:
//! ```
//! scheduler.clear_global_keys();
//! ```
//!
//! <a name="scheduler_callbacks_property"></a>
//! ##### 1.3.3.15 Property binds
//!
//! You can bind callbacks to changes in widget properties or custom properties. This is done
//! differently from widget callbacks. Instead, you can use the "bind" method on the property you
//! wish to bind to. There is an example of how to bind to each property of each widget in
//! [reference]. Here is an example of binding a callback to a labels height property,
//! using a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = |context: Context| {
//!
//!     true
//! };
//!
//! let state = state_tree.get("my_label").as_label_mut();
//! state.size.height.bind(Box::new(my_callback), &mut scheduler);
//! ```
//! The same example but using a function:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: Context) -> bool {
//!
//!     true
//! };
//!
//! let state = state_tree.get("my_label").as_label_mut();
//! state.size.height.bind(Box::new(my_callback), &mut scheduler);
//! ```
//!
//! <a name="scheduler_tasks"></a>
//! ### 1.3.4 Managing scheduled tasks
//!
//! Scheduled tasks are closures or functions that are executed according to time parameters. There
//! are three types of schedules tasks:
//!
//! - Single execution
//! - Recurring execution
//! - Threaded execution
//!
//! **Important:** Single- and recurring tasks should only be used to manipulate the UI; this is
//! because these tasks are expected to return immediately. If they do not, the UI will hang.
//!
//! Therefore, in order to execute (parts of) your personal app code, always use 'schedule_threaded',
//! which will be explained below in the chapter "Threaded execution".
//!
//! <a name="scheduler_tasks_single"></a>
//! #### 1.3.4.1 Single execution
//!
//! A single execution task is a closure or function that is executed only once, after a specified
//! delay. As the delay can be 0, it can also be executed on the next frame. The scheduler method
//! we will use is 'schedule_once'. The first parameter of this function is a &str which is the task
//! name. You can use this name to cancel the task before it executes (see example at the bottom of
//! this chapter). Here is  an example of scheduled a single execution task with a closure, changing
//! a label text after 10 seconds:
//! ```
//! use std::time::Duration;
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_task = |context: Context| {
//!
//!     let state = state_tree.get("my_label").as_label_mut();
//!     state.set_text("10 seconds have passed!".to_string());
//!     state.update(context.scheduler);
//! };
//!
//! scheduler.schedule_once("my_task", Box::new(my_task), Duration::from_secs(10));
//! ```
//!
//! The same example with a function:
//! ```
//! use std::time::Duration;
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_task(context: Context) {
//!
//!     let state = state_tree.get_by_id("my_label").as_label_mut();
//!     state.set_text("10 seconds have passed!".to_string());
//!     state.update(context.scheduler);
//! };
//!
//! scheduler.schedule_once("my_task", Box::new(my_task), Duration::from_secs(10));
//! ```
//!
//! It is of course possible to schedule a task from a callback. For example, scheduling a task with
//! delay after a button is pushed. Here is an example: we will bind a callback to a button, that
//! changed the text of a label after 10 seconds, using a closure:
//! ```
//! use std::time::Duration;
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_task = |context: Context| {
//!
//!     let state = state_tree.get("my_label").as_label_mut();
//!     state.set_text("Button was pressed 10 seconds ago!".to_string());
//!     state.update(context.scheduler);
//! };
//!
//! let my_callback = |context: Context| {
//!
//!     scheduler.schedule_once("my_task", Box::new(my_task), Duration::from_secs(10));
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
//! scheduler.update_callback_config("my_button", new_callback_config);
//!
//! ```
//!
//! To cancel a run-once task use the 'cancel_task' method of the scheduler. Of course, this only
//! works if it is called before the task is executed. If the task no longer exists this method
//! will not panic, so it is always safe to try. Example:
//! ```
//! scheduler.cancel_task("my_task");
//! ```
//!
//! <a name="scheduler_tasks_recurring"></a>
//! #### 1.3.4.2 Recurring execution
//!
//! A recurring execution task is executed according to its interval. If the interval is set to
//! 1 second, then the task will be executed every second. The closure or function used to create
//! this task must return a bool. If 'true' is returned, the function will be scheduled again. If
//! 'false' is returned, the task is dropped and never executed again. To schedule a recurring task
//! we will use the 'schedule_recurring' method of the scheduler. The first parameter is a &str,
//! which is the task name. You can use this name to cancel the recurring task manually (as an
//! alternative to returning 'false' from the function). See an example at the bottom of this chapter.
//! Here is an example of updating the text of a label every second; the label text will be
//! counting along with the seconds. Once 10 is reached we stop executing the task. For this we
//! can only use a closure, because we cannot capture variables in regular functions:
//! ```
//! use ez_term::*;
//! use std::time::Duration;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let mut counter: usize = 1;
//! let my_task = move |context: Context| {
//!
//!     let state = context.state_tree.get_mut("my_label").as_label_mut();
//!     state.set_text(format!("Counting {}", counter));
//!     state.update(context.scheduler);
//!     counter += 1;
//!     return if counter <= 10 {
//!         true
//!     } else {
//!         false
//!     };
//! };
//!
//! scheduler.schedule_once("my_task", Box::new(my_task), Duration::from_secs(1));
//! ```
//!
//! It is of course possible to schedule a task from a callback. For example, scheduling a recurring
//! task with delay after a button is pushed. We will move the above example into a button "on_press"
//! callback. Now, the label will not start counting until the user presses a button:
//!
//! ```
//! use ez_term::*;
//! use std::time::Duration;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let mut counter: usize = 1;
//! let my_task = move |context: Context| {
//!
//!     let state = context.state_tree.get_mut("my_label").as_label_mut();
//!     state.set_text(format!("Counting {}", counter));
//!     state.update(context.scheduler);
//!     counter += 1;
//!     return if counter <= 10 {
//!         true
//!     } else {
//!         false
//!     };
//! };
//! let my_callback = |context: Context| {
//!
//!     scheduler.schedule_recurring("my_task", Box::new(my_task), Duration::from_secs(1));
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
//! scheduler.update_callback_config("my_button", new_callback_config);
//! ```
//!
//! To cancel a recurring task use the 'cancel_recurring_task' method of the scheduler. Of course,
//! this only works if called while the task is still running (i.e. it is still returning true). If
//! the task no longer exists this method will not panic, so it is always safe to try. Example:
//! ```
//! scheduler.cancel_recurring_task("my_task");
//! ```
//!
//! <a name="scheduler_tasks_threaded"></a>
//! #### 1.3.4.3 Threaded execution
//!
//! To run custom code that will not return immediately (i.e. your app that you are building the UI
//! for), you must used threaded execution to avoid blocking the UI in the main thread.
//!
//! Working with the UI from a thread is nearly identical to working with the UI from a normal task
//! or callback. The small difference is that in a normal callback you have a "Context" object
//! available, giving you access to the scheduler (context.scheduler) and state tree
//! (context.state_tree). This is the case in threads as well, but now you have a "ThreadedContext"
//! object instead. Where in a normal Context you have a "&mut Scheduler" and "&mut StateTree",
//! in a "ThreadedContext" you have a full "Scheduler" and a full "StateTree" (because we can't use
//! the mutable references in threads). So where in a normal callback you would write
//! "context.scheduler", you now write "&mut context.scheduler", and where you write
//! "context.state_tree", you will now write "&mut context.state_tree". Any changes you make in a
//! threaded state tree will be synced to the main thread automatically. Any methods you call on the
//! scheduler will be passed to the main thread for you as well. Changes to the state tree from the
//! main thread are not synced to threads automatically for performance reasons (often you don't
//! need them), so you can call 'context.scheduler.sync_state_tree()' from a thread to sync the
//! state tree, or 'context.scheduler.sync_properties()' to sync custom properties.
//!
//! When scheduling a threaded task, the first parameter is the function you want to execute, and
//! the second parameter will be an optional on_finish function or closure. The on_finish will be
//! executed when the thread terminates. We'll now look at examples of scheduling threaded functions,
//! and two ways to work with the UI from a thread (first with the state tree, then with a
//! custom property):
//!
//! <a name="scheduler_tasks_state"></a>
//! ##### 1.3.4.3.1 Using StateTree
//!
//! Let's look at an example using a mock app. The mock app will sleep regularly to simulate a long
//! running function and will manipulate the UI using the state tree. We'll assume we have a UI that
//! contains a progress bar and a label, and every time we finish sleeping we will update the
//! progress bar and the label. We will also use an on_finish closure to make the label say
//! "finished!" when the thread terminates (if you do not want an on_finish function, just pass
//! "None"):
//! ```
//! use ez_term::*;
//! use std::time::Duration;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn mock_app(mut context: ThreadedContext) {
//!
//!    for x in 1..=5 {
//!        context.state_tree.get_mut("progress_bar").as_progress_bar_mut().set_value(x*20);
//!        context.state_tree.get_mut("progress_label").as_label_mut().set_text(format!("{}%", x*20));
//!        std::thread::sleep(Duration::from_secs(1)) };
//! }
//!
//! let on_finish = |context: Context| {
//!
//!        let state = context.state_tree.get_mut("progress_label").as_label_mut();
//!        state.set_text("Finished!".to_string());
//!        state.update(context.scheduler);
//! };
//!
//! scheduler.schedule_threaded(Box::new(mock_app), Some(Box::new(on_finish)));
//! ```
//!
//! <a name="scheduler_tasks_property"></a>
//! ##### 1.3.4.3.2 Using custom properties
//!
//! Another way to manipulate the UI from a background thread is through custom properties. These
//! will be discussed in detail in their own chapter below, but we will show an example here. The
//! upshot is that we will create a custom property, bind it to a widget property in the .ez file
//! and then use that custom property in our threaded function. Every time we change it, the widget
//! property it is bound to will update as well.
//!
//! Thinking about our example with the progress bar, we could create a custom property called
//! "my_progress" and bind it to the "value" property of the progress bar. Now, every time we update
//! our custom property in our function, the progress bar will also update. Using custom properties
//! can be more ergonomic if you already have a variable in your custom code, and you always want
//! that variable to be reflected in the UI. Instead of constantly manually updating the UI when
//! your app variable changes, you can just change your app variable to be a custom EzTerm property
//! and save yourself the effort of updating the UI. Let's change the above example to use a custom
//! property; we will first show the .ez file:
//! ```
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - Label:
//!         id: progress_label
//!         text: properties.my_progress
//!     - ProgressBar:
//!         id: progress_bar
//!         max: 100
//!         value: properties.my_progress
//!         border: true
//! ```
//! As you can see we bound the "my_progress" custom property to the progress bar and the label.
//! We can bind a usize property to label.text because every type of property can be converted to a
//! String. For any other property than String, the property types must match.
//! We now need to make sure that the custom property actually exists at run time, and we need to
//! change our mock_app function to make use of it:
//! ```
//! use ez_term::*;
//! use std::time::Duration;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! // We must register our custom property!
//! scheduler.new_usize_property("my_progress", 0);
//!
//! fn mock_app(mut context: ThreadedContext) {
//!
//!    for x in 1..=5 {
//!        let my_progress = context.scheduler.get_property_mut("my_progress").unwrap();
//!        my_progress.as_usize_mut().set(x*20);
//!        std::thread::sleep(Duration::from_secs(1)) };
//! }
//!
//! scheduler.schedule_threaded(Box::new(mock_app), None);
//! ```
//! Because we bound our label text to our custom property, we cannot manually update it to say
//! "Finished!" any more, so we removed the on_finish closure. Of course we could just not bind
//! the label text to "my_progress", but it was useful to see an example of binding any type of
//! property to a string property, and scheduling a threaded task without an on_finish function.
//!
//!
//! <a name="scheduler_properties"></a>
//! ### 1.3.5 Creating custom properties
//!
//! It is possible to create custom EzTerm properties. You can then bind widget properties to these
//! custom properties, so that when the custom property is updated the widget will automatically be
//! updated as well. This is useful when executing your personal app code in a scheduled
//! background task (see above chapter). If you have a variable in your app that you always want to
//! see in the UI, you would have to write code that constantly changes the UI when your variable
//! changes. Instead, you can change your app variable to be an EzProperty; this way, the UI will
//! be automatically updated.
//!
//! In order for this to work, the type of the custom property must be the same
//! as the type of the widget property. The exception is if the widget property is of the String
//! type (like the Label Text property), because every property can be converted to a String.
//! Here is the shortest possible example, binding a custom usize property to a progress bar; first
//! the code, where we register a new custom property:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.new_usize_property("my_progress", 0);
//!
//! run(root_widget, state_tree, scheduler);
//!
//! ```
//!
//! Now the .ez file, where we bind the custom property to the progress bar:
//! ```
//! - Layout:
//!     mode: box
//!     - ProgressBar:
//!         value: properties.my_progress
//! ```
//!
//! Using the above code, we have created and bound a custom property. Let's look at how to actually
//! use it. We will create an entire mock app, meaning both the UI and personal app code in the form
//! of a long running function. When clicking a button, our personal app code will run, and slowly
//! fill up a progress bar to show our user how far along our function is. Here is our .ez file:
//! ```
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - Button:
//!         id: start_button
//!         text: Start app
//!     - Label:
//!         id: progress_label
//!         text: properties.my_progress
//!     - ProgressBar:
//!         id: progress_bar
//!         max: 100
//!         value: properties.my_progress
//!         border: true
//! ```
//! Our progress bar and label both bind to a custom property called "my_progress". Even though
//! "my_progress" will be a usize property, we can bind the text property to it because a String
//! property can bind to everything. Now we write the code that creates the custom property, our
//! mock app code, and a callback for the button to start our app:
//! ```
//! use ez_term::*;
//! use std::time::Duration;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! // We must register our custom property!
//! scheduler.new_usize_property("my_progress", 0);
//!
//! // This is our mock app, it sleeps regularly to simulate a long running function
//! fn mock_app(mut context: ThreadedContext) {
//!
//!    for x in 1..=5 {
//!        let my_progress = context.scheduler.get_property("my_progress").unwrap();
//!        my_progress.as_usize_mut().set(x*20);
//!        std::thread::sleep(Duration::from_secs(1)) };
//! }
//!
//! // We bind a callback to start our mock app on button press
//! let start_button_callback = |context: Context| {
//!     context.scheduler.schedule_threaded(Box::new(mock_app), None);
//! };
//! let callback_config = CallbackConfig::from_on_press(Box::new(start_button_callback));
//! scheduler.update_callback_config("start_button", callback_config);
//! run(root_widget, state_tree, scheduler)
//!
//! ```
//! As you can see, we update the "my_progress" custom property in our mock app. When we do this,
//! the progress bar and the label will update automatically too, because we bound their properties
//! to the custom property. If you imagine that our mock app needed a usize variable to function
//! anyway, then we saved ourselves some effort by replacing the usize variable with an EzTerm usize
//! property, and binding it to the UI for automatic updates.
//!
//! You can access custom properties outside of threads through the scheduler.get_property(name)
//! and scheduler.get_property_mut(name) methods:
//! ```
//! use ez_term::*;
//! use std::time::Duration;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.new_usize_property("my_progress", 0);
//! let property = scheduler.get_property("my_progress");
//! let property_mut = scheduler.get_property_mut("my_progress");
//! property_mut.as_usize_mut().set(10);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! The following functions are available from the scheduler to create custom properties:
//!
//! - new_usize_property(name: &str, value: usize)
//! - new_f64_property(name: &str, value: f64)
//! - new_string_property(name: &str, value: String)
//! - new_bool_property(name: &str, value: bool)
//! - new_color_property(name: &str, value: Color)
//! - new_layout_mode_property(name: &str, value: LayoutMode)
//! - new_layout_orientation_property(name: &str, value: LayoutOrientation)
//! - new_vertical_alignment_property(name: &str, value: VerticalAlignment)
//! - new_horizontal_alignment_property(name: &str, value: HorizontalAlignment)
//! - new_vertical_pos_hint_property(name: &str, value: VerticalPosHint)
//! - new_horizontal_pos_hint_property(name: &str, value: HorizontalPosHint)
//! - new_size_hint_property(name: &str, value: SizeHint)
//!
//!
//! <a name="scheduler_modals"></a>
//! ### 1.3.6 Managing modals (popups)
//!
//! A modal is a layout that is shown in front of the main UI; one of its main use cases is creating
//! popups. A modal is always created from a Layout template created in an .ez file. In other words,
//! you first define what the modal looks like in the .ez file, and then you spawn it from code.
//!
//!
//! You can spawn a popup anytime you have access to the scheduler (i.e. when initializing the UI,
//! from a callback, or from a scheduled task). Only one modal can exist at any time; if a modal is
//! opened when another one already exists, the existing one is dismissed first. The ID of a modal
//! will *always* be 'modal'. The full path of a modal will *always* be '/root/modal'. Any widgets
//! defined inside of the template layout will have their own IDs, e.g. '/root/modal/my_popup_label'.
//!
//! The modal is spawned in the root layout, so size hints and position hints will size and position
//! the modal relative to the root layout. Modals can be dismissed from the scheduler. If you want
//! a button in your modal that dismisses it, you need to bind a callback to the button in the modal
//! that calls the dismiss_modal method of the scheduler. Modals can be dragged across the screen by
//! the user (as long as they click on an empty part of the modal); if you want to disable this
//! behavior set can_drag to false on the layout template used to spawn the modal. Lastly, when a
//! modal is open only widgets in the modal can be selected or clicked; events do not reach the main
//! UI when a modal is open.
//!
//! Let's look at an example. We want to create a popup that appears when we click a button. The
//! popup should be half the size of the terminal and appear in the center. It should have some
//! text and a dismiss button that allows the user to close the popup.
//!
//! First, we define the layout template in an .ez file; we will use this to spawn the popup later.
//! We'll also create a button in the main UI, which will spawn the popup:
//! ```
//! - Layout:
//!     mode: box
//!     - Button:
//!         id: create_button
//!         text: Create popup
//!
//! - <MyPopup@Layout>:
//!     mode: box
//!     orientation: vertical
//!     size_hint: 0.5, 0.5
//!     pos_hint: center, middle
//!     - Label:
//!         text: This is my popup!
//!         size_hint_y: 0.8
//!     - Button:
//!         id: dismiss_button
//!         text: Close popup
//!         size_hint_y: 0.2
//! ```
//! We now have a template we can use to spawn the popup. Note that you can only use Layout
//! templates to spawn modals, widget templates won't work. Now we'll callbacks to create and
//! dismiss the popup:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let dismiss_popup_callback = |context: Context| {
//!     context.scheduler.dismiss_modal(context.state_tree);
//! };
//!
//! let create_popup_callback = |context: Context| {
//!     context.scheduler.open_modal("MyPopup", context.state_tree);
//!     let callback_config = CallbackConfig::from_on_press(Box::new(create_popup_callback));
//!     scheduler.update_callback_config("dismiss_button", callback_config);
//! };
//!
//! let callback_config = CallbackConfig::from_on_press(Box::new(create_popup_callback));
//! scheduler.update_callback_config("create_button", callback_config);
//!
//! ```
//! Now when the create button is pressed, the popup will open. When the dismiss button is pressed
//! from the modal, it will close again. Note that we referred to the template name when opening
//! the popup; this is separate from any ID, it comes from the line: "- <MyPopup@Layout>:".
//! Note also that we bind the dismiss callback to the modal button inside of the callback where we
//! spawn the modal. We cannot bind the dismiss callback when initializing the UI, because the modal
//! does not exist at that time. It only exists after the "open_modal" method of the scheduler is
//! called.
//!
//!
//! <a name="scheduler_programmatic_widgets"></a>
//! ### 1.3.7 Managing widgets programmatically
//!
//! <a name="scheduler_programmatic_widgets_create"></a>
//! #### 1.3.7.1 Creating widgets from code
//!
//! The static parts of your UI are created from the .ez files. In some cases however you need to
//! create widgets dynamically. Maybe you are retrieving records from a database and need to display
//! them. They will be retrieved at runtime and so cannot be known in advance (and even if they could,
//! it would be too much work to put them all in the .ez files). In cases like this you could create
//! widgets from code.
//!
//! Creating a widget from code is a 3-step process:
//! 1. Call 'prepare_create_widget' to get the new widget object and widget state(s);
//! 2. Optionally modify the widget states;
//! 3. Call 'create_widget' to create the widgets.
//!
//! Once you've called the scheduler.create_widget method (example below), the
//! widgets will be added to the UI on the next frame.
//!
//! You can spawn any kind of layout or widget from code, including templates. In fact, creating
//! widgets from templates is usually the best way to do it. Let's use the SQL record example we used
//! above: we will create a layout template that can display an entire SQL record. Then we'll
//! iterate over SQL records from code and create widgets for them. We'll also create a UI that
//! can display the sql record widgets. First the .ez file:
//! ```
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - Label:
//!         text: Retrieved SQL records:
//!         auto_scale: true, true
//!     - Layout:
//!         id: sql_records_layout
//!         mode: box
//!         orientation: vertical
//!
//! - <SqlRecord@Layout>:
//!     mode: box
//!     orientation: horizontal
//!     - Label:
//!         id: record_id
//!         auto_scale: true, true
//!     - Label:
//!         id: record_name
//!         auto_scale: true, true
//!     - Label:
//!         id: record_date
//!         auto_scale: true, true
//! ```
//! We now have a main UI that can hold our records. We also have a Layout template that can be
//! spawned to display a record. Let's now go to the code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let sql_records = get_sql_records();
//! for (i, sql_record) in sql_records.iter().enumerate() {
//!
//!     let template_name = "SqlRecord";
//!     let parent_id = "sql_records_layout";
//!     let new_id = format!("record_{}", i).as_str();
//!
//!     let (new_widget, new_states) =
//!             scheduler.prepare_create_widget(template_name, new_id, parent_id, &mut state_tree);
//!
//!     new_states.get("record_id").as_label_mut().set_text(sql_record.id);
//!     new_states.get("record_name").as_label_mut().set_text(sql_record.name);
//!     new_states.get("record_date").as_label_mut().set_text(sql_record.date);
//!
//!     scheduler.create_widget(new_widget, new_states, &mut state_tree);
//!
//! }
//! run(root_widget, state_tree, scheduler);
//! ```
//! We saw the process in action above: get the new widget object and states, alter the states,
//! then create the widget. When calling 'prepare_create_widget' we need a template or widget name
//! (like 'Layout', 'Label' or 'MyTemplate'), an ID (the new widget will receive this ID) and a
//! parent ID (the new widget will be added to this parent layout). In return we get the new
//! widget object and a state tree which contains the state(s) of the new widget. This gives you
//! a change to programmatically alter the states before the widget is created (in our example,
//! loading the SQL results into the labels). We then use the widget object and widget states to
//! actually create the widget.
//!
//! <a name="scheduler_programmatic_remove"></a>
//! #### 1.3.7.1 Removing widgets from code
//!
//! It's also possible to remove widgets from code. Simply use the 'remove_widget' method of the
//! scheduler and the path or ID of the widget you wish to remove. If you use an ID, it must be
//! globally unique:
//! ```
//! scheduler.remove_widget("/root/layout/widget");
//! ```
//! The widget will be removed on the next frame after you call remove_widget.
//!
//!
//! <a name="scheduler_updating"></a>
//! ### 1.3.8 Updating widgets
//!
//! <a name="scheduler_updating_individual"></a>
//! #### 1.3.8.1 Updating individual widgets
//!
//! If you change a widget state through code, that widget is not updated automatically. Usually you
//! want to call the 'update' method from the widget state; for example if we're in a callback:
//! ```
//! use ez_term::*;
//! fn my_callback(context: Context) {
//!     let label_state = context.state_tree.get_mut("my_label").to_label_mut();
//!     label_state.set_text("new_text".to_string());
//!     label_state.update(context.scheduler);
//! }
//! ```
//! The update method is in fact a convenience that calls "scheduler.update_widget". This scheduler
//! method takes a full path parameter (IDs cannot be used here). It is therefore almost always more
//! convenient to call 'update' on the widget state.
//!
//! <a name="scheduler_updating_global"></a>
//! #### 1.3.8.2 Global update (force redraw)
//!
//! It is also possible to call a global screen update. In this case, all widgets, starting from the
//! root layout, will be redrawn. For performance reasons, only changed pixels will actually be
//! redrawn, but global updates will still be more costly than updating individual states. The
//! option is made available but should generally not be used.
//!
//! <a name="scheduler_updating_threaded"></a>
//! #### 1.3.8.2 Custom properties and threads
//!
//! There are two cases where you do not need to manually update a state. If you bind a widget
//! property to a custom property, and you change the value of the custom property, the widgets
//! bound to it will update automatically.
//!
//! Also, if you manipulate the state tree from a background thread, any state that changes will
//! trigger a widget update automatically (because you do not have access to the scheduler from
//! threads, as its not thread-safe).
//!
//! <a name="scheduler_selection"></a>
//! ### 1.3.9 Managing widget selection
//!
//! Widgets can be selected by the user by mouse hover and up/down arrow keyboard keys (as long as
//! the widget has the selection_order property set). Widgets can only be selected if they're
//! selectable in general; selectable widgets are:
//! - Layout (when scrolling or when tabbed)
//! - Button
//! - Checkbox
//! - Radio button
//! - Dropdown
//! - Slider
//! - Text input (selectable by keyboard up/down or mouse click, not hover)
//!
//! Only one selection can be active at any time. You can control selection from code as well,
//! using scheduler.set_selected_widget and scheduler.deselect_widget:
//!
//!
//! <a name="scheduler_selection_selecting"></a>
//! #### 1.3.9.1 Selecting
//!
//! A widget can be selected through code. You have to pass the widget path or ID (must be globally
//! unique), and an optional mouse_pos parameter. As selection can occur through the mouse, you have
//! the option of passing custom coordinates, or just passing None; let's select a text input from
//! code on the first character (position (1, 1):
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.set_selected_widget("my_text_input", Some(Coordinates::new(1, 1)));
//!
//! run(root_widget, state_tree, scheduler)
//! ```
//!
//! If we don't care about coordinates it could look like this:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.set_selected_widget("my_text_input", None);
//!
//! run(root_widget, state_tree, scheduler)
//! ```
//!
//! <a name="scheduler_selection_selecting"></a>
//! #### 1.3.9.2 Deselection
//!
//! To deselect a widget from code we do not need any parameters. Only one widget can be selected at
//! any time, so deselect simply deselects any active selection, or does nothing if there is no
//! selection. It's always safe to call this method. Let's deselect:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.deselect_widget();
//!
//! run(root_widget, state_tree, scheduler)
//! ```
//!
//! <a name="keybindings"></a>
//! ## 1.4 Global (key)bindings
//!
//! EzTerm UIs come with some global built-in bindings, which allow the user to navigate the UI.
//! There are:
//!
//! **Keyboard escape key:**
//! Exits the process.
//!
//!
//! **Keyboard up key:**
//! Selects the previous widget according to the 'selection_order' properties of widgets. So if a
//! widget with order 4 is selected and a widget with order 3 exists, 3 will be selected next.
//!
//!
//! **Keyboard down key:**
//! Selects the next widget according to the 'selection_order' properties of widgets. So if a
//! widget with order 4 is selected and a widget with order 5 exists, 5 will be selected next.
//!
//!
//! **Keyboard enter key:**
//! Calls the on_keyboard_enter callback on the currently selected widget (if any). Certain widgets
//! have a standard implementation for this callback; for example, checkboxes toggle on/off.
//!
//!
//! **Keyboard right key:**
//! If a horizontal scroll bar is selected, this scrolls to the right. If a tab header is selected,
//! this selects the tab header to the right.
//!
//!
//! **Keyboard left key:**
//! If a horizontal scroll bar is selected, this scrolls to the left. If a tab header is selected,
//! this selects the tab header to the left.
//!
//!
//! **Keyboard PageUp key:**
//! If a vertical scroll bar is selected, this scrolls to up.
//!
//!
//! **Keyboard PageDown key:**
//! If a vertical scroll bar is selected, this scrolls to down.
//!
//!
//! **Keyboard CTRL+V key:**
//! Pasts content into the terminal.
//!
//!
//! **Mouse scroll up:**
//! If a layout has a vertical scrollbar, this will scroll it up.
//! If a layout has a horizontal scrollbar, this will scroll it left.
//!
//!
//! **Mouse scroll down:**
//! If a layout has a vertical scrollbar, this will scroll it down.
//! If a layout has a horizontal scrollbar, this will scroll it right.
//!
//!
//! **Mouse drag:**
//! Layout scrollbars can be dragged up/down/left/right. Modals can by default be dragged around the
//! screen. Sliders can be dragged left/right.
//!
//!
//! **Mouse hover:**
//! Mouse hovering selectable widgets (except text inputs) will select them, to make it clear to the
//! user that something can be interacted with.
//!
//!
//! **Mouse left click:**
//! All interactive widgets implement an on_left_mouse_click callback with default behavior.
//!
//!
//! <a name="reference"></a>
//! # 2. Reference
//!
//! This section will provide the details on every widget: properties, available callbacks, and
//! default callback implementations. It will also have a reference for all the methods of the
//! scheduler.
//!
//! <a name="reference_common"></a>
//! ## 2.1 Common properties reference
//!
//! Here we'll look at the properties that are available on every available widget and layout.
//! These properties, along with the specific properties mentioned in the specific widget reference,
//! form a complete reference of properties.
//!
//! <a name="reference_common_x"></a>
//! ##### x
//!
//! The horizontal position of the widget going left to right (so 0 is the left edge of a layout).
//! Only works if the layout the object is placed in has the layout_mode property set to float.
//! Unless you are explicitly doing manual positioning, you usually do not set this property.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Any usize value that falls within the width of the parent layout.
//!
//! **Default value:**
//!
//! 0
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: float
//!     - Label:
//!         x: 5
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_x(5);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_y"></a>
//! ##### y
//!
//! The vertical position of the widget going top to bottom (so 0 is the top edge of a layout).
//! Only works if the layout the object is placed in has the layout_mode property set to float.
//! Unless you are explicitly doing manual positioning, you usually do not set this property.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Any usize value that falls within the height of the parent layout.
//!
//! **Default value:**
//!
//! 0
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: float
//!     - Label:
//!         y: 5
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_y(5);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_pos"></a>
//! ##### pos
//!
//! This is an EzLang convenience that allows you to set 'x' and 'y' at the same time (in that order).
//! See the individual properties for more info.
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: float
//!     - Label:
//!         pos: 5, 3
//! ```
//!
//! <a name="reference_common_width"></a>
//! ##### width
//!
//! The absolute width of the widget. Only works if the 'size_hint_x' property is set to None
//! (because relative sizing is the default). Unless you are explicitly doing manual sizing,
//! you usually do not set this property.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Any usize value that fits within the width of the parent.
//!
//! **Default value:**
//!
//! 0
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         width: 20
//!         size_hint_x: none
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_width(20);
//! state.set_size_hint_x(None);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_height"></a>
//! ##### height
//!
//! The absolute height of the widget. Only works if the 'size_hint_y' property is set to None
//! (because relative sizing is the default). Unless you are explicitly doing manual sizing,
//! you usually do not set this property.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Any usize value that fits within the height of the parent.
//!
//! **Default value:**
//!
//! 0
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         height: 20
//!         size_hint_y: none
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_height(20);
//! state.set_size_hint_y(None);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_size"></a>
//! ##### size
//!
//! This is an EzLang convenience that allows you to set 'width' and 'height' at the same time (
//! in that order). See the individual properties for more info.
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         size: 20, 10
//!         size_hint: none, none
//! ```
//!
//! <a name="reference_common_size_hint_x"></a>
//! ##### size_hint_x
//!
//! Horizontal size hint, which can be none, or a value between 0 and 1. If a value is set, the
//! width of the widget will be set relative to the width of the parent; so 0.5 makes the widget
//! half as wide as the parent, 1.0 equal width to the parent, etc. If set to none, some other
//! method of sizing should be set; such as absolute size (width property) or scaling
//! (auto_scale_width property).
//!
//! This is the default method of sizing, meaning this is set to 1.0 by default, making the widget
//! as large as the parent. If there are multiple widgets in a layout, all with size_hint_x 1.0,
//! their size hint will change to 1 divided by number of widgets (i.e. all widgets will be equal
//! size).
//!
//! **Property type:**
//!
//! SizeHint (Option<f64>)
//!
//! **Possible values:**
//!
//! - f64 between 0.0 and 1.0
//! - None
//!
//! **Default value:**
//!
//! 1.0
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         size_hint_x: 0.5
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_size_hint_x(Some(0.5));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_size_hint_y"></a>
//! ##### size_hint_y
//!
//! Vertical size hint, which can be none, or a value between 0 and 1. If a value is set, the
//! height of the widget will be set relative to the height of the parent; so 0.5 makes the widget
//! half as high as the parent, 1.0 equal height to the parent, etc. If set to none, some other
//! method of sizing should be set; such as absolute size (height property) or scaling
//! (auto_scale_height property).
//!
//! This is the default method of sizing, meaning this is set to 1.0 by default, making the widget
//! as large as the parent. If there are multiple widgets in a layout, all with size_hint_y 1.0,
//! their size hint will change to 1 divided by number of widgets (i.e. all widgets will be equal
//! size).
//!
//! **Property type:**
//!
//! SizeHint (Option<f64>)
//!
//! **Possible values:**
//!
//! - f64 between 0.0 and 1.0
//! - None
//!
//! **Default value:**
//!
//! 1.0
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         size_hint_y: 0.5
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_size_hint_y(Some(0.5));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_size_hint"></a>
//! ##### size_hint
//!
//! This is an EzLang convenience that allows you to set 'size_hint_x' and 'size_hint_y' at the
//! same time (in that order). See the individual properties for more info.
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         size_hint: 0.5, none
//! ```
//!
//! <a name="reference_common_pos_hint_x"></a>
//! ##### pos_hint_x
//!
//! Horizontal position hint; works only when the parent layout is set to float mode. Positions
//! widget horizontally relative to the parent, according to keywords (left/center/right). Also
//! takes an optional value between 0 and 1, which allows you to adjust the keyword position; e.g.
//! 'right: 0.9' is 90% of the way towards the right edge of the parent layout. You cannot use the
//! optional value with the 'left' keyword, as it represents position 0. By default the adjustment
//! value is set to 1.0, so just the keyword is used.
//!
//! **Property type:**
//!
//! HorizontalPositionHint (Option<(HorizontalAlignment, f64>)
//!
//! **Possible values:**
//!
//! Possible keyword values:
//!     - left
//!     - center
//!     - right
//! Optional adjustment value:
//!     - f64 between 0.0 and 1.0
//!
//! **Default value:**
//!
//! None
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: float
//!     - Label:
//!         pos_hint_x: right: 0.75
//! ```
//! ```
//! - Layout:
//!     mode: float
//!     - Label:
//!         pos_hint_x: right
//! ```
//! ```
//! - Layout:
//!     mode: float
//!     - Label:
//!         pos_hint_x: none
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_pos_hint_x(Some((HorizontalAlignment::Right, 0.75)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_pos_hint_y"></a>
//! ##### pos_hint_y
//!
//! Vertical position hint; works only when the parent layout is set to float mode. Positions
//! widget vertically relative to the parent, according to keywords (top/middle/bottom). Also
//! takes an optional value between 0 and 1, which allows you to adjust the keyword position; e.g.
//! 'bottom: 0.9' is 90% of the way towards the bottom edge of the parent layout. You cannot use the
//! optional value with the 'top' keyword, as it represents position 0. By default the adjustment
//! value is set to 1.0, so just the keyword is used.
//!
//! **Property type:**
//!
//! VerticalPositionHint (Option<(VerticalAlignment, f64>)
//!
//! **Possible values:**
//!
//! Possible keyword values:
//!     - top
//!     - middle
//!     - bottom
//! Optional adjustment value:
//!     - f64 between 0.0 and 1.0
//!
//! **Default value:**
//!
//! None
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: float
//!     - Label:
//!         pos_hint_y: bottom: 0.75
//! ```
//! ```
//! - Layout:
//!     mode: float
//!     - Label:
//!         pos_hint_y: bottom
//! ```
//! ```
//! - Layout:
//!     mode: float
//!     - Label:
//!         pos_hint_y: none
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_pos_hint_y(Some((VerticalAlignment::Bottom, 0.75)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_pos_hint"></a>
//! ##### pos_hint
//!
//! This is an EzLang convenience that allows you to set 'pos_hint_x' and 'pos_hint_y' at the
//! same time (in that order). See the individual properties for more info.
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         pos_hint: right: 0.75, bottom: 0.75
//! ```
//!
//! <a name="reference_common_auto_scale_width"></a>
//! ##### auto_scale_width
//!
//! This property automatically scales the width of the widget to its content. This property has
//! priority over the size_hint_x and width properties. Initially, the widget is given infinite
//! width to create its content. Then, when the content is created, the widget width is set to the
//! content width.
//!
//! **Property type:**
//!
//! bool
//!
//! **Possible values:**
//!
//! - true
//! - false
//!
//! **Default value:**
//!
//! false
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         auto_scale_width: true
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_auto_scale_width(true);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_auto_scale_height"></a>
//! ##### auto_scale_height
//!
//! This property automatically scales the height of the widget to its content. This property has
//! priority over the size_hint_y and height properties. Initially, the widget is given infinite
//! height to create its content. Then, when the content is created, the widget height is set to the
//! content height.
//!
//! **Property type:**
//!
//! bool
//!
//! **Possible values:**
//!
//! - true
//! - false
//!
//! **Default value:**
//!
//! false
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         auto_scale_height: true
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_auto_scale_height(true);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_auto_scale"></a>
//! ##### auto_scale
//!
//! This is an EzLang convenience that allows you to set 'auto_scale_width' and 'auto_scale_height'
//! at the same time (in that order). See the individual properties for more info.
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         auto_scale: true, true
//! ```
//!
//! <a name="reference_common_padding_top"></a>
//! ##### padding_top
//!
//! This property determines if, and how many, empty pixels should be above the widget. This allows
//! you to create space between widgets, or between a widget and the edge of a layout.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! any usize value
//!
//! **Default value:**
//!
//! 0
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         padding_top: 2
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_padding_top(2);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_padding_bottom"></a>
//! ##### padding_bottom
//!
//! This property determines if, and how many, empty pixels should be below the widget. This allows
//! you to create space between widgets, or between a widget and the edge of a layout.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! any usize value
//!
//! **Default value:**
//!
//! 0
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         padding_bottom: 2
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_padding_bottom(2);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_padding_left"></a>
//! ##### padding_left
//!
//! This property determines if, and how many, empty pixels should be added to the left of the
//! widget. This allows you to create space between widgets, or between a widget and the edge of a
//! layout.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! any usize value
//!
//! **Default value:**
//!
//! 0
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         padding_left: 2
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_padding_left(2);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_padding_right"></a>
//! ##### padding_right
//!
//! This property determines if, and how many, empty pixels should be added to the right of the
//! widget. This allows you to create space between widgets, or between a widget and the edge of a
//! layout.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! any usize value
//!
//! **Default value:**
//!
//! 0
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         padding_right: 2
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_padding_top(2);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_common_padding_x"></a>
//! ##### padding_x
//!
//! This is an EzLang convenience that allows you to set 'pos_left' and 'padding_right',
//! at the same time (in that order). See the individual properties for more info.
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         padding_x: 2, 2
//! ```
//!
//! <a name="reference_common_padding_y"></a>
//! ##### padding_y
//!
//! This is an EzLang convenience that allows you to set 'pos_top' and 'padding_bottom',
//! at the same time (in that order). See the individual properties for more info.
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         padding_y: 2, 2
//! ```
//!
//! <a name="reference_common_padding"></a>
//! ##### padding
//!
//! This is an EzLang convenience that allows you to set 'pos_top', 'padding_bottom',
//! 'padding_left' and 'padding_right' at the same time (in that order). See the individual
//! properties for more info.
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         padding: 2, 2, 2, 2
//! ```
//!
//! <a name="reference_common_halign"></a>
//! ##### halign
//!
//! This property aligns the widget horizontally; this only works if the parent layout is in box
//! mode with orientation vertical, or if the parent layout is in table mode.
//!
//! **Property type:**
//!
//! HorizontalAlignment
//!
//! **Possible values:**
//!
//! - left
//! - center
//! - right
//!
//! **Default value:**
//!
//! left
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - Label:
//!         halign: right
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_halign(HorizontalAlignment::Right);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_valign"></a>
//! ##### valign
//!
//! This property aligns the widget vertically; this only works if the parent layout is in box
//! mode with orientation horizontal, or if the parent layout is in table mode.
//!
//! **Property type:**
//!
//! HorizontalAlignment
//!
//! **Possible values:**
//!
//! - top
//! - middle
//! - bottom
//!
//! **Default value:**
//!
//! top
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: box
//!     orientation: horizontal
//!     - Label:
//!         valign: bottom
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_valign(VerticalAlignment::Bottom);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_disabled"></a>
//! ##### disabled
//!
//! If set to true, disables the widget for all callbacks. E.g., button cannot be pressed, layout
//! cannot scroll, etc.
//!
//! **Property type:**
//!
//! bool
//!
//! **Possible values:**
//!
//! - true
//! - false
//!
//! **Default value:**
//!
//! false
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Button:
//!         disabled: true
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_button").as_layout_mut();
//!
//! state.set_disabled(true);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_selection_order"></a>
//! ##### selection_order
//!
//! The global order in which this widget will be selected by keyboard arrows up/down. Pressing
//! keyboard down increases the order (1>2>3) and keyboard up the opposite (3>2>1). These number
//! do not have to be sequential, so 1>5>10 works as well. The number 0 means no order (cannot
//! select).
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! 0 (cannot select), or any usize number above (can select)
//!
//! **Default value:**
//!
//! 0
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Button:
//!         selection_order: 10
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_selection_order(10);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_border"></a>
//! ##### border
//!
//! If set to true, draws a border around the widget. Uses border_fg_color and border_bg_color for
//! coloring. You can set the symbols using horizontal_symbol, vertical_symbol, top_left_symbol,
//! top_right_symbol, bottom_left_symbol and bottom_right_symbol.
//!
//! **Property type:**
//!
//! bool
//!
//! **Possible values:**
//!
//! - true
//! - false
//!
//! **Default value:**
//!
//! false
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         border: true
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! state.set_border(true);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_horizontal_symbol"></a>
//! ##### horizontal_symbol
//!
//! Sets the horizontal symbol used to draw a border; only works when border is set to true.
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any String 1 character long
//!
//! **Default value:**
//!
//! ─
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         border: true
//!         horizontal_symbol: #
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! // Border properties are grouped in a BorderConfig object, which we must access first:
//! state.get_border_config().set_horizontal_symbol("#");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_vertical_symbol"></a>
//! ##### vertical_symbol
//!
//! Sets the vertical symbol used to draw a border; only works when border is set to true.
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any String 1 character long
//!
//! **Default value:**
//!
//! │
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         border: true
//!         vertical_symbol: #
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! // Border properties are grouped in a BorderConfig object, which we must access first:
//! state.get_border_config().set_vertical_symbol("#");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_top_left_symbol"></a>
//! ##### top_left_symbol
//!
//! Sets the top left symbol used to draw a border; only works when border is set to true.
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any String 1 character long
//!
//! **Default value:**
//!
//! ┌
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         border: true
//!         top_left_symbol: #
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! // Border properties are grouped in a BorderConfig object, which we must access first:
//! state.get_border_config().set_top_left_symbol("#");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_top_right_symbol"></a>
//! ##### top_right_symbol
//!
//! Sets the top right symbol used to draw a border; only works when border is set to true.
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any String 1 character long
//!
//! **Default value:**
//!
//! ┐
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         border: true
//!         top_right_symbol: #
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! // Border properties are grouped in a BorderConfig object, which we must access first:
//! state.get_border_config().set_top_right_symbol("#");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_bottom_left_symbol"></a>
//! ##### bottom_left_symbol
//!
//! Sets the bottom left symbol used to draw a border; only works when border is set to true.
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any String 1 character long
//!
//! **Default value:**
//!
//! └
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         border: true
//!         bottom_left_symbol: #
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! // Border properties are grouped in a BorderConfig object, which we must access first:
//! state.get_border_config().set_bottom_left_symbol("#");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_bottom_right_symbol"></a>
//! ##### bottom_right_symbol
//!
//! Sets the bottom right symbol used to draw a border; only works when border is set to true.
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any String 1 character long
//!
//! **Default value:**
//!
//! ┘
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         border: true
//!         bottom_right_symbol: #
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! // Border properties are grouped in a BorderConfig object, which we must access first:
//! state.get_border_config().set_bottom_right_symbol("#");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_fg_color"></a>
//! ##### fg_color
//!
//! Foreground color used for the widget, i.e. the color used for the symbol of the pixel.
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! white
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         fg_color: red
//! ```
//! ```
//! - Layout:
//!     - Label:
//!         fg_color: 255, 0, 0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_fg_color(Color::Red);
//! state.get_color_config_mut().set_fg_color(Color::from((255, 0, 0)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_bg_color"></a>
//! ##### bg_color
//!
//! Background color used for the widget, i.e. the empty space around the symbol of the pixel.
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! Black
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         bg_color: red
//! ```
//! ```
//! - Layout:
//!     - Label:
//!         bg_color: 255, 0, 0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_bg_color(Color::Red);
//! state.get_color_config_mut().set_bg_color(Color::from((255, 0, 0)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_selection_fg_color"></a>
//! ##### selection_fg_color
//!
//! Foreground color used for the widget when it is selected (through keyboard up/down, mouse hover,
//! or scheduler method).
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! yellow
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         selection_fg_color: red
//! ```
//! ```
//! - Layout:
//!     - Label:
//!         selection_fg_color: 255, 0, 0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_selection_fg_color(Color::Red);
//! state.get_color_config_mut().set_selection_fg_color(Color::from((255, 0, 0)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_selection_bg_color"></a>
//! ##### selection_bg_color
//!
//! Background color used for the widget when it is selected (through keyboard up/down, mouse hover,
//! or scheduler method).
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! blue
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         selection_bg_color: red
//! ```
//! ```
//! - Layout:
//!     - Label:
//!         selection_bg_color: 255, 0, 0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_selection_bg_color(Color::Red);
//! state.get_color_config_mut().set_selection_bg_color(Color::from((255, 0, 0)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_disabled_fg_color"></a>
//! ##### disabled_fg_color
//!
//! Foreground color used for the widget when it is disabled.
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! white
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         disabled_fg_color: red
//! ```
//! ```
//! - Layout:
//!     - Label:
//!         disabled_fg_color: 255, 0, 0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_disabled_fg_color(Color::Red);
//! state.get_color_config_mut().set_disabled_fg_color(Color::from((255, 0, 0)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_disabled_bg_color"></a>
//! ##### disabled_bg_color
//!
//! Background color used for the widget when it is disabled.
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! white
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         disabled_bg_color: red
//! ```
//! ```
//! - Layout:
//!     - Label:
//!         disabled_bg_color: 255, 0, 0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_disabled_bg_color(Color::Red);
//! state.get_color_config_mut().set_disabled_bg_color(Color::from((255, 0, 0)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_common_border_fg_color"></a>
//! ##### border_fg_color
//!
//! Foreground color used for the border of a widget if enabled.
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! white
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         border_fg_color: red
//!         border: true
//! ```
//! ```
//! - Layout:
//!     - Label:
//!         border_fg_color: 255, 0, 0
//!         border: true
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_border_fg_color(Color::Red);
//! state.get_color_config_mut().set_border_fg_color(Color::from((255, 0, 0)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_common_border_bg_color"></a>
//! ##### border_bg_color
//!
//! Background color used for the border of a widget if enabled.
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! white
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         border_bg_color: red
//!         border: true
//! ```
//! ```
//! - Layout:
//!     - Label:
//!         border_bg_color: 255, 0, 0
//!         border: true
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_border_bg_color(Color::Red);
//! state.get_color_config_mut().set_border_bg_color(Color::from((255, 0, 0)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_layout"></a>
//! ## 2.2 Layout:
//!
//! The layout is needed to place widgets in the UI. It has different modes to provide control over
//! how to place the widgets. Some properties are specific to one or more modes.
//!
//! <a name="reference_layout_properties"></a>
//! #### 2.2.1 Properties
//!
//! <a name="reference_layout_properties_mode"></a>
//! ##### mode
//!
//! Sets the layout mode, which controls how to place child widgets on the screen. See the
//! tutorial for a full explanation of each mode.
//!
//! **Property type:**
//!
//! LayoutMode
//!
//! **Possible values:**
//! - box
//! - stack
//! - table
//! - float
//! - tab
//! - screen
//!
//! **Default value:**
//!
//! box
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: box
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! state.set_mode(LayoutMode::Box);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_orientation"></a>
//! ##### orientation
//!
//! Sets the layout orientation, which controls how some layout modes behave. See the
//! tutorial for a full explanation of each mode, which includes the available orientations. In
//! short: Box mode support horizontal/vertical and stack/table mode supports the 8 bidirectional
//! orientations (such as tb-lr).
//!
//! **Property type:**
//!
//! LayoutOrientation
//!
//! **Possible values:**
//! - horizontal
//! - vertical
//! - tb-lr
//! - tb-rl
//! - bt-lr
//! - bt-rl
//! - lr-tb
//! - lr-bt
//! - rl-tb
//! - rl-bt
//!
//! **Default value:**
//!
//! For box mode default is 'horizontal'. For stack and table mode default is: 'tb-lr'.
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     orientation: horizontal
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! state.set_orientation(LayoutOrientation::Horizontal);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_active_screen"></a>
//! ##### active_screen
//!
//! Only works when the layout mode property is set to 'screen'. This property determines which
//! child layout (i.e. screen) is shown to the user. By default the first screen is shown, so it is
//! not required to set this property. If you want to set this property, use the ID of the child
//! layout you want to show.
//!
//! **Property type:**
//!
//! String
//!
//! **Default value:**
//!
//! Empty string. When string is empty, active_screen will initially be set to first child layout.
//!
//! **Possible values:**
//!
//! The ID property of any of the child layouts (i.e. screens).
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: screen
//!     active_screen: my_screen_2
//!     - Layout:
//!         id: my_screen_1
//!     - Layout:
//!         id: my_screen_2
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! state.set_active_screen("my_screen_2");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_active_tab"></a>
//! ##### active_tab
//!
//! Only works when the layout mode property is set to 'tab'. This property determines which
//! child layout (i.e. tab) is shown to the user. By default the first tab is shown, so it is
//! not required to set this property. If you want to set this property, use the ID of the child
//! layout you want to show.
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! The ID property of any of the child layouts (i.e. tabs).
//!
//! **Default value:**
//!
//! Empty string. When string is empty, active_tab is initially set to the first child layout.
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: screen
//!     active_tab: my_tab_2
//!     - Layout:
//!         id: my_tab_1
//!     - Layout:
//!         id: my_tab_2
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! state.set_active_tab("my_tab_2");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_tab_name"></a>
//! ##### tab_name
//!
//! Only works when the *parent* layout mode property is set to 'tab'. This property determines the
//! name displayed in the tab header, which when clicked, makes this layout the active tab in
//! the parent. By default, this is set to the ID property, so it is not required to set.
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any string. You might want to keep it somewhat short, otherwise the tab header buttons will be
//! very wide.
//!
//! **Default value:**
//!
//! Empty string. When string is empty, ID of the layout is used as tab header name.
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: tab
//!     - Layout:
//!         id: my_tab_1
//!         tab_name: My tab
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! state.set_tab_name("My tab");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_tab_header_fg_color"></a>
//! ##### tab_header_fg_color
//!
//! Only works when the layout is in tab mode. Sets the fg_color of the tab header button. In other
//! words, the color of the text on the tab header.
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! white
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: tab
//!     tab_header_fg_color: red
//! ```
//! ```
//! - Layout:
//!     mode: tab
//!     tab_header_fg_color: 255, 0, 0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_tab_header_fg_color(Color::Red);
//! state.get_color_config_mut().set_tab_header_fg_color(Color::from((255, 0, 0)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_tab_header_bg_color"></a>
//! ##### tab_header_bg_color
//!
//! Only works when the layout is in tab mode. Sets the background color of the tab header button.
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! black
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: tab
//!     tab_header_bg_color: blue
//! ```
//! ```
//! - Layout:
//!     mode: tab
//!     tab_header_bg_color: 0, 0, 255
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_tab_header_bg_color(Color::Blue);
//! state.get_color_config_mut().set_tab_header_bg_color(Color::from((0, 0, 255)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_tab_header_border_fg_color"></a>
//! ##### tab_header_border_fg_color
//!
//! Only works when the layout is in tab mode. Sets the foreground color of the tab header button
//! borders.
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! white
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: tab
//!     tab_header_border_fg_color: blue
//! ```
//! ```
//! - Layout:
//!     mode: tab
//!     tab_header_border_fg_color: 0, 0, 255
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_tab_header_fg_color(Color::Blue);
//! state.get_color_config_mut().set_tab_header_fg_color(Color::from((0, 0, 255)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_tab_header_border_bg_color"></a>
//! ##### tab_header_border_bg_color
//!
//! Only works when the layout is in tab mode. Sets the background color of the tab header button
//! borders.
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! black
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: tab
//!     tab_header_border_bg_color: blue
//! ```
//! ```
//! - Layout:
//!     mode: tab
//!     tab_header_border_bg_color: 0, 0, 255
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_tab_header_bg_color(Color::Blue);
//! state.get_color_config_mut().set_tab_header_bg_color(Color::from((0, 0, 255)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_tab_header_active_fg_color"></a>
//! ##### tab_header_active_fg_color
//!
//! Foreground color used for the tab header button that is active (meaning the associated tab is
//! currently displayed).
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! white
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         tab_header_active_fg_color: red
//! ```
//! ```
//! - Layout:
//!     - Label:
//!         tab_header_active_fg_color: 255, 0, 0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_tab_header_active_fg_color(Color::Red);
//! state.get_color_config_mut().set_tab_header_active_fg_color(Color::from((255, 0, 0)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_tab_header_active_bg_color"></a>
//! ##### tab_header_active_bg_color
//!
//! Background color used for the tab header button that is active (meaning the associated tab is
//! currently displayed).
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! black
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         tab_header_active_bg_color: red
//! ```
//! ```
//! - Layout:
//!     - Label:
//!         tab_header_active_bg_color: 255, 0, 0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_tab_header_active_bg_color(Color::Red);
//! state.get_color_config_mut().set_tab_header_active_bg_color(Color::from((255, 0, 0)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_layout_properties_can_drag"></a>
//! ##### can_drag
//!
//! Only works on a Layout template which is spawned as a modal (see tutorial). When can_drag is
//! true, the modal can be dragged across the screen.
//!
//! **Property type:**
//!
//! Bool
//!
//! **Possible values:**
//!
//! - true
//! - false
//!
//! **Default value:**
//!
//! true
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - <MyPopup@Layout>:
//!     can_drag: false
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.open_modal("MyPopup", &mut state_tree);
//! let state = state_tree.get_mut("modal").as_layout_mut();
//!
//! state.set_can_drag(false);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_fill"></a>
//! ##### fill
//!
//! A bool that determines whether fill is enabled. If enabled, all empty pixels in the layout will
//! be filled with a symbol. The symbol is determined by the property 'filler_symbol' (see property
//! below). The color of the filler is determined by properties 'filler_fg_color' and
//! 'filler_bg_color'.
//!
//! **Property type:**
//!
//! bool
//!
//! **Possible values:**
//!
//! - true
//! - false
//!
//! **Default value:**
//!
//! false
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     fill: true
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! state.set_fill(true);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_filler_symbol"></a>
//! ##### filler_symbol
//!
//! The symbol that will be used to fill empty pixels in the layout if the 'fill' property is set
//! to true.
//!
//! **Property type:**
//!
//! String (only 1st character is used)
//!
//! **Possible values:**
//!
//! Any String, but only first character is used. Unicode allowed.
//!
//! **Default value:**
//!
//! Empty string.
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     filler_symbol: ░
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! state.set_fill(true);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_filler_fg_color"></a>
//! ##### filler_fg_color
//!
//! Only works when the layout is in box, stack, table or float mode, and the fill property is set
//! to true. Determines the foreground color of the filler pixels (i.e. the color of the filler
//! symbol).
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! white
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     filler_fg_color: red
//! ```
//! ```
//! - Layout:
//!     filler_fg_color: 255, 0, 0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_filler_fg_color()Color::Blue);
//! state.get_color_config_mut().set_filler_fg_color(Color::from((0, 0, 255)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_filler_bg_color"></a>
//! ##### filler_bg_color
//!
//! Only works when the layout is in box, stack, table or float mode, and the fill property is set
//! to true. Determines the background color of the filler pixels.
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! black
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: tab
//!     filler_bg_color: red
//! ```
//! ```
//! - Layout:
//!     mode: tab
//!     filler_bg_color: 255, 0, 0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_filler_bg_color(Color::Blue);
//! state.get_color_config_mut().set_filler_bg_color(Color::from((0, 0, 255)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_view_size"></a>
//! ##### view_size
//!
//! Only works when the layout mode is box, float, stack or table. Determines the max amount of children
//! displayed at one time. Default is 0, which displays all children. If set to a value higher than
//! 0, the children displayed are determined by the 'view_page' property (see below).
//! If this layout has e.g. 30 children, view size is set to 10, and view_page is 2, then
//! children 10-20 are displayed.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Any usize.
//!
//! **Default value:**
//!
//! 0 (display all children).
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     view_size: 10
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! state.set_view_size(1);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_view_size"></a>
//! ##### view_page
//!
//! Only works when the layout mode is box, float, stack or table, and view_size is set to a value higher
//! than 0. Determines which children are displayed. If this layout has e.g. 30 children, view size
//! is set to 10, and view_page is 2, then children 10-20 are displayed.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Any usize higher than 0.
//!
//! **Default value:**
//!
//! 1 (first page).
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     view_page: 10
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! state.set_view_page(2);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_scroll_x"></a>
//! ##### scroll_x
//!
//! Only works when the layout mode is 'box', 'table', 'stack' or 'float'. Makes the width of the
//! layout infinite and adds a horizontal scrollbar to move through the content. In most cases the
//! easiest way to implement scrolling is to enable it on a box layout.
//!
//! **Property type:**
//!
//! bool
//!
//! **Possible values:**
//!
//! - true
//! - false
//!
//! **Default value:**
//!
//! false
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     scroll_x: true
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Scrolling properties are wrapping into a ScrollingConfig object, which we have to retrieve
//! // first:
//! state.get_scrolling_config_mut().set_scroll_x(true);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_scroll_y"></a>
//! ##### scroll_y
//!
//! Only works when the layout mode is 'box', 'table', 'stack' or 'float'. Makes the height of the
//! layout infinite and adds a vertical scrollbar to move through the content. In most cases the
//! easiest way to implement scrolling is to enable it on a box layout.
//!
//! **Property type:**
//!
//! bool
//!
//! **Possible values:**
//!
//! - true
//! - false
//!
//! **Default value:**
//!
//! false
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     scroll_y: true
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Scrolling properties are wrapping into a ScrollingConfig object, which we have to retrieve
//! // first:
//! state.get_scrolling_config_mut().set_scroll_y(true);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_scroll_start_x"></a>
//! ##### scroll_start_x
//!
//! Only works when the layout is scrolling horizontally. Determines the view of the scroll by a
//! value between 0 and 1. If the total content is 100 pixels wide, then a scroll_start_x of 0.5 will
//! show content starting from pixel 50. A value of 0 means the beginning of the content, a value
//! of 1 means show the tail end of the content. Use with EzLang to optionally set a custom initial
//! view for the scrollbar. Use in code to programmatically control the scrollbar.
//!
//! **Property type:**
//!
//! f64
//!
//! **Possible values:**
//!
//! F64 value between 0 and 1.
//!
//! **Default value:**
//!
//! 0.0
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     scroll_start_x: 1.0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Scrolling properties are wrapping into a ScrollingConfig object, which we have to retrieve
//! // first:
//! state.get_scrolling_config_mut().set_scroll_start_x(1.0);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_scroll_start_y"></a>
//! ##### scroll_start_y
//!
//! Only works when the layout is scrolling vertically. Determines the view of the scroll by a
//! value between 0 and 1. If the total content is 100 pixels high, then a scroll_start_y of 0.5 will
//! show content starting from pixel 50. A value of 0 means the beginning of the content, a value
//! of 1 means show the tail end of the content. Use with EzLang to optionally set a custom initial
//! view for the scrollbar. Use in code to programmatically control the scrollbar.
//!
//! **Property type:**
//!
//! f64
//!
//! **Possible values:**
//!
//! F64 value between 0 and 1.
//!
//! **Default value:**
//!
//! 0.0
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     scroll_start_y: 1.0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Scrolling properties are wrapping into a ScrollingConfig object, which we have to retrieve
//! // first:
//! state.get_scrolling_config_mut().set_scroll_start_y(1.0);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_rows"></a>
//! ##### rows
//!
//! Only works when the layout is in table mode. Fixes the amount of rows in the table. If you set
//! only the amount of rows, the amount of columns will grow to fit all the content. If you set both,
//! the table will be fixed in size. It is mandatory to set either rows or cols, because the table
//! needs to know if it should grow, and if so, in which direction. By default rows is 0 (will grow)
//! and cols is 4 (will not grow).
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Any usize number
//!
//! **Default value:**
//!
//! 0
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: table
//!     rows: 10
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_table_config_mut().set_rows(10);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_cols"></a>
//! ##### cols
//!
//! Only works when the layout is in table mode. Fixes the amount of columns in the table. If you set
//! only the amount of columns, the amount of rows will grow to fit all the content. If you set both,
//! the table will be fixed in size. It is mandatory to set either rows or cols, because the table
//! needs to know if it should grow, and if so, in which direction. By default rows is 0 (will grow)
//! and cols is 4 (will not grow).
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Any usize number
//!
//! **Default value:**
//!
//! 4
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: table
//!     cols: 3
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_table_config_mut().set_cols(3);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_col_default_width"></a>
//! ##### col_default_width
//!
//! Only works when the layout is in table mode. Sets the default width for columns, meaning that
//! columns will be at least that width, but can still grow to accommodate widgets wider than the
//! default width. Without setting this property, each individual column will be the width of its
//! widest widget. Setting this property is useful if you want all your columns to be at least a
//! certain width, even if all its widgets are smaller. Another use case is using this property in
//! combination wih 'force_default_col_width: true' (see property below). In this case, you are
//! hard coding each column to be exactly the col_default_width, without the ability to grow.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Any usize number
//!
//! **Default value:**
//!
//! 0 (each column grows to the widest widget).
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: table
//!     col_default_width: 20
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_table_config_mut().set_col_default_width(20);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_force_default_col_width"></a>
//! ##### force_default_col_width
//!
//! Only works when the layout is in table mode and the 'col_default_width' property is set to a
//! value higher than 0. Ensures that each column is *exactly* the col_default_width, without any
//! ability to grow.
//!
//! **Property type:**
//!
//! bool
//!
//! **Possible values:**
//!
//! - true
//! - false
//!
//! **Default value:**
//!
//! false
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: table
//!     col_default_width: 20
//!     force_default_col_width: true
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_table_config_mut().set_col_default_width(20);
//! state.get_table_config_mut().set_force_default_col_width(true);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_row_default_height"></a>
//! ##### row_default_height
//!
//! Only works when the layout is in table mode. Sets the default height for rows, meaning that
//! rows will be at least that height, but can still grow to accommodate widgets higher than the
//! default height. Without setting this property, each individual row will be the height of its
//! highest widget. Setting this property is useful if you want all your rows to be at least a
//! certain height, even if all its widgets are smaller. Another use case is using this property in
//! combination wih 'force_default_row_height: true' (see property below). In this case, you are
//! hard coding each row to be exactly the row_default_height, without the ability to grow.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Any usize number
//!
//! **Default value:**
//!
//! 0 (each row grows to the highest widget).
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: table
//!     row_default_height: 20
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_table_config_mut().set_row_default_height(20);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_properties_force_default_row_height"></a>
//! ##### force_default_row_height
//!
//! Only works when the layout is in table mode and the 'row_default_height' property is set to a
//! value higher than 0. Ensures that each row is *exactly* the row_default_height, without any
//! ability to grow.
//!
//! **Property type:**
//!
//! bool
//!
//! **Possible values:**
//!
//! - true
//! - false
//!
//! **Default value:**
//!
//! false
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     mode: table
//!     row_default_height: 20
//!     force_default_row_height: true
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_table_config_mut().set_row_default_height(20);
//! state.get_table_config_mut().set_force_default_row_height(true);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_layout_default_callbacks"></a>
//! #### 2.2.2 Default callback implementations
//!
//! <a name="reference_layout_default_callbacks_scrollup"></a>
//! ##### on_scroll_up
//!
//! If the layout is scrolling horizontally, this scrolls the layout to the left. If the layout is
//! scrolling vertically, this scrolls the layout up.
//! If you implement your own callback, returns true to surpress this behavior, or false to
//! supplement it.
//!
//!
//! <a name="reference_layout_default_callbacks_scrolldown"></a>
//! ##### on_scroll_down
//!
//! If the layout is scrolling horizontally, this scrolls the layout to the right. If the layout is
//! scrolling vertically, this scrolls the layout down.
//! If you implement your own callback, returns true to surpress this behavior, or false to
//! supplement it.
//!
//! <a name="reference_layout_available_callbacks"></a>
//! #### 2.2.3 Available callbacks
//!
//! - on_scroll_up
//! - on_scroll_down
//! - on_hover
//! - on_drag
//! - on_left_click
//! - on_right_click
//! - on_select (called when a scrollbar is selected)
//!
//!
//! <a name="reference_label"></a>
//! ## 2.3 Label:
//!
//! A widget that displays (colored) text.
//!
//! <a name="reference_label_properties"></a>
//! ### 2.3.1 Properties
//!
//! <a name="reference_label_properties_text"></a>
//! #### text
//!
//! The text that will be displayed in the label. If the label has a height more than 1, it will
//! wrap text respecting word boundaries.
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any String (including unicode)
//!
//! **Default value:**
//!
//! Empty string
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Label:
//!     text: Hello world!
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_label_mut();
//!
//! state.set_text("Hello world!".to_string());
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_label_properties_from_file"></a>
//! #### from_file
//!
//! Label will display text loaded from this file path. Overwrites the text property if set. The
//! file content will be merged into your binary, so you do not need to ship the text file with
//! your binary.
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any file path to a text file.
//!
//! **Default value:**
//!
//! Empty string
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Label:
//!     from_file: ./my_text_file.txt
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_label").as_label_mut();
//!
//!
//! state.set_from_file("./my_text_file.txt");
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_label_default_callbacks"></a>
//! ### 2.3.2 Default callbacks implementations
//!
//! The Label widget does not have any default callback implementations.
//!
//! <a name="reference_label_available_callbacks"></a>
//! ### 2.3.3 Available callbacks
//!
//! - on_scroll_up
//! - on_scroll_down
//! - on_hover
//! - on_drag
//! - on_left_click
//! - on_right_click
//!
//! <a name="reference_button"></a>
//! ## 2.4 Button:
//!
//! A button that shows a small animation when clicked. Has text and always has a border. Usually
//! you want to bind an on_press callback to make something happen when pressed.
//!
//! <a name="reference_button_properties"></a>
//! ### 2.4.1 Properties
//!
//! <a name="reference_button_properties_text"></a>
//! #### text
//!
//! The text that will be displayed in the button. If the button has a height more than 1, it will
//! wrap text respecting word boundaries.
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any String (including unicode)
//!
//! **Default value:**
//!
//! Empty string
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Button:
//!     text: Press me
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_button").as_button_mut();
//!
//! state.set_text("Press me".to_string());
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_button_properties_flash_fg_color"></a>
//! #### flash_fg_color
//!
//! Foreground color used for the button flash animation shown after the button is clicked.
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! white
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         flash_fg_color: red
//! ```
//! ```
//! - Layout:
//!     - Label:
//!         flash_fg_color: 255, 0, 0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_button").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_flash_fg_color(Color::Red);
//! state.get_color_config_mut().set_flash_fg_color(Color::from((255, 0, 0)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_button_properties_flash_bg_color"></a>
//! #### flash_bg_color
//!
//! Background color used for the button flash animation shown after the button is clicked.
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! white
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         flash_bg_color: red
//! ```
//! ```
//! - Layout:
//!     - Label:
//!         flash_bg_color: 255, 0, 0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_button").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_flash_bg_color(Color::Red);
//! state.get_color_config_mut().set_flash_bg_color(Color::from((255, 0, 0)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_button_default_callbacks"></a>
//! ### 2.4.2 Default callbacks implementations
//!
//! <a name="reference_button_default_callbacks_leftclick"></a>
//! #### on_left_click
//!
//! Calls the on_press callback.
//!
//! <a name="reference_button_default_callbacks_enter"></a>
//! #### on_keyboard_enter
//!
//! Calls the on_press callback.
//!
//! <a name="reference_button_available_callbacks"></a>
//! ### 2.4.3 Available callbacks
//!
//! - on_keyboard_enter
//! - on_press
//! - on_select
//! - on_deselect
//! - on_scroll_up
//! - on_scroll_down
//! - on_hover
//! - on_drag
//! - on_left_click
//! - on_right_click
//! - Custom keys
//!
//! <a name="reference_checkbox"></a>
//! ## 2.5 Checkbox:
//!
//! A simple switch that can be toggled on and off. A checkbox is always 5 width and 1 height.
//! Often used with on_value_change callback.
//!
//! <a name="reference_checkbox_properties"></a>
//! ### 2.5.1 Properties
//!
//! <a name="reference_checkbox_properties_active"></a>
//! #### active
//!
//! Property that represents whether the checkbox is on (active) or off.
//!
//! **Property type:**
//!
//! bool
//!
//! **Possible values:**
//!
//! - true
//! - false
//!
//! **Default value:**
//!
//! false
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Checkbox:
//!     active: true
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_checkbox").as_checkbox_mut();
//!
//! state.set_active(true);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_checkbox_properties_active_symbol"></a>
//! #### active_symbol
//!
//! The symbol used to indicate the checkbox is active. If for example the symbol is X, then the
//! checkbox will look like this when active: [ X ].
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any one character string.
//!
//! **Default value:**
//!
//! X
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Checkbox:
//!     active_symbol: 0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_checkbox").as_checkbox_mut();
//!
//! state.set_active_symbol("0");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_checkbox_properties_inactive_symbol"></a>
//! #### inactive_symbol
//!
//! The symbol used to indicate the checkbox is inactive. If for example the symbol is -, then the
//! checkbox will look like this when inactive: [ - ].
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any one character string.
//!
//! **Default value:**
//!
//! -
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Checkbox:
//!     inactive_symbol: .
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_checkbox").as_checkbox_mut();
//!
//! state.set_inactive_symbol(".");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_checkbox_default_callbacks"></a>
//! ### 2.5.2 Default callbacks implementations
//!
//! <a name="reference_checkbox_default_callbacks_leftclick"></a>
//! #### on_left_click
//!
//! Calls the on_press callback.
//!
//! <a name="reference_checkbox_default_callbacks_enter"></a>
//! #### on_keyboard_enter
//!
//! Calls the on_press callback.
//!
//! <a name="reference_checkbox_available_callbacks"></a>
//! ### 2.5.3 Available callbacks
//!
//! - on_value_change
//! - on_keyboard_enter
//! - on_press
//! - on_select
//! - on_deselect
//! - on_scroll_up
//! - on_scroll_down
//! - on_hover
//! - on_drag
//! - on_left_click
//! - on_right_click
//! - Custom keys
//!
//!
//! <a name="reference_radiobutton"></a>
//! ## 2.6 Radio button:
//!
//! A simple switch that can be toggled on and off. A radio button is always 5 width and 1 height.
//! A radio button can be part of a group (by setting the group property of multiple radio buttons
//! to the same value). A group of radio buttons is mutually exclusive, meaning that when one
//! becomes active, the others become inactive. Often used with on_value_change callbacs. Only the
//! radio button that become active triggers an on_value_change callback.
//!
//! <a name="reference_radiobutton_properties"></a>
//! ### 2.6.1 Properties
//!
//! <a name="reference_radiobutton_properties_active"></a>
//! #### active
//!
//! Property that represents whether the radio button is on (active) or off.
//!
//! **Property type:**
//!
//! bool
//!
//! **Possible values:**
//!
//! - true
//! - false
//!
//! **Default value:**
//!
//! false
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Radiobutton:
//!     active: true
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_radio_button").as_radio_button_mut();
//!
//! state.set_active(true);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_radiobutton_properties_active_symbol"></a>
//! #### active_symbol
//!
//! The symbol used to indicate the radio button is active. If for example the symbol is X, then the
//! checkbox will look like this when active: ( X ).
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any one character string.
//!
//! **Default value:**
//!
//! X
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - RadioButton:
//!     active_symbol: 0
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_radio_button").as_radio_button_mut();
//!
//! state.set_active_symbol("0");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_radiobutton_properties_inactive_symbol"></a>
//! #### inactive_symbol
//!
//! The symbol used to indicate the radio button is inactive. If for example the symbol is -, then the
//! radio button will look like this when inactive: ( - ).
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any one character string.
//!
//! **Default value:**
//!
//! -
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - RadioButton:
//!     inactive_symbol: .
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_radio_button").as_radio_button_mut();
//!
//! state.set_inactive_symbol(".");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_radiobutton_properties_group"></a>
//! #### group
//!
//! This property determines which radio buttons belong together, and are therefore mutually
//! exclusive. Set multiple radio buttons to the same group value to make them part of the same
//! group.
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any string.
//!
//! **Default value:**
//!
//! Empty string
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - RadioButton:
//!     group: my_group
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_radio_button").as_radio_button_mut();
//!
//! state.set_group("my_group");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_radiobutton_default_callbacks"></a>
//! ### 2.6.2 Default callbacks implementations
//!
//! <a name="reference_radiobutton_default_callbacks_leftclick"></a>
//! #### on_left_click
//!
//! Calls the on_press callback.
//!
//! <a name="reference_radiobutton_default_callbacks_enter"></a>
//! #### on_keyboard_enter
//!
//! Calls the on_press callback.
//!
//! <a name="reference_radiobutton_available_callbacks"></a>
//! ### 2.6.3 Available callbacks
//!
//! - on_value_change (only if radio button became active)
//! - on_keyboard_enter
//! - on_press
//! - on_select
//! - on_deselect
//! - on_scroll_up
//! - on_scroll_down
//! - on_hover
//! - on_drag
//! - on_left_click
//! - on_right_click
//! - Custom keys
//!
//!
//! <a name="reference_dropdown"></a>
//! ## 2.7 Dropdown:
//!
//! A widget with a list op options (Vec<String>). Initially, only the active option is visible.
//! When pressed, the full list drops down for the user to choose from.
//!
//! <a name="reference_dropdown_properties"></a>
//! ### 2.7.1 Properties
//!
//! <a name="reference_dropdown_properties_choice"></a>
//! #### choice
//!
//! Property that determines the active option, i.e. current choice.
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any string.
//!
//! **Default value:**
//!
//! Empty value if allow_none is true, otherwise the first option from the options property.
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Dropdown:
//!     choice: my_option_1
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_dropdown").as_dropdown_mut();
//!
//! state.set_choice("my_option_1");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_dropdown_properties_options"></a>
//! #### options
//!
//! The list of options the user can choose from. If you want to allow an empty choice, set
//! allow_none to true rather than adding an empty string. A dropdown must have at least one value,
//! so either this property must be set, or the allow_none property.
//!
//! **Property type:**
//!
//! Vec<String>
//!
//! **Possible values:**
//!
//! A vector of >0 character Strings.
//!
//! **Default value:**
//!
//! Empty vec.
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Dropdown:
//!     options: my_option_1, my_option_2, my_option_3
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_dropdown").as_dropdown_mut();
//!
//! let options = vec!("my_option_1".to_string(),
//!                    "my_option_2".to_string(),
//!                    "my_option_3".to_string());
//! state.set_options(options);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_dropdown_properties_allow_none"></a>
//! #### allow_none
//!
//! If set to true, includes an empty choice in the dropdown. Enabled by default.
//!
//! **Property type:**
//!
//! bool
//!
//! **Possible values:**
//!
//! - true
//! - false
//!
//! **Default value:**
//!
//! true
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Dropdown:
//!     allow_none: false
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_dropdown").as_dropdown_mut();
//!
//! state.set_allow_none(false);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_dropdown_default"></a>
//! ### 2.7.2 Default callbacks implementations
//!
//! <a name="reference_dropdown_default_callbacks_leftclick"></a>
//! #### on_left_click
//!
//! Opens the dropped down menu.
//!
//! <a name="reference_dropdown_default_callbacks_enter"></a>
//! #### on_keyboard_enter
//!
//! Opens the dropped down menu.
//!
//! <a name="reference_dropdown_available_callbacks"></a>
//! ### 2.7.3 Available callbacks
//!
//! - on_value_change
//! - on_keyboard_enter
//! - on_press
//! - on_select
//! - on_deselect
//! - on_scroll_up
//! - on_scroll_down
//! - on_hover
//! - on_drag
//! - on_left_click
//! - on_right_click
//! - Custom keys
//!
//!
//! <a name="reference_slider"></a>
//! ## 2.8 Slider:
//!
//! A widget representing a numerical value. User can slide left or right to decrease/increase the
//! value (using left click, mouse drag, or left/right keyboard arrow keys.
//! Thee slider has a minimum-, maximum-, current- and step value. The step value determines the
//! amount by which the value can be adjusted; it also determines (together with min and max) how
//! many possible values there are with the equation: (max - min) / step.
//! Auto scaling for sliders depends on the amount of possible values; 1 width will be added for
//! each possible value (the amount of values depends on min, max and step as explained above).
//!
//! <a name="reference_slider_properties"></a>
//! ### 2.8.1 Properties
//!
//! <a name="reference_slider_properties_value"></a>
//! #### value
//!
//! Current value of the widget. *Must* be a multiple of the step value, not be higher than the
//! minimum value, and not be higher than the maximum value. If minimum is 0, maximum is 10,
//! and step is 5, then value can be 0, 5 or 10.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Multiple of step between minimum and maximum.
//!
//! **Default value:**
//!
//! minimum
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Slider:
//!     value: 10
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_slider").as_slider_mut();
//!
//! state.set_value(10);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_slider_properties_min"></a>
//! #### min
//!
//! Minimum possible value of the widget. *Must* be a multiple of the step value.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Multiple of step.
//!
//! **Default value:**
//!
//! 0
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Slider:
//!     min: 1
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_slider").as_slider_mut();
//!
//! state.set_min(1);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_slider_properties_max"></a>
//! #### max
//!
//! Maximum possible value of the widget. *Must* be a multiple of the step value.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Multiple of step.
//!
//! **Default value:**
//!
//! 100
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Slider:
//!     max: 50
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_slider").as_slider_mut();
//!
//! state.set_max(50);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_slider_properties_step"></a>
//! #### step
//!
//! Determines the amount by which the value may be adjusted. Keep in mind that the min, max and
//! value properties must all be multiples of step.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Any usize.
//!
//! **Default value:**
//!
//! 1
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Slider:
//!     step: 5
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_slider").as_slider_mut();
//!
//! state.set_step(5);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_slider_default_callbacks"></a>
//! ### 2.8.2 Default callbacks implementations
//!
//! <a name="reference_slider_default_callbacks_leftclick"></a>
//! #### on_left_click
//!
//! Instantly sets the slider to the value under the cursor.
//!
//! <a name="reference_slider_default_callbacks_drag"></a>
//! #### on_drag
//!
//! Drags the slider along the cursor.
//!
//! <a name="reference_slider_available_callbacks"></a>
//! ### 2.8.3 Available callbacks
//!
//! - on_value_change
//! - on_keyboard_enter
//! - on_press
//! - on_select
//! - on_deselect
//! - on_scroll_up
//! - on_scroll_down
//! - on_hover
//! - on_drag
//! - on_left_click
//! - on_right_click
//! - Custom keys
//!
//!
//! <a name="reference_textinput"></a>
//! ## 2.2.9 Text input:
//!
//! An interactive textbox. User can select it through keyboard up/down (if selection_order) is set,
//! or by clicking in the textbox. A cursor will appear and the user can type text, or
//! backspace/delete text.
//!
//! <a name="reference_textinput_properties"></a>
//! ### 2.9.1 Properties
//!
//! <a name="reference_textinput_properties_text"></a>
//! #### text
//!
//! The current text of the textbox.
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any string.
//!
//! **Default value:**
//!
//! Empty String.
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - TextInput:
//!     text: Hello world!
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_text_input").as_text_input_mut();
//!
//! state.set_text("Hello world!".to_string());
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_textinput_properties_max_length"></a>
//! #### max_length
//!
//! Maximum amount of allowed characters. When reached, typing in the texbox does nothing. If set to
//! 0 then the max_length is infinite.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Any usize (0 means infinite)
//!
//! **Default value:**
//!
//! 0 (infinite)
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - TextInput:
//!     max_length: 100
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_text_input").as_text_input_mut();
//!
//! state.set_max_length(100);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_textinput_properties_cursor_color"></a>
//! #### cursor_color
//!
//! Color used for the cursor of the text input.
//!
//! **Property type:**
//!
//! Color
//!
//! **Possible values:**
//!
//! - RGB value: 0-255, 0-255, 0-255
//! - Color words:
//!     - black
//!     - blue
//!     - dark_blue
//!     - cyan
//!     - dark_cyan
//!     - green
//!     - dark_green
//!     - grey
//!     - dark_grey
//!     - magenta
//!     - dark_magenta
//!     - red
//!     - dark_red
//!     - white
//!     - yellow
//!     - dark_yellow
//!
//! **Default value:**
//!
//! dark_yellow
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Layout:
//!     - Label:
//!         cursor_color: red
//! ```
//! ```
//! - Layout:
//!     - Label:
//!         cursor_color: 255, 0, 0
//!         border: true
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! use ez_term::GenericState;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_layout").as_layout_mut();
//!
//! // Table properties are wrapping into a TableConfig object, which we have to retrieve
//! // first:
//! state.get_color_config_mut().set_cursor_fg_color(Color::Red);
//! state.get_color_config_mut().set_cursor_fg_color(Color::from((255, 0, 0)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_textinput_default_callbacks"></a>
//! ### 2.9.2 Default callbacks implementations
//!
//! <a name="reference_textinput_default_callbacks_leftclick"></a>
//! #### on_left_click
//!
//! Calls on-select.
//!
//! <a name="reference_textinput_default_callbacks_select"></a>
//! #### on_select
//!
//! Enters text editing mode.
//!
//! <a name="reference_textinput_available_callbacks"></a>
//! ### 2.9.3 Available callbacks
//!
//! - on_value_change
//! - on_keyboard_enter
//! - on_press
//! - on_select
//! - on_deselect
//! - on_scroll_up
//! - on_scroll_down
//! - on_hover
//! - on_drag
//! - on_left_click
//! - on_right_click
//!
//!
//! <a name="reference_progressbar"></a>
//! ## 2.10 Progress bar:
//!
//! A widget with a max- and current value used to display progress
//!
//! <a name="reference_progressbar_properties"></a>
//! ### 2.10.1 Properties
//!
//! <a name="reference_progressbar_properties_value"></a>
//! #### value
//!
//! Property that determines the current value; used together with max to determine progress
//! (minimum value is always implicitly 0).
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Any usize.
//!
//! **Default value:**
//!
//! 0
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - ProgressBar:
//!     value: 50
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_progress_bar").as_progress_bar_mut();
//!
//! state.set_value(50);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_progressbar_properties_max"></a>
//! #### max
//!
//! Maximum value; i.e. value where progress is 100%. Must be larger than 0.
//!
//! **Property type:**
//!
//! usize
//!
//! **Possible values:**
//!
//! Any usize > 0.
//!
//! **Default value:**
//!
//! 100
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - ProgressBar:
//!     max: 50
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_progress_bar").as_progress_bar_mut();
//!
//! state.set_max(50);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_progressbar_default_callbacks"></a>
//! ### 2.10.2 Default callbacks implementations
//!
//! The Progress bar widget does not have any default callback implementations.
//!
//! <a name="reference_progressbar_available_callbacks"></a>
//! ### 2.10.3 Available callbacks
//!
//! - on_scroll_up
//! - on_scroll_down
//! - on_hover
//! - on_drag
//! - on_left_click
//! - on_right_click
//! - Custom keys
//!
//!
//! <a name="reference_canvas"></a>
//! ## 2.11 Canvas:
//!
//! The canvas is a widget that allows custom content. It can load its graphic content from a text
//! file, or it can be set programmatically.
//!
//! <a name="reference_canvas_properties"></a>
//! ### 2.11.1 Properties
//!
//! <a name="reference_canvas_properties_from_file"></a>
//! #### from_file
//!
//! File path to a text file from which to load visual context. Empty string means don't load from
//! file. The file content will be merged into your binary, so you do not need to ship the text
//! file with your binary.
//!
//! **Property type:**
//!
//! String
//!
//! **Possible values:**
//!
//! Any valid file path.
//!
//! **Default value:**
//!
//! Empty string.
//!
//! **Usage examples:**
//!
//! In EzLang files:
//! ```
//! - Canvas:
//!     from_file: ./canvas_content.txt
//! ```
//!
//! In code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_canvas").as_canvas_mut();
//!
//! state.set_from_file("./canvas_content.txt");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_canvas_properties_from_code"></a>
//! #### Setting content from code
//!
//! The alternative to loading content from a text file is setting content from code. You have to
//! manually create the pixels and use them to construct a PixelMap (Vec<Vec<Pixel>>) Here is an
//! example to create a square like this:
//! ```
//! S S S
//! S S S
//! S S S
//! ```
//!
//! The code:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let state = state_tree.get_mut("my_canvas").as_canvas_mut();
//!
//! // We create a single pixel
//! let pixel = Pixel::new("S".to_string(), Color::White, Color::Black);
//!
//! // We create a row of three pixels
//! let row = vec!(pixel.clone(); 3);
//!
//! // We create a pixelmap of three rows (we now have a square)
//! let content = vec!(row.clone(); 3);
//!
//! state.set_contents(content);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_canvas_default_callbacks"></a>
//! ### 2.11.2 Default callbacks implementations
//!
//! The Canvas widget does not have any default callback implementations.
//!
//! <a name="reference_canvas_available_callbacks"></a>
//! ### 2.11.3 Available callbacks
//!
//! - on_scroll_up
//! - on_scroll_down
//! - on_hover
//! - on_drag
//! - on_left_click
//! - on_right_click
//!
//!
//! <a name="reference_scheduler"></a>
//! ## 2.12 Scheduler:
//!
//! Here you'll find a reference to all scheduler methods, with short examples. If you need more
//! explanation, then most scheduler function are discussed in detail in
//! [scheduler tutorial](#scheduler).
//!
//! <a name="reference_scheduler_schedule_once"></a>
//! ### 2.12.1 schedule_once:
//!
//! Method that allows you to schedule a closure or function for single execution after a delay
//! (which can be 0).
//! Only intended for code that returns immediately (like manipulating the UI); to run blocking
//! app-code, use schedule threaded.
//! For a tutorial see: [Single execution](#scheduler_tasks_single).
//!
//! #### Parameters:
//!
//! - Scheduled task name: String
//! - Function: Box<FnMut(Context)>
//! - Delay: std::Duration;
//!
//! #### Example:
//!
//! As an example, we'll create a scheduled task that changes a label text:
//! ```
//! use std::time::Duration;
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_closure = |context: Context| {
//!     let state = context.state_tree.get_mut("my_label").as_label_mut();
//!     state.set_text("New text!".to_string());
//!     state.update(context.scheduler);
//! };
//! scheduler.schedule_once("my_task", Box::new(my_closure), Duration::from_secs(0));
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_cancel_task"></a>
//! ### 2.12.2 cancel_task:
//!
//! Cancel a run-once task. This of course only works if called before the task was executed (possible
//! if it had a delay). This function is always safe to call, if there's no task to cancel it will
//! not panic.
//!
//! #### Parameters:
//!
//! - Scheduled task name: String
//!
//! #### Example:
//!
//! We'll schedule a run-once task and then cancel it:
//!
//! ```
//! use std::time::Duration;
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_closure = |context: Context| {
//!     let state = context.state_tree.get_mut("my_label").as_label_mut();
//!     state.set_text("New text!".to_string());
//!     state.update(context.scheduler);
//! };
//! scheduler.schedule_once("my_task", Box::new(my_closure), Duration::from_secs(0));
//!
//! scheduler.cancel_task("my_task");
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_schedule_recurring"></a>
//! ### 2.12.3 schedule_recurring:
//!
//! Method that allows you to schedule a closure or function for recurring execution; it will be
//! executed on an interval as long as the function keeps returning true.
//! Only intended for code that returns immediately (like manipulating the UI); to run blocking
//! app-code, use schedule threaded.
//! For more a tutorial see: [Recurring execution](#scheduler_tasks_recurring)
//!
//! #### Parameters:
//!
//! - Scheduled task name: String
//! - Function: Box<FnMut(Context) -> bool>
//! - Interval: std::Duration;
//!
//! #### Example:
//!
//! As an example, we'll create a scheduled task that changes a label text to count time. When 60
//! seconds have been counted, the function will be cancelled:
//! ```
//! use std::time::Duration;
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let counter: usize = 0;
//! let my_closure = move |context: Context| {
//!     let state = context.state_tree.get_mut("my_label").as_label_mut();
//!     state.set_text(format!("Time passed {}", counter));
//!     state.update(context.scheduler);
//!     return if counter == 60 {
//!         false
//!     } else {
//!         true
//!    }
//! };
//! scheduler.schedule_recurring("my_task", Box::new(my_closure), Duration::from_secs(1));
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_cancel_recurring"></a>
//! ### 2.12.4 cancel_recurring_task:
//!
//! Cancel a recurring task. This function is always safe to call, if there's no task to cancel it
//! will not panic. An alternative way to cancel a recurring task is to return false from the
//! scheduled function (which is recommended for most use cases. This function lets you cancel a
//! recurring task 'from the outside'.
//!
//! #### Parameters:
//!
//! - Scheduled task name: String
//!
//! #### Example:
//!
//! We'll schedule a recurring task and then cancel it:
//! ```
//! use std::time::Duration;
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let counter: usize = 0;
//! let my_closure = move |context: Context| {
//!     let state = context.state_tree.get_mut("my_label").as_label_mut();
//!     state.set_text(format!("Time passed {}", counter));
//!     state.update(context.scheduler);
//!     return if counter == 60 {
//!         false
//!     } else {
//!         true
//!    }
//! };
//! scheduler.schedule_recurring("my_task", Box::new(my_closure), Duration::from_secs(1));
//!
//! scheduler.cancel_recurring_task("my_task");
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_schedule_threaded"></a>
//! ### 2.12.5 schedule_threaded:
//!
//! Method that allows you to schedule a closure or function for threaded execution. This allows
//! you to run code that does not return immediately (like your app code). You can use the
//! state tree from the threaded function to manipulate the UI, but the scheduler will not be
//! available from a thread. A hashmap with custom properties is also available.
//! For more a tutorial see: [Threaded execution](#scheduler_tasks_threaded)
//!
//! #### Parameters:
//!
//! - Threaded function: Box<dyn FnOnce(HashMap<String, EzProperty>, StateTree) + Send>
//! - On_finish callback function: Option<Box<FnMut(Context)>>>
//!
//! #### Example:
//!
//! We'll schedule a counter function that does not return immediately:
//! ```
//! use std::time::Duration;
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn example_app(mut properties: EzPropertiesMap, mut state_tree: StateTree) {
//!
//!     let state = state_tree.get_mut("my_label").as_label_mut();
//!     for x in 1..10 {
//!         state.set_text(format!("Time passed {}", x));
//!         std::thread::sleep(Duration::from_secs(1))
//!     };
//! }
//!
//! fn on_finish_callback(context: Context) {
//!     let state = state_tree.get_mut("my_label").as_label_mut();
//!     state.set_text("Finished!".to_string());
//! }
//!
//! scheduler.schedule_threaded(Box::new(example_app), Some(Box::new(on_finish_callback)));
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_open_modal"></a>
//! ### 2.12.6 open_modal:
//!
//! Method that allows you to open a modal (e.g. a popup). To open a modal you need to define a
//! Layout template in an .ez file. You can then spawn an instance of the template as a modal using
//! this method. The ID of the layout spawned as a modal will be 'modal', its full path will be
//! '/root/modal'.
//! For a tutorial on modals see: [Managing popups](#scheduler_modals)
//!
//! #### Parameters:
//!
//! - Layout template name: &str
//! - State tree: &mut StateTree
//!
//! #### Example:
//!
//! We'll create a popup and open it. The popup will have some text and a dismiss button.
//! First we need to define a Layout template in an .ez file:
//! ```
//! - <MyPopupTemplate@Layout>:
//!     mode: float
//!     size_hint: 0.5, 0.5
//!     pos_hint: center, middle
//!     border: true
//!     - Label:
//!         text: This is a test popup.
//!         auto_scale: true, true
//!         pos_hint: center, top
//!     - Button:
//!         id: dismiss_button
//!         text: Dismiss
//!         size_hint_x: 1
//!         auto_scale_height: true
//!         pos_hint: center, bottom
//!         selection_order: 1
//! ```
//! Now we'll spawn the popup from code based on this template.
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let dismiss = |context: Context| {
//!
//!     context.scheduler.dismiss_modal(context.state_tree);
//!     false
//! };
//!
//! scheduler.update_callback_config("dismiss_button",
//!                                  CallbackConfig::from_on_press(Box::new(dismiss)));
//!
//! scheduler.open_modal("MyPopupTemplate", &mut state_tree);
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_dismiss_modal"></a>
//! ### 2.12.7 dismiss_modal:
//!
//! Dismiss the open modal. Can always be called safely even if one no longer exists (though this
//! does trigger a screen redraw so try to avoid that).
//! For a tutorial on modals see: [Managing popups](#scheduler_modals)
//!
//! #### Parameters:
//!
//! - State tree: &mut StateTree
//!
//! #### Example:
//!
//! We'll dismiss any open modal:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.dismiss_modal(&mut state_tree);
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_overwrite_callback_config"></a>
//! ### 2.12.8 overwrite_callback_config:
//!
//! Replace the entire CallbackConfig of a widget on the next frame. You can pass an empty
//! CallbackConfig to remove all callbacks for a widget.
//! For a tutorial on callbacks (including callback configs) see:
//! [Managing callbacks](#scheduler_callbacks)
//!
//! #### Parameters:
//!
//! - Widget ID or path: &str
//! - New CallbackConfig: CallbackConfig
//!
//! #### Example:
//!
//! We'll remove any existing callbacks by overwriting with an empty CallbackConfig for a button
//! with id "my_button".
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let callback_config = CallbackConfig::default();
//! scheduler.overwrite_callback_config("my_button", callback_config);
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_update_callback_config"></a>
//! ### 2.12.9 update_callback_config:
//!
//! Update the CallbackConfig of a widget on the next frame. Any callback set on the new callback
//! config will be set on the existing one (overwriting if one already exists). Any existing
//! callbacks that are *not* set on the new config are left intact. In other words, this allows
//! you to set new callbacks without removing the existing ones.
//! For a tutorial on callbacks (including callback configs) see:
//! [Managing callbacks](#scheduler_callbacks)
//!
//! #### Parameters:
//!
//! - Widget ID or path: &str
//! - New CallbackConfig: CallbackConfig
//!
//! #### Example:
//!
//! We'll add a callback for a button with id "my_button".
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = |context: Context| {
//!     let state = context.state_tree.get_mut("my_button").as_button_mut();
//!     state.set_disabled(true);
//!     state.update(context.scheduler);
//! };
//!
//! let callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
//! scheduler.update_callback_config("my_button", callback_config);
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_bind_property_callback"></a>
//! ### 2.12.10 bind_property_callback:
//!
//! Bind a callback to a property (can be a widget property or a custom property). Whenever the
//! property value changes, the callback is called. This method only takes a full property path
//! (e.g. "/root/layout/my_label/width"); it is usually more convenient to get the property from
//! the widget state, and then call "property.bind".
//!
//! For a tutorial on this see: [Managing callbacks](#scheduler_callbacks)
//!
//! #### Parameters:
//!
//! - property path: &str
//! - callback function: Box<dyn FnMut(Context)>
//!
//! #### Example:
//!
//! Let's bind a property; we'll show how to do it from the scheduler, and how to do it from the
//! property itself. We'll create a callback that displays the width of a label in its text:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: Context) {
//!     let state = context.state_tree.get_mut("my_label").as_label_mut();
//!     let width = state.get_size().get_width();
//!     state.set_text(format!("My width is: {}", width));
//! }
//!
//! // bind from scheduler
//! scheduler.bind_property_callback("/root/layout/my_label/width", Box::new(my_callback));
//!
//! // bind from property
//!     let state = state_tree.get_mut("my_label").as_label_mut();
//!     state.size.width.bind(Box::new(my_callback), &mut scheduler);
//!
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_bind_global_key"></a>
//! ### 2.12.11 bind_global_key:
//!
//! Bind a callback to a custom key being pressed anywhere in the UI. Global key binds take
//! priority over widget key binds.
//! For a tutorial on this see: [Managing callbacks](#scheduler_callbacks)
//!
//! #### Parameters:
//!
//! - key: KeyCode
//! - modifiers: Option<Vec<KeyModifiers>>
//! - callback function: Box<dyn FnMut(Context, KeyCode, KeyModifiers)>
//!
//! #### Example:
//!
//! We'll bind the key combination "shift + A" to change a label text:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: Context) {
//!     let state = context.state_tree.get_mut("my_label").as_label_mut();
//!     state.set_text("Shift A was pressed!".to_string());
//! }
//!
//! scheduler.bind_global_key(KeyCode::Char('a'), Some(vec!(KeyModifiers::SHIFT)),
//!                           Box::new(my_callback));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_remove_global_key"></a>
//! ### 2.12.12 remove_global_key:
//!
//! Remove one specific global key bind.
//! For a tutorial on this see: [Managing callbacks](#scheduler_callbacks)
//!
//! #### Parameters:
//!
//! - key: KeyCode
//! - modifiers: Option<Vec<KeyModifiers>>
//!
//! #### Example:
//!
//! We'll remove the key combination "shift + A" from the global key binds:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//!
//! scheduler.remove_global_key(KeyCode::Char('a'), Some(vec!(KeyModifiers::SHIFT);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_clear_global_keys"></a>
//! ### 2.12.13 clear_global_keys:
//!
//! Remove all global key binds.
//! For a tutorial on this see: [Managing callbacks](#scheduler_callbacks)
//!
//! #### Parameters:
//!
//! This method takes no parameters.
//!
//! #### Example:
//!
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//!
//! scheduler.clear_global_keys();
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_create_widget"></a>
//! ### 2.12.14 create_widget:
//!
//! Create a widget from a template or base widget type and add it to a layout. This allows you to
//! create widgets from code.
//! For a tutorial on this see: [Managing widgets programmatically](#scheduler_widgets_from_code)
//!
//! #### Parameters:
//!
//! - Widget type or template: &str
//! - ID of new widget: &str
//! - ID or path of parent layout: &str
//! - State tree: &mut StateTree
//!
//! #### Example:
//!
//! We'll add labels to a layout from a for loop; after creating the labels we'll alter the text of
//! each label based on the for loop number:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let parent_id = "my_layout";
//!
//! for x in 1..=10 {
//!
//!     let new_id = format!("label_{}", x).as_str();
//!     scheduler.create_widget("Label", new_id, parent_id, &mut state_tree);
//!
//!     let new_label_state = state_tree.get_mut(new_id);
//!     new_label_state.set_text(format!("Hello world {}!", x));
//!
//! }
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_remove_widget"></a>
//! ### 2.12.15 remove_widget:
//!
//! Remove a widget from a layout. Removing a layout will also remove all children of that layout.
//! You cannot remove the root layout. You cannot remove a modal root, use scheduler.dismiss_modal
//! instead.
//! For a tutorial on this see: [Managing widgets programmatically](#scheduler_widgets_from_code)
//!
//! #### Parameters:
//!
//! - ID or path of widget to remove: &str
//!
//! #### Example:
//!
//! We'll remove labels from a layout from a for loop:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! for x in 1..=10 {
//!
//!     let label_id = format!("label_{}", x).as_str();
//!     scheduler.remove_widget(label_id);
//! }
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_new_usize_property"></a>
//! ### 2.12.16 new_usize_property:
//!
//! Create a custom property. You can bind this property to widget properties of the same type;
//! then when you update the custom property, the widget will update automatically as well.
//! The name of custom properties may not contain any '/'.
//! For a tutorial on this see: [Creating custom properties](#scheduler_properties).
//!
//! #### Parameters:
//!
//! - Name of the new property: &str
//! - Value of the new property: usize
//!
//! #### Example:
//!
//! We'll create a custom property and bind it to a widget in an .ez file.
//! First the code:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.new_usize_property("my_property", 1);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//! Now the .ez file:
//! ```
//! - Layout:
//!     - ProgressBar:
//!         value: properties.my_property
//! ```
//!
//!
//! <a name="reference_scheduler_new_f64_property"></a>
//! ### 2.12.17 new_f64_property:
//!
//! Create a custom property. You can bind this property to widget properties of the same type;
//! then when you update the custom property, the widget will update automatically as well.
//! The name of custom properties may not contain any '/'.
//! For a tutorial on this see: [Creating custom properties](#scheduler_properties).
//!
//! #### Parameters:
//!
//! - Name of the new property: &str
//! - Value of the new property: f64
//!
//! #### Example:
//!
//! We'll create a custom property and bind it to a widget in an .ez file.
//! First the code:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.new_f64_property("my_property", 0.5);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//! Now the .ez file:
//! ```
//! - Layout:
//!     scroll_y: true
//!     scroll_start_y: properties.my_property
//! ```
//!
//!
//! <a name="reference_scheduler_new_string_property"></a>
//! ### 2.12.18 new_string_property:
//!
//! Create a custom property. You can bind this property to widget properties of the same type;
//! then when you update the custom property, the widget will update automatically as well.
//! The name of custom properties may not contain any '/'.
//! For a tutorial on this see: [Creating custom properties](#scheduler_properties).
//!
//! #### Parameters:
//!
//! - Name of the new property: &str
//! - Value of the new property: String
//!
//! #### Example:
//!
//! We'll create a custom property and bind it to a widget in an .ez file.
//! First the code:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.new_string_property("my_property", "Hello world!".to_string());
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//! Now the .ez file:
//! ```
//! - Layout:
//!     - Label:
//!         text: properties.my_property
//! ```
//!
//!
//! <a name="reference_scheduler_new_bool_property"></a>
//! ### 2.12.19 new_bool_property:
//!
//! Create a custom property. You can bind this property to widget properties of the same type;
//! then when you update the custom property, the widget will update automatically as well.
//! The name of custom properties may not contain any '/'.
//! For a tutorial on this see: [Creating custom properties](#scheduler_properties).
//!
//! #### Parameters:
//!
//! - Name of the new property: &str
//! - Value of the new property: bool
//!
//! #### Example:
//!
//! We'll create a custom property and bind it to a widget in an .ez file.
//! First the code:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.new_bool_property("my_property", true);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//! Now the .ez file:
//! ```
//! - Layout:
//!     - Button:
//!         disabled: properties.my_property
//! ```
//!
//! <a name="reference_scheduler_new_color_property"></a>
//! ### 2.12.20 new_color_property:
//!
//! Create a custom property. You can bind this property to widget properties of the same type;
//! then when you update the custom property, the widget will update automatically as well.
//! The name of custom properties may not contain any '/'.
//! For a tutorial on this see: [Creating custom properties](#scheduler_properties).
//!
//! #### Parameters:
//!
//! - Name of the new property: &str
//! - Value of the new property: Color
//!
//! #### Example:
//!
//! We'll create a custom property and bind it to a widget in an .ez file.
//! First the code:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.new_color_property("my_property", Color::Yellow);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//! Now the .ez file:
//! ```
//! - Layout:
//!     - Label:
//!         fg_color: properties.my_property
//! ```
//!
//!
//! <a name="reference_scheduler_new_layout_mode_property"></a>
//! ### 2.12.21 new_layout_mode_property:
//!
//! Create a custom property. You can bind this property to widget properties of the same type;
//! then when you update the custom property, the widget will update automatically as well.
//! The name of custom properties may not contain any '/'.
//! For a tutorial on this see: [Creating custom properties](#scheduler_properties).
//!
//! #### Parameters:
//!
//! - Name of the new property: &str
//! - Value of the new property: LayoutMode
//!
//! #### Example:
//!
//! We'll create a custom property and bind it to a widget in an .ez file.
//! First the code:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.new_layout_mode_property("my_property", LayoutMode::Table);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//! Now the .ez file:
//! ```
//! - Layout:
//!     mode: properties.my_property
//! ```
//!
//! <a name="reference_scheduler_new_layout_orientation_property"></a>
//! ### 2.12.22 new_layout_orientation_property:
//!
//! Create a custom property. You can bind this property to widget properties of the same type;
//! then when you update the custom property, the widget will update automatically as well.
//! The name of custom properties may not contain any '/'.
//! For a tutorial on this see: [Creating custom properties](#scheduler_properties).
//!
//! #### Parameters:
//!
//! - Name of the new property: &str
//! - Value of the new property: LayoutOrientation
//!
//! #### Example:
//!
//! We'll create a custom property and bind it to a widget in an .ez file.
//! First the code:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.new_string_property("my_property", LayoutOrientation::Vertical);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//! Now the .ez file:
//! ```
//! - Layout:
//!     orientation: properties.my_property
//! ```
//!
//! <a name="reference_scheduler_new_horizontal_alignment_property"></a>
//! ### 2.12.23 new_horizontal_alignment_property:
//!
//! Create a custom property. You can bind this property to widget properties of the same type;
//! then when you update the custom property, the widget will update automatically as well.
//! The name of custom properties may not contain any '/'.
//! For a tutorial on this see: [Creating custom properties](#scheduler_properties).
//!
//! #### Parameters:
//!
//! - Name of the new property: &str
//! - Value of the new property: HorizontalAlignment
//!
//! #### Example:
//!
//! We'll create a custom property and bind it to a widget in an .ez file.
//! First the code:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.new_horizontal_alignment_property("my_property", HorizontalAlignment::Center);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//! Now the .ez file:
//! ```
//! - Layout:
//!     - Label:
//!         halign: properties.my_property
//! ```
//!
//!
//! <a name="reference_scheduler_new_vertical_alignment_property"></a>
//! ### 2.12.24 new_vertical_alignment_property:
//!
//! Create a custom property. You can bind this property to widget properties of the same type;
//! then when you update the custom property, the widget will update automatically as well.
//! The name of custom properties may not contain any '/'.
//! For a tutorial on this see: [Creating custom properties](#scheduler_properties).
//!
//! #### Parameters:
//!
//! - Name of the new property: &str
//! - Value of the new property: VerticalAlignment
//!
//! #### Example:
//!
//! We'll create a custom property and bind it to a widget in an .ez file.
//! First the code:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.new_vertical_alignment_property("my_property", VerticalAlignment::Middle);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//! Now the .ez file:
//! ```
//! - Layout:
//!     - Label:
//!         valign: properties.my_property
//! ```
//!
//!
//! <a name="reference_scheduler_new_horizontal_pos_hint_property"></a>
//! ### 2.12.25 new_horizontal_pos_hint_property:
//!
//! Create a custom property. You can bind this property to widget properties of the same type;
//! then when you update the custom property, the widget will update automatically as well.
//! The name of custom properties may not contain any '/'.
//! For a tutorial on this see: [Creating custom properties](#scheduler_properties).
//!
//! #### Parameters:
//!
//! - Name of the new property: &str
//! - Value of the new property: HorizontalPosHint
//!
//! #### Example:
//!
//! We'll create a custom property and bind it to a widget in an .ez file.
//! First the code:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.new_horizontal_pos_hint_property("my_property",
//!                                            Some((HorizontalAlignment::Right, 0.75)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//! Now the .ez file:
//! ```
//! - Layout:
//!     mode: float
//!     - Label:
//!         pos_hint_x: properties.my_property
//! ```
//!
//!
//! <a name="reference_scheduler_new_vertical_pos_hint_property"></a>
//! ### 2.12.26 new_vertical_pos_hint_property:
//!
//! Create a custom property. You can bind this property to widget properties of the same type;
//! then when you update the custom property, the widget will update automatically as well.
//! The name of custom properties may not contain any '/'.
//! For a tutorial on this see: [Creating custom properties](#scheduler_properties).
//!
//! #### Parameters:
//!
//! - Name of the new property: &str
//! - Value of the new property: VerticalPosHint
//!
//! #### Example:
//!
//! We'll create a custom property and bind it to a widget in an .ez file.
//! First the code:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.new_vertical_pos_hint_property("my_property",
//!                                            Some((VerticalAlignment::Middle, 0.75)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//! Now the .ez file:
//! ```
//! - Layout:
//!     mode: float
//!     - Label:
//!         pos_hint_y: properties.my_property
//! ```
//!
//!
//! <a name="reference_scheduler_new_size_hint_property"></a>
//! ### 2.12.27 new_size_hint_property:
//!
//! Create a custom property. You can bind this property to widget properties of the same type;
//! then when you update the custom property, the widget will update automatically as well.
//! The name of custom properties may not contain any '/'.
//! For a tutorial on this see: [Creating custom properties](#scheduler_properties).
//!
//! #### Parameters:
//!
//! - Name of the new property: &str
//! - Value of the new property: SizeHint
//!
//! #### Example:
//!
//! We'll create a custom property and bind it to a widget in an .ez file.
//! First the code:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.new_size_hint_property("my_property", Some(0.75));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//! Now the .ez file:
//! ```
//! - Layout:
//!     mode: float
//!     - Label:
//!         size_hint_x: properties.my_property
//! ```
//!
//!
//! <a name="reference_scheduler_get_property"></a>
//! ### 2.12.28 get_property:
//!
//! Get a reference to a  custom property created earlier.
//! For a tutorial on this see: [Creating custom properties](#scheduler_properties).
//!
//! #### Parameters:
//!
//! - Name of the custom property: &str
//!
//! #### Example:
//!
//! We'll retrieve a custom property:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.new_usize_property("my_property", 10);
//! let property = scheduler.get_property("my_property");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//! <a name="reference_scheduler_get_property_mut"></a>
//! ### 2.12.29 get_property_mut:
//!
//! Get a mutable reference to a  custom property created earlier.
//! For a tutorial on this see: [Creating custom properties](#scheduler_properties).
//!
//! #### Parameters:
//!
//! - Name of the custom property: &str
//!
//! #### Example:
//!
//! We'll retrieve a custom property and modify it:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.new_usize_property("my_property", 10);
//! let property = scheduler.get_property_mut("my_property");
//! property.set(20);
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_update_widget"></a>
//! ### 2.12.30 update_widget:
//!
//! Schedule a widget to be redrawn on the next frame. If you are working with a widget state,
//! it is usually more convenient to call "state.update" instead of this method. This method
//! only accepts a full widget path, not an ID.
//! For a tutorial on this see: [Updating widgets](#scheduler_updating)
//!
//! #### Parameters:
//!
//! - Path of the widget: &str
//!
//! #### Example:
//!
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.update_widget("/root/my_layout/my_label");
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_force_redraw"></a>
//! ### 2.12.31 force_redraw:
//!
//! Forces a global screen redraw (though only changed pixels will actually be redrawn). While this
//! method if exposed to give you the option to use it, this is generally not recommended for
//! performance reasons. It's preferred to call updates on changed widgets, rather than global
//! redraws.
//! For a tutorial on this see: [Updating widgets](#scheduler_updating)
//!
//! #### Parameters:
//!
//! This method takes no parameters.
//!
//! #### Example:
//!
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.force_redraw();
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_set_selected_widget"></a>
//! ### 2.12.32 set_selected_widget:
//!
//! Users can select certain widget types through mouse or keyboard. This method allows selecting
//! widgets programmatically.
//! For a tutorial on this see: [Managing widget selection](#scheduler_selection)
//!
//! #### Parameters:
//!
//! - Widget ID or path: &str
//! - Optional mouse coordinates for selection: Option<Coordinates>
//!
//! #### Example:
//!
//! We'll select a widget once with no mouse_pos, and once with mouse_pos:
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.set_selected_widget("my_button", None);
//! scheduler.set_selected_widget("my_button", Some(Coordinates::new(3, 1)));
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_deselect_widget"></a>
//! ### 2.12.33 deselect_widget:
//!
//! Users can select certain widget types through mouse or keyboard. This method allows deselecting
//! the current widget programmatically.
//! For a tutorial on this see: [Managing widget selection](#scheduler_selection)
//!
//! #### Parameters:
//!
//! This method takes no parameters.
//!
//! #### Example:
//!
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.deselect_widget();
//!
//! run(root_widget, state_tree, scheduler);
//! ```
//!
//!
//! <a name="reference_scheduler_exit"></a>
//! ### 2.12.34 exit:
//!
//! Exit the program gracefully. EzTerm makes several changes to the terminal to display the UI,
//! so if you do not exit gracefully it may leave the terminal in an unusable state.
//!
//! #### Parameters:
//!
//! This method takes no parameters.
//!
//! #### Example:
//!
//! ```
//! use ez_term::*;
//!
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! scheduler.exit();
//! ```
//!


mod run;
mod scheduler;
mod widgets;
mod states;
mod property;
mod parser;


pub use crate::parser::parse_lang::load_ui;
pub use crate::run::run::run;

pub use crate::run::definitions::Coordinates;
pub use crossterm::event::{KeyCode, KeyModifiers};
pub use crossterm::style::Color;

pub use crate::scheduler::definitions::{Context, ThreadedContext, EzPropertiesMap};
pub use crate::run::definitions::{StateTree, PixelMap, Pixel};
pub use crate::scheduler::scheduler::SchedulerFrontend;

pub use crate::property::ez_properties::EzProperties;
pub use crate::property::ez_property::EzProperty;

pub use crate::states::definitions::{CallbackConfig, KeyMap, LayoutMode, LayoutOrientation,
                                     VerticalAlignment, HorizontalAlignment, VerticalPosHint,
                                     HorizontalPosHint, SizeHint};
pub use crate::states::ez_state::GenericState;
pub use crate::widgets::ez_object::EzObject;

