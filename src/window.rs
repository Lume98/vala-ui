use crate::{Element, Result, Widget};

pub type ActionHandler = Box<dyn FnMut(&str) + 'static>;

pub struct Window {
    title: String,
    width: i32,
    height: i32,
    root: Element,
    action_handler: Option<ActionHandler>,
}

impl Window {
    pub fn new(title: impl Into<String>, root: impl Widget) -> Self {
        Self {
            title: title.into(),
            width: 800,
            height: 600,
            root: root.render(),
            action_handler: None,
        }
    }

    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn on_action(mut self, handler: impl FnMut(&str) + 'static) -> Self {
        self.action_handler = Some(Box::new(handler));
        self
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn root(&self) -> &Element {
        &self.root
    }

    pub(crate) fn into_parts(self) -> (String, i32, i32, Element, Option<ActionHandler>) {
        (
            self.title,
            self.width,
            self.height,
            self.root,
            self.action_handler,
        )
    }

    pub fn run(self) -> Result<()> {
        crate::platform::run_window(self)
    }
}
