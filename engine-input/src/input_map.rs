use std::collections::HashMap;
use engine_core::Vec2;
use crate::keyboard::KeyCode;
use crate::mouse::MouseButton;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Jump,
    Action1,
    Action2,
    Action3,
    Custom(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputAxis {
    Horizontal,
    Vertical,
    Custom(u32),
}

pub struct InputMap {
    key_bindings: HashMap<InputAction, KeyCode>,
    mouse_bindings: HashMap<InputAction, MouseButton>,
    axis_bindings: HashMap<InputAxis, (KeyCode, KeyCode)>,
    deadzone: f32,
}

impl InputMap {
    pub fn new() -> Self {
        let mut map = Self {
            key_bindings: HashMap::new(),
            mouse_bindings: HashMap::new(),
            axis_bindings: HashMap::new(),
            deadzone: 0.1,
        };

        map.bind_key(InputAction::MoveUp, KeyCode::W);
        map.bind_key(InputAction::MoveDown, KeyCode::S);
        map.bind_key(InputAction::MoveLeft, KeyCode::A);
        map.bind_key(InputAction::MoveRight, KeyCode::D);
        map.bind_key(InputAction::Jump, KeyCode::Space);

        map.bind_axis(InputAxis::Horizontal, KeyCode::A, KeyCode::D);
        map.bind_axis(InputAxis::Vertical, KeyCode::W, KeyCode::S);

        map
    }

    pub fn bind_key(&mut self, action: InputAction, key: KeyCode) {
        self.key_bindings.insert(action, key);
    }

    pub fn bind_mouse(&mut self, action: InputAction, button: MouseButton) {
        self.mouse_bindings.insert(action, button);
    }

    pub fn bind_axis(&mut self, axis: InputAxis, negative: KeyCode, positive: KeyCode) {
        self.axis_bindings.insert(axis, (negative, positive));
    }

    pub fn set_deadzone(&mut self, deadzone: f32) {
        self.deadzone = deadzone.clamp(0.0, 1.0);
    }

    pub fn is_action_pressed(
        &self,
        action: InputAction,
        keyboard: &crate::keyboard::Keyboard,
        mouse: &crate::mouse::Mouse,
    ) -> bool {
        if let Some(key) = self.key_bindings.get(&action) {
            if keyboard.is_pressed(*key) {
                return true;
            }
        }

        if let Some(button) = self.mouse_bindings.get(&action) {
            if mouse.is_pressed(*button) {
                return true;
            }
        }

        false
    }

    pub fn is_action_just_pressed(
        &self,
        action: InputAction,
        keyboard: &crate::keyboard::Keyboard,
        mouse: &crate::mouse::Mouse,
    ) -> bool {
        if let Some(key) = self.key_bindings.get(&action) {
            if keyboard.is_just_pressed(*key) {
                return true;
            }
        }

        if let Some(button) = self.mouse_bindings.get(&action) {
            if mouse.is_just_pressed(*button) {
                return true;
            }
        }

        false
    }

    pub fn get_axis(
        &self,
        axis: InputAxis,
        keyboard: &crate::keyboard::Keyboard,
    ) -> f32 {
        if let Some((negative, positive)) = self.axis_bindings.get(&axis) {
            let mut value: f32 = 0.0;

            if keyboard.is_pressed(*positive) {
                value += 1.0;
            }

            if keyboard.is_pressed(*negative) {
                value -= 1.0;
            }

            if value.abs() < self.deadzone {
                return 0.0;
            }

            value
        } else {
            0.0
        }
    }

    pub fn get_axis_vector(
        &self,
        horizontal: InputAxis,
        vertical: InputAxis,
        keyboard: &crate::keyboard::Keyboard,
    ) -> Vec2 {
        let x = self.get_axis(horizontal, keyboard);
        let y = self.get_axis(vertical, keyboard);

        let result = Vec2::new(x, y);
        if result.length() > 0.0 {
            result.normalize()
        } else {
            Vec2::ZERO
        }
    }
}

impl Default for InputMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyboard::Keyboard;
    use crate::mouse::Mouse;

    #[test]
    fn test_input_map_creation() {
        let map = InputMap::new();
        assert!(map.key_bindings.contains_key(&InputAction::MoveUp));
    }

    #[test]
    fn test_input_map_bind_key() {
        let mut map = InputMap::new();
        map.bind_key(InputAction::Jump, KeyCode::Space);

        let _keyboard = Keyboard::new();
        assert!(map.key_bindings.get(&InputAction::Jump).is_some());
    }

    #[test]
    fn test_input_map_is_action_pressed() {
        let map = InputMap::new();
        let mut keyboard = Keyboard::new();
        let mouse = Mouse::new();

        keyboard.press(KeyCode::W);

        assert!(map.is_action_pressed(InputAction::MoveUp, &keyboard, &mouse));
    }

    #[test]
    fn test_input_map_get_axis() {
        let map = InputMap::new();
        let mut keyboard = Keyboard::new();

        keyboard.press(KeyCode::D);

        let axis = map.get_axis(InputAxis::Horizontal, &keyboard);
        assert!((axis - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_input_map_deadzone() {
        let mut map = InputMap::new();
        map.set_deadzone(0.5);

        let _keyboard = Keyboard::new();
        let axis = map.get_axis(InputAxis::Horizontal, &_keyboard);
        assert_eq!(axis, 0.0);
    }
}
