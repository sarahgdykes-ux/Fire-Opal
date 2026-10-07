use winit::{
    application::ApplicationHandler,
    event::{WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

use engine_core::Time;
use engine_graphics::{Renderer, Camera};
use engine_input::{Keyboard, Mouse, InputMap, KeyCode, MouseButton};
use engine_audio::{AudioManager, SoundSettings};
use engine_ui::UIManager;

pub struct Engine {
    window: Option<Window>,
    renderer: Option<Renderer>,
    camera: Camera,
    keyboard: Keyboard,
    mouse: Mouse,
    input_map: InputMap,
    audio: Option<AudioManager>,
    ui: UIManager,
    time: Time,
    is_running: bool,
}

impl Engine {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        Self {
            window: None,
            renderer: None,
            camera: Camera::new(window_width as f32, window_height as f32),
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
            input_map: InputMap::new(),
            audio: AudioManager::new().ok(),
            ui: UIManager::new(),
            time: Time::new(1.0 / 60.0),
            is_running: true,
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn renderer(&self) -> Option<&Renderer> {
        self.renderer.as_ref()
    }

    pub fn renderer_mut(&mut self) -> Option<&mut Renderer> {
        self.renderer.as_mut()
    }

    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }

    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }

    pub fn input_map(&self) -> &InputMap {
        &self.input_map
    }

    pub fn audio(&mut self) -> Option<&mut AudioManager> {
        self.audio.as_mut()
    }

    pub fn ui(&self) -> &UIManager {
        &self.ui
    }

    pub fn ui_mut(&mut self) -> &mut UIManager {
        &mut self.ui
    }

    pub fn delta_time(&self) -> f32 {
        self.time.delta_time().as_secs_f32()
    }

    pub fn fps(&self) -> f32 {
        self.time.fps()
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    fn update(&mut self) {
        let _delta = self.time.tick();
        self.keyboard.update();
        self.mouse.update();
        
        if let Some(audio) = &mut self.audio {
            audio.cleanup_finished();
        }
        
        let mouse_pos = self.mouse.position();
        let mouse_pressed = self.mouse.is_pressed(engine_input::MouseButton::Left);
        self.ui.update(mouse_pos, mouse_pressed);
    }

    fn handle_window_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.is_running = false;
            }
            WindowEvent::Resized(physical_size) => {
                self.camera.set_viewport_size(engine_core::Vec2::new(
                    physical_size.width as f32,
                    physical_size.height as f32,
                ));
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let Some(key) = self.map_winit_key(event.physical_key) {
                    if event.state == winit::event::ElementState::Pressed {
                        self.keyboard.press(key);
                    } else {
                        self.keyboard.release(key);
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let mouse_button = self.map_winit_mouse_button(button);
                if state == winit::event::ElementState::Pressed {
                    self.mouse.press(mouse_button);
                } else {
                    self.mouse.release(mouse_button);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse.set_position(engine_core::Vec2::new(
                    position.x as f32,
                    position.y as f32,
                ));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => y,
                    winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                };
                self.mouse.set_scroll(scroll);
            }
            _ => {}
        }
    }

    fn map_winit_key(&self, key: winit::keyboard::PhysicalKey) -> Option<KeyCode> {
        use winit::keyboard::KeyCode as WinitKeyCode;
        match key {
            WinitKeyCode::KeyA => Some(KeyCode::A),
            WinitKeyCode::KeyB => Some(KeyCode::B),
            WinitKeyCode::KeyC => Some(KeyCode::C),
            WinitKeyCode::KeyD => Some(KeyCode::D),
            WinitKeyCode::KeyE => Some(KeyCode::E),
            WinitKeyCode::KeyF => Some(KeyCode::F),
            WinitKeyCode::KeyG => Some(KeyCode::G),
            WinitKeyCode::KeyH => Some(KeyCode::H),
            WinitKeyCode::KeyI => Some(KeyCode::I),
            WinitKeyCode::KeyJ => Some(KeyCode::J),
            WinitKeyCode::KeyK => Some(KeyCode::K),
            WinitKeyCode::KeyL => Some(KeyCode::L),
            WinitKeyCode::KeyM => Some(KeyCode::M),
            WinitKeyCode::KeyN => Some(KeyCode::N),
            WinitKeyCode::KeyO => Some(KeyCode::O),
            WinitKeyCode::KeyP => Some(KeyCode::P),
            WinitKeyCode::KeyQ => Some(KeyCode::Q),
            WinitKeyCode::KeyR => Some(KeyCode::R),
            WinitKeyCode::KeyS => Some(KeyCode::S),
            WinitKeyCode::KeyT => Some(KeyCode::T),
            WinitKeyCode::KeyU => Some(KeyCode::U),
            WinitKeyCode::KeyV => Some(KeyCode::V),
            WinitKeyCode::KeyW => Some(KeyCode::W),
            WinitKeyCode::KeyX => Some(KeyCode::X),
            WinitKeyCode::KeyY => Some(KeyCode::Y),
            WinitKeyCode::KeyZ => Some(KeyCode::Z),
            WinitKeyCode::Space => Some(KeyCode::Space),
            WinitKeyCode::Enter => Some(KeyCode::Enter),
            WinitKeyCode::Escape => Some(KeyCode::Escape),
            WinitKeyCode::Tab => Some(KeyCode::Tab),
            WinitKeyCode::Backspace => Some(KeyCode::Backspace),
            WinitKeyCode::ArrowUp => Some(KeyCode::Up),
            WinitKeyCode::ArrowDown => Some(KeyCode::Down),
            WinitKeyCode::ArrowLeft => Some(KeyCode::Left),
            WinitKeyCode::ArrowRight => Some(KeyCode::Right),
            WinitKeyCode::Digit0 => Some(KeyCode::Key0),
            WinitKeyCode::Digit1 => Some(KeyCode::Key1),
            WinitKeyCode::Digit2 => Some(KeyCode::Key2),
            WinitKeyCode::Digit3 => Some(KeyCode::Key3),
            WinitKeyCode::Digit4 => Some(KeyCode::Key4),
            WinitKeyCode::Digit5 => Some(KeyCode::Key5),
            WinitKeyCode::Digit6 => Some(KeyCode::Key6),
            WinitKeyCode::Digit7 => Some(KeyCode::Key7),
            WinitKeyCode::Digit8 => Some(KeyCode::Key8),
            WinitKeyCode::Digit9 => Some(KeyCode::Key9),
            _ => Some(KeyCode::Unknown),
        }
    }

    fn map_winit_mouse_button(&self, button: winit::event::MouseButton) -> MouseButton {
        match button {
            winit::event::MouseButton::Left => MouseButton::Left,
            winit::event::MouseButton::Right => MouseButton::Right,
            winit::event::MouseButton::Middle => MouseButton::Middle,
            _ => MouseButton::Unknown,
        }
    }
}

pub trait Game {
    fn update(&mut self, engine: &mut Engine);
    fn render(&mut self, engine: &mut Engine);
}

pub fn run<T: Game + 'static>(mut game: T) {
    let event_loop = EventLoop::new().unwrap();
    let mut app = EngineApp::new(game);
    event_loop.run_app(&mut app).unwrap();
}

struct EngineApp<T: Game> {
    game: T,
    engine: Engine,
}

impl<T: Game> EngineApp<T> {
    fn new(game: T) -> Self {
        Self {
            game,
            engine: Engine::new(800, 600),
        }
    }
}

impl<T: Game> ApplicationHandler for EngineApp<T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Fire Opal Engine")
            .with_inner_size(winit::dpi::PhysicalSize::new(800, 600));

        self.engine.window = Some(event_loop.create_window(window_attributes).unwrap());

        if let Some(window) = &self.engine.window {
            let size = window.inner_size();
            self.engine.renderer = Some(Renderer::new(size.width, size.height));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        self.engine.handle_window_event(event);

        if !self.engine.is_running {
            event_loop.exit();
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.engine.update();

        self.game.update(&mut self.engine);
        self.game.render(&mut self.engine);

        if let Some(window) = &self.engine.window {
            window.request_redraw();
        }
    }
}
