///
mod run;
mod scheduler;
mod widgets;
mod states;
mod property;
mod parser;


pub use crate::parser::parse_lang::{load_ui};
pub use crate::run::run::{run};

pub use crate::scheduler::definitions::{EzContext, EzPropertiesMap};

pub use crate::property::ez_properties::EzProperties;
pub use crate::property::ez_property::EzProperty;

pub use crate::states::definitions::CallbackConfig;
pub use crate::states::ez_state::GenericState;

pub use crate::widgets::ez_object::EzObject;

