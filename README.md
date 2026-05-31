# Vela UI

`vela-ui` is a lightweight Rust UI library. It currently targets Windows
with a minimal native Win32 backend.

The crate starts with a minimal virtual UI tree:

- `Element` describes nodes, attributes, text, and children.
- `Widget` converts reusable components into `Element`.
- `widgets` contains simple starter widgets such as `Text`, `Button`, `Input`, `Checkbox`, `Form`, and `Column`.
- `Style` adds basic layout and typography styling for widgets.
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

Form widgets:

```rust
use vela_ui::prelude::*;

let form = Form::new()
    .style(Style::new().max_width(380).gap(9).align(Align::Center))
    .child(
        Heading::new("Create your account")
            .style(
                Style::new()
                    .font_size(24)
                    .font_weight(FontWeight::Semibold)
                    .color(Color::rgb(31, 41, 55)),
            ),
    )
    .child(Label::new("Email address"))
    .child(
        Input::new()
            .name("email")
            .placeholder("name@example.com")
            .style(Style::new().height(34).background(Color::rgb(255, 255, 255))),
    )
    .child(Checkbox::new("Send product updates").name("updates"))
    .child(
        Button::new("Create account")
            .primary()
            .style(Style::new().height(36).font_weight(FontWeight::Semibold))
            .on_click("create-account"),
    );

assert_eq!(form.render().name(), "form");
```

Run the Windows example:

```powershell
cargo run --example hello
```

```rust
use vela_ui::prelude::*;

fn main() -> Result<()> {
    let app = Form::new()
        .style(Style::new().max_width(380).gap(9).align(Align::Center))
        .child(
            Heading::new("Create your account")
                .style(
                    Style::new()
                        .font_size(24)
                        .font_weight(FontWeight::Semibold)
                        .color(Color::rgb(31, 41, 55)),
                ),
        )
        .child(
            Text::new("Start with a clean profile for your workspace.")
                .style(Style::new().font_size(13).color(Color::rgb(75, 85, 99))),
        )
        .child(Spacer::new(8))
        .child(Divider::new().style(Style::new().background(Color::rgb(209, 213, 219))))
        .child(Spacer::new(6))
        .child(Label::new("Full name"))
        .child(
            Input::new()
                .name("name")
                .placeholder("Ada Lovelace")
                .style(Style::new().height(34).background(Color::rgb(255, 255, 255))),
        )
        .child(Label::new("Email address"))
        .child(
            Input::new()
                .name("email")
                .placeholder("name@example.com")
                .style(Style::new().height(34).background(Color::rgb(255, 255, 255))),
        )
        .child(
            Checkbox::new("Send product updates")
                .name("updates")
                .checked(true),
        )
        .child(Spacer::new(4))
        .child(
            Button::new("Create account")
                .primary()
                .style(Style::new().height(36).font_weight(FontWeight::Semibold))
                .on_click("create-account"),
        );

    Window::new("Vela UI demo", app)
        .size(560, 360)
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
