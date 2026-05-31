//! A lightweight Rust UI library for native Windows apps.
//!
//! The crate exposes a minimal virtual UI tree and starter widgets that can be
//! expanded into a renderer, native backend, or web backend over time.

pub mod element;
pub mod error;
mod platform;
pub mod prelude;
pub mod widget;
pub mod widgets;
pub mod window;

pub use element::{Attribute, Element};
pub use error::{Error, Result};
pub use widget::Widget;
pub use window::{ActionHandler, Window};

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn builds_nested_ui_tree() {
        let app = Column::new()
            .child(Text::new("Hello"))
            .child(Button::new("Save").on_click("save"));

        let element = app.render();

        assert_eq!(element.name(), "column");
        assert_eq!(element.children().len(), 2);
        assert_eq!(element.children()[0].text_content(), Some("Hello"));
        assert_eq!(element.children()[1].attributes()[0].name(), "on_click");
    }

    #[test]
    fn formats_element_tree() {
        let button = Button::new("Save").on_click("save").render();

        assert_eq!(
            button.to_string(),
            "<button on_click=\"save\">Save</button>"
        );
    }

    #[test]
    fn configures_window() {
        let window = Window::new("Demo", Text::new("Hello")).size(320, 240);

        assert_eq!(window.title(), "Demo");
        assert_eq!(window.width(), 320);
        assert_eq!(window.height(), 240);
        assert_eq!(window.root().text_content(), Some("Hello"));
    }
}
