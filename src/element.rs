use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attribute {
    name: String,
    value: String,
}

impl Attribute {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Element {
    name: String,
    attributes: Vec<Attribute>,
    children: Vec<Element>,
    text: Option<String>,
}

impl Element {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            attributes: Vec::new(),
            children: Vec::new(),
            text: None,
        }
    }

    pub fn text(value: impl Into<String>) -> Self {
        Self::new("text").with_text(value)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }

    pub fn children(&self) -> &[Element] {
        &self.children
    }

    pub fn text_content(&self) -> Option<&str> {
        self.text.as_deref()
    }

    pub fn with_attribute(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        let name = name.into();
        let value = value.into();

        if let Some(attribute) = self
            .attributes
            .iter_mut()
            .find(|attribute| attribute.name == name)
        {
            attribute.value = value;
            return self;
        }

        self.attributes.push(Attribute::new(name, value));
        self
    }

    pub fn with_child(mut self, child: Element) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }
}

impl fmt::Display for Element {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "<{}", self.name)?;

        for attribute in &self.attributes {
            write!(formatter, " {}=\"{}\"", attribute.name, attribute.value)?;
        }

        if self.children.is_empty() && self.text.is_none() {
            return write!(formatter, " />");
        }

        write!(formatter, ">")?;

        if let Some(text) = &self.text {
            write!(formatter, "{text}")?;
        }

        for child in &self.children {
            write!(formatter, "{child}")?;
        }

        write!(formatter, "</{}>", self.name)
    }
}
