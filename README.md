# EzTerm

A YAML(ish) based terminal GUI framework for- and by Rust, focussed on making it quick and easy to create a
functional UI for an app or game. Based on Crossterm and inspired by Kivy.

![img.png](img.png)


# Introduction
The vision is an easy-to-use terminal UI framework that can be used by anyone almost immediately without much study.
You should be able to write most of the UI through config files using high-level layouts that do the work for you 
(e.g. box layouts, table layouts, stacking layouts, etc.). Sizing and positioning should be possible through size hints
and positions hints, unless you specifically want to hard code size/position.

Code should only be necessary where it makes sense (writing callbacks for example). This way you can focus on coding
your app, not a UI. Widgets should (at least initially) be elementary: buttons, labels, checkboxes, etc. Combining 
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
a bit more. To top it off, we'll introduce a template to define our label style, so we don't have to copy paste it
over and over.

```
- <MyLabelTemplate@Label>:
    text: hello
    border: True
    auto_scale: true, true

- Layout:
    id: my_float_layout
    mode: float
    - MyLabelTemplate:
        id: my_upper_left_label
        pos_hint: left, top
    - MyLabelTemplate:
        id: my_middle_label
        pos_hint: center, middle
    - MyLabelTemplate:
        id: my_lower_left_label
        pos_hint: left, bottom
    - MyLabelTemplate:
        id: my_upper_right_label
        pos_hint: right, top
    - MyLabelTemplate:
        id: my_lower_right_label
        pos_hint: right, bottom

```
![img_2.png](img_2.png)

Combining simple concepts such as size hints, position hints, horizontal/vertical alignment, padding, etc. should
allow you to make relatively complex layouts without painstakingly hardcoding sizes, or writing your own scaling
formulas.

If this seems useful to you please let me know or star the repo, so I can guage interest.

# Current state
Very much a work in progress and still not available on Cargo. See the projects page for what I'm working on. 

Currently supports the following:

- Ez language:
  - Define entire UI in an Ez file using simple english
  - Supports templates and template inheritance
- Widgets:
  - Screen layout (multiple screens, one active at a time)
  - Tabbed layout
  - Box layout (automatically place widgets next to each other or below each
    other)
  - Float layout (use size hints and position hints, or hard code positions/sizes)
  - Label 
  - Text input
  - Checkbox 
  - Radio buttons
  - Dropdowns 
  - Canvases (load content from text file or fill manually through code)
  - Popups (widget templates that pop up on callback)
- Widget features:
  - Size hints
  - Position hints
  - Padding
  - Vertical/horizontal alignment
  - Auto scaling for most widgets (adjust widget size to actual content minimizing size)
  - Colors and borders.
  - Resize if terminal resizes
- Callbacks:
  - On keyboard enter
  - On left/right click
  - On value change
  - On press (both keyboard enter and left mouse click)
  