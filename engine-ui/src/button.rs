use engine_core::{Vec2, Rect, Color};
use crate::element::{UIElement, UIElementKind};

#[derive(Debug, Clone)]
pub struct Button {
    pub element: UIElement,
    pub text: String,
    pub text_color: Color,
    pub hover_color: Color,
    pub click_color: Color,
    pub is_hovered: bool,
    pub is_clicked: bool,
}

impl Button {
    pub fn new(bounds: Rect, text: impl Into<String>) -> Self {
        let text = text.into();
        Self {
            element: UIElement::new(UIElementKind::Button, bounds)
                .with_background_color(Color::new(0.2, 0.2, 0.2, 1.0)),
            text,
            text_color: Color::WHITE,
            hover_color: Color::new(0.3, 0.3, 0.3, 1.0),
            click_color: Color::new(0.4, 0.4, 0.4, 1.0),
            is_hovered: false,
            is_clicked: false,
        }
    }

    pub fn with_text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn with_hover_color(mut self, color: Color) -> Self {
        self.hover_color = color;
        self
    }

    pub fn with_click_color(mut self, color: Color) -> Self {
        self.click_color = color;
        self
    }

    pub fn update(&mut self, mouse_pos: Vec2, mouse_pressed: bool) {
        self.is_hovered = self.element.contains(mouse_pos);
        self.is_clicked = self.is_hovered && mouse_pressed;
    }

    pub fn get_current_color(&self) -> Color {
        if self.is_clicked {
            self.click_color
        } else if self.is_hovered {
            self.hover_color
        } else {
            self.element.background_color
        }
    }

    pub fn is_just_clicked(&self, mouse_pressed: bool, mouse_was_pressed: bool) -> bool {
        self.is_hovered && mouse_pressed && !mouse_was_pressed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_creation() {
        let bounds = Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(100.0, 30.0));
        let button = Button::new(bounds, "Click Me");
        
        assert_eq!(button.text, "Click Me");
        assert!(!button.is_hovered);
    }

    #[test]
    fn test_button_hover() {
        let bounds = Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(100.0, 30.0));
        let mut button = Button::new(bounds, "Click Me");
        
        button.update(Vec2::new(50.0, 15.0), false);
        assert!(button.is_hovered);
    }
}
