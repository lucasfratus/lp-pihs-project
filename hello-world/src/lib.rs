#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

const SCREEN_WIDTH: i32 = 160; // Largura da tela

const PLAYER_WIDTH: i32 = 9; // Largura do sprite do jogador
const PLAYER_HEIGHT: i32 = 7;  // Altura do sprite do jogador

const GRAVITY: f32 = 0.7; // Força da gravidade
const JUMP_FORCE: f32 = -10.0; // Força do pulo
const FLOOR_Y: i32 = 148; // Y do chão

pub(crate) struct Player {
    x: i32,
    y: i32,
    velocity_y: f32,
    is_jumping: bool,
    score: u8,
    lives: u8,
}

static mut PLAYER: Player = Player { x: 45, y: 140, velocity_y: 0.0, is_jumping: false, score: 0, lives: 3 };

//static mut PREVIOUS_GAMEPAD: u8 = 0;

#[no_mangle]
fn update() {
    // Paleta de cores
    unsafe {
    *PALETTE = [
        0x0b0630,
        0xf8e3c4,
        0xcc3495,
        0x6b1fb1,
    ];
    }

    let score = unsafe { PLAYER.score };
    let lives = unsafe { PLAYER.lives };

    let texto_score = format!("Pontuacao: {}", score);
    let texto_lives = format!("Vidas: {}", lives);

    unsafe { *DRAW_COLORS = 2 }
    text(&texto_score, 10, 10);
    text(&texto_lives, 10, 20);

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

    // Limitar o jogador dentro da tela
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

    // Fazer o desenho do jogador
    unsafe { *DRAW_COLORS = 3 } // Cor do sprite do jogador
    // Desenhar o sprite do jogador
    // O sprite é um array de bytes representando o sprite em 2bpp
    blit(
        &[0x95,0x55,0x95,0x55,0x54,0x55,0x45,0x41,0x05,0x55,0x55,0x65,0x01,0x6a,0x55,0x68],// seu sprite
        unsafe { 
            PLAYER.x as i32
        },
        unsafe {
            PLAYER.y as i32
        },
        9,                  // largura
        7,                  // altura
        BLIT_2BPP,
    );
}