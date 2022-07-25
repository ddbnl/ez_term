//! # EzTerm
//!
//! A terminal-UI framework focussed on simplicity. Create interfaces through YAML-ish config files
//! using smart-layouts and basic widgets. No need to specify specific sizes or positions (unless
//! you want to) and no need to write code except for callbacks. Focus on coding your app, not the
//! UI.
//!
//! **Docs table of contents:**
//! 1. [How to use](#how_to_use)
//!     1. [Project structure](#structure)
//!     2. [Minimal example](#minimal_example)
//!     3. [Ez language](#)
//!     4. [Scheduler](#)
//!         4.1 [Setting callbacks]
//!         4.1 [Creating widget programmatically]
//!         4.3. [Creating ez properties](#)
//!     6. [Global (key)bindings](#)
//! 2. Layouts
//!     1. [General]
//!     2. [General - scrolling]
//!     3. [General - Properties]
//!     4. [Box Layout]
//!     5. [Stack Layout]
//!     6. [Table Layout]
//!     7. [Float Layout]
//!     8. [Tab Layout]
//!     9. [Screen Layout]
//! 3. Widgets
//!     1. [General]
//!     2. [General - Properties]
//!     3. [Label widget]
//!     4. [Button widget]
//!     5. [Checkbox widget]
//!     6. [Radio button widget]
//!     7. [Slider widget]
//!     8. [Text input widget]
//!     9. [Dropdown widget]
//!     10. [Progress bar widget]
//!     11. [Canvas widget]
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
//!     let (root_widget, mut scheduler) = load_ui();
//!     run(root_widget, scheduler);
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

pub use crate::property::ez_properties::EzProperties;
pub use crate::property::ez_property::EzProperty;

pub use crate::states::definitions::CallbackConfig;
pub use crate::states::ez_state::GenericState;
pub use crate::widgets::ez_object::EzObject;

