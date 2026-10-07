use engine_core::{Vec2, Rect, Color};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UIElementKind {
    Panel,
    Button,
    Label,
}

#[derive(Debug, Clone)]
pub struct UIElement {
    pub kind: UIElementKind,
    pub bounds: Rect,
    pub background_color: Color,
    pub visible: bool,
    pub enabled: bool,
    pub z_index: f32,
}

impl UIElement {
    pub fn new(kind: UIElementKind, bounds: Rect) -> Self {
        Self {
            kind,
            bounds,
            background_color: Color::TRANSPARENT,
            visible: true,
            enabled: true,
            z_index: 0.0,
        }
    }

    pub fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn with_visibility(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_z_index(mut self, z_index: f32) -> Self {
        self.z_index = z_index;
        self
    }

    pub fn contains(&self, point: Vec2) -> bool {
        self.bounds.contains(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_element_creation() {
        let bounds = Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(100.0, 50.0));
        let element = UIElement::new(UIElementKind::Button, bounds);
        
        assert_eq!(element.kind, UIElementKind::Button);
        assert!(element.visible);
        assert!(element.enabled);
    }

    #[test]
    fn test_ui_element_contains() {
        let bounds = Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(100.0, 50.0));
        let element = UIElement::new(UIElementKind::Button, bounds);
        
        assert!(element.contains(Vec2::new(50.0, 25.0)));
        assert!(!element.contains(Vec2::new(150.0, 25.0)));
    }
}
