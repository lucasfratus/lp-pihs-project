#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

// Game State
static mut GAME_OVER: bool = false;
static mut FRAME_COUNT: u32 = 0;
// static mut POINTS: u32 = 0; TO IMPLEMENT

// Scenario Settings
const FLOOR_HEIGHT: i32 = 148;
const GRAVITY: i32 = 1;
static mut SCENARIO_SPEED: i32 = 1;

// Player Settings
const PLAYER_WIDTH: i32 = 8;
const PLAYER_HEIGHT: i32 = 8;
const PLAYER_JUMP_FORCE: i32 = -14;

// Coins Settings
const COIN_Y: i32 = FLOOR_HEIGHT - 8;
const COIN_WIDTH: i32 = 4;
const COIN_HEIGTH: i32 = 4;
const GAP: i32 = 10;
const MAX_COIN_QTDY: usize = 5;
// const MIN_COIN_QTDY: usize = 2; TO IMPLEMENT

// Barrier Settings
const BARRIER_WIDTH: i32 = 12;
const BARRIER_UP_MAX_HEIGHT: i32 = 100;
// const BARRIER_UP_MIN_HEIGHT: i32 = 110; TO IMPLEMENT
const BARRIER_DOWN_MAX_HEIGHT: i32 = 80;
// const BARRIER_DOWN_MIN_HEIGHT: i32 = 50; TO IMPLEMENT

// Player Struct
pub(crate) struct Player {
    x: i32,
    y: i32,
    velocity_y: i32,
    is_jumping: bool,
    score: u8,
    lives: u8,
}

// Coin Struct and Implementations
struct Coin {
    x: i32,
    y: i32,
    not_collected: bool,
}
impl Coin {
    fn new(x: i32, y: i32) -> Self {
        Self {x, y, not_collected: true }
    }

    fn update(&mut self) {
        unsafe {
            self.x -= SCENARIO_SPEED;
            if self.x + COIN_WIDTH < 0 {
                self.x = SCREEN_SIZE as i32 + 20;
                self.not_collected = true;
            }
        }
    }

    fn draw(&self) {
        if self.not_collected {
            unsafe {*DRAW_COLORS = 0x21 }
            oval(self.x, self.y, COIN_WIDTH as u32, COIN_HEIGTH as u32)
        }
    }
}

// Barrier Struct and Implementations
struct Barrier {
    x: i32,
    y: i32,
    height: i32,
}
impl Barrier {
    fn new(x: i32, y: i32, height: i32) -> Self {
        Self {x, y, height}
    }

    fn update(&mut self) {
        unsafe {
            self.x -= SCENARIO_SPEED;
            if self.x + BARRIER_WIDTH < 0 {
                self.x = SCREEN_SIZE as i32 + 40;
            }
        }
    }

    fn draw(&self){
        unsafe {
            *DRAW_COLORS = 0x33;
        }
        rect(self.x, self.y, BARRIER_WIDTH as u32, self.height as u32);
    }
}

// Build Global Structs
static mut PLAYER: Player = Player { 
    x: 45,
    y: 30,
    velocity_y: 0,
    is_jumping: false,
    score: 0,
    lives: 3
};
static mut COINS: Option<Vec<Coin>> = None;
static mut BARRIERS: Option<Vec<Barrier>> = None;

// Functions
fn restart() {
    // Rebuild Player
    unsafe {
        PLAYER.x = 45;
        PLAYER.y = 30;
        PLAYER.velocity_y = 0;
        PLAYER.is_jumping = false;
        PLAYER.score = 0;
        PLAYER.lives = 3;
        SCENARIO_SPEED = 1;
        FRAME_COUNT = 0;
        GAME_OVER = false;
    }

    start(); // Rebuild Coins and Barriers
}

fn collision(a_x: i32, a_y: i32, a_w: i32, a_h: i32, b_x: i32, b_y: i32, b_w: i32, b_h: i32) -> bool {
    a_x < b_x + b_w &&
    a_x + a_w > b_x &&
    a_y < b_y + b_h &&
    a_y + a_h > b_y
}


#[no_mangle]
pub fn start() {
    // Coins
    let mut coins = Vec::new();
    
    for i in 0..MAX_COIN_QTDY {
        let x = 160 + i as i32 * GAP;
        coins.push(Coin::new(x, COIN_Y));
    }

    unsafe {
        COINS = Some(coins);
    }

    // Barriers
    let mut barriers = Vec::new();

    barriers.push(Barrier::new(SCREEN_SIZE as i32 + 100, 0, BARRIER_UP_MAX_HEIGHT));  // Up Barrier
    barriers.push(Barrier::new(SCREEN_SIZE as i32, BARRIER_DOWN_MAX_HEIGHT, FLOOR_HEIGHT - BARRIER_DOWN_MAX_HEIGHT)); // Down Barrier

    unsafe {
        BARRIERS = Some(barriers);
    }
}

#[no_mangle]
fn update() {
    // Color Pallete
    unsafe {
    *PALETTE = [
        0x7e1f23,
        0x5e4069,
        0xc4181f,
        0x120a19,
    ];
    }
    unsafe { 
        *DRAW_COLORS = 0x33
    }
    oval(-30,-30, 80, 80);
    let gamepad = unsafe { *wasm4 ::GAMEPAD1 };

    // Checks if the player has lost all lives
    unsafe {
        if PLAYER.lives == 0 {
            GAME_OVER = true;
        }

        if GAME_OVER {
            // Defines the game over background and text
            *DRAW_COLORS = 1; 
            rect(0, 0, SCREEN_SIZE, 160); // Cleans screen with background color

            *DRAW_COLORS = 4;
            text("GAME OVER", 40, 60);
            let final_score = format!("Score: {}", PLAYER.score);
            text(&final_score, 45, 70);
            text("Press X", 30, 90);
            text("to restart", 30, 100);
            // Checks if the X button was pressed
            if gamepad & BUTTON_1 != 0 {
                restart();
            }
            return;
        }
    }

    unsafe {
    if let Some(coins) = &mut COINS {
            for coin in coins.iter_mut() {
                if coin.not_collected && collision(PLAYER.x, PLAYER.y, PLAYER_WIDTH, PLAYER_HEIGHT, coin.x, coin.y, COIN_WIDTH, COIN_HEIGTH){
                    coin.not_collected = false;
                    PLAYER.score += 1;
                }
                coin.update();
                coin.draw();
            }
    }

    *DRAW_COLORS = 4;
    let coins_text = format!("Coins: {}", PLAYER.score);
    text(&coins_text, 10, 10);

    let lives_text = format!("Lives: {}", PLAYER.lives);
    text(&lives_text, 10, 20);
    }

    unsafe {
        if let Some(barriers) = &mut BARRIERS {
            for barrier in barriers.iter_mut() {
                if collision(PLAYER.x, PLAYER.y, PLAYER_WIDTH, PLAYER_HEIGHT, barrier.x, barrier.y, BARRIER_WIDTH, barrier.height) {
                    PLAYER.lives = PLAYER.lives.saturating_sub(1);
                    barrier.x = SCREEN_SIZE as i32 + 40 + (FRAME_COUNT % 100) as i32;
                }
                barrier.update();
                barrier.draw();
            }
        }
    }

    unsafe {
    // Horizontal Movement
    if gamepad & BUTTON_LEFT != 0 {
        PLAYER.x -= 2;
    }
    if gamepad & BUTTON_RIGHT != 0 {
        PLAYER.x += 2;
    }

    // Jump Movement
    if gamepad & BUTTON_1 != 0 && !PLAYER.is_jumping {
        PLAYER.velocity_y = PLAYER_JUMP_FORCE;
        PLAYER.is_jumping = true;
    }

    // Apply Gravity
    PLAYER.velocity_y += GRAVITY;
    PLAYER.y += PLAYER.velocity_y;


    // Checks if Player has reached the floor
    if PLAYER.y >= FLOOR_HEIGHT {
        PLAYER.y = FLOOR_HEIGHT;
        PLAYER.velocity_y = 0;
        PLAYER.is_jumping = false;
    }

    // Limit the Player on the screen
    if PLAYER.x < 0 {
        PLAYER.x = 0;
    }
    if PLAYER.y < 0 {
        PLAYER.y = 0;
    }
    if PLAYER.x > SCREEN_SIZE as i32 - PLAYER_WIDTH {
        PLAYER.x = SCREEN_SIZE as i32 - PLAYER_WIDTH;
    }
    if PLAYER.y > FLOOR_HEIGHT - PLAYER_HEIGHT {
        PLAYER.y = FLOOR_HEIGHT - PLAYER_HEIGHT;
    }
    }

    // Draw the floor
    unsafe {*DRAW_COLORS = 0x4;}
    rect(0, FLOOR_HEIGHT, 160, FLOOR_HEIGHT as u32);


    // Draw the Player
    unsafe { *DRAW_COLORS = 0x4142} // Player sprite color
    blit(
        // Player sprite: Byte Array in 2BPP
        &[0x80,0x0a,0x00,0x02,0x2f,0xfa,0x8f,0xfa,0x80,0xfe,0xbf,0xff,0x8f,0xfa,0x04,0x6a],
        unsafe {
            PLAYER.x
        },
        unsafe {
            PLAYER.y
        },
        PLAYER_WIDTH as u32,
        PLAYER_HEIGHT as u32,
        BLIT_2BPP,
    );

    unsafe {
        FRAME_COUNT += 1;

        /*
        // Change Scenario Speed
        if FRAME_COUNT % 1000 == 0 && SCENARIO_SPEED < 2 {
            SCENARIO_SPEED += 1;
        }
        */
    }   
}