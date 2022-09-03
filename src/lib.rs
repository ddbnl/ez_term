//! # EzTerm
//!
//! ## Wiki
//!
//! For a tutorial, examples, and API reference, see the wiki in the
//! [Github repo](https://github.com/ddbnl/ez_term/wiki)
mod parser;
mod property;
mod run;
mod scheduler;
mod states;
mod widgets;

pub use crate::parser::parse_lang::load_ui;
pub use crate::run::run::run;

pub use crate::run::definitions::Coordinates;
pub use crossterm::event::{KeyCode, KeyModifiers};
pub use crossterm::style::Color;

pub use crate::run::definitions::{Pixel, PixelMap, StateTree};
pub use crate::scheduler::definitions::{Context, EzPropertiesMap, ThreadedContext};
pub use crate::scheduler::scheduler::SchedulerFrontend;

pub use crate::property::ez_properties::EzProperties;
pub use crate::property::ez_property::EzProperty;

pub use crate::states::definitions::{
    CallbackConfig, HorizontalAlignment, HorizontalPosHint, KeyMap, LayoutMode, LayoutOrientation,
    SizeHint, VerticalAlignment, VerticalPosHint,
};
pub use crate::states::ez_state::GenericState;
pub use crate::widgets::ez_object::EzObject;
