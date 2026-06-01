use crate::Element;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Style {
    width: Option<i32>,
    min_width: Option<i32>,
    max_width: Option<i32>,
    height: Option<i32>,
    gap: Option<i32>,
    align: Option<Align>,
    font_size: Option<i32>,
    font_weight: Option<FontWeight>,
    color: Option<Color>,
    background: Option<Color>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Align {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontWeight {
    Normal,
    Semibold,
    Bold,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: i32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn min_width(mut self, min_width: i32) -> Self {
        self.min_width = Some(min_width);
        self
    }

    pub fn max_width(mut self, max_width: i32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    pub fn height(mut self, height: i32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn gap(mut self, gap: i32) -> Self {
        self.gap = Some(gap);
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    pub fn font_size(mut self, font_size: i32) -> Self {
        self.font_size = Some(font_size);
        self
    }

    pub fn font_weight(mut self, font_weight: FontWeight) -> Self {
        self.font_weight = Some(font_weight);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    pub(crate) fn apply_to(&self, mut element: Element) -> Element {
        if let Some(width) = self.width {
            element = element.with_attribute("width", width.to_string());
        }

        if let Some(min_width) = self.min_width {
            element = element.with_attribute("min_width", min_width.to_string());
        }

        if let Some(max_width) = self.max_width {
            element = element.with_attribute("max_width", max_width.to_string());
        }

        if let Some(height) = self.height {
            element = element.with_attribute("height", height.to_string());
        }

        if let Some(gap) = self.gap {
            element = element.with_attribute("gap", gap.to_string());
        }

        if let Some(align) = self.align {
            element = element.with_attribute("align", align.as_str());
        }

        if let Some(font_size) = self.font_size {
            element = element.with_attribute("font_size", font_size.to_string());
        }

        if let Some(font_weight) = self.font_weight {
            element = element.with_attribute("font_weight", font_weight.as_str());
        }

        if let Some(color) = self.color {
            element = element.with_attribute("color", color.to_hex());
        }

        if let Some(background) = self.background {
            element = element.with_attribute("background", background.to_hex());
        }

        element
    }
}

impl Color {
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}

impl Align {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
            Self::Stretch => "stretch",
        }
    }
}

impl FontWeight {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Semibold => "semibold",
            Self::Bold => "bold",
        }
    }
}
