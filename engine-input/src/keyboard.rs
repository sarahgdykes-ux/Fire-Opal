use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Space,
    Enter,
    Escape,
    Tab,
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Released,
    Pressed,
}

pub struct Keyboard {
    pressed: HashSet<KeyCode>,
    just_pressed: HashSet<KeyCode>,
    just_released: HashSet<KeyCode>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }

    pub fn press(&mut self, key: KeyCode) {
        if !self.pressed.contains(&key) {
            self.just_pressed.insert(key);
        }
        self.pressed.insert(key);
    }

    pub fn release(&mut self, key: KeyCode) {
        self.pressed.remove(&key);
        self.just_released.insert(key);
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }

    pub fn is_just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed.contains(&key)
    }

    pub fn is_just_released(&self, key: KeyCode) -> bool {
        self.just_released.contains(&key)
    }

    pub fn state(&self, key: KeyCode) -> KeyState {
        if self.pressed.contains(&key) {
            KeyState::Pressed
        } else {
            KeyState::Released
        }
    }

    pub fn update(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_press() {
        let mut keyboard = Keyboard::new();
        keyboard.press(KeyCode::A);

        assert!(keyboard.is_pressed(KeyCode::A));
        assert!(keyboard.is_just_pressed(KeyCode::A));
    }

    #[test]
    fn test_keyboard_release() {
        let mut keyboard = Keyboard::new();
        keyboard.press(KeyCode::A);
        keyboard.release(KeyCode::A);

        assert!(!keyboard.is_pressed(KeyCode::A));
        assert!(keyboard.is_just_released(KeyCode::A));
    }

    #[test]
    fn test_keyboard_update() {
        let mut keyboard = Keyboard::new();
        keyboard.press(KeyCode::A);
        keyboard.update();

        assert!(keyboard.is_pressed(KeyCode::A));
        assert!(!keyboard.is_just_pressed(KeyCode::A));
    }
}
