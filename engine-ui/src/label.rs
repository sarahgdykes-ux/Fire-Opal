use engine_core::{Rect, Color};
use crate::element::{UIElement, UIElementKind};

#[derive(Debug, Clone)]
pub struct Label {
    pub element: UIElement,
    pub text: String,
    pub text_color: Color,
    /// Integer scale of the 8x8 font (2 = 16px text).
    pub text_scale: u32,
}

impl Label {
    pub fn new(bounds: Rect, text: impl Into<String>) -> Self {
        let text = text.into();
        Self {
            element: UIElement::new(UIElementKind::Label, bounds),
            text,
            text_color: Color::WHITE,
            text_scale: 2,
        }
    }

    pub fn with_text_scale(mut self, scale: u32) -> Self {
        self.text_scale = scale.max(1);
        self
    }

    pub fn with_text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Vec2;

    #[test]
    fn test_label_creation() {
        let bounds = Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(100.0, 30.0));
        let label = Label::new(bounds, "Hello");
        
        assert_eq!(label.text, "Hello");
        assert_eq!(label.text_color, Color::WHITE);
    }

    #[test]
    fn test_label_set_text() {
        let bounds = Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(100.0, 30.0));
        let mut label = Label::new(bounds, "Hello");
        
        label.set_text("World");
        assert_eq!(label.text, "World");
    }
}
