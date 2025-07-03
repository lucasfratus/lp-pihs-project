#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

const SCREEN_WIDTH: i32 = 160;
const SCREEN_HEIGHT: i32 = 160;

const PLAYER_WIDTH: i32 = 8;
const PLAYER_HEIGHT: i32 = 8;

const GRAVITY: f32 = 0.7;
const JUMP_FORCE: f32 = -12.0;
const FLOOR_Y: i32 = 148; 

#[rustfmt::skip]
const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];

pub(crate) struct Player {
    x: i32,
    y: i32,
    velocity_y: f32,
    is_jumping: bool,
}

static mut PLAYER: Player = Player { x: 45, y: 140, velocity_y: 0.0, is_jumping: false};

//static mut PREVIOUS_GAMEPAD: u8 = 0;

#[no_mangle]
fn update() {
    unsafe { *DRAW_COLORS = 2 }
    text("Jump!", 10, 10);

    /* 
    let (pressed_this_frame, ..) = unsafe {
        let previous = PREVIOUS_GAMEPAD;
        let gamepad = *GAMEPAD1;
        // Only the buttons that were pressed down this frame
        let pressed_this_frame = gamepad & (gamepad ^ previous);
        PREVIOUS_GAMEPAD = gamepad;

        (pressed_this_frame, gamepad, previous)
    };

    if pressed_this_frame & BUTTON_RIGHT != 0 {
        trace("Right button was just pressed!");
    }
    */

    let gamepad = unsafe { *wasm4 ::GAMEPAD1 };
    unsafe {
    
    // Movimento lateral
    if gamepad & BUTTON_LEFT != 0 {
        PLAYER.x -= 2 
    }
    if gamepad & BUTTON_RIGHT != 0 {
        PLAYER.x += 2;
    }

    // Pular
    if gamepad & BUTTON_1 != 0 && !PLAYER.is_jumping {
        PLAYER.velocity_y = JUMP_FORCE;
        PLAYER.is_jumping = true;
    }

    // Aplicar gravidade
    PLAYER.velocity_y += GRAVITY;
    PLAYER.y += PLAYER.velocity_y as i32;


    // Verificar se atingiu o chão
    if PLAYER.y >= FLOOR_Y {
        PLAYER.y = FLOOR_Y;
        PLAYER.velocity_y = 0.0;
        PLAYER.is_jumping = false;
    }

    if PLAYER.x < 0 {
        PLAYER.x = 0;
    }
    if PLAYER.y < 0 {
        PLAYER.y = 0;
    }
    if PLAYER.x > SCREEN_WIDTH - PLAYER_WIDTH {
        PLAYER.x = SCREEN_WIDTH - PLAYER_WIDTH;
    }
    if PLAYER.y > FLOOR_Y - PLAYER_HEIGHT {
        PLAYER.y = FLOOR_Y - PLAYER_HEIGHT;
    }
    }

    blit(
        &SMILEY,        // seu sprite
        unsafe { PLAYER.x },
        unsafe {
            PLAYER.y as i32
        },
        8,                  // largura
        8,                  // altura
        BLIT_1BPP,
    );
    
}
