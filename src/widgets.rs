use crate::{Align, Element, Style, Widget};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Text {
    value: String,
    style: Style,
}

impl Text {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            style: Style::new(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Text {
    fn render(&self) -> Element {
        self.style.apply_to(Element::text(self.value.clone()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Button {
    label: String,
    action: Option<String>,
    primary: bool,
    style: Style,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            action: None,
            primary: false,
            style: Style::new(),
        }
    }

    pub fn on_click(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    pub fn primary(mut self) -> Self {
        self.primary = true;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Button {
    fn render(&self) -> Element {
        let mut element = Element::new("button").with_text(self.label.clone());

        if self.primary {
            element = element.with_attribute("variant", "primary");
        }

        let element = match &self.action {
            Some(action) => element.with_attribute("on_click", action.clone()),
            None => element,
        };

        self.style.apply_to(element)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Heading {
    value: String,
    style: Style,
}

impl Heading {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            style: Style::new(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Heading {
    fn render(&self) -> Element {
        self.style
            .apply_to(Element::new("heading").with_text(self.value.clone()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Spacer {
    height: i32,
    style: Style,
}

impl Spacer {
    pub fn new(height: i32) -> Self {
        Self {
            height,
            style: Style::new(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Spacer {
    fn render(&self) -> Element {
        self.style
            .apply_to(Element::new("spacer").with_attribute("height", self.height.to_string()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Divider {
    style: Style,
}

impl Divider {
    pub fn new() -> Self {
        Self {
            style: Style::new(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Divider {
    fn render(&self) -> Element {
        self.style.apply_to(Element::new("divider"))
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Form {
    children: Vec<Element>,
    style: Style,
}

impl Form {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Widget) -> Self {
        self.children.push(child.render());
        self
    }

    pub fn max_width(mut self, width: i32) -> Self {
        self.style = self.style.max_width(width);
        self
    }

    pub fn centered(mut self) -> Self {
        self.style = self.style.align(Align::Center);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Form {
    fn render(&self) -> Element {
        let element = self.style.apply_to(Element::new("form"));

        self.children
            .iter()
            .cloned()
            .fold(element, Element::with_child)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Label {
    value: String,
    style: Style,
}

impl Label {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            style: Style::new(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Label {
    fn render(&self) -> Element {
        self.style
            .apply_to(Element::new("label").with_text(self.value.clone()))
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Input {
    name: Option<String>,
    value: String,
    placeholder: Option<String>,
    style: Style,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Input {
    fn render(&self) -> Element {
        let mut element = Element::new("input").with_text(self.value.clone());

        if let Some(name) = &self.name {
            element = element.with_attribute("name", name.clone());
        }

        if let Some(placeholder) = &self.placeholder {
            element = element.with_attribute("placeholder", placeholder.clone());
        }

        self.style.apply_to(element)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Checkbox {
    label: String,
    name: Option<String>,
    checked: bool,
    action: Option<String>,
    style: Style,
}

impl Checkbox {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            name: None,
            checked: false,
            action: None,
            style: Style::new(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn on_toggle(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Checkbox {
    fn render(&self) -> Element {
        let mut element = Element::new("checkbox").with_text(self.label.clone());

        if let Some(name) = &self.name {
            element = element.with_attribute("name", name.clone());
        }

        if self.checked {
            element = element.with_attribute("checked", "true");
        }

        if let Some(action) = &self.action {
            element = element.with_attribute("on_toggle", action.clone());
        }

        self.style.apply_to(element)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Column {
    children: Vec<Element>,
    style: Style,
}

impl Column {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Widget) -> Self {
        self.children.push(child.render());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Column {
    fn render(&self) -> Element {
        self.children.iter().cloned().fold(
            self.style.apply_to(Element::new("column")),
            Element::with_child,
        )
    }
}
