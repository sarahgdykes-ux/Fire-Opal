use engine::{Game, Engine, InputAction, Button, Label, Panel, UIElementId};
use engine_core::{Color, Vec2, Rect};
use engine_graphics::Color as GColor;

struct PlatformerGame {
    player_pos: (f32, f32),
    player_velocity: (f32, f32),
    jump_button_id: Option<UIElementId>,
    score: i32,
    score_label_id: Option<UIElementId>,
}

impl PlatformerGame {
    fn new() -> Self {
        Self {
            player_pos: (400.0, 300.0),
            player_velocity: (0.0, 0.0),
            jump_button_id: None,
            score: 0,
            score_label_id: None,
        }
    }

    fn setup_ui(&mut self, engine: &mut Engine) {
        let ui = engine.ui_mut();

        // Create HUD panel
        let hud_panel = Panel::new(Rect::from_min_size(Vec2::new(10.0, 10.0), Vec2::new(200.0, 80.0)));
        ui.add_panel(hud_panel);

        // Create score label
        let score_label = Label::new(
            Rect::from_min_size(Vec2::new(20.0, 20.0), Vec2::new(180.0, 30.0)),
            format!("Score: {}", self.score)
        );
        self.score_label_id = Some(ui.add_label(score_label));

        // Create jump button
        let jump_button = Button::new(
            Rect::from_min_size(Vec2::new(20.0, 50.0), Vec2::new(100.0, 30.0)),
            "Jump"
        );
        self.jump_button_id = Some(ui.add_button(jump_button));
    }
}

impl Game for PlatformerGame {
    fn update(&mut self, engine: &mut Engine) {
        // Setup UI on first frame
        if self.jump_button_id.is_none() {
            self.setup_ui(engine);
        }

        let keyboard = engine.keyboard();
        let input_map = engine.input_map();
        let delta = engine.delta_time();

        let move_speed = 200.0;
        let jump_force = -400.0;
        let gravity = 800.0;

        // Keyboard movement
        if input_map.is_action_pressed(InputAction::MoveLeft, keyboard, engine.mouse()) {
            self.player_velocity.0 = -move_speed;
        } else if input_map.is_action_pressed(InputAction::MoveRight, keyboard, engine.mouse()) {
            self.player_velocity.0 = move_speed;
        } else {
            self.player_velocity.0 = 0.0;
        }

        // Jump via keyboard or UI button
        let jump_triggered = input_map.is_action_just_pressed(InputAction::Jump, keyboard, engine.mouse())
            || engine.ui().is_button_clicked(self.jump_button_id.unwrap_or(UIElementId(0)));

        if jump_triggered {
            self.player_velocity.1 = jump_force;
            self.score += 10;
            
            // Update score label
            if let Some(label_id) = self.score_label_id {
                if let Some(label) = engine.ui_mut().get_label_mut(label_id) {
                    label.set_text(format!("Score: {}", self.score));
                }
            }
        }

        self.player_velocity.1 += gravity * delta;
        self.player_pos.0 += self.player_velocity.0 * delta;
        self.player_pos.1 += self.player_velocity.1 * delta;

        // Simple floor collision
        if self.player_pos.1 > 550.0 {
            self.player_pos.1 = 550.0;
            self.player_velocity.1 = 0.0;
        }

        // Camera follow
        let camera = engine.camera_mut();
        camera.set_position(Vec2::new(self.player_pos.0, self.player_pos.1));
    }

    fn render(&mut self, engine: &mut Engine) {
        engine.set_clear_color(Color::new(0.1, 0.1, 0.2, 1.0));
        
        // For now, just clear the screen with a dark blue background
        // The GPU renderer will display this to the window
        // Full rendering implementation would add dynamic vertex buffers for sprites/rects
    }
}

fn main() {
    let game = PlatformerGame::new();
    engine::run(game);
}
