use std::collections::HashMap;
use engine_core::Vec2;
use crate::element::UIElement;
use crate::button::Button;
use crate::label::Label;
use crate::panel::Panel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UIElementId(usize);

pub struct UIManager {
    elements: HashMap<UIElementId, UIElement>,
    buttons: HashMap<UIElementId, Button>,
    labels: HashMap<UIElementId, Label>,
    panels: HashMap<UIElementId, Panel>,
    next_id: usize,
    mouse_was_pressed: bool,
}

impl UIManager {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            buttons: HashMap::new(),
            labels: HashMap::new(),
            panels: HashMap::new(),
            next_id: 0,
            mouse_was_pressed: false,
        }
    }

    pub fn add_panel(&mut self, panel: Panel) -> UIElementId {
        let id = UIElementId(self.next_id);
        self.next_id += 1;
        self.elements.insert(id, panel.element.clone());
        self.panels.insert(id, panel);
        id
    }

    pub fn add_button(&mut self, button: Button) -> UIElementId {
        let id = UIElementId(self.next_id);
        self.next_id += 1;
        self.elements.insert(id, button.element.clone());
        self.buttons.insert(id, button);
        id
    }

    pub fn add_label(&mut self, label: Label) -> UIElementId {
        let id = UIElementId(self.next_id);
        self.next_id += 1;
        self.elements.insert(id, label.element.clone());
        self.labels.insert(id, label);
        id
    }

    pub fn remove(&mut self, id: UIElementId) {
        self.elements.remove(&id);
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

    pub fn update(&mut self, mouse_pos: Vec2, mouse_pressed: bool) {
        for button in self.buttons.values_mut() {
            button.update(mouse_pos, mouse_pressed);
        }

        self.mouse_was_pressed = mouse_pressed;
    }

    pub fn is_button_clicked(&self, id: UIElementId) -> bool {
        if let Some(button) = self.buttons.get(&id) {
            button.is_just_clicked(button.is_clicked, self.mouse_was_pressed)
        } else {
            false
        }
    }

    pub fn get_visible_elements(&self) -> Vec<(UIElementId, &UIElement)> {
        let mut elements: Vec<_> = self.elements
            .iter()
            .filter(|(_, e)| e.visible)
            .map(|(id, e)| (*id, e))
            .collect();
        
        elements.sort_by(|a, b| a.1.z_index.partial_cmp(&b.1.z_index).unwrap());
        elements
    }

    pub fn clear(&mut self) {
        self.elements.clear();
        self.buttons.clear();
        self.labels.clear();
        self.panels.clear();
    }
}

impl Default for UIManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine_core::Rect;

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
}
