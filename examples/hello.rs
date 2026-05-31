use vela_ui::prelude::*;

fn main() -> Result<()> {
    let app = Column::new()
        .child(Text::new("Hello from Vela UI"))
        .child(Button::new("Save").on_click("save"));

    Window::new("Vela UI demo", app)
        .size(420, 240)
        .on_action(|action| {
            println!("action: {action}");
        })
        .run()
}
