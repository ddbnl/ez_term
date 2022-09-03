/// A module containing functions that handle user input.
mod input;

/// A module that interfaces with the actual terminal (using Crossterm)
mod terminal;

/// A module containing functions that handle selecting widgets
pub mod select;

/// A module containing the run loop and supporting functions
pub mod run;

/// A module containing structs for the StateTree, WidgetTree, CallbackTree and ViewTree
pub mod tree;

/// A module containing definitions used by run modules
pub mod definitions;
