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
//! 1. [How to use EzTerm](#how_to_use)
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
//!         4. [Widget overview](#widget_overview)
//!         5. [Sizing](#sizing)
//!             1. [Relative sizing: size hints](#sizing_relative)
//!             2. [Auto-scaling](#sizing_scaling)
//!             3. [Absolute size](#sizing_absolute)
//!         6. [Positioning](#)
//!             1. [Automatic positioning: layout modes](#positioning_automatic)
//!             2. [Relative positioning: position hints](#positioning_relative)
//!             3. [Absolute positions](#positioning_absolute)
//!             4. [Adjusting position: aligning and padding](#positioning_adjusting)
//!         7. [Keyboard selection](#keyboard_selection)
//!         8. [Binding properties](#binding_properties)
//!     3. [Scheduler](#scheduler)
//!         1. [Widget states and the State Tree](#scheduler_states)
//!         2. [Using the scheduler object](#scheduler_object)
//!         3. [Managing callbacks](#scheduler_callbacks)
//!             1. [General callback structure][#scheduler_callbacks_structure]
//!             2. [Callback config][#scheduler_callbacks_config]
//!             3. [On_keyboard_enter][#scheduler_callbacks_enter]
//!             4. [On_left_mouse_click][#scheduler_callbacks_left]
//!             5. [On_press][#scheduler_callbacks_press]
//!             6. [On_select][#scheduler_callbacks_select]
//!             7. [On_deselect][#scheduler_callbacks_deselect]
//!             8. [On_right_mouse_click][#scheduler_callbacks_right]
//!             9. [On_hover][#scheduler_callbacks_hover]
//!             10. [On_drag][#scheduler_callbacks_drag]
//!             11. [On_scroll_up][#scheduler_callbacks_up]
//!             12. [On_scroll_down][#scheduler_callbacks_down]
//!             13. [On_value_change][#scheduler_callbacks_value]
//!             14. [Custom key binds][#scheduler_callbacks_keymap]
//!             15. [Property binds][#scheduler_callbacks_property]
//!         4. [Managing scheduled tasks](#scheduler_tasks)
//!         5. [Opening popups](#scheduler_popups)
//!         6. [Creating widgets programmatically](#scheduler_widgets_from_code)
//!         7. [Creating custom properties](#)(#scheduler_properties)
//!         8. [Updating widgets](#scheduler_updating)
//!         9. [Managing widget selection](#scheduler_selection)
//!     4. [Global (key)bindings](#)
//! 2. [Reference]
//!     1. [Layouts]
//!         1. [General]
//!         2. [General - scrolling]
//!         3. [General - Properties]
//!         4. [Box Layout]
//!         5. [Stack Layout]
//!         6. [Table Layout]
//!         7. [Float Layout]
//!         8. [Tab Layout]
//!         9. [Screen Layout]
//!     2. Widgets
//!         1. [General]
//!         2. [General - Properties]
//!         3. [Label widget]
//!         4. [Button widget]
//!         5. [Checkbox widget]
//!         6. [Radio button widget]
//!         7. [Slider widget]
//!         8. [Text input widget]
//!         9. [Dropdown widget]
//!         10. [Progress bar widget]
//!         11. [Canvas widget]
//! 4. Examples
//!
//!
//! <a name="how_to_use"></a>
//! ## 1. How to use
//!
//! This section will explain how to use this framework step-by-step. This concerns only the basics.
//! There will be links to other doc pages showing more advanced uses of the various components. It
//! might be easiest to read this section first, then use the examples to get you started on your
//! own UI, and finally using the docs of specific components to fill in any gaps as you work on
//! your project.
//!
//! <a name="structure"></a>
//! ### 1.1 The structure on an EzTerm project
//!
//! An EzTerm project consists of three parts:
//! - UI config files (files with the '.ez' extension)
//! - UI Rust module(s)
//! - Your actual app (also Rust modules)
//!
//! <a name="ui_config_files"></a>
//! #### 1.1.1 UI config files
//!
//! UI config files have the '.ez' extension. They define what your UI will look like using layouts
//! and widgets. You can have as many .ez files as you like, so you can split up your UI along
//! multiple files. The docs for the .ez file syntax can be found under [ez_lang]. It helps looking
//! at the examples as well.
//!
//! When you compile your project, the .ez files are automatically merged into your binary, so you
//! do not have to ship them alongside your executable. In order to merge the .ez files into your
//! binary, cargo needs to know where they are. You declare this in an environment variable before
//! you compile (EZ_FOLDER). Let's say you put the .ez files in your project root in a folder
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

//! <a name="ui_rust_modules"></a>
//!  #### 1.1.2 UI Rust module(s)
//!
//! We now have our .ez files describing what our UI should look like. Now we need a rust module
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
//! Initializing- and starting the UI are separate steps, because you might want to use the
//! initialized [Scheduler] to schedule callbacks, register new [EzProperty], etc., before you
//! actually start the UI. More on the Scheduler will follow later, for now we will only note that
//! callbacks can be closures or fully defined functions. If you will make use of full functions as
//! callbacks you could define them in this same module, or a separate one as you like.
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
//! a scheduled task (e.g. run every 10 seconds without user input). Your app can communicate with
//! the UI through [EzProperty]. For example, you could create a 'usize' [EzProperty] and bind it
//! to the 'value' parameter of a [ProgressBar] widget. Then, if your app increments this property,
//! the progress bar widget will update in the UI automatically. This will all be explained later.
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
//! <a name="small_example"></a>
//! #### 1.1.4 Minimal example
//!
//! Now that we know the structure of an EzTerm project, we'll create the smallest working example
//! possible to get the structure into our fingers. After that we will move on to explain the
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
//! Cargo needs to know the location of our .ez files so it can merge them into the binary.
//! Now run the following cargo command in any OS terminal:
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
//! can find every possible property of each widget in the docs (see table of contents on the top of
//! this page).
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
//! when we discuss callbacks and EzProperties. Don't worry if the properties look unfamiliar,
//! we'll get into them later; for now we are just discussing the basics of the syntax.
//!
//! <a name="ez_language_templates"></a>
//! #### 1.2.2 Templates
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
//!         auto_scaling: true, true
//!     - Label:
//!         text: Label 2
//!         fg_color: yellow
//!         bg_color: blue
//!         auto_scaling: true, true
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
//!     auto_scaling: true, true
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
//! - MyCustomLabel:
//!     text: Red Label
//!     fg_color: red
//! ```
//! The text of the above label will be red, because the fg_color defined when using a template
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
//!     auto_scaling: true, true
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
//! short overview of the layout modes (for detailed info, see the dedicated entries in the table
//! of contents at the top of this page).
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
//! - Layout:
//!     mode: box
//!     orientation: vertical
//!     - Button:
//!         text: Option 1
//!     - Button:
//!         text: Option 2
//!     - Button:
//!         text: Option 3
//! ```
//! It can often be useful to nest Box layouts in other Box layouts to divide the screen into
//! rectangles. Let's say for example we want two menus, one on the left side of the screen and one
//! on the right. We could divide the screen horizontally with a Box layout, and then add two
//! vertical Box layouts (one menu for each side of the screen):
//! ```
//! - Layout:
//!     mode: box
//!     orientation: horizontal
//!     - Layout:
//!         mode: box
//!         orientation: vertical
//!         - Button:
//!             text: Left option 1
//!         - Button:
//!             text: Left option 2
//!         - Button:
//!             text: Left option 3
//!     - Layout:
//!         mode: box
//!         orientation: vertical
//!         - Button:
//!             text: Right option 1
//!         - Button:
//!             text: Right option 2
//!         - Button:
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
//! - Layout:
//!     mode: stack
//!     orientation: lr-tb
//!     - Label:
//!         text: Hi,
//!         auto_scale: true, true
//!     - Label:
//!         text: The size
//!         auto_scale: true, true
//!     - Label:
//!         text: Of these labels
//!         auto_scale: true, true
//!     - Label:
//!         text: Keeps increasing in length!
//!         auto_scale: true, true
//! ```
//! The widgets will be stacked automatically. If you resize the window, the row and column each
//! widget occupies may change, as the stack layout rearranges the widgets, but the orientation
//! will always be respected. As you can see, stack layouts are an easy way to make sure your
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
//! ##### 1.2.3.4 Tab mode
//!
//! Tab mode creates tabs for you based on child layouts. This means that in tab mode, you can only
//! add other layouts, not individual widgets. A tab button will automatically be created for each
//! child layout, using the ID property of the child layout as a tab name. Here is an example with
//! two tabs:
//! ```
//! - Layout:
//!     id: my_tab_layout
//!     mode: tab
//!     active_tab: Tab one
//!     - Layout:
//!         id: Tab one
//!         mode: box
//!         - Label:
//!             text: Hello tab one!
//!     - Layout:
//!         id: Tab two
//!         mode: box
//!         - Label:
//!             text: Hello tab two!
//! ```
//! Note that the active_tab property is optional, by default the first tab is active. Users can
//! then switch tabs using the tab header buttons that are created automatically. Keep in mind that
//! these buttons are three pixels high, so the effective height of your layout will be three
//! pixels smaller.
//!
//! It is possible to change the active tab from code. Although callbacks will be discussed in a
//! later chapter, we'll look at an example just for reference:
//! ```
//! use ez_term::*;
//! fn change_tab_callback(context: EzContext) {
//!     let state = context.state_tree.get_by_id("my_tab_layout").as_layout_mut();
//!     state.set_active_tab("Tab two");
//!     state.update(context.scheduler);
//! }
//! ```
//!
//! <a name="ez_language_screen"></a>
//! ##### 1.2.3.5 Screen mode
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
//! (for example in a main menu). Here is an example of the EzLang and rust code needed for this:
//!
//! **EzLang**
//! ```
//! - Layout:
//!     id: my_screen_layout
//!     mode: screen
//!     active_screen: screen_1
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
//! // We load the UI from the .ez files
//! let (root_layout, mut state_tree, mut scheduler) = load_ui();
//!
//! // We update the callbacks for the buttons, using the functions defined below
//! scheduler.update_callback_config("to_screen_2_btn",
//!                                 CallbackConfig::from_on_press(Box::new(to_screen_two_callback)));
//!
//! scheduler.update_callback_config("to_screen_1_btn",
//!                                 CallbackConfig::from_on_press(Box::new(to_screen_one_callback)));
//!
//! // We run the UI
//! run(root_layout, state_tree, scheduler);
//!
//! // We define the callback functions. We could also use closures if we wanted to.
//! fn to_screen_one_callback(context: EzContext) {
//!     let state = context.state_tree.get_by_id("my_screen_layout").as_layout_mut();
//!     state.set_active_screen("screen_1");
//!     state.update(context.scheduler);
//! }
//!
//! fn to_screen_two_callback(context: EzContext) {
//!     let state = context.state_tree.get_by_id("my_screen_layout").as_layout_mut();
//!     state.set_active_screen("screen_2");
//!     state.update(context.scheduler);
//! }
//! ```
//! This example used a button callback, but it could of course be any kind of callback or
//! scheduled task. More on callbacks and scheduling tasks later (see table of contents at top of
//! page).
//!
//! <a name="ez_language_scrolling"></a>
//! ##### 1.2.3.6 Scrolling
//!
//! Scrolling is *not* a dedicated layout mode. Instead, it is a property that can be enabled for
//! Box, Stack, Table and Float layouts. If vertical scrolling is enabled and the content height
//! is bigger than the layout height, a vertical scrolling bar will be created automatically. The
//! same is true if horizontal scrolling is enabled and the width of the content is larger than the
//! width of the layout. Scrolling can be enabled with the following properties:
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
//!     scrolling_y: true
//!     - Label:
//!         auto_scale_height: true
//!         from_file: lorem_ipsum.txt
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
//! text will be automatically wrapped to respect word boundaries. Formatted text and justify
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
//! adjusted. A slider with minimum 0, maximum 20, and step 5, has 5 possible values (0, 5, 10
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
//! ##### 1.2.3.1 Relative sizing: size hints
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
//! above example shorter by removing the 'size_hint_x' properties, because they are already set to
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
//! ##### 1.2.3.2 Auto scaling
//!
//! All widgets support auto-scaling; when enabled, they will automatically size themselves to their
//! contents. Auto-scaling is turned off by default, and overwrites size_hint if enabled.
//! A widget with auto-scaling enabled for one of both axes (auto_scale_height and/or
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
//! property that allows us to set both at the same time in the format: "auto_scale: width, height":
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
//! ##### 1.2.3.3 Absolute size:
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
//! The available setting for pos_hint_x are:
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
//! ##### 1.2.6.2 Absolute positioning: manual positions
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
//! <a name="positioning_adjusting"></a>
//! ##### 1.2.6.3 Adjusting position: padding and aligning
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
//!
//! It is possible to bind one property to another in EzLang, as long as the properties are of the
//! same type (you can find the type of each property in [Reference]. Here is an example of binding
//! the width of one widget to another:
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
//! To provide a relative path there are two possibilities: "self", "parent". "self" refers to the
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
//! that you have scheduled with the scheduler. We will discuss these in the coming [Scheduler]
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
//! state tree to get a widget state using the "get_by_id" or "get_by_path" methods. So if we wanted
//! to change the text of a label with the id "my_label" when initializing the UI, we would do this:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//! let label_state = state_tree.get_by_id_mut("my_label").as_label_mut();
//! label_state.set_text("new text".to_string());
//! ```
//! Note that after getting the label state, we have to call the "to_label_mut" method to change
//! the generic state into a label state. This is because the state tree contains generic states (due
//! to Rusts strict type requirements), and so we have to cast the state into the right type before
//! we use it. This will become second nature quickly when working with EzTerm.
//!
//! We will describe callbacks in details below, but we will note for now that the state tree is
//! available in callbacks through the "context" parameter. So if we wanted to change the text of
//! our label from a callback it would look like this:
//! ```
//! use ez_term::*;
//! fn my_callback(context: EzContext) -> bool {
//!
//!     let label_state = context.state_tree.get_by_id_mut("my_label").as_label_mut();
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
//! Now we will start putting our new knowledge about states to use when discussing the features of
//! the scheduler.
//!
//! <a name="scheduler_object"></a>
//! #### 1.3.2 Using the scheduler object:
//!
//! The scheduler is an object which you can use in two places: when initializing the UI, and when
//! inside of a callback. Here is an example of using the scheduler when initializing the UI:
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
//! fn my_callback(context: EzContext) -> bool {
//!
//!     context.scheduler.set_selected_widget("my_widget", None);
//!     true
//! }
//! ```
//!
//! Now that we know how to use the scheduler object when initializing the UI and from callbacks,
//! we will look at the manu things we can do with the scheduler. Here is a short overview of the
//! features we will look at:
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
//! All callbacks take an "context: EzContext" parameter. The EzContext object contains the
//! StateTree object (context.state_tree) and the Scheduler object (context.scheduler). We can use
//! these to manage the UI, as is being explained in this chapter. The EzContext also contains the
//! path of the widget for which the callback was called (context.widget_path). Some callbacks have
//! more parameters (for example, mouse callbacks have a mouse_pos parameter), but we will discuss
//! these separately for each callback when relevant.
//!
//! Finally, each callback returns a bool. The bool indicates whether the event should be consumed.
//! If the event is not consumed, the widget is allowed to execute its default behavior if it has
//! any. For example, the checkbox has default "on_press" behavior: when pressed, it will toggle
//! on/off. If you bind a custom "on_press" callback for a checkbox, you control whether the default
//! behavior will be executed by returning 'true' (allowed to run) or 'false' (not allowed to run).
//! This gives you the option to overwrite default widget behavior, or supplement it. If you want to
//! know if returning true for a widget callback would overwrite default behavior, see the
//! [reference] entry for that widget and check the callback chapter.
//!
//! To summarize, here are the two examples of default callback structure:
//!
//! **Callback from closure**
//! ```
//! use ez_term::*;
//! let my_callback = |context: EzContext| {
//!     true
//! };
//! ```
//! **Callback from function**
//! ```
//! use ez_term::*;
//! fn my_callback(context: EzContext) -> bool {
//!     true
//! };
//! ```
//!
//! Now that we know what a callback should look like, let's see how to set new callbacks.
//!
//! <a name="scheduler_callbacks_config"></a>
//! ##### 1.3.3.2 Callback config
//!
//! Each widget active in your UI has an associated callback config. This config contains all
//! callbacks that are active for that widget. Initially, the callback config for each widget is
//! empty. To manage the callbacks for a widget, we create a new CallbackConfig object and load
//! our callbacks into it. When we have a new CallbackConfig object, we can either overwrite the
//! current callback config of a widget, or update it. Overwriting it will delete the current config.
//! When updating, any callbacks configured in the new config will be set on the current config
//! (while leaving the the others intact).
//! Let's say we want to set an "on_press" callback on a button with the ID: "my_button".
//! We want the callback to change the text on a label. This is how we would do it:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = |context: EzContext| {
//!
//!     let label_state = context.state_tree.get_by_id_mut("my_label").as_label_mut();
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
//! fn my_callback(context: EzContext) -> bool {
//!
//!     let label_state = context.state_tree.get_by_id_mut("my_label").as_label_mut();
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
//! let counter: usize = 0;
//! let my_callback = move |context: EzContext| {
//!
//!     let label_state = context.state_tree.get_by_id_mut("my_label").as_label_mut();
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
//! let my_callback = move |context: EzContext| {
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
//! fn my_callback(context: EzContext) -> bool {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
//! scheduler.update_callback_config("my_button", new_callback_config);
//! ```
//! <a name="scheduler_callbacks_left"></a>
//! ##### 1.3.3.4 On_left_mouse_click
//!
//! This callback is activated when a widget is clicked by the left mouse button. Keep in mind that
//! when a widget is clicked, any layouts underneath it are also clicked. The root layout is the
//! first to receive the mouse click event, followed by sub layouts, and finally the widget. If any
//! layout has a callback that returns true, the event is consumed and does not reach further
//! layouts or widgets. To set this callback with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: EzContext, mouse_pos: Coordinates| {
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
//! fn my_callback(context: EzContext, mouse_pos: Coordinates) -> bool {
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
//! let my_callback = move |context: EzContext| {
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
//! fn my_callback(context: EzContext) -> bool {
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
//! Is a widget was selected by keyboard, this argument will be None. If it was selected by mouse,
//! it will contains coordinates.To set this callback with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: EzContext, mouse_pos: Option<Coordinates>| {
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
//! fn my_callback(context: EzContext, mouse_pos: Option<Coordinates>) -> bool {
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
//! let my_callback = move |context: EzContext| {
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
//! fn my_callback(context: EzContext) -> bool {
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
//! layouts or widgets. To set this callback with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: EzContext, mouse_pos: Coordinates| {
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
//! fn my_callback(context: EzContext, mouse_pos: Coordinates) -> bool {
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
//! layouts or widgets. To set this callback with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: EzContext, mouse_pos: Coordinates| {
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
//! fn my_callback(context: EzContext, mouse_pos: Coordinates) -> bool {
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
//! previous position, you know which direction the drag is going.
//! To set this callback with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: EzContext, previous_mouse_pos: Option<Coordinates>,
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
//! fn my_callback(context: EzContext, previous_mouse_pos: Option<Coordinates>,
//!                mouse_pos: Coordinates) -> bool {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_drag(Box::new(my_callback));
//! scheduler.update_callback_config("my_label", new_callback_config);
//! ```
//! <a name="scheduler_callbacks_up"></a>
//! ##### 1.3.3.11 On_scroll_up
//!
//! This callback is activated when a widget is scrolled up by the mouse. Keep in mind that
//! when a widget is scrolled, any layouts underneath it are also scrolled. The root layout is the
//! first to receive the scroll event, followed by sub layouts, and finally the widget. If any
//! layout has a callback that returns true, the event is consumed and does not reach further
//! layouts or widgets. To set this callback with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: EzContex| {
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
//! fn my_callback(context: EzContext) -> bool {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_scroll_up(Box::new(my_callback));
//! scheduler.update_callback_config("my_label", new_callback_config);
//! ```
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
//! let my_callback = move |context: EzContex| {
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
//! fn my_callback(context: EzContext) -> bool {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_scroll_down(Box::new(my_callback));
//! scheduler.update_callback_config("my_label", new_callback_config);
//! ```
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
//! let my_callback = move |context: EzContext| {
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
//! fn my_callback(context: EzContext) -> bool {
//!
//!     true
//! };
//! let new_callback_config = CallbackConfig::from_on_value_change(Box::new(my_callback));
//! scheduler.update_callback_config("my_checkbox", new_callback_config);
//! ```
//! <a name="scheduler_callbacks_keymap"></a>
//! ##### 1.3.3.14 Custom key binds
//!
//! Custom keymaps allow you to bind keyboard keys to a callback. Keep in mind that for this to work,
//! a widget must already be selected; only then will it receive the keyboard event.
//! To bind a custom key, you must first create a KeyMap object This KeyMap object is then inserted
//! into a CallbackConfig object, which is bound to a widget as normal. To bind for example the "a"
//! key with a closure:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! let my_callback = move |context: EzContext, keycode: KeyCode| {
//!
//!     true
//! };
//!
//! let mut keymap = KeyMap::new();
//! keymap.insert(KeyCode::Char('a'), Box::new(my_callback));
//!
//! let new_callback_config = CallbackConfig::from_keymap(keymap);
//! scheduler.update_callback_config("my_checkbox", new_callback_config);
//! ```
//! To do the same with a function:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: EzContext, keycode: KeyCode) -> bool {
//!
//!     true
//! };
//!
//! let mut keymap = KeyMap::new();
//! keymap.insert(KeyCode::Char('a'), Box::new(my_callback));
//!
//! let new_callback_config = CallbackConfig::from_keymap(keymap);
//! scheduler.update_callback_config("my_checkbox", new_callback_config);
//! ```
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
//! let my_callback = |context: EzContext| {
//!
//!     true
//! };
//!
//! let state = state_tree.get_by_id("my_label").as_label_mut();
//! state.size.height.bind(Box::new(my_callback), &mut scheduler);
//! ```
//! The same example but using a function:
//! ```
//! use ez_term::*;
//! let (root_widget, mut state_tree, mut scheduler) = load_ui();
//!
//! fn my_callback(context: EzContext) -> bool {
//!
//!     true
//! };
//!
//! let state = state_tree.get_by_id("my_label").as_label_mut();
//! state.size.height.bind(Box::new(my_callback), &mut scheduler);
//! ```
//!
//! ### 1.3.4 Managing scheduled tasks
//!
//! Scheduled tasks are closures or functions that are executed according to time parameters. There
//! are three types of schedules tasks:
//!
//! - Single execution
//! - Regular execution
//! - Threaded execution
mod run;
mod scheduler;
mod widgets;
mod states;
mod property;
mod parser;


pub use crate::parser::parse_lang::load_ui;
pub use crate::run::run::run;

pub use crate::run::definitions::Coordinates;
pub use crossterm::event::KeyCode;

pub use crate::scheduler::definitions::{EzContext, EzPropertiesMap};
pub use crate::scheduler::scheduler::SchedulerFrontend;

pub use crate::property::ez_properties::EzProperties;
pub use crate::property::ez_property::EzProperty;

pub use crate::states::definitions::{CallbackConfig, KeyMap};
pub use crate::states::ez_state::GenericState;
pub use crate::widgets::ez_object::EzObject;

