use crate::{Element, Widget};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Text {
    value: String,
}

impl Text {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl Widget for Text {
    fn render(&self) -> Element {
        Element::text(self.value.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Button {
    label: String,
    action: Option<String>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            action: None,
        }
    }

    pub fn on_click(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }
}

impl Widget for Button {
    fn render(&self) -> Element {
        let element = Element::new("button").with_text(self.label.clone());

        match &self.action {
            Some(action) => element.with_attribute("on_click", action.clone()),
            None => element,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Column {
    children: Vec<Element>,
}

impl Column {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Widget) -> Self {
        self.children.push(child.render());
        self
    }
}

impl Widget for Column {
    fn render(&self) -> Element {
        self.children
            .iter()
            .cloned()
            .fold(Element::new("column"), Element::with_child)
    }
}
