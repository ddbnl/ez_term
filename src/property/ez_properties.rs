//! A module implementing the [EzProperties] enum.
use crossterm::style::Color;

use crate::property::ez_property::EzProperty;
use crate::states::definitions::{HorizontalAlignment, HorizontalPosHint, LayoutMode, LayoutOrientation, VerticalAlignment, VerticalPosHint};


/// An enum that contains all possible implementations of the generic [EzProperty].
///
/// This is used by the scheduler which keeps the collection of all active EzProperty objects at
/// runtime. This enum contains functions to downcast to specific variants. This is possible
/// because it is always known which variant something is. E.g., a "height" EzProperty is known to
/// always be EzProperty<usize>. We need the enum to keep all EzProperty objects in one HashMap.
/// Examples of setting and getting an EzProperty from an EzProperties enum:
/// ```
/// ez_properties.as_usize_mut.set(10);
/// ```
/// ```
/// let number = *ez_properties.as_usize.get();
/// ```
#[derive(Clone, Debug)]
pub enum EzProperties {

    /// usize EzProperty
    Usize(EzProperty<usize>),

    /// f64 EzProperty
    F64(EzProperty<f64>),

    /// Bool EzProperty
    Bool(EzProperty<bool>),

    /// String EzProperty
    String(EzProperty<String>),

    /// CrossTerm Color EzProperty
    Color(EzProperty<Color>),

    /// [LayoutMode] EzProperty
    LayoutMode(EzProperty<LayoutMode>),

    /// [LayoutMode] EzProperty
    LayoutOrientation(EzProperty<LayoutOrientation>),

    /// [VerticalAlignment] EzProperty
    VerticalAlignment(EzProperty<VerticalAlignment>),

    /// [HorizontalAlignment] EzProperty
    HorizontalAlignment(EzProperty<HorizontalAlignment>),

    /// [VerticalPosHint] EzProperty
    VerticalPosHint(EzProperty<VerticalPosHint>),

    /// [HorizontalPosHint] EzProperty
    HorizontalPosHint(EzProperty<HorizontalPosHint>),

    /// [SizeHint] EzProperty
    SizeHint(EzProperty<Option<f64>>),
}
impl EzProperties {

    /// Get a [EzProperty<usize>] ref from this enum. You must be sure this is a usize property
    /// or it will panic.
    pub fn as_usize(&self) -> &EzProperty<usize> {
        if let EzProperties::Usize(i) = self { i }
            else {panic!("Wrong property, expected UsizeProperty") }
    }

    /// Get a mutable ref [EzProperty<usize>] from this enum. You must be sure this is a usize
    /// property or it will panic.
    pub fn as_usize_mut(&mut self) -> &mut EzProperty<usize> {
        if let EzProperties::Usize(i) = self { i }
            else { panic!("Wrong property, expected UsizeProperty") }
    }

    /// Get a [EzProperty<f64>] ref from this enum. You must be sure this is an f64 property
    /// or it will panic.
    pub fn as_f64(&self) -> &EzProperty<f64> {
        if let EzProperties::F64(i) = self { i }
        else {panic!("Wrong property, expected F64Property") }
    }

    /// Get a mutable ref [EzProperty<f64>] from this enum. You must be sure this is an f64
    /// property or it will panic.
    pub fn as_f64_mut(&mut self) -> &mut EzProperty<f64> {
        if let EzProperties::F64(i) = self { i }
        else { panic!("Wrong property, expected F64Property") }
    }

    /// Get a mutable ref [EzProperty<String>] from this enum. You must be sure this is a String
    /// property or it will panic.
    pub fn as_string(&self) -> &EzProperty<String> {
        if let EzProperties::String(i) = self { i }
            else { panic!("Wrong property, expected StringProperty") }
    }

    /// Get a mutable ref [EzProperty<String>] from this enum. You must be sure this is a String
    /// property or it will panic.
    pub fn as_string_mut(&mut self) -> &mut EzProperty<String> {
        if let EzProperties::String(i) = self { i }
            else { panic!("Wrong property, expected StringProperty") }
    }

    /// Get a mutable ref [EzProperty<bool>] from this enum. You must be sure this is a bool
    /// property or it will panic.
    pub fn as_bool(&self) -> &EzProperty<bool> {
        if let EzProperties::Bool(i) = self { i }
        else { panic!("Wrong property, expected BoolProperty") }
    }

    /// Get a mutable ref [EzProperty<bool>] from this enum. You must be sure this is a bool
    /// property or it will panic.
    pub fn as_bool_mut(&mut self) -> &mut EzProperty<bool> {
        if let EzProperties::Bool(i) = self { i }
        else { panic!("Wrong property, expected BoolProperty") }
    }

    /// Get a mutable ref [EzProperty<Color>] from this enum. You must be sure this is a Color
    /// property or it will panic.
    pub fn as_color(&self) -> &EzProperty<Color> {
        if let EzProperties::Color(i) = self { i }
        else { panic!("Wrong property, expected ColorProperty") }
    }

    /// Get a mutable ref [EzProperty<Color>] from this enum. You must be sure this is a color
    /// property or it will panic.
    pub fn as_color_mut(&mut self) -> &mut EzProperty<Color> {
        if let EzProperties::Color(i) = self { i }
        else { panic!("Wrong property, expected ColorProperty") }
    }

    /// Get a mutable ref [EzProperty<LayoutMode>] from this enum. You must be sure this
    /// is a LayoutMode property or it will panic.
    pub fn as_layout_mode(&self) -> &EzProperty<LayoutMode> {
        if let EzProperties::LayoutMode(i) = self { i }
        else { panic!("Wrong property, expected LayoutMode") }
    }

    /// Get a mutable ref [EzProperty<LayoutMode>] from this enum. You must be sure this
    /// is a LayoutMode property or it will panic.
    pub fn as_layout_mode_mut(&mut self) -> &mut EzProperty<LayoutMode> {
        if let EzProperties::LayoutMode(i) = self { i }
        else { panic!("Wrong property, expected LayoutMode") }
    }

    /// Get a mutable ref [EzProperty<LayoutOrientation>] from this enum. You must be sure this
    /// is a LayoutOrientation property or it will panic.
    pub fn as_layout_orientation(&self) -> &EzProperty<LayoutOrientation> {
        if let EzProperties::LayoutOrientation(i) = self { i }
        else { panic!("Wrong property, expected LayoutOrientation") }
    }

    /// Get a mutable ref [EzProperty<LayoutOrientation>] from this enum. You must be sure this
    /// is a LayoutOrientation property or it will panic.
    pub fn as_layout_orientation_mut(&mut self) -> &mut EzProperty<LayoutOrientation> {
        if let EzProperties::LayoutOrientation(i) = self { i }
        else { panic!("Wrong property, expected LayoutOrientation") }
    }

    /// Get a mutable ref [EzProperty<VerticalAlignment>] from this enum. You must be sure this
    /// is a VerticalAlignment property or it will panic.
    pub fn as_vertical_alignment(&self) -> &EzProperty<VerticalAlignment> {
        if let EzProperties::VerticalAlignment(i) = self { i }
        else { panic!("Wrong property, expected VerticalAlignmentProperty") }
    }

    /// Get a mutable ref [EzProperty<VerticalAlignment>] from this enum. You must be sure this
    /// is a VerticalAlignment property or it will panic.
    pub fn as_vertical_alignment_mut(&mut self) -> &mut EzProperty<VerticalAlignment> {
        if let EzProperties::VerticalAlignment(i) = self { i }
        else { panic!("Wrong property, expected VerticalAlignmentProperty") }
    }

    /// Get a mutable ref [EzProperty<HorizontalAlignment>] from this enum. You must be sure
    /// this is a HorizontalAlignment property or it will panic.
    pub fn as_horizontal_alignment(&self) -> &EzProperty<HorizontalAlignment> {
        if let EzProperties::HorizontalAlignment(i) = self { i }
        else { panic!("Wrong property, expected HorizontalAlignmentProperty") }
    }

    /// Get a mutable ref [EzProperty<HorizontalAlignment>] from this enum. You must be sure this
    /// is a HorizontalAlignment property or it will panic.
    pub fn as_horizontal_alignment_mut(&mut self) -> &mut EzProperty<HorizontalAlignment> {
        if let EzProperties::HorizontalAlignment(i) = self { i }
        else { panic!("Wrong property, expected HorizontalAlignmentProperty") }
    }

    /// Get a mutable ref [EzProperty<HorizontalPosHint>] from this enum. You must be sure this
    /// is a HorizontalPosHint property or it will panic.
    pub fn as_horizontal_pos_hint(&self) -> &EzProperty<HorizontalPosHint> {
        if let EzProperties::HorizontalPosHint(i) = self { i }
        else { panic!("Wrong property, expected HorizontalPosHintProperty") }
    }

    /// Get a mutable ref [EzProperty<HorizontalPosHint>] from this enum. You must be sure this
    /// is a HorizontalPosHint property or it will panic.
    pub fn as_horizontal_pos_hint_mut(&mut self)
        -> &mut EzProperty<HorizontalPosHint> {
        if let EzProperties::HorizontalPosHint(i) = self { i }
        else { panic!("Wrong property, expected HorizontalPosHintProperty") }
    }

    /// Get a mutable ref [EzProperty<VerticalPosHint>] from this enum. You must be sure this
    /// is a VerticalPosHint property or it will panic.
    pub fn as_vertical_pos_hint(&self) -> &EzProperty<VerticalPosHint> {
        if let EzProperties::VerticalPosHint(i) = self { i }
        else { panic!("Wrong property, expected VerticalPosHintProperty") }
    }

    /// Get a mutable ref [EzProperty<VerticalPosHint>] from this enum. You must be sure this
    /// is a VerticalPosHint property or it will panic.
    pub fn as_vertical_pos_hint_mut(&mut self)
                                      -> &mut EzProperty<VerticalPosHint> {
        if let EzProperties::VerticalPosHint(i) = self { i }
        else { panic!("Wrong property, expected VerticalPosHintProperty") }
    }

    /// Get a mutable ref [EzProperty<SizeHint>] from this enum. You must be sure this is a SizeHint
    /// property or it will panic.
    pub fn as_size_hint(&self) -> &EzProperty<Option<f64>> {
        if let EzProperties::SizeHint(i) = self { i }
        else { panic!("Wrong property, expected SizeHintProperty") }
    }

    /// Get a mutable ref [EzProperty<SizeHint>] from this enum. You must be sure this is a SizeHint
    /// property or it will panic.
    pub fn as_size_hint_mut(&mut self) -> &mut EzProperty<Option<f64>> {
        if let EzProperties::SizeHint(i) = self { i }
        else { panic!("Wrong property, expected SizeHintProperty") }
    }
}