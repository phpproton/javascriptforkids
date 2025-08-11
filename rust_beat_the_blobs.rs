// Rohit: Beat the Blobs for Samar/Samir in Rust.
use macroquad::prelude::*;
use macroquad::rand::*;

// Defining the Enemy Blob.
struct Blob {
    radius: f32,
    position: Vec2,
    speed: Vec2,
}

impl Blob {
    fn new() -> Self {
        Self {
            radius: 20.0,
            position: vec2(gen_range(0.0, screen_width()), -20.0),
            speed: vec2(gen_range(-15.0, 15.0), gen_range(15.0, 25.0)),
        }
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, RED);
    }

    fn update(&mut self) {
        self.position += self.speed;
        // Collision detection with basic physics. You invert the x sign on velocity vector when hitting a wall.
        if self.position.x - self.radius <= 0.0 || self.position.x + self.radius >= screen_width() {
            self.speed.x = -self.speed.x;
        }
    }

    // Collision detection using Co-ordinate Geometry Distance Formula/Pythagorean Theorem.
    // Distance between Circle centre and closest point on Rectangle. Here the length() fn does it automatically over distance Vec2.
    // You can write it yourself also. clamp() checks circle-centre every frame with rectangle length. circle centre is clamped to min/max of rectangle.
    // According to Game engine principles, collision detection is checked for in every frame only on relatively faster moving objects.
    fn collides_with(&self, player: &Player) -> bool {
        let closest_x = self.position.x.clamp(player.position.x, player.position.x + player.player_width);
        let closest_y = self.position.y.clamp(player.position.y, player.position.y + player.player_height);
        let distance = Vec2::new(self.position.x - closest_x, self.position.y - closest_y);
        distance.length() < self.radius
    }
}

// Defining the Player
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

// The Game Loop
#[macroquad::main("BEAT THE BLOBS")]
async fn main() {
    clear_background(BLACK);
    let mut game_over = false;
    let mut blobs = vec![];
    let mut player = Player::new();
    let mut last_spawn_time = get_time();

    loop {
        let screen_w = screen_width();
        let screen_h = screen_height();

        if !game_over {
            // Limit blob spawning. 0.1 will make them appear insanely fast.
            if get_time() - last_spawn_time > 0.2 {
                blobs.push(Blob::new());
                last_spawn_time = get_time();
            }

            // Update and draw blobs
            // Game engine principles tell you update() is always first and draw() is second.
            blobs.retain_mut(|blob| {
                blob.update();
                blob.draw();

                // Collision detection applied. Blobs are faster moving than the relatively slower player.
                if blob.collides_with(&player) {
                    game_over = true;
                }

                // Remove off-screen blobs and increase score
                if blob.position.y - blob.radius > screen_h {
                    player.player_score += 1;
                    false
                } else {
                    true
                }
            });

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
                blobs.clear();
                player = Player::new();
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
