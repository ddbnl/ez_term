# EzTerm

A YAML(ish) based terminal GUI framework for- and by Rust, focussed on making it quick and easy to create a
functional UI for an app or game. Based on Crossterm and inspired by Kivy.

![img.png](img.png)


# Introduction
The vision is an easy-to-use terminal UI framework that can be used by anyone almost immediately. Deciding that you
want your app to have a UI only to find out that writing the UI will take at least as much study as your original
idea is what I'm trying to prevent.
You should be able to write most of the UI through config files using high-level layouts that do the work for you 
(e.g. box layouts, table layouts, stacking layouts, etc.). Sizing and positioning should be possible through size hints
and positions hints, unless you specifically want to hard code size/position.

Code should only be necessary where it makes sense (writing callbacks for example). This way you can focus on coding
an App, not a UI. Widgets should (at least initially) be elementary: buttons, labels, checkboxes, etc. Combining 
simple widgets with smart layouts can still yield impressive UIs, while still maintaining a simple API that doesn't
require much study.

Dividing the screen in four text labels, should be as simple as:
```

- Layout: my_horizontal_box
    mode: box
    orientation: horizontal
    - Layout: my_vertical_box_left
        mode: box
        orientation: vertical
        - Label: my_upper_left_label
            text: hello
        - Label: my_lower_left_label
            text: hello
    - Layout: my_vertical_box_right
        mode: box
        orientation: vertical
        - Label: my_upper_right_label
            text: hello
        - Label: my_lower_right_label
            text: hello
```
![img_1.png](img_1.png)

Positioning those labels to be on the outer edges of the screen increases complexity, 
but if the framework is doing its job not by much. We could use a float layout to have more control over position,
auto size the labels to be as large as their text content (so they don't fill the entire layout) and then use 
position hints to place them. We'll also create a new label in the middle of the screen, to show off position hints
a bit more:

```
- Layout: my_float_layout
    mode: float
    - Label: my_upper_left_label
        text: hello
        border: True
        auto_scale_width: true
        auto_scale_height: true
        pos_hint_x: left
        pos_hint_y: top
    - Label: my_middle_label
        text: hello
        border: True
        auto_scale_width: true
        auto_scale_height: true
        pos_hint_x: center
        pos_hint_y: middle
    - Label: my_lower_left_label
        text: hello
        border: True
        auto_scale_width: true
        auto_scale_height: true
        pos_hint_x: left
        pos_hint_y: bottom
    - Label: my_upper_right_label
        text: hello
        border: True
        auto_scale_width: true
        auto_scale_height: true
        pos_hint_x: right
        pos_hint_y: top
    - Label: my_lower_right_label
        text: hello
        border: True
        auto_scale_width: true
        auto_scale_height: true
        pos_hint_x: right
        pos_hint_y: bottom
```
![img_2.png](img_2.png)

Combining simple concepts such as size hints, position hints, horizontal/vertical alignment, padding, etc. should
allow you to make relatively complex layouts without painstakingly hardcoding sizes, or writing your own scaling
formulas.

If this seems useful to you please let me know or star the repo, so I can guage interest.

# Current state
Very much a work in progress and still not available on Cargo. See the projects page for what I'm working on. 

Currently supports the following:

- Widgets:
  - Box layouts (automatically place widgets next to each other or below each
    other)
  - Float layouts (hard coded widget positions)
  - Label (text displaying widget)
  - Text input (input and display text)
  - Checkbox (simple on/off switch)
  - Radio buttons (mutually exclusive groups of switches)
  - Dropdowns (list of values from which one can be chosen)
  - Canvases (load content from text file or can be painted manually)
  - Colors and borders for widgets.
- Widget placement:
  - Size hints
  - Position hints
  - Padding
  - Vertical/horizontal alignment
  - Auto scaling for most widgets (adjust widget size to actual content minimizing size)
- Callbacks:
  - On keyboard enter
  - On left/right click
  - On value change
  