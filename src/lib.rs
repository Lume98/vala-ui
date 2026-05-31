//! A lightweight Rust UI library for native Windows apps.
//!
//! The crate exposes a minimal virtual UI tree and starter widgets that can be
//! expanded into a renderer, native backend, or web backend over time.

pub mod element;
pub mod error;
mod platform;
pub mod prelude;
pub mod style;
pub mod widget;
pub mod widgets;
pub mod window;

pub use element::{Attribute, Element};
pub use error::{Error, Result};
pub use style::{Align, Color, FontWeight, Style};
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
    fn builds_form_ui_tree() {
        let form = Form::new()
            .max_width(360)
            .centered()
            .child(Label::new("Email"))
            .child(Input::new().name("email").placeholder("name@example.com"))
            .child(
                Checkbox::new("Subscribe")
                    .name("newsletter")
                    .checked(true)
                    .on_toggle("toggle-newsletter"),
            )
            .child(Button::new("Submit").on_click("submit"));

        let element = form.render();

        assert_eq!(element.name(), "form");
        assert_eq!(element.attributes()[0].name(), "max_width");
        assert_eq!(element.attributes()[0].value(), "360");
        assert_eq!(element.attributes()[1].name(), "align");
        assert_eq!(element.attributes()[1].value(), "center");
        assert_eq!(element.children().len(), 4);
        assert_eq!(element.children()[0].name(), "label");
        assert_eq!(element.children()[1].name(), "input");
        assert_eq!(element.children()[1].attributes()[0].name(), "name");
        assert_eq!(element.children()[1].attributes()[1].name(), "placeholder");
        assert_eq!(element.children()[2].name(), "checkbox");
        assert_eq!(element.children()[2].attributes()[1].name(), "checked");
        assert_eq!(element.children()[2].attributes()[2].name(), "on_toggle");
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
    fn formats_input_element() {
        let input = Input::new()
            .name("email")
            .value("hello@example.com")
            .placeholder("Email")
            .render();

        assert_eq!(
            input.to_string(),
            "<input name=\"email\" placeholder=\"Email\">hello@example.com</input>"
        );
    }

    #[test]
    fn builds_presentation_widgets() {
        let app = Column::new()
            .child(Heading::new("Create your account"))
            .child(Spacer::new(12))
            .child(Divider::new())
            .child(Button::new("Create account").primary().on_click("create"));

        let element = app.render();

        assert_eq!(element.children()[0].name(), "heading");
        assert_eq!(element.children()[1].name(), "spacer");
        assert_eq!(element.children()[1].attributes()[0].value(), "12");
        assert_eq!(element.children()[2].name(), "divider");
        assert_eq!(element.children()[3].attributes()[0].name(), "variant");
        assert_eq!(element.children()[3].attributes()[0].value(), "primary");
        assert_eq!(element.children()[3].attributes()[1].name(), "on_click");
    }

    #[test]
    fn applies_widget_styles() {
        let form = Form::new()
            .style(Style::new().max_width(380).gap(12).align(Align::Center))
            .child(
                Heading::new("Create your account").style(
                    Style::new()
                        .font_size(24)
                        .font_weight(FontWeight::Semibold)
                        .color(Color::rgb(31, 41, 55)),
                ),
            )
            .child(
                Input::new().style(
                    Style::new()
                        .height(34)
                        .background(Color::rgb(255, 255, 255)),
                ),
            );

        let element = form.render();

        assert_eq!(element.attributes()[0].name(), "max_width");
        assert_eq!(element.attributes()[0].value(), "380");
        assert_eq!(element.attributes()[1].name(), "gap");
        assert_eq!(element.attributes()[1].value(), "12");
        assert_eq!(element.attributes()[2].name(), "align");
        assert_eq!(element.attributes()[2].value(), "center");
        assert_eq!(element.children()[0].attributes()[0].name(), "font_size");
        assert_eq!(element.children()[0].attributes()[0].value(), "24");
        assert_eq!(element.children()[0].attributes()[1].name(), "font_weight");
        assert_eq!(element.children()[0].attributes()[1].value(), "semibold");
        assert_eq!(element.children()[0].attributes()[2].name(), "color");
        assert_eq!(element.children()[0].attributes()[2].value(), "#1f2937");
        assert_eq!(element.children()[1].attributes()[0].name(), "height");
        assert_eq!(element.children()[1].attributes()[0].value(), "34");
        assert_eq!(element.children()[1].attributes()[1].name(), "background");
        assert_eq!(element.children()[1].attributes()[1].value(), "#ffffff");
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
