use engine_core::{Vec2, Rect, Color};
use crate::element::{UIElement, UIElementKind};

#[derive(Debug, Clone)]
pub struct Panel {
    pub element: UIElement,
}

impl Panel {
    pub fn new(bounds: Rect) -> Self {
        Self {
            element: UIElement::new(UIElementKind::Panel, bounds)
                .with_background_color(Color::new(0.1, 0.1, 0.1, 0.8)),
        }
    }

    pub fn with_background_color(mut self, color: Color) -> Self {
        self.element = self.element.with_background_color(color);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_creation() {
        let bounds = Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(200.0, 150.0));
        let panel = Panel::new(bounds);
        
        assert_eq!(panel.element.kind, UIElementKind::Panel);
    }
}
