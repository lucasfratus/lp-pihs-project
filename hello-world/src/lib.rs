#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

const SCREEN_WIDTH: i32 = 160; // Largura da tela

const PLAYER_WIDTH: i32 = 8; // Largura do Sprite do jogador
const PLAYER_HEIGHT: i32 = 8;  // Altura do Sprite do jogador

static mut GAME_OVER: bool = false;
const GRAVITY: f32 = 0.7; // Força da gravidade
const JUMP_FORCE: f32 = -11.0; // Força do pulo
const FLOOR_Y: i32 = 148; // Altura do chão
static mut VELOCIDADE_ATUAL: f32 = 0.5; // Velocidade do cenário
static mut FRAME_COUNT: u32 = 0; // Contador de Frames

// Coletáveis
const VELOCIDADE: f32 = 1.0; // Velocidade dos Coletáveis
const POSICOES_Y: [f32; 1] = [140.0]; // Linhas fixas
const NUM_COLETAVEIS_POR_LINHA: usize = 5;
const ESPACAMENTO_X: f32 = 10.0; // Distância entre os coletáveis

// Obstaculos
static mut OBSTACULOS: Option<Vec<Obstaculo>> = None;

pub(crate) struct Player {
    x: f32,
    y: f32,
    velocity_y: f32,
    is_jumping: bool,
    score: u8,
    lives: u8,
}

struct Coletavel {
    x: f32,
    y: f32,
    width: i32,
    height: i32,
    ativo: bool,
}

struct Obstaculo {
    x: f32,
    y: f32,
    width: i32,
    height: i32,
}

impl Obstaculo {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y, width: 12, height: 60 }
    }

    fn update(&mut self) {
        unsafe {
            self.x -= VELOCIDADE_ATUAL;
            if self.x + (self.width as f32) < 0.0 {
                self.x = (SCREEN_WIDTH as f32) + 40.0;
            }
        }
    }

    fn draw(&self){
        unsafe {
            *DRAW_COLORS = 0x33;
        }
        rect(self.x as i32, self.y as i32, self.width as u32, self.height as u32);
    }
}
static mut PONTOS: u32 = 0;

impl Coletavel {
    fn new(x: f32, y: f32) -> Self {
        // cria uma nova "instância" da struct Coletavel
        Self { x, y, width: 4, height: 4, ativo: true }
    }

    fn update(&mut self) {
        unsafe {
            self.x -= VELOCIDADE_ATUAL;
            if self.x + (self.width as f32) < 0.0 {
                self.x = SCREEN_WIDTH as f32 + 20.0;
                self.ativo = true; // Reativa o item para aparecer novamente
            }
        }
    }

    fn draw(&self) {
        if self.ativo {
            unsafe {*DRAW_COLORS = 0x21 }
            oval(self.x as i32, self.y as i32, self.width as u32, self.height as u32)
        }
    }
}

fn reiniciar_jogo() {
    unsafe {
        PLAYER.x = 45.0;
        PLAYER.y = 140.0;
        PLAYER.velocity_y = 0.0;
        PLAYER.is_jumping = false;
        PLAYER.score = 0;
        PLAYER.lives = 3;
        VELOCIDADE_ATUAL = 0.5;
        FRAME_COUNT = 0;
        GAME_OVER = false;
    }

    start(); // reinicializa itens e obstáculos
}

static mut PLAYER: Player = Player { 
    x: 45.0, 
    y: 140.0, 
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
            let x = 160 + i as i32 * ESPACAMENTO_X as i32 + (linha as i32 * 10);
            itens.push(Coletavel::new(x as f32, y));
        }
    }

    unsafe {
        ITENS = Some(itens);
    }

    let mut obstaculos = Vec::new();

    obstaculos.push(Obstaculo::new(180.0, 110.0)); // no chão
    obstaculos.push(Obstaculo::new(280.0, 55.0));  // no teto

    unsafe {
        OBSTACULOS = Some(obstaculos);
    }
}

fn colisao(a_x: f32, a_y: f32, a_w: i32, a_h: i32, b_x: f32, b_y: f32, b_w: i32, b_h: i32) -> bool {
    a_x < b_x + b_w as f32 &&
    a_x + a_w as f32 > b_x &&
    a_y < b_y + b_h as f32 &&
    a_y + a_h as f32 > b_y
}

#[no_mangle]
fn update() {
    // Paleta de cores
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

    // Verifica se o jogador perdeu todas as vidas
    unsafe {
        if PLAYER.lives == 0 {
            GAME_OVER = true;
        }

        if GAME_OVER {
            // Define plano de fundo e texto no mesmo comando
            *DRAW_COLORS = 1; 
            rect(0, 0, SCREEN_WIDTH as u32, 160); // limpa a tela com cor de fundo

            *DRAW_COLORS = 4;
            text("GAME OVER", 40, 60);
            let final_score = format!("Score: {}", PLAYER.score);
            text(&final_score, 45, 70);
            text("Pressione X", 30, 90);
            text("para reiniciar", 30, 100);
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

    *DRAW_COLORS = 4;
    let texto_pontos = format!("Pontuacao: {}", PLAYER.score);
    text(&texto_pontos, 10, 10);

    let texto_lives = format!("Vidas: {}", PLAYER.lives);
    unsafe { *DRAW_COLORS = 4 }
    text(&texto_lives, 10, 20);
    }

    unsafe {
        if let Some(obstaculos) = &mut OBSTACULOS {
            for obs in obstaculos.iter_mut() {
                if colisao(PLAYER.x, PLAYER.y, PLAYER_WIDTH, PLAYER_HEIGHT, obs.x, obs.y, obs.width, obs.height) {
                    PLAYER.lives = PLAYER.lives.saturating_sub(1);
                    obs.x = SCREEN_WIDTH as f32 + 40.0 + (FRAME_COUNT % 100) as f32;
                }
                obs.update();
                obs.draw();
            }
        }
    }

    unsafe {
    // Movimento lateral
    if gamepad & BUTTON_LEFT != 0 {
        PLAYER.x -= 2.0
    }
    if gamepad & BUTTON_RIGHT != 0 {
        PLAYER.x += 2.0;
    }

    // Pular
    if gamepad & BUTTON_1 != 0 && !PLAYER.is_jumping {
        PLAYER.velocity_y = JUMP_FORCE;
        PLAYER.is_jumping = true;
    }

    // Aplicar gravidade
    PLAYER.velocity_y += GRAVITY;
    PLAYER.y += PLAYER.velocity_y;


    // Verificar se atingiu o chão
    if PLAYER.y >= FLOOR_Y as f32 {
        PLAYER.y = FLOOR_Y as f32;
        PLAYER.velocity_y = 0.0;
        PLAYER.is_jumping = false;
    }

    // Limitar o jogador dentro da tela
    if PLAYER.x < 0.0 {
        PLAYER.x = 0.0;
    }
    if PLAYER.y < 0.0 {
        PLAYER.y = 0.0;
    }
    if PLAYER.x > (SCREEN_WIDTH - PLAYER_WIDTH) as f32 {
        PLAYER.x = (SCREEN_WIDTH - PLAYER_WIDTH) as f32;
    }
    if PLAYER.y > (FLOOR_Y - PLAYER_HEIGHT) as f32 {
        PLAYER.y = (FLOOR_Y - PLAYER_HEIGHT) as f32;
    }
    }
    // Desenhar o chão
    unsafe {*DRAW_COLORS = 0x4;}
    rect(0, 149, 160, 149);


    // Fazer o desenho do jogador
    unsafe { *DRAW_COLORS = 0x4142} // Cor do sprite do jogador
    blit(
        // O sprite é um array de bytes representando o sprite em 2bpp
        &[0x80,0x0a,0x00,0x02,0x2f,0xfa,0x8f,0xfa,0x80,0xfe,0xbf,0xff,0x8f,0xfa,0x04,0x6a],
        unsafe { 
            PLAYER.x as i32
        },
        unsafe {
            PLAYER.y as i32
        },
        8,                  // largura
        8,                  // altura
        BLIT_2BPP,
    );
    unsafe {
        FRAME_COUNT += 1;

        // A cada 1000 frames, aumenta a velocidade
        if FRAME_COUNT % 500 == 0 && VELOCIDADE_ATUAL < 3.0 {
            VELOCIDADE_ATUAL += 0.2;
        }
    }   
}