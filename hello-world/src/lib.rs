#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

const SCREEN_WIDTH: i32 = 160; // Largura da tela

const PLAYER_WIDTH: i32 = 9; // Largura do sprite do jogador
const PLAYER_HEIGHT: i32 = 7;  // Altura do sprite do jogador

static mut GAME_OVER: bool = false;
const GRAVITY: f32 = 0.7; // Força da gravidade
const JUMP_FORCE: f32 = -11.0; // Força do pulo
const FLOOR_Y: i32 = 148; // Y do chão
static mut VELOCIDADE_ATUAL: i32 = 1;
static mut FRAME_COUNT: u32 = 0;

// Coletaveis
const VELOCIDADE: i32 = 1;
const POSICOES_Y: [i32; 1] = [140]; // Linhas fixas
const NUM_COLETAVEIS_POR_LINHA: usize = 5;
const ESPACAMENTO_X: i32 = 10; // Distância entre os coletáveis

// Obstaculos
static mut OBSTACULOS: Option<Vec<Obstaculo>> = None;

pub(crate) struct Player {
    x: i32,
    y: i32,
    velocity_y: f32,
    is_jumping: bool,
    score: u8,
    lives: u8,
}

struct Coletavel {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    ativo: bool,
}

struct Obstaculo {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Obstaculo {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y, width: 12, height: 60 }
    }

    fn update(&mut self) {
        unsafe {
            self.x -= VELOCIDADE_ATUAL;
            if self.x + self.width < 0 {
                self.x = SCREEN_WIDTH + 40;
            }
        }
    }

    fn draw(&self){
        unsafe {
            *DRAW_COLORS = 0x32;
        }
        rect(self.x, self.y, self.width as u32, self.height as u32);
    }
}
static mut PONTOS: u32 = 0;

impl Coletavel {
    fn new(x: i32, y: i32) -> Self {
        // cria uma nova "instância" da struct Coletavel
        Self { x, y, width: 4, height: 4, ativo: true }
    }

    fn update(&mut self) {
        unsafe {
            self.x -= VELOCIDADE_ATUAL;
            if self.x + self.width < 0 {
                self.x = SCREEN_WIDTH + 20;
                self.ativo = true; // Reativa o item para aparecer novamente
            }
        }
    }

    fn draw(&self) {
        if self.ativo {
            unsafe {*DRAW_COLORS = 0x21 }
            oval(self.x, self.y, self.width as u32, self.height as u32)
        }
    }
}

fn reiniciar_jogo() {
    unsafe {
        PLAYER.x = 45;
        PLAYER.y = 140;
        PLAYER.velocity_y = 0.0;
        PLAYER.is_jumping = false;
        PLAYER.score = 0;
        PLAYER.lives = 3;
        VELOCIDADE_ATUAL = 1;
        FRAME_COUNT = 0;
        GAME_OVER = false;
    }

    start(); // reinicializa itens e obstáculos
}

static mut PLAYER: Player = Player { 
    x: 45, 
    y: 140, 
    velocity_y: 0.0, 
    is_jumping: false, 
    score: 0, 
    lives: 3 
};

static mut ITENS: Option<Vec<Coletavel>> = None;

#[no_mangle]
pub fn start() {
    let mut itens = Vec::new();

    for (linha, &y) in POSICOES_Y.iter().enumerate() {
        for i in 0..NUM_COLETAVEIS_POR_LINHA {
            let x = 160 + i as i32 * ESPACAMENTO_X + (linha as i32 * 10);
            itens.push(Coletavel::new(x, y));
        }
    }

    unsafe {
        ITENS = Some(itens);
    }

    let mut obstaculos = Vec::new();

    obstaculos.push(Obstaculo::new(180, 110)); // no chão
    obstaculos.push(Obstaculo::new(280, 55));         // no teto

    unsafe {
        OBSTACULOS = Some(obstaculos);
    }
}

fn colisao(a_x: i32, a_y: i32, a_w: i32, a_h: i32, b_x: i32, b_y: i32, b_w: i32, b_h: i32) -> bool {
    a_x < b_x + b_w &&
    a_x + a_w > b_x &&
    a_y < b_y + b_h &&
    a_y + a_h > b_y
}

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

    let gamepad = unsafe { *wasm4 ::GAMEPAD1 };

    // Verifica se o jogador perdeu todas as vidas
    unsafe {
        if PLAYER.lives == 0 {
            GAME_OVER = true;
        }

        if GAME_OVER {
            // Define plano de fundo e texto no mesmo comando
            *DRAW_COLORS = 0x23; 
            rect(0, 0, SCREEN_WIDTH as u32, 160); // limpa a tela com cor de fundo

            text("GAME OVER", 40, 60);
            let final_score = format!("Score: {}", PLAYER.score);
            text(&final_score, 45, 70);
            text("Pressione X", 45, 90);
            text("para reiniciar", 35, 100);
            // Verifica se o botão X foi pressionado
            if gamepad & BUTTON_1 != 0 {
                reiniciar_jogo();
            }
            return;
        }
    }

    unsafe {
    if let Some(itens) = &mut ITENS {
            for item in itens.iter_mut() {
                if item.ativo && colisao(PLAYER.x, PLAYER.y, PLAYER_WIDTH, PLAYER_HEIGHT, item.x, item.y, item.width, item.height){
                item.ativo = false;
                PLAYER.score += 1;
                }
                item.update();
                item.draw();
            }
    }

    *DRAW_COLORS = 3;
    let texto_pontos = format!("Pontuacao: {}", PLAYER.score);
    text(&texto_pontos, 10, 10);

    let texto_lives = format!("Vidas: {}", PLAYER.lives);
    unsafe { *DRAW_COLORS = 3 }
    text(&texto_lives, 10, 20);
    }

    unsafe {
        if let Some(obstaculos) = &mut OBSTACULOS {
            for obs in obstaculos.iter_mut() {
                if colisao(PLAYER.x, PLAYER.y, PLAYER_WIDTH, PLAYER_HEIGHT, obs.x, obs.y, obs.width, obs.height) {
                    PLAYER.lives = PLAYER.lives.saturating_sub(1);
                    obs.x = SCREEN_WIDTH + 40 + (FRAME_COUNT % 100) as i32;
                }
                obs.update();
                obs.draw();
            }
        }
    }

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
    // Desenhar o chão
    unsafe {*DRAW_COLORS = 0x44;}
    rect(0, 149, 160, 149);


    // Fazer o desenho do jogador
    unsafe { *DRAW_COLORS = 32 } // Cor do sprite do jogador
    blit(
        // O sprite é um array de bytes representando o sprite em 2bpp
        &[0x95,0x55,0x95,0x55,0x54,0x55,0x45,0x41,0x05,0x55,0x55,0x65,0x01,0x6a,0x55,0x68],
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
    unsafe {
        FRAME_COUNT += 1;

        // A cada 1000 frames, aumenta a velocidade
        if FRAME_COUNT % 1000 == 0 && VELOCIDADE_ATUAL < 5 {
            VELOCIDADE_ATUAL += 1;
        }
    }   
}