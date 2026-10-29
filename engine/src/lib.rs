use std::num::NonZeroU32;
use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

use engine_core::Time;
use engine_graphics::{Renderer, Camera};
use engine_input::{Keyboard, Mouse, InputMap, KeyCode, MouseButton};
// use engine_audio::AudioManager;  // Temporarily disabled
use engine_ui::UIManager;

// Re-export commonly used types
pub use engine_input::InputAction;
pub use engine_ui::{Button, Label, Panel, UIElementId, UIElementKind};

pub struct Engine {
    window: Option<Arc<Window>>,
    surface: Option<softbuffer::Surface<Arc<Window>, Arc<Window>>>,
    renderer: Option<Renderer>,
    camera: Camera,
    keyboard: Keyboard,
    mouse: Mouse,
    input_map: InputMap,
    // audio: Option<AudioManager>,  // Temporarily disabled
    ui: UIManager,
    time: Time,
    is_running: bool,
}

impl Engine {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        Self {
            window: None,
            surface: None,
            renderer: None,
            camera: Camera::new(window_width as f32, window_height as f32),
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
            input_map: InputMap::new(),
            // audio: AudioManager::new().ok(),  // Temporarily disabled
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

    /// Copies the software framebuffer to the window.
    pub fn present(&mut self) {
        let (Some(surface), Some(renderer)) = (self.surface.as_mut(), self.renderer.as_ref()) else {
            return;
        };

        let fb = renderer.framebuffer();
        let (Some(w), Some(h)) = (NonZeroU32::new(fb.width), NonZeroU32::new(fb.height)) else {
            return; // minimized / zero-sized window: nothing to draw
        };

        if let Err(e) = surface.resize(w, h) {
            eprintln!("fire-opal: failed to resize surface: {e}");
            return;
        }

        match surface.buffer_mut() {
            Ok(mut buffer) => {
                fb.blit_to_u32(&mut buffer, fb.width, fb.height);
                if let Err(e) = buffer.present() {
                    eprintln!("fire-opal: failed to present frame: {e}");
                }
            }
            Err(e) => eprintln!("fire-opal: failed to get surface buffer: {e}"),
        }
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

    // pub fn audio(&mut self) -> Option<&mut AudioManager> {  // Temporarily disabled
    //     self.audio.as_mut()
    // }

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

    /// Length of one fixed simulation step, in seconds.
    pub fn fixed_delta_time(&self) -> f32 {
        self.time.fixed_time_step()
    }

    /// Progress (0..1) between the last and next fixed step, for render interpolation.
    pub fn alpha(&self) -> f32 {
        self.time.alpha()
    }

    pub fn frame_count(&self) -> u64 {
        self.time.frame_count()
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Start-of-frame work. Per-frame input state (just_pressed, scroll, ...)
    /// must survive until the game has seen it, so it is cleared in `end_frame`.
    fn update(&mut self) {
        let _delta = self.time.tick();

        // if let Some(audio) = &mut self.audio {  // Temporarily disabled
        //     audio.cleanup_finished();
        // }
        
        let mouse_pos = self.mouse.position();
        let mouse_pressed = self.mouse.is_pressed(engine_input::MouseButton::Left);
        self.ui.update(mouse_pos, mouse_pressed);
    }

    /// End-of-frame work: clear one-frame input state after the game has run.
    fn end_frame(&mut self) {
        self.keyboard.update();
        self.mouse.update();
    }

    fn handle_window_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.is_running = false;
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(physical_size.width, physical_size.height);
                }
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
        use winit::keyboard::{KeyCode as WinitKeyCode, PhysicalKey};
        match key {
            PhysicalKey::Code(WinitKeyCode::KeyA) => Some(KeyCode::A),
            PhysicalKey::Code(WinitKeyCode::KeyB) => Some(KeyCode::B),
            PhysicalKey::Code(WinitKeyCode::KeyC) => Some(KeyCode::C),
            PhysicalKey::Code(WinitKeyCode::KeyD) => Some(KeyCode::D),
            PhysicalKey::Code(WinitKeyCode::KeyE) => Some(KeyCode::E),
            PhysicalKey::Code(WinitKeyCode::KeyF) => Some(KeyCode::F),
            PhysicalKey::Code(WinitKeyCode::KeyG) => Some(KeyCode::G),
            PhysicalKey::Code(WinitKeyCode::KeyH) => Some(KeyCode::H),
            PhysicalKey::Code(WinitKeyCode::KeyI) => Some(KeyCode::I),
            PhysicalKey::Code(WinitKeyCode::KeyJ) => Some(KeyCode::J),
            PhysicalKey::Code(WinitKeyCode::KeyK) => Some(KeyCode::K),
            PhysicalKey::Code(WinitKeyCode::KeyL) => Some(KeyCode::L),
            PhysicalKey::Code(WinitKeyCode::KeyM) => Some(KeyCode::M),
            PhysicalKey::Code(WinitKeyCode::KeyN) => Some(KeyCode::N),
            PhysicalKey::Code(WinitKeyCode::KeyO) => Some(KeyCode::O),
            PhysicalKey::Code(WinitKeyCode::KeyP) => Some(KeyCode::P),
            PhysicalKey::Code(WinitKeyCode::KeyQ) => Some(KeyCode::Q),
            PhysicalKey::Code(WinitKeyCode::KeyR) => Some(KeyCode::R),
            PhysicalKey::Code(WinitKeyCode::KeyS) => Some(KeyCode::S),
            PhysicalKey::Code(WinitKeyCode::KeyT) => Some(KeyCode::T),
            PhysicalKey::Code(WinitKeyCode::KeyU) => Some(KeyCode::U),
            PhysicalKey::Code(WinitKeyCode::KeyV) => Some(KeyCode::V),
            PhysicalKey::Code(WinitKeyCode::KeyW) => Some(KeyCode::W),
            PhysicalKey::Code(WinitKeyCode::KeyX) => Some(KeyCode::X),
            PhysicalKey::Code(WinitKeyCode::KeyY) => Some(KeyCode::Y),
            PhysicalKey::Code(WinitKeyCode::KeyZ) => Some(KeyCode::Z),
            PhysicalKey::Code(WinitKeyCode::Space) => Some(KeyCode::Space),
            PhysicalKey::Code(WinitKeyCode::Enter) => Some(KeyCode::Enter),
            PhysicalKey::Code(WinitKeyCode::Escape) => Some(KeyCode::Escape),
            PhysicalKey::Code(WinitKeyCode::Tab) => Some(KeyCode::Tab),
            PhysicalKey::Code(WinitKeyCode::Backspace) => Some(KeyCode::Backspace),
            PhysicalKey::Code(WinitKeyCode::ArrowUp) => Some(KeyCode::Up),
            PhysicalKey::Code(WinitKeyCode::ArrowDown) => Some(KeyCode::Down),
            PhysicalKey::Code(WinitKeyCode::ArrowLeft) => Some(KeyCode::Left),
            PhysicalKey::Code(WinitKeyCode::ArrowRight) => Some(KeyCode::Right),
            PhysicalKey::Code(WinitKeyCode::Digit0) => Some(KeyCode::Key0),
            PhysicalKey::Code(WinitKeyCode::Digit1) => Some(KeyCode::Key1),
            PhysicalKey::Code(WinitKeyCode::Digit2) => Some(KeyCode::Key2),
            PhysicalKey::Code(WinitKeyCode::Digit3) => Some(KeyCode::Key3),
            PhysicalKey::Code(WinitKeyCode::Digit4) => Some(KeyCode::Key4),
            PhysicalKey::Code(WinitKeyCode::Digit5) => Some(KeyCode::Key5),
            PhysicalKey::Code(WinitKeyCode::Digit6) => Some(KeyCode::Key6),
            PhysicalKey::Code(WinitKeyCode::Digit7) => Some(KeyCode::Key7),
            PhysicalKey::Code(WinitKeyCode::Digit8) => Some(KeyCode::Key8),
            PhysicalKey::Code(WinitKeyCode::Digit9) => Some(KeyCode::Key9),
            _ => None,
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
    /// Runs zero or more times per frame at a constant `dt` (see
    /// `Engine::fixed_delta_time`). Put physics and other simulation here so it
    /// behaves the same regardless of frame rate. Optional.
    fn fixed_update(&mut self, _engine: &mut Engine, _dt: f32) {}

    /// Runs once per frame with a variable delta. Good for input and UI.
    fn update(&mut self, engine: &mut Engine);
    fn render(&mut self, engine: &mut Engine);
}

pub fn run<T: Game + 'static>(game: T) {
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
        // `resumed` can fire more than once (e.g. mobile); only create the window once.
        if self.engine.window.is_some() {
            return;
        }

        let window_attributes = Window::default_attributes()
            .with_title("Fire Opal Engine")
            .with_inner_size(winit::dpi::PhysicalSize::new(800, 600));

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let size = window.inner_size();

        let context = softbuffer::Context::new(window.clone())
            .expect("failed to create softbuffer context");
        let surface = softbuffer::Surface::new(&context, window.clone())
            .expect("failed to create softbuffer surface");

        self.engine.renderer = Some(Renderer::new(size.width, size.height));
        self.engine.camera.set_viewport_size(engine_core::Vec2::new(
            size.width as f32,
            size.height as f32,
        ));
        self.engine.surface = Some(surface);
        self.engine.window = Some(window);
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

        let step = self.engine.fixed_delta_time();
        while self.engine.time.consume_fixed_step() {
            self.game.fixed_update(&mut self.engine, step);
        }
        self.game.update(&mut self.engine);
        self.game.render(&mut self.engine);
        self.engine.present();
        self.engine.end_frame();

        // Update window title
        if let Some(window) = &self.engine.window {
            window.set_title(&format!("Fire Opal Engine - FPS: {:.0}", self.engine.fps()));
            window.request_redraw();
        }
    }
}
