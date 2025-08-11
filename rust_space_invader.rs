// Rohit: Space Invaders for Samar/Samir in Rust
use macroquad::prelude::*;

// Defining Invader
struct Invader {
    position: Vec2,
    velocity: Vec2,
    invader_width: f32,
    invader_height: f32,
}

impl Invader {
    fn new(x: f32, y: f32) -> Self {
        Self {
            position: vec2(x, y),
            velocity: vec2(2.0, 0.0), // Slower movement
            invader_width: 40.0,
            invader_height: 20.0,
        }
    }

    fn update(&mut self) {
        self.position += self.velocity;

        if self.position.x <= 0.0 || self.position.x + self.invader_width >= screen_width() {
            self.velocity.x = -self.velocity.x;
            self.position.y += 20.0; // Move down when hitting the edge
        }
    }

    fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.invader_width,
            self.invader_height,
            RED,
        );
    }
}

// Defining Player
struct Player {
    position: Vec2,
    player_width: f32,
    player_height: f32,
    player_score: u32,
}

impl Player {
    fn new() -> Self {
        Self {
            position: vec2(screen_width() / 2.0, screen_height() - 30.0),
            player_width: 80.0,
            player_height: 20.0,
            player_score: 0,
        }
    }

    fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.player_width,
            self.player_height,
            BLUE,
        );
    }

    fn update(&mut self) {
        let screen_w = screen_width();
        self.position.x = self.position.x.clamp(0.0, screen_w - self.player_width);

        if is_key_down(KeyCode::Left) {
            self.position.x -= 10.0;
        }
        if is_key_down(KeyCode::Right) {
            self.position.x += 10.0;
        }
    }
}

// Defining Bullet
struct Bullet {
    position: Vec2,
    bullet_width: f32,
    bullet_height: f32,
}

impl Bullet {
    fn new(player_position: Vec2, player_width: f32) -> Self {
        Self {
            position: vec2(player_position.x + player_width / 2.0 - 5.0, player_position.y - 10.0),
            bullet_width: 10.0, // Larger bullets
            bullet_height: 20.0,
        }
    }

    fn update(&mut self) {
        self.position.y -= 5.0; // Slower bullets
    }

    fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.bullet_width,
            self.bullet_height,
            WHITE,
        );
    }

    fn collides_with(&self, invader: &Invader) -> bool {
        self.position.x < invader.position.x + invader.invader_width
            && self.position.x + self.bullet_width > invader.position.x
            && self.position.y < invader.position.y + invader.invader_height
            && self.position.y + self.bullet_height > invader.position.y
    }
}

// The Game Loop
#[macroquad::main("SPACE INVADERS")]
async fn main() {
    let mut game_over = false;
    let mut invaders: Vec<Invader> = vec![
        Invader::new(50.0, 50.0),
        Invader::new(150.0, 50.0),
        Invader::new(250.0, 50.0),
        Invader::new(350.0, 50.0),
        Invader::new(450.0, 50.0),
    ];
    let mut player = Player::new();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut last_shot_time = 0.0;

    loop {
        let screen_w = screen_width();
        let screen_h = screen_height();

        clear_background(BLACK);

        if !game_over {
            // Handle Bullets
            if is_key_pressed(KeyCode::Space) && get_time() - last_shot_time > 0.2 {
                bullets.push(Bullet::new(player.position, player.player_width));
                last_shot_time = get_time();
            }

            // Update and draw bullets
            bullets.retain_mut(|bullet| {
                bullet.update();
                bullet.draw();
                bullet.position.y > 0.0 // Retain bullets that are on-screen
            });

            // Update and draw invaders
            invaders.retain_mut(|invader| {
                invader.update();
                invader.draw();
                invader.position.y < screen_h // Retain invaders that are on-screen
            });

            // Collision detection between bullets and invaders
            bullets.retain(|bullet| {
                for invader in invaders.iter_mut() {
                    if bullet.collides_with(invader) {
                        invader.position.y = -100.0; // Move invader off-screen
                        player.player_score += 1;
                        return false; // Remove the bullet
                    }
                }
                true // Keep the bullet
            });

            // Check if all invaders are destroyed
            if invaders.is_empty() {
                game_over = true;
            }

            // Update and draw player
            player.update();
            player.draw();
        } else {
            draw_text(
                "Game Over",
                screen_w / 2.0 - 100.0,
                screen_h / 2.0,
                50.0,
                RED,
            );
            draw_text(
                "Press SPACE to Restart",
                screen_w / 2.0 - 150.0,
                screen_h / 2.0 + 60.0,
                30.0,
                WHITE,
            );
            if is_key_pressed(KeyCode::Space) {
                invaders = vec![
                    Invader::new(50.0, 50.0),
                    Invader::new(150.0, 50.0),
                    Invader::new(250.0, 50.0),
                    Invader::new(350.0, 50.0),
                    Invader::new(450.0, 50.0),
                ];
                player = Player::new();
                bullets.clear();
                game_over = false;
            }
        }

        // Draw UI
        draw_text(
            &format!("Score: {}", player.player_score),
            10.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text("ROHIT CHAUHAN", screen_w - 200.0, 30.0, 30.0, WHITE);

        next_frame().await;
    }
}
