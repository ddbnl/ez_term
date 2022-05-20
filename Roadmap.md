- App struct for convenience


- Closures for callbacks to capture environment


- Docs:
  - Document possible callbacks for each widget clearly with examples
  - Document Ez file params


- New widgets:
    - Progress bar
    - Scrollview
    - Modal/popup
      - right click context menu as a modal of buttons
    - Window manager
    - Tabs


- Extra features for existing widgets:
    - general:
      - padding
      - alignment
      - size_hints
      - change border symbols to struct, clean up code
    - text input:
        - multi line
    - label
        - text alignment
        - inline formatting


- Ez format:
    - template inheritance
    - allow references to other widgets
    - call a validation func on each widget after init
  

- Mouse event handler
  - on_drag
  - on_double_click


- tests:
    - right mouse click
    - no selectable widgets
    - cycling through selectable widgets