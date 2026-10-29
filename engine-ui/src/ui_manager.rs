use std::collections::HashMap;
use engine_core::{Rect, Vec2};
use engine_graphics::{font::measure_text, Renderer};
use crate::element::UIElement;
use crate::button::Button;
use crate::label::Label;
use crate::panel::Panel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UIElementId(pub usize);

/// Owns all UI widgets. Each widget (and its `UIElement`) is stored exactly once,
/// in the map for its kind, so mutating a widget (e.g. `button.element.visible`)
/// is always reflected when drawing.
pub struct UIManager {
    buttons: HashMap<UIElementId, Button>,
    labels: HashMap<UIElementId, Label>,
    panels: HashMap<UIElementId, Panel>,
    next_id: usize,
    mouse_pressed: bool,
    mouse_was_pressed: bool,
}

impl UIManager {
    pub fn new() -> Self {
        Self {
            buttons: HashMap::new(),
            labels: HashMap::new(),
            panels: HashMap::new(),
            next_id: 0,
            mouse_pressed: false,
            mouse_was_pressed: false,
        }
    }

    fn alloc_id(&mut self) -> UIElementId {
        let id = UIElementId(self.next_id);
        self.next_id += 1;
        id
    }

    pub fn add_panel(&mut self, panel: Panel) -> UIElementId {
        let id = self.alloc_id();
        self.panels.insert(id, panel);
        id
    }

    pub fn add_button(&mut self, button: Button) -> UIElementId {
        let id = self.alloc_id();
        self.buttons.insert(id, button);
        id
    }

    pub fn add_label(&mut self, label: Label) -> UIElementId {
        let id = self.alloc_id();
        self.labels.insert(id, label);
        id
    }

    pub fn remove(&mut self, id: UIElementId) {
        self.buttons.remove(&id);
        self.labels.remove(&id);
        self.panels.remove(&id);
    }

    pub fn get_button(&self, id: UIElementId) -> Option<&Button> {
        self.buttons.get(&id)
    }

    pub fn get_button_mut(&mut self, id: UIElementId) -> Option<&mut Button> {
        self.buttons.get_mut(&id)
    }

    pub fn get_label(&self, id: UIElementId) -> Option<&Label> {
        self.labels.get(&id)
    }

    pub fn get_label_mut(&mut self, id: UIElementId) -> Option<&mut Label> {
        self.labels.get_mut(&id)
    }

    pub fn get_panel(&self, id: UIElementId) -> Option<&Panel> {
        self.panels.get(&id)
    }

    pub fn get_panel_mut(&mut self, id: UIElementId) -> Option<&mut Panel> {
        self.panels.get_mut(&id)
    }

    /// The common `UIElement` data of any widget, regardless of its kind.
    pub fn get_element(&self, id: UIElementId) -> Option<&UIElement> {
        self.buttons.get(&id).map(|b| &b.element)
            .or_else(|| self.labels.get(&id).map(|l| &l.element))
            .or_else(|| self.panels.get(&id).map(|p| &p.element))
    }

    pub fn get_element_mut(&mut self, id: UIElementId) -> Option<&mut UIElement> {
        if let Some(b) = self.buttons.get_mut(&id) {
            return Some(&mut b.element);
        }
        if let Some(l) = self.labels.get_mut(&id) {
            return Some(&mut l.element);
        }
        self.panels.get_mut(&id).map(|p| &mut p.element)
    }

    pub fn update(&mut self, mouse_pos: Vec2, mouse_pressed: bool) {
        // Remember last frame's pressed state *before* overwriting it, so
        // `is_button_clicked` can detect the pressed/not-pressed edge correctly.
        self.mouse_was_pressed = self.mouse_pressed;
        self.mouse_pressed = mouse_pressed;

        for button in self.buttons.values_mut() {
            if button.element.visible && button.element.enabled {
                button.update(mouse_pos, mouse_pressed);
            } else {
                // Hidden or disabled buttons can't be hovered or clicked.
                button.is_hovered = false;
                button.is_clicked = false;
            }
        }
    }

    pub fn is_button_clicked(&self, id: UIElementId) -> bool {
        if let Some(button) = self.buttons.get(&id) {
            // Use the raw current mouse-pressed state (not `button.is_clicked`, which
            // is already gated on hover) so the edge detection in `is_just_clicked`
            // works correctly.
            button.is_just_clicked(self.mouse_pressed, self.mouse_was_pressed)
        } else {
            false
        }
    }

    /// Visible elements in draw order: ascending z-index, ties broken by
    /// creation order (id) so the order is deterministic from frame to frame.
    pub fn get_visible_elements(&self) -> Vec<(UIElementId, &UIElement)> {
        let mut elements: Vec<(UIElementId, &UIElement)> = self.buttons.iter().map(|(id, b)| (*id, &b.element))
            .chain(self.labels.iter().map(|(id, l)| (*id, &l.element)))
            .chain(self.panels.iter().map(|(id, p)| (*id, &p.element)))
            .filter(|(_, e)| e.visible)
            .collect();

        elements.sort_by(|a, b| {
            a.1.z_index.total_cmp(&b.1.z_index).then_with(|| a.0.cmp(&b.0))
        });
        elements
    }

    /// Draws every visible widget in z-order (screen space): panel and label
    /// backgrounds, button backgrounds in their hover/click colors, and text.
    /// Labels are left-aligned and buttons centered, both vertically centered
    /// in their bounds. Text is not clipped to the widget, so keep it short
    /// enough to fit or lower `text_scale`.
    pub fn render(&self, renderer: &mut Renderer) {
        for (id, element) in self.get_visible_elements() {
            let bounds = element.bounds;

            if let Some(button) = self.buttons.get(&id) {
                renderer.fill_rect(bounds, button.get_current_color());
                let mut color = button.text_color;
                if !element.enabled {
                    color.a *= 0.5; // dim disabled buttons
                }
                let (x, y) = text_origin(bounds, &button.text, button.text_scale, true);
                renderer.draw_text(x, y, &button.text, color, button.text_scale);
            } else if let Some(label) = self.labels.get(&id) {
                renderer.fill_rect(bounds, element.background_color);
                let (x, y) = text_origin(bounds, &label.text, label.text_scale, false);
                renderer.draw_text(x, y, &label.text, label.text_color, label.text_scale);
            } else {
                renderer.fill_rect(bounds, element.background_color);
            }
        }
    }

    pub fn clear(&mut self) {
        self.buttons.clear();
        self.labels.clear();
        self.panels.clear();
    }
}

/// Top-left pixel for text inside `bounds`: vertically centered, and either
/// horizontally centered or left-aligned with a small padding. Never starts
/// left of the widget, so over-long text spills to the right only.
fn text_origin(bounds: Rect, text: &str, scale: u32, center_x: bool) -> (i32, i32) {
    const PADDING: f32 = 4.0;
    let (tw, th) = measure_text(text, scale);
    let size = bounds.size();

    let x = if center_x {
        bounds.min.x + (size.x - tw as f32) * 0.5
    } else {
        bounds.min.x + PADDING
    }
    .max(bounds.min.x);
    let y = bounds.min.y + (size.y - th as f32) * 0.5;

    (x.round() as i32, y.round() as i32)
}

impl Default for UIManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::{Color, Rect};

    #[test]
    fn test_ui_manager_creation() {
        let manager = UIManager::new();
        assert_eq!(manager.next_id, 0);
    }

    #[test]
    fn test_add_button() {
        let mut manager = UIManager::new();
        let bounds = Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(100.0, 30.0));
        let button = Button::new(bounds, "Test");
        
        let id = manager.add_button(button);
        assert!(manager.get_button(id).is_some());
    }

    #[test]
    fn test_add_label() {
        let mut manager = UIManager::new();
        let bounds = Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(100.0, 30.0));
        let label = Label::new(bounds, "Test");
        
        let id = manager.add_label(label);
        assert!(manager.get_label(id).is_some());
    }

    #[test]
    fn test_hiding_widget_removes_it_from_draw_list() {
        let mut manager = UIManager::new();
        let bounds = Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(100.0, 30.0));
        let id = manager.add_button(Button::new(bounds, "Test"));
        assert_eq!(manager.get_visible_elements().len(), 1);

        manager.get_button_mut(id).unwrap().element.visible = false;
        assert!(manager.get_visible_elements().is_empty());
    }

    #[test]
    fn test_draw_order_is_deterministic_for_equal_z() {
        let mut manager = UIManager::new();
        let bounds = Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let ids = [
            manager.add_panel(Panel::new(bounds)),
            manager.add_label(Label::new(bounds, "a")),
            manager.add_button(Button::new(bounds, "b")),
            manager.add_label(Label::new(bounds, "c")),
        ];
        for _ in 0..20 {
            let order: Vec<_> = manager.get_visible_elements().into_iter().map(|(id, _)| id).collect();
            assert_eq!(order, ids);
        }
    }

    #[test]
    fn test_z_index_overrides_creation_order() {
        let mut manager = UIManager::new();
        let bounds = Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let front = manager.add_panel(Panel::new(bounds));
        let back = manager.add_panel(Panel::new(bounds));
        manager.get_element_mut(front).unwrap().z_index = 5.0;
        let order: Vec<_> = manager.get_visible_elements().into_iter().map(|(id, _)| id).collect();
        assert_eq!(order, vec![back, front]);
    }

    #[test]
    fn test_hidden_button_cannot_be_clicked() {
        let mut manager = UIManager::new();
        let bounds = Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(100.0, 30.0));
        let id = manager.add_button(Button::new(bounds, "Test"));
        manager.get_button_mut(id).unwrap().element.visible = false;

        manager.update(Vec2::new(5.0, 5.0), false);
        manager.update(Vec2::new(5.0, 5.0), true);
        assert!(!manager.is_button_clicked(id));
    }

    fn ui_scene() -> (UIManager, UIElementId, UIElementId, UIElementId) {
        let mut m = UIManager::new();
        let panel = m.add_panel(Panel::new(Rect::from_min_size(Vec2::new(0.0, 0.0), Vec2::new(120.0, 60.0))));
        let label = m.add_label(Label::new(
            Rect::from_min_size(Vec2::new(5.0, 5.0), Vec2::new(110.0, 20.0)),
            "Score",
        ));
        let button = m.add_button(Button::new(
            Rect::from_min_size(Vec2::new(5.0, 30.0), Vec2::new(60.0, 24.0)),
            "Go",
        ));
        (m, panel, label, button)
    }

    fn white_bounds(r: &Renderer) -> Option<(u32, u32, u32, u32)> {
        let fb = r.framebuffer();
        let (mut x0, mut y0, mut x1, mut y1) = (u32::MAX, u32::MAX, 0, 0);
        let mut any = false;
        for y in 0..fb.height {
            for x in 0..fb.width {
                if fb.get_pixel(x, y) == Some(Color::WHITE) {
                    any = true;
                    x0 = x0.min(x);
                    y0 = y0.min(y);
                    x1 = x1.max(x);
                    y1 = y1.max(y);
                }
            }
        }
        any.then_some((x0, y0, x1, y1))
    }

    #[test]
    fn test_render_draws_label_text_inside_label_bounds() {
        let (mut m, _panel, label, button) = ui_scene();
        m.get_element_mut(button).unwrap().visible = false;
        let mut r = Renderer::new(120, 60);
        r.clear();
        m.render(&mut r);

        let (x0, y0, x1, y1) = white_bounds(&r).expect("label text should be drawn");
        let b = m.get_element(label).unwrap().bounds;
        assert!(x0 as f32 >= b.min.x && (x1 as f32) < b.max.x);
        assert!(y0 as f32 >= b.min.y && (y1 as f32) < b.max.y);
    }

    #[test]
    fn test_render_centers_button_text() {
        let (mut m, _panel, label, button) = ui_scene();
        m.get_element_mut(label).unwrap().visible = false;
        let mut r = Renderer::new(120, 60);
        r.clear();
        m.render(&mut r);

        let (x0, _y0, x1, _y1) = white_bounds(&r).expect("button text should be drawn");
        let b = m.get_element(button).unwrap().bounds;
        let text_center = (x0 + x1) as f32 * 0.5;
        let box_center = (b.min.x + b.max.x) * 0.5;
        assert!((text_center - box_center).abs() <= 8.0, "text center {} box center {}", text_center, box_center);
    }

    #[test]
    fn test_transparent_label_does_not_erase_panel() {
        let (m, _p, _l, _b) = ui_scene();
        let mut r = Renderer::new(120, 60);
        r.set_clear_color(Color::BLACK);
        r.clear();
        m.render(&mut r);

        // a pixel inside the label's area but away from its text must still show the panel
        let inside_label = r.framebuffer().get_pixel(110, 15).unwrap();
        let panel_only = r.framebuffer().get_pixel(110, 58).unwrap();
        assert_eq!(inside_label, panel_only);
        assert_ne!(inside_label, Color::BLACK);
    }

    #[test]
    fn test_hidden_and_disabled_widgets() {
        let (mut m, _panel, label, button) = ui_scene();
        m.get_element_mut(label).unwrap().visible = false;
        m.get_element_mut(button).unwrap().visible = false;
        let mut r = Renderer::new(120, 60);
        r.clear();
        m.render(&mut r);
        assert!(white_bounds(&r).is_none(), "hidden widgets must draw no text");

        // disabled button text is dimmed, so it is no longer pure white
        let (mut m2, _p, l2, b2) = ui_scene();
        m2.get_element_mut(l2).unwrap().visible = false;
        m2.get_element_mut(b2).unwrap().enabled = false;
        let mut r2 = Renderer::new(120, 60);
        r2.clear();
        m2.render(&mut r2);
        assert!(white_bounds(&r2).is_none());
    }

    #[test]
    fn test_label_text_change_is_rendered() {
        let (mut m, _panel, label, button) = ui_scene();
        m.get_element_mut(button).unwrap().visible = false;
        let mut r = Renderer::new(120, 60);
        r.clear();
        m.render(&mut r);
        let before = r.framebuffer().data.clone();

        m.get_label_mut(label).unwrap().set_text("Score: 99");
        r.clear();
        m.render(&mut r);
        assert_ne!(before, r.framebuffer().data);
    }
}
