use crate::run::definitions::{IsizeCoordinates, Pixel, PixelMap, Size};
use crate::states::definitions::{
    BorderConfig, ColorConfig, HorizontalAlignment, Padding, ScrollingConfig, VerticalAlignment,
};
use crate::states::ez_state::{EzState, GenericState};
use crossterm::style::Color;
use unicode_segmentation::UnicodeSegmentation;
use crate::parser::parse_properties::parse_color_property;

/// Resize an object according to its' size hint values.
pub fn resize_with_size_hint(state: &mut EzState, parent_width: usize, parent_height: usize) {
    let mut_state = state.as_generic_mut();
    if let Some(size_hint_x) = mut_state.get_size_hint().get_size_hint_x() {
        let raw_child_size = parent_width as f64 * size_hint_x;
        let child_size = raw_child_size.floor() as usize;
        mut_state.get_size_mut().set_width(child_size);
    }

    if let Some(size_hint_y) = mut_state.get_size_hint().get_size_hint_y() {
        let raw_child_size = parent_height as f64 * size_hint_y;
        let child_size = raw_child_size.floor() as usize;
        mut_state.get_size_mut().set_height(child_size);
    }
}

/// Set the positions of an object that uses pos_hint(s) using its parents proportions and position.
pub fn reposition_with_pos_hint(
    parent_width: usize,
    parent_height: usize,
    child_state: &mut dyn GenericState,
) {
    // Set x by pos_hint if any
    if let Some((keyword, fraction)) = child_state.get_pos_hint().get_pos_hint_x() {
        let initial_pos = match keyword {
            HorizontalAlignment::Left => 0,
            HorizontalAlignment::Right => parent_width - child_state.get_size().get_width(),
            HorizontalAlignment::Center => {
                (parent_width as f64 / 2.0).round() as usize
                    - (child_state.get_size().get_width() as f64 / 2.0).round() as usize
            }
        };
        let x = (initial_pos as f64 * fraction).round() as usize;
        child_state.get_position_mut().set_x(x);
    }
    // Set y by pos hint if any
    if let Some((keyword, fraction)) = child_state.get_pos_hint().get_pos_hint_y() {
        let initial_pos = match keyword {
            VerticalAlignment::Top => 0,
            VerticalAlignment::Bottom => parent_height - child_state.get_size().get_height(),
            VerticalAlignment::Middle => {
                (parent_height as f64 / 2.0).round() as usize
                    - (child_state.get_size().get_height() as f64 / 2.0).round() as usize
            }
        };
        let y = (initial_pos as f64 * fraction).round() as usize;
        child_state.get_position_mut().set_y(y);
    }
}

/// Add a border around a PixelMap.
pub fn add_border(mut content: PixelMap, config: &BorderConfig, colors: &ColorConfig) -> PixelMap {
    if content.is_empty() {
        return content;
    }
    // Create border elements
    let (foreground_color, background_color) =
        (colors.get_border_fg_color(), colors.get_border_bg_color());
    let horizontal_border = Pixel::new(
        config.get_horizontal_symbol(),
        foreground_color.clone(),
        background_color.clone(),
    );
    let vertical_border = Pixel::new(
        config.get_vertical_symbol(),
        foreground_color.clone(),
        background_color.clone(),
    );
    let top_left_border = Pixel::new(
        config.get_top_left_symbol(),
        foreground_color.clone(),
        background_color.clone(),
    );
    let top_right_border = Pixel::new(
        config.get_top_right_symbol(),
        foreground_color.clone(),
        background_color.clone(),
    );
    let bottom_left_border = Pixel::new(
        config.get_bottom_left_symbol(),
        foreground_color.clone(),
        background_color.clone(),
    );
    let bottom_right_border = Pixel::new(
        config.get_bottom_right_symbol(),
        foreground_color.clone(),
        background_color.clone(),
    );
    // Create horizontal borders
    for x in 0..content.len() {
        let mut new_x = vec![horizontal_border.clone()];
        for y in &content[x] {
            new_x.push(y.clone());
        }
        new_x.push(horizontal_border.clone());
        content[x] = new_x
    }
    // Create left border
    let mut left_border = vec![top_left_border];
    for _ in 0..content[0].len() - 2 {
        left_border.push(vertical_border.clone());
    }
    left_border.push(bottom_left_border);
    // Create right border
    let mut right_border = vec![top_right_border];
    for _ in 0..content[0].len() - 2 {
        right_border.push(vertical_border.clone())
    }
    right_border.push(bottom_right_border);
    // Merge all borders around the content
    let mut new_content = vec![left_border];
    for x in content {
        new_content.push(x);
    }
    new_content.push(right_border);
    new_content
}

/// Add padding around a PixelMap.
pub fn add_padding(
    mut content: PixelMap,
    padding: &Padding,
    bg_color: Color,
    fg_color: Color,
) -> PixelMap {
    if content.is_empty() {
        return content;
    }
    let padding_pixel = Pixel::new(" ".to_string(), fg_color, bg_color);

    // Create vertical padding
    let mut vertical_padding = Vec::new();
    for _ in 0..content[0].len() {
        vertical_padding.push(padding_pixel.clone());
    }
    for _ in 0..padding.get_padding_left() {
        content.insert(0, vertical_padding.clone());
    }
    for _ in 0..padding.get_padding_right() {
        content.push(vertical_padding.clone());
    }
    if padding.get_padding_top() != 0 {
        for x in content.iter_mut() {
            for _ in 0..padding.get_padding_top() {
                x.insert(0, padding_pixel.clone());
            }
        }
    }
    if padding.get_padding_bottom() != 0 {
        for x in content.iter_mut() {
            for _ in 0..padding.get_padding_bottom() {
                x.push(padding_pixel.clone());
            }
        }
    }
    content
}

/// Align the passed content horizontally within a desired total width. Return the aligned
/// [PixelMap] and an offset for how much the content moved horizontally. E.g. content:
///
/// XXX...
///
/// With halign [state::HorizontalAlignment::Middle] and total width 5 would return:
///
/// .XXX.
///
/// With offset 1.
pub fn align_content_horizontally(
    mut content: PixelMap,
    halign: HorizontalAlignment,
    total_width: usize,
    filler_symbol: String,
    fg_color: Color,
    bg_color: Color,
) -> (PixelMap, usize) {
    if content.len() >= total_width {
        return (content, 0);
    }
    let empty_pixel =
        Pixel::new(filler_symbol, fg_color, bg_color);
    let mut offset = 0;
    for i in 0..total_width - content.len() {
        match halign {
            // Widgets are aligned left by default
            HorizontalAlignment::Left => {}
            // We align right by filling out empty space from the left
            HorizontalAlignment::Right => {
                content.insert(0, Vec::new());
                offset += 1;
                for _ in 0..content.last().unwrap().len() {
                    content.first_mut().unwrap().push(empty_pixel.clone());
                }
            }
            // We align in the center by filling out empty space alternating left and right
            HorizontalAlignment::Center => {
                if i % 2 == 0 {
                    content.push(Vec::new());
                    for _ in 0..content[0].len() {
                        content.last_mut().unwrap().push(empty_pixel.clone());
                    }
                } else {
                    content.insert(0, Vec::new());
                    offset += 1;
                    for _ in 0..content.last().unwrap().len() {
                        content.first_mut().unwrap().push(empty_pixel.clone());
                    }
                }
            }
        }
    }
    (content, offset)
}

/// Align the passed content vertically within a desired total height. Return the aligned
/// [PixelMap] and an offset for how much the content moved vertically. E.g. content:
/// ```
/// XXX
/// ```
/// With valign [state::VerticalAlignment::Middle] and total height 3 would return:
/// ```
///
/// XXX
///
/// ````
/// With offset 1.
pub fn align_content_vertically(
    mut content: PixelMap,
    valign: VerticalAlignment,
    total_height: usize,
    filler_symbol: String,
    fg_color: Color,
    bg_color: Color,
) -> (PixelMap, usize) {
    if content.is_empty() {
        return (content, 0);
    }
    if content[0].len() >= total_height {
        return (content, 0);
    }

    let empty_pixel =
        Pixel::new(filler_symbol, fg_color, bg_color);

    let mut offset = 0;
    for (i, x) in content.iter_mut().enumerate() {
        for j in 0..total_height - x.len() {
            match valign {
                // Widgets are aligned to top by default.
                VerticalAlignment::Top => {}
                // We align bottom by filling out empty space to the top
                VerticalAlignment::Bottom => {
                    x.insert(0, empty_pixel.clone());
                    if i == 0 {
                        offset += 1;
                    }
                }
                // We align in the middle by filling out empty space alternating top and bottom
                VerticalAlignment::Middle => {
                    if j % 2 == 0 {
                        x.push(empty_pixel.clone());
                    } else {
                        x.insert(0, empty_pixel.clone());
                        if i == 0 {
                            offset += 1;
                        }
                    }
                }
            }
        }
    }
    (content, offset)
}

// Make list of lines, splitting into lines at line breaks in the text or when the widget width
// has been exceeded. If the latter occurs, we will try to split on a word boundary if there is
// any in that chunk of text, to keep things readable.
pub fn wrap_text(mut text: String, width: usize, mut pixels: Vec<Pixel>, default_pixel: Pixel)
    -> Vec<Vec<Pixel>> {

    let mut content_lines: Vec<Vec<Pixel>> = Vec::new();
    loop {
        if width == 0 {
            break;
        } // edge case: widget size 0
        if text.len() >= width {
            let peek = text[0..width].to_string();
            let lines: Vec<&str> = peek.lines().collect();
            // There's a line break in the sentence.
            if lines.len() > 1 {
                // Push all lines except the last one. If there's line breaks within a text
                // chunk that's smaller than widget width, we know for sure that all lines
                // within it fit as well. We don't push the last line because it might be part
                // of a larger sentence, and we're no longer filling full widget width.
                for line in lines[0..lines.len() - 1].iter() {
                    if line.is_empty() {
                        content_lines.push(vec!());
                    } else {
                        let mut new = Vec::new();

                        for char in line.graphemes(true) {
                            let mut pixel;
                            loop {
                                pixel = pixels.remove(0);
                                if pixel.symbol == char { break }
                            }
                            new.push(pixel);
                        }
                        content_lines.push(new);
                    }
                }
                if !lines.last().unwrap().is_empty() {
                    text = text[peek.rfind(lines.last().unwrap()).unwrap()..].to_string();
                } else {
                    text = text[width..].to_string();
                }
            }
            // Chunk naturally ends on word boundary, so just push the chunk.
            else if peek.ends_with(' ') {
                let mut new = Vec::new();
                for char in peek.graphemes(true) {
                    let mut pixel;
                    loop {
                        pixel = pixels.remove(0);
                        if pixel.symbol == char { break }
                    }
                    new.push(pixel);
                }
                content_lines.push(new);
                text = text[width..].to_string();
                // We can find a word boundary somewhere to split the string on. Push up until the
                // boundary.
            } else if let Some(index) = peek.rfind(' ') {

                let mut new = Vec::new();
                for char in peek[..index].graphemes(true) {
                    let mut pixel;
                    loop {
                        pixel = pixels.remove(0);
                        if pixel.symbol == char { break }
                    }
                    new.push(pixel);
                }
                pixels.remove(0); // whitespace
                content_lines.push(new);
                text = text[index + 1..].to_string();
                // No boundaries at all, just push the entire chunk.
            } else {
                let mut new = Vec::new();
                for char in peek.graphemes(true) {
                    let mut pixel;
                    loop {
                        pixel = pixels.remove(0);
                        if pixel.symbol == char { break }
                    }
                    new.push(pixel);
                }
                content_lines.push(new);
                text = text[width..].to_string();
            }
            // Not enough content left to fill widget width. Just push entire text
        } else {
            content_lines.push(pixels);
            break;
        }
    }
    content_lines
}

/// Adjust an absolute position based on scrolling config and size of the parent layout.
pub fn offset_scrolled_absolute_position(
    mut absolute_position: IsizeCoordinates,
    scrolling: &ScrollingConfig,
    size: &Size,
) -> IsizeCoordinates {
    if scrolling.get_is_scrolling_x() && size.width > 0 {
        let view_start = scrolling.get_absolute_scroll_start_x(size.width);
        let offset = ((view_start / size.width) * size.width) + (view_start % size.width);
        absolute_position.x -= offset as isize;
    }
    if scrolling.get_is_scrolling_y() && size.height > 0 {
        let view_start = scrolling.get_absolute_scroll_start_y(size.height);
        let offset = ((view_start / size.height) * size.height) + (view_start % size.height);
        absolute_position.y -= offset as isize;
    }
    absolute_position
}


/// Create pixels from text, consuming formatting text, e.g. \[underline\] becomes a formatted pixel.
pub fn format_text(
    text: String,
    default: Pixel,
) -> (String, Vec<Pixel>) {

    let mut underline = false;
    let mut bold = false;
    let mut italic = false;
    let mut strike_through = false;
    let mut color = None;
    let mut bg_color = None;

    let mut tag_start = false;
    let mut tag_content = String::new();
    let mut escaped = false;

    let mut pixels = Vec::new();
    let mut formatted_text = String::new();

    for grapheme in text.graphemes(true).into_iter() {

        if grapheme == "]" && !escaped {
            let close = if tag_content.starts_with('/') {
                tag_content = tag_content.strip_prefix('/').unwrap().to_string();
                true
            } else {
                false
            };
            tag_content = tag_content.to_lowercase();
            if tag_content == "b" {
                if !close {
                    bold = true
                } else {
                    bold = false
                }
            } else if tag_content == "u" {
                if !close {
                    underline = true
                } else {
                    underline = false
                }
            } else if tag_content == "i" {
                if !close {
                    italic = true
                } else {
                    italic = false
                }
            } else if tag_content == "s" {
                if !close {
                    strike_through = true
                } else {
                    strike_through = false
                }
            } else if tag_content.starts_with("color") {
                if !close {
                    let color_str = tag_content.split_once("color=").unwrap().1;
                    color = Some(parse_color_property(color_str).unwrap());
                } else {
                    color = None;
                }
            } else if tag_content.starts_with("bg_color") {
                if !close {
                    let color_str = tag_content.split_once("bg_color=").unwrap().1;
                    bg_color = Some(parse_color_property(color_str).unwrap());
                } else {
                    bg_color = None;
                }
            }
            tag_start = false;
            tag_content.clear();
            continue
        }

        if tag_start {
            tag_content.push(grapheme.chars().next().unwrap());
            continue
        }

        if grapheme == "[" && !escaped {
            tag_start = true;
            continue
        }
        escaped =  grapheme == "\\" ;

        let mut new_pixel = default.clone();
        new_pixel.symbol = grapheme.to_string();
        if bold { new_pixel.bold = true }
        if underline { new_pixel.underline = true }
        if italic { new_pixel.italic = true }
        if strike_through { new_pixel.strike_through = true }
        if color.is_some() { new_pixel.foreground_color = color.unwrap()}
        if bg_color.is_some() { new_pixel.background_color = color.unwrap() }
        pixels.push(new_pixel);
        formatted_text.push(grapheme.chars().next().unwrap())
    }

    (formatted_text, pixels)

}
