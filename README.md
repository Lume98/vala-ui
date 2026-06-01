# Vela UI

`vela-ui` is a lightweight native Windows UI library for Rust. The first
stage targets Win32 directly through `windows-sys` and renders with native
Windows windows and controls.

This project is intentionally not built on egui, iced, Dioxus, winit, Skia,
or wgpu. Those projects are useful, but `vela-ui` is focused on a small Rust
component API over Win32 itself.

The crate starts with a minimal virtual UI tree:

- `Element` describes nodes, attributes, text, and children.
- `Widget` converts reusable components into `Element`.
- `widgets` contains simple starter widgets such as `Text`, `Button`, `Input`, `Checkbox`, `Form`, and `Column`.
- `Style` adds basic layout, spacing, typography, and color styling for widgets.
- `Window` opens a native Windows window and renders the starter widgets.

The current backend supports native mappings for `Button`, `Input`,
`Checkbox`, `Label`, and `Text`. `Column`, `Form`, `Spacer`, and `Divider`
provide layout and presentation helpers.

## Example

```rust
use vela_ui::prelude::*;

fn main() -> Result<()> {
    let app = Form::new()
        .child(Label::new("Email"))
        .child(Input::new().name("email").placeholder("name@example.com"))
        .child(Button::new("Submit").primary().default().on_click("submit"));

    Window::new("Demo", app)
        .size(480, 320)
        .on_action(|action| println!("{action}"))
        .run()
}
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
            .value("")
            .placeholder("name@example.com")
            .style(Style::new().height(34).background(Color::rgb(255, 255, 255))),
    )
    .child(
        Checkbox::new("Send product updates")
            .name("updates")
            .checked(true)
            .on_toggle("toggle-updates"),
    )
    .child(
        Button::new("Create account")
            .primary()
            .default()
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
                .default()
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

Manual Windows checks:

- `cargo run --example hello`
- Confirm the window opens.
- Confirm clicking the button prints an action.
- Confirm resizing the window lays out controls again.
- Confirm input placeholder text, checked checkbox state, and primary/default
  button styling are visible.
