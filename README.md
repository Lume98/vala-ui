# Vela UI

`vela-ui` is a lightweight Rust UI library. It currently targets Windows
with a minimal native Win32 backend.

The crate starts with a minimal virtual UI tree:

- `Element` describes nodes, attributes, text, and children.
- `Widget` converts reusable components into `Element`.
- `widgets` contains simple starter widgets such as `Text`, `Button`, and `Column`.
- `Window` opens a native Windows window and renders the starter widgets.

## Example

```rust
use vela_ui::prelude::*;

let app = Column::new()
    .child(Text::new("Hello"))
    .child(Button::new("Save").on_click("save"));

let element = app.render();

assert_eq!(element.name(), "column");
assert_eq!(element.children().len(), 2);
```

Run the Windows example:

```powershell
cargo run --example hello
```

```rust
use vela_ui::prelude::*;

fn main() -> Result<()> {
    let app = Column::new()
        .child(Text::new("Hello from Vela UI"))
        .child(Button::new("Save").on_click("save"));

    Window::new("Vela UI demo", app)
        .size(420, 240)
        .on_action(|action| println!("action: {action}"))
        .run()
}
```

## Development

```powershell
cargo fmt
cargo test
cargo check --examples
```
