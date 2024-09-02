use macroquad::prelude::{
    RED,
    BLACK,
    WHITE,
    TextParams,
    is_key_pressed,
    is_quit_requested,
    clear_background,
    get_char_pressed,
    KeyCode,
    draw_text,
    draw_text_ex,
    BLUE,
};

const THE_STRONGEST_TEST: &str = "3.141592653589793238462643383279502884197";

pub const W: i32 = 800;
pub const H: i32 = 700;

enum GameState {
    InputPi,
    WrongPi,
    TryExit,
    Completed,
}

pub struct Game {
    pi_input: String,
    state: GameState,
    stuck_time: f32,
    pub quit_ok: bool
}

impl Default for Game {
    fn default() -> Self {
        Game {
            pi_input: String::new(),
            state: GameState::InputPi,
            stuck_time: 0.,
            quit_ok: false
        }
    }
}

pub fn update(game: &mut Game, delta: f32) {
    match game.state {
        GameState::InputPi => {
            game.stuck_time = 0.;

            if is_key_pressed(KeyCode::Enter) {
                if game.pi_input != THE_STRONGEST_TEST {
                    game.state = GameState::WrongPi;
                } else {game.state = GameState::Completed}
    
                game.pi_input.clear();
            } else if is_key_pressed(KeyCode::Backspace) {
                game.pi_input.pop();
            } else if is_key_pressed(KeyCode::Delete) {
                game.pi_input.clear();
            }

            if is_quit_requested() || is_key_pressed(KeyCode::Escape) {game.state = GameState::TryExit;}

            let characters = get_char_pressed();
            if characters.is_some_and(|x| (x.is_numeric() || x == '.')) {game.pi_input.push(characters.expect("\x1b[33mError: Something happened but idk why lol :) \x1b[0m"));}
        }

        GameState::TryExit | GameState::WrongPi => {
            game.stuck_time += delta;
            if game.stuck_time > 2. {game.state = GameState::InputPi;}
        }

        GameState::Completed => if is_quit_requested() || is_key_pressed(KeyCode::Escape) {game.quit_ok = true;}
    }
}

pub fn draw(game: &Game, param1: &TextParams<'_>, param2: &TextParams<'_>) {
    match game.state {
        GameState::InputPi => {
            clear_background(BLACK);

            let mut answer_spacing = (W as f32 / 2.) - 140. - game.pi_input.len() as f32 * 10.;
            if answer_spacing <= 0. {answer_spacing -= game.pi_input.len() as f32 * 5.;}


            draw_text_ex("INSERT PI HERE: ", W as f32 / 2. - 140., 100., param1.clone());
            draw_text_ex("INSERT PI HERE: ", W as f32 / 2. - 140., 120., param2.clone());

            draw_text_ex(&game.pi_input, answer_spacing, 210., param2.clone());
        }

        GameState::TryExit => {
            clear_background(RED);

            draw_text_ex("INSERT PI HERE: ", W as f32 / 2. - 140., 100., TextParams {
                color: BLACK,
                ..*param1
            });
            draw_text_ex("INSERT PI HERE: ", W as f32 / 2. - 140., 120., TextParams {
                color: BLACK,
                ..*param2
            });

            draw_text_ex("YOU WON'T BE GOING ANYWHERE! NOT WITHOUT COMPLETING YOUR NONSENSE!", 5., 170., TextParams {
                font_size: 30,
                color: BLACK,
                ..*param1
            });

            draw_text_ex("YOU WON'T BE GOING ANYWHERE! NOT WITHOUT COMPLETING YOUR NONSENSE!", 5., 190., TextParams {
                font_size: 20,
                color: BLACK,
                ..*param2
            });
        }

        GameState::WrongPi => {
            clear_background(RED);

            draw_text_ex("WRONG YOU IDIOT!", W as f32 / 2. - 130., 260., TextParams {
                color: BLACK,
                ..*param1
            });
            draw_text_ex("WRONG YOU FAILURE", W as f32 / 2. - 130., 280., TextParams {
                color: BLACK,
                ..*param2
            });
        }

        GameState::Completed => {
            clear_background(WHITE);
            draw_text("well done, you may rest now", W as f32 / 2. - 270., 300., 50., BLUE);
        }
    }
}