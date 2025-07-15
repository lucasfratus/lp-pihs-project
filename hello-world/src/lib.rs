#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

// Global and Objects ------------------------------------------------------------------------------------------------------
// Game State and Settings
static mut GAME_START: bool = true;
static mut GAME_OVER: bool = false;
static mut FRAME_COUNT: u32 = 0;
static mut RNG_SEED: u32 = 123456789;

// Scenario Settings
const FLOOR_HEIGHT: i32 = 148;
const GRAVITY: i32 = 1;
static mut SCENARIO_SPEED: i32 = 1;

// Player Settings
const PLAYER_WIDTH: i32 = 8;
const PLAYER_HEIGHT: i32 = 8;
const PLAYER_JUMP_FORCE: i32 = -14;

// Coins Settings
static mut COIN_VELOCITY: i32 = 2;
const COIN_WIDTH: i32 = 10;
const COIN_HEIGTH: i32 = 10;
const COIN_MAX_Y: i32 = 60;
const COIN_MIN_Y: i32 = 135;

// Barrier Settings
const BARRIER_GAP: i32 = 50;
const BARRIER_WIDTH: i32 = 12;
const BARRIER_UP_HEIGHT: i32 = 90;
const BARRIER_DOWN_HEIGHT: i32 = 80;

// Player Struct
pub(crate) struct Player {
    x: i32,
    y: i32,
    velocity_y: i32,
    is_jumping: bool,
    score: u8,
    lives: u8,
}

// Coin Struct
struct Coin {
    x: i32,
    y: i32,
    not_collected: bool,
}

// Barrier Displacement and Barrier Structs
#[derive(Copy, Clone)]
pub enum BarrierDisplacement {
    Minus, Equal, Plus
}
impl BarrierDisplacement {
    fn adjust_displacement(self) -> i32 {
        match self {
            BarrierDisplacement::Minus => -10,
            BarrierDisplacement::Equal => 0,
            BarrierDisplacement::Plus => 10,
        }
    }
    fn random() -> Self {
        use BarrierDisplacement::*;
        match random_range(0, 2) {
            0 => Minus,
            1 => Equal,
            _ => Plus,
        }
    }
}
struct Barrier {
    x: i32,
    y: i32,
    height: i32,
    active: bool,
    displacement: BarrierDisplacement,
}

// Build Global Structs ----------------------------------------------------------------------------------------------------
static mut PLAYER: Player = Player { 
    x: 45,
    y: 30,
    velocity_y: 0,
    is_jumping: true,
    score: 0,
    lives: 3
};

static mut COIN: Coin = Coin {
    x: SCREEN_SIZE as i32 + 20,
    y: 100,
    not_collected: true,
};

static mut BARRIERS: [Barrier; 2] = [
    Barrier {
        x: SCREEN_SIZE as i32 + BARRIER_WIDTH,
        y: BARRIER_DOWN_HEIGHT,
        height: FLOOR_HEIGHT - BARRIER_DOWN_HEIGHT,
        active: true,
        displacement: BarrierDisplacement::Equal,
    },
    Barrier {
        x: SCREEN_SIZE as i32 + BARRIER_WIDTH + BARRIER_GAP,
        y: 0,
        height: BARRIER_UP_HEIGHT,
        active: true,
        displacement: BarrierDisplacement::Equal,
    }
];

// Game Start Functions ----------------------------------------------------------------------------------------------------
fn restart() {
    let gamepad = unsafe { *wasm4 ::GAMEPAD1 };
    if gamepad & BUTTON_1 != 0 {
        start()
    }
}

#[no_mangle]
pub fn start() {
    // Define Color Palette
    unsafe {
    *PALETTE = [
        0x7e1f23,
        0x5e4069,
        0xc4181f,
        0x120a19,
    ];

    // Game State Settings
    SCENARIO_SPEED = 1;
    FRAME_COUNT = 0;
    GAME_OVER = false;
    GAME_START = false;
    
    // Player
    PLAYER.x = 45;
    PLAYER.y = 30;
    PLAYER.velocity_y = 0;
    PLAYER.is_jumping = true;
    PLAYER.score = 0;
    PLAYER.lives = 3;

    // Coin
    COIN.x = SCREEN_SIZE as i32 + 20;
    COIN.y = random_range(COIN_MAX_Y, COIN_MIN_Y);
    COIN.not_collected = true;

    // Barriers
    BARRIERS[0].x = SCREEN_SIZE as i32 + BARRIER_WIDTH;
    BARRIERS[0].y = BARRIER_DOWN_HEIGHT;
    BARRIERS[0].height = FLOOR_HEIGHT - BARRIER_DOWN_HEIGHT;
    BARRIERS[0].active = true;
    BARRIERS[0].displacement = BarrierDisplacement::Equal;
    BARRIERS[1].x = SCREEN_SIZE as i32 + BARRIER_WIDTH + BARRIER_GAP;
    BARRIERS[1].y = 0;
    BARRIERS[1].height = BARRIER_UP_HEIGHT;
    BARRIERS[1].active = true;
    BARRIERS[1].displacement = BarrierDisplacement::Equal;
    }
}

// Logical Functions -------------------------------------------------------------------------------------------------------
fn random_u32() -> u32 {
    unsafe {
        RNG_SEED = RNG_SEED.wrapping_mul(1664525).wrapping_add(1013904223);
        RNG_SEED
    }
}

fn random_range(min: i32, max: i32) -> i32 {
    let r = random_u32();
    min + (r % ((max - min + 1) as u32)) as i32
}

fn collision(a_x: i32, a_y: i32, a_w: i32, a_h: i32, b_x: i32, b_y: i32, b_w: i32, b_h: i32) -> bool {
    a_x < b_x + b_w &&
    a_x + a_w > b_x &&
    a_y < b_y + b_h &&
    a_y + a_h > b_y
}

// Game Update Functions ---------------------------------------------------------------------------------------------------
fn update_player_position() {
    unsafe {
        let gamepad = *wasm4::GAMEPAD1;
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
}

fn check_player_death() {
    unsafe {
        if PLAYER.lives == 0 {
                GAME_OVER = true;
        }
    }
}

fn update_player() {
    update_player_position();
    check_player_death();
}

fn update_coin() {
    unsafe {
        COIN.x -= COIN_VELOCITY;
        if COIN.x + COIN_WIDTH < 0 {
            COIN.x = SCREEN_SIZE as i32 + 20;
            COIN.not_collected = true;
            COIN.y = random_range(COIN_MAX_Y, COIN_MIN_Y);
        }
    }
}

fn update_barriers() {
    unsafe {
        BARRIERS[0].x -= SCENARIO_SPEED;
        BARRIERS[1].x -= SCENARIO_SPEED;

        if BARRIERS[0].x + BARRIER_WIDTH < 0 {
            BARRIERS[0].displacement = BarrierDisplacement::random();
            BARRIERS[0].x = SCREEN_SIZE as i32 + 20 + BARRIERS[0].displacement.adjust_displacement();
            BARRIERS[0].active = true;
            BARRIERS[0].y = BARRIER_DOWN_HEIGHT + BARRIERS[0].displacement.adjust_displacement();
            BARRIERS[0].height = FLOOR_HEIGHT - BARRIERS[0].y;
        }
        if BARRIERS[1].x + BARRIER_WIDTH < 0 {
            BARRIERS[1].displacement = BarrierDisplacement::random();
            BARRIERS[1].x = BARRIERS[0].x + BARRIER_GAP + BARRIERS[1].displacement.adjust_displacement();
            BARRIERS[1].active = true;
            BARRIERS[1].height = BARRIER_UP_HEIGHT + BARRIERS[1].displacement.adjust_displacement();
        }
    }
}

fn player_coin_interaction() {
    unsafe {
        if COIN.not_collected && collision(PLAYER.x, PLAYER.y, PLAYER_WIDTH, PLAYER_HEIGHT, COIN.x, COIN.y, COIN_WIDTH, COIN_HEIGTH){
            COIN.not_collected = false;
            PLAYER.score += 1;
        }
    }
}

fn player_barrier_interaction() {
    unsafe {
        if BARRIERS[0].active && collision(PLAYER.x, PLAYER.y, PLAYER_WIDTH, PLAYER_HEIGHT, BARRIERS[0].x, BARRIERS[0].y, BARRIER_WIDTH, BARRIERS[0].height) {
            PLAYER.lives = PLAYER.lives.saturating_sub(1);
            BARRIERS[0].active = false;
        }
        if BARRIERS[1].active && collision(PLAYER.x, PLAYER.y, PLAYER_WIDTH, PLAYER_HEIGHT, BARRIERS[1].x, BARRIERS[1].y, BARRIER_WIDTH, BARRIERS[1].height) {
            PLAYER.lives = PLAYER.lives.saturating_sub(1);
            BARRIERS[1].active = false;
        }
    }
}

// Draw Functions ----------------------------------------------------------------------------------------------------------
fn draw_sky() {
    unsafe { *DRAW_COLORS = 0x33 }
    oval(-30,-30, 80, 80);
}

fn draw_floor() {
    unsafe { *DRAW_COLORS = 0x4; }
    rect(0, FLOOR_HEIGHT, 160, FLOOR_HEIGHT as u32);
}

fn draw_barriers() {
    unsafe {
        *DRAW_COLORS = 0x33;
        if BARRIERS[0].active {
            rect(BARRIERS[0].x, BARRIERS[0].y, BARRIER_WIDTH as u32, BARRIERS[0].height as u32);
        }
        if BARRIERS[1].active {
            rect(BARRIERS[1].x, BARRIERS[1].y, BARRIER_WIDTH as u32, BARRIERS[1].height as u32);
        }
    }
}

fn draw_coin() {
    unsafe {
        if COIN.not_collected {
            *DRAW_COLORS = 0x21;
            oval(COIN.x, COIN.y, COIN_WIDTH as u32, COIN_HEIGTH as u32)
        }
    }
}

fn draw_player() {
    unsafe {
        *DRAW_COLORS = 0x4142;
        blit(
            // Player sprite: Byte Array in 2BPP
            &[0x80,0x0a,0x00,0x02,0x2f,0xfa,0x8f,0xfa,0x80,0xfe,0xbf,0xff,0x8f,0xfa,0x04,0x6a],
            PLAYER.x,
            PLAYER.y,
            PLAYER_WIDTH as u32,
            PLAYER_HEIGHT as u32,
            BLIT_2BPP,
        );
    }
}

fn draw_hud() {
    unsafe {
        *DRAW_COLORS = 4;
        let coins_qnty = PLAYER.score;
        let coins_text = format!("Coins: {}", coins_qnty);
        text(&coins_text, 10, 10);

        let lives_qnty = PLAYER.lives;
        let lives_text = format!("Lives: {}", lives_qnty);
        text(&lives_text, 10, 20);
    }
}

fn draw_scenario_screen() {
    draw_sky();
    draw_floor();
    draw_barriers();
    draw_coin();
    draw_player();
    draw_hud();
}

fn draw_gameover_screen() {
    unsafe {
        // Clean screen
        *DRAW_COLORS = 1; 
        rect(0, 0, SCREEN_SIZE, 160);
        // Draw Game Over text
        *DRAW_COLORS = 4;
        text("GAME OVER", 40, 60);
        let coins_qnty = PLAYER.score;
        let final_score = format!("Score: {}", coins_qnty);
        text(&final_score, 45, 70);
        text("Press X", 30, 90);
        text("to restart", 30, 100);
    }
}

fn draw_start_screen() {
    unsafe {
        // Clean screen
        *DRAW_COLORS = 1; 
        rect(0, 0, SCREEN_SIZE, 160);
        // Draw Game Start text
        *DRAW_COLORS = 4;
        text("TITULO DO JOGO", 40, 60);
        text("Press X", 30, 90);
        text("to start", 30, 100);
    }
}

// Update Function ---------------------------------------------------------------------------------------------------------
#[no_mangle]
fn update() {
    // Checks Game State
    unsafe {
        if GAME_START {
            // START STATE
            restart();
            draw_start_screen();
        } else if GAME_OVER {
            // GAME OVER STATE
            restart();
            draw_gameover_screen();
        } else {
            // SCENARIO STATE
            update_player();
            update_coin();
            update_barriers();
            player_coin_interaction();
            player_barrier_interaction();
            draw_scenario_screen();
        }
    }

    // Increment Frame Count
    unsafe {
        FRAME_COUNT += 1;
        /*
        TO IMPLEMENT
        // Change Scenario Speed
        if FRAME_COUNT % 1000 == 0 && SCENARIO_SPEED < 2 {
            SCENARIO_SPEED += 1;
        }
        */
    }
}