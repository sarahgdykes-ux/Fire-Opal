use engine_core::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseState {
    Released,
    Pressed,
}

pub struct Mouse {
    position: Vec2,
    delta: Vec2,
    scroll: f32,
    left_pressed: bool,
    right_pressed: bool,
    middle_pressed: bool,
    left_just_pressed: bool,
    right_just_pressed: bool,
    middle_just_pressed: bool,
    left_just_released: bool,
    right_just_released: bool,
    middle_just_released: bool,
    visible: bool,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            delta: Vec2::ZERO,
            scroll: 0.0,
            left_pressed: false,
            right_pressed: false,
            middle_pressed: false,
            left_just_pressed: false,
            right_just_pressed: false,
            middle_just_pressed: false,
            left_just_released: false,
            right_just_released: false,
            middle_just_released: false,
            visible: true,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.delta = position - self.position;
        self.position = position;
    }

    pub fn delta(&self) -> Vec2 {
        self.delta
    }

    pub fn scroll(&self) -> f32 {
        self.scroll
    }

    pub fn set_scroll(&mut self, scroll: f32) {
        self.scroll = scroll;
    }

    pub fn press(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
                if !self.left_pressed {
                    self.left_just_pressed = true;
                }
                self.left_pressed = true;
            }
            MouseButton::Right => {
                if !self.right_pressed {
                    self.right_just_pressed = true;
                }
                self.right_pressed = true;
            }
            MouseButton::Middle => {
                if !self.middle_pressed {
                    self.middle_just_pressed = true;
                }
                self.middle_pressed = true;
            }
            MouseButton::Unknown => {}
        }
    }

    pub fn release(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
                self.left_pressed = false;
                self.left_just_released = true;
            }
            MouseButton::Right => {
                self.right_pressed = false;
                self.right_just_released = true;
            }
            MouseButton::Middle => {
                self.middle_pressed = false;
                self.middle_just_released = true;
            }
            MouseButton::Unknown => {}
        }
    }

    pub fn is_pressed(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => self.left_pressed,
            MouseButton::Right => self.right_pressed,
            MouseButton::Middle => self.middle_pressed,
            MouseButton::Unknown => false,
        }
    }

    pub fn is_just_pressed(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => self.left_just_pressed,
            MouseButton::Right => self.right_just_pressed,
            MouseButton::Middle => self.middle_just_pressed,
            MouseButton::Unknown => false,
        }
    }

    pub fn is_just_released(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => self.left_just_released,
            MouseButton::Right => self.right_just_released,
            MouseButton::Middle => self.middle_just_released,
            MouseButton::Unknown => false,
        }
    }

    pub fn state(&self, button: MouseButton) -> MouseState {
        if self.is_pressed(button) {
            MouseState::Pressed
        } else {
            MouseState::Released
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn update(&mut self) {
        self.delta = Vec2::ZERO;
        self.scroll = 0.0;
        self.left_just_pressed = false;
        self.right_just_pressed = false;
        self.middle_just_pressed = false;
        self.left_just_released = false;
        self.right_just_released = false;
        self.middle_just_released = false;
    }
}

impl Default for Mouse {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_position() {
        let mut mouse = Mouse::new();
        mouse.set_position(Vec2::new(100.0, 200.0));

        assert_eq!(mouse.position(), Vec2::new(100.0, 200.0));
    }

    #[test]
    fn test_mouse_press() {
        let mut mouse = Mouse::new();
        mouse.press(MouseButton::Left);

        assert!(mouse.is_pressed(MouseButton::Left));
        assert!(mouse.is_just_pressed(MouseButton::Left));
    }

    #[test]
    fn test_mouse_release() {
        let mut mouse = Mouse::new();
        mouse.press(MouseButton::Left);
        mouse.release(MouseButton::Left);

        assert!(!mouse.is_pressed(MouseButton::Left));
        assert!(mouse.is_just_released(MouseButton::Left));
    }

    #[test]
    fn test_mouse_update() {
        let mut mouse = Mouse::new();
        mouse.press(MouseButton::Left);
        mouse.update();

        assert!(mouse.is_pressed(MouseButton::Left));
        assert!(!mouse.is_just_pressed(MouseButton::Left));
    }
}
