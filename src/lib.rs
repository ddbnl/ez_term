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
//! either continue reading the general tutorial at the [EzLanguage](#ez_lang) section, or use
//! the [Examples](#examples) if you prefer that over reading.
//!
//! Once you are familiar with the basics and working on your own projects, you can use the
//! [Reference](#reference) section to look up details on available properties, callbacks, etc.
//!
//! **Docs table of contents:**
//! 1. [How to use EzTerm](#how_to_use)
//!     1. [Project structure](#structure)
//!     2. [Minimal example](#minimal_example)
//!     3. [Ez language](#)
//!         1. [General](#)
//!         2. [Templates](#)
//!         3. [Layouts Modes](#)
//!             1. [Box Mode](#)
//!             2. [Stack Mode](#)
//!             3. [Table Mode](#)
//!             4. [Float Mode](#)
//!             5. [Tab Mode](#)
//!             6. [Screen Mode](#)
//!             7. [Scrolling](#)
//!         4. [Widget overview](#)
//!         5. [Sizing](#)
//!             1. [Relative sizing: size hints]
//!             2. [Auto-scaling]
//!             3. [Absolute size]
//!         6. [Positioning](#)
//!             1. [Automatic positioning: layout modes]
//!             2. [Relative positioning: position hints]
//!             3. [Absolute positions]
//!             4. [Adjusting position: aligning and padding]
//!         7. [Keyboard selection]
//!         8. [Binding properties](#)
//!     4. [Scheduler](#)
//!         1. [Setting callbacks]
//!         2. [Opening popups]
//!         3. [Creating widgets programmatically]
//!         4. [Creating ez properties](#)
//!     5. [Global (key)bindings](#)
//! 2. [Reference]
//!     1. Layouts
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
//! #### 1.1.1 Project structure: UI config files
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

//!  #### 1.1.2 Project structure: UI Rust module(s)
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
//! #### 1.1.3 Project structure: Your app
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
//! ### 1.2 Minimal example
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
//! ## 2. Ez language
//!
//! ### 2.1 Basics
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
//! ### 2.2 Templates
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
//! ### 2.3 Layout modes
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
//! #### 2.3.1 Box mode
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
//! #### 2.3.2 Stack mode
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
//! #### 2.3.3 Table mode
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
//! #### 2.3.4 Float mode
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
//! #### 2.3.4 Tab mode
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
//! #### 2.3.5 Screen mode
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
//! #### 2.3.6 Scrolling
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
//!
//! **Text input:**
//! The text input is essentially an interactive Label. The user can select the input through mouse or
//! keyboard, and then type content into it. Selecting the widget will spawn a cursor that the user
//! can control with the left/right buttons. Backspace and delete will remove content as expected.
//! If the text of the input grows larger than the widget, the view will automatically move with
//! the cursor.
//!
//! **Button:**
//! Clickable button; displays a small animation when clicked. Bind an on_press callback to a button
//! to make it functional. This will be explained in the callback section of the scheduler (see
//! table of contents).
//!
//! **Checkbox:**
//! A clickable switch; has two states: on or off. Bind an on_value_change callback to a checkbox
//! to make it functional. This will be explained in the callback section of the scheduler (see
//! table of contents).
//!
//! **Radio button:**
//! A radio button is also a clickable switch that can be either on or off. The difference with the
//! checkbox is that the radio button is meant to be part of a group (which can be set using the
//! 'group' EzLang property). Only one radio button can be active in a group, so when the user
//! clicks one, the others will be off. To make the radio buttons functional, bind on_value_change
//! callbacks to each button in the group. Only the radio button that became active will receive
//! an on_value_change event.
//!
//! **Dropdown:**
//! A dropdown is a list of items (including an optional empty choice). Initially, only the active
//! choice is displayed. When clicked, a dropdown list becomes visible, from which the user can
//! choose a new option. Use the 'options' EzLang property to set the possible choices. Use the
//! 'allow_none' EzLang property to enable or disable an empty choice. Use the 'choice' EzLang
//! property to set the initial choice; if you don't do this, the empty choice will be active if
//! 'allow_none' is true; if not, then the first option will be active by default.
//!
//! **Slider:**
//! The slider allows a user to choose a numerical value by dragging the slider to the left or
//! the right (using keyboard or mouse). A slider has a value, a minimum value, a maximum value,
//! and a step value. The step value determines the minimum amount by which the value can be
//! adjusted. A slider with minimum 0, maximum 20, and step 5, has 5 possible values (0, 5, 10
//! , 15, 20).
//!
//! **Canvas**:
//! The canvas is a widget that you either 'paint' yourself, or that gets its' content from a text
//! file. You can set the content of a canvas programmatically through its' "set_content" function.
//! You can load the content from a text file using the EzLang property 'from_file'.
//!
//!
//! ### 2.5 Sizing
//!
//! Now we'll learn about the different ways to size widgets and layouts.
//!
//! There are three ways to size widgets and layouts:
//! - Size relative to parent layout;
//! - Auto scale to content;
//! - Absolute size.
//!
//! #### 2.3.1 Relate sizing: size hints
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
//! #### 2.3.2 Auto scaling
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
//! #### 2.3.3 Absolute size:
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
//! ### 2.6 Positioning:
//!
//! There a multiple ways to control the positioning of widgets; which ways are available depends
//! on the mode a layout is in. There are four ways to control positioning:
//! - Automatic positioning with layout modes
//! - Relative positioning with position hints
//! - Absolute positioning with manual positions
//! - Adjust position through padding and aligning
//!
//! #### 2.6.1 Automatic positioning: layout modes:
//!
//! Most layout modes do not support manual positioning or relative positioning. This is because
//! the point of these layouts is that they do the work for you. Only the float layout, which exists
//! specifically to give you manual control over position, supports position hints or the manual
//! position property. The other widgets, such as box mode, stack mode and table mode, will handle
//! the positioning for you (see their docs for more info). It is however possible to adjust the
//! position of widgets in these modes; see the entry on padding and aligning below for more on that.
//!
//! #### 2.6.2 Relative positioning: position hints
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
//! #### 2.6.2 Absolute positioning: manual positions
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
//! #### 2.6.3 Adjusting position: padding and aligning
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
//! ### 2.7 Keyboard selection:
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
//! ### 2.8 Binding properties:
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
pub use crossterm::event::KeyCode;

pub use crate::scheduler::definitions::{EzContext, EzPropertiesMap};
pub use crate::scheduler::scheduler::SchedulerFrontend;

pub use crate::property::ez_properties::EzProperties;
pub use crate::property::ez_property::EzProperty;

pub use crate::states::definitions::CallbackConfig;
pub use crate::states::ez_state::GenericState;
pub use crate::widgets::ez_object::EzObject;

