use macroquad::prelude::*;

// Tile enumeration
#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Value(u32),
}

// Game Structure
#[derive(Clone)]
struct Game {
    board: [[Tile; 4]; 4],
    score: u32,
    game_over: bool,
}

impl Game {
    fn new() -> Self {
        let mut game = Game {
            board: [[Tile::Empty; 4]; 4],
            score: 0,
            game_over: false,
        };
        game.add_random_tile();
        game.add_random_tile();
        game
    }

// First find empty tiles
    fn add_random_tile(&mut self) {
        let mut empty: Vec<(usize, usize)> = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                if let Tile::Empty = self.board[i][j] {
                    empty.push((i, j));
                }
            }
        }
// Generate 2 with 90% possibilty in empty tuples
        if !empty.is_empty() {
            let (i, j) = empty[rand::gen_range(0, empty.len())];
            self.board[i][j] = if rand::gen_range(0.0, 1.0) < 0.9 {
                Tile::Value(2)
            } else {
                Tile::Value(4)
            };
        }
    }

// You need to use Slide, Merge, and then again Slide (SMS)
    fn slide_left(&mut self) -> bool {
        let mut moved = false;
        for i in 0..4 {
            let mut new_row = [Tile::Empty; 4];
            let mut pos = 0;
            let mut prev = Tile::Empty;

            for j in 0..4 {
                match self.board[i][j] {
                    Tile::Empty => continue,
                    Tile::Value(curr) => {
                        match prev {
                            Tile::Empty => {
                                prev = Tile::Value(curr);
                            }
                            Tile::Value(prev_val) => {
                                if prev_val == curr {
                                    new_row[pos] = Tile::Value(prev_val * 2);
                                    self.score += prev_val * 2;
                                    pos += 1;
                                    prev = Tile::Empty;
                                    moved = true;
                                } else {
                                    new_row[pos] = Tile::Value(prev_val);
                                    pos += 1;
                                    prev = Tile::Value(curr);
                                    moved = true;
                                }
                            }
                        }
                    }
                }
            }
            if let Tile::Value(val) = prev {
                new_row[pos] = Tile::Value(val);
            }
            if new_row != self.board[i] {
                moved = true;
                self.board[i] = new_row;
            }
        }
        moved
    }

    // Matrix transpose function. Easy in school but mind-bender in Rust
    fn transpose(&mut self) {
        let mut new_board = [[Tile::Empty; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                new_board[i][j] = self.board[j][i];
            }
        }
        self.board = new_board;
    }

    // Reverse function to flip rows horizontally in case of up or down.
    // Do reverse again after SMS
    fn reverse(&mut self) {
        let mut new_board = [[Tile::Empty; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                new_board[i][j] = self.board[i][3 - j];
            }
        }
        self.board = new_board;
    }

fn move_board(&mut self, direction: KeyCode) -> bool {
    let moved = match direction {
        KeyCode::Left => {
            self.slide_left()
        }
        KeyCode::Right => {
            self.reverse();         // Flip horizontally
            let moved = self.slide_left();
            self.reverse();         // Flip back
            moved
        }
        KeyCode::Up => {
            self.transpose();      // Swap rows and columns
            let moved = self.slide_left();
            self.transpose();      // Swap back
            moved
        }
        KeyCode::Down => {
            self.transpose();      // Swap rows and columns
            self.reverse();        // Flip horizontally
            let moved = self.slide_left();
            self.reverse();        // Flip back
            self.transpose();      // Swap back
            moved
        }
        _ => return false,
    };

    if moved {
        self.add_random_tile();
        self.check_game_over();
    }
    moved
}
    fn check_game_over(&mut self) {
        if self.board.iter().flatten().any(|&tile| tile == Tile::Empty) {
            return;
        }

        let mut temp = self.clone();
        for dir in [KeyCode::Left, KeyCode::Up, KeyCode::Right, KeyCode::Down] {
            if temp.move_board(dir) {
                self.game_over = false;
                return;
            }
        }
        self.game_over = true;
    }
}

#[macroquad::main("SAMAR 2048")]
async fn main() {
    let mut game = Game::new();
    let tile_size = 100.0;
    let padding = 10.0;

    loop {
        clear_background(BLACK);

        if let Some(key) = get_last_key_pressed() {
            game.move_board(key);
        }

        let offset_x = (screen_width() - (4.0 * tile_size + 3.0 * padding)) / 2.0;
        let offset_y = (screen_height() - (4.0 * tile_size + 3.0 * padding)) / 2.0;

        for i in 0..4 {
            for j in 0..4 {
                let x = offset_x + (j as f32 * (tile_size + padding));
                let y = offset_y + (i as f32 * (tile_size + padding));
                
                draw_rectangle(x, y, tile_size, tile_size, GRAY);
                
                if let Tile::Value(val) = game.board[i][j] {
                    let color = match val {
                        2 => LIGHTGRAY,
                        4 => BEIGE,
                        8 => ORANGE,
                        16 => PINK,
                        32 => PURPLE,
                        64 => RED,
                        128 => BLUE,
                        256 => GREEN,
                        512 => GOLD,
                        1024 => MAGENTA,
                        2048 => WHITE,
                        _ => DARKGRAY,
                    };
                    draw_rectangle(x, y, tile_size, tile_size, color);
                    let text = val.to_string();
                    let text_dims = measure_text(&text, None, 32, 1.0);
                    draw_text(
                        &text,
                        x + (tile_size - text_dims.width) / 2.0,
                        y + (tile_size + text_dims.height) / 2.0,
                        32.0,
                        BLACK,
                    );
                }
            }
        }

        draw_text(
            &format!("Score: {}", game.score),
            20.0,
            40.0,
            40.0,
            WHITE,
        );

        if game.game_over {
            draw_text(
                "Game Over!",
                screen_width() / 2.0 - 100.0,
                screen_height() / 2.0,
                60.0,
                RED,
            );
        }

        next_frame().await;
    }
}
