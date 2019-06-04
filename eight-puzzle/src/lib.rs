use eight_puzzle_core::hint::Hinter;
use eight_puzzle_core::state::*;
use find_folder::Search;
use math::Scalar;
use piston_window::*;
use std::time::Instant;

type Board = HashState;

#[derive(Debug, Clone, Copy)]
struct InGame {
    move_count: usize,
    hint_count: usize,
    initial: Board,
    board: Board,
    show_hint: bool,
}

impl InGame {
    fn move_board<F: Fn(&Board) -> Option<Board>>(&self, move_fn: F) -> InGame {
        if let Some(new_board) = move_fn(&self.board) {
            InGame {
                move_count: self.move_count + 1,
                hint_count: self.hint_count,
                board: new_board,
                show_hint: false,
                ..*self
            }
        } else {
            *self
        }
    }
}

#[derive(Debug)]
enum Status {
    Menu,
    InGame(InGame),
    Finish(Finish),
}

#[derive(Debug, Clone, Copy)]
struct Finish {
    move_count: usize,
    hint_count: usize,
    board: Board,
}
impl Status {
    fn start(hinter: &Hinter<Board>) -> Status {
        let initial = *hinter.random_state().unwrap();
        Status::InGame(InGame {
            move_count: 0,
            hint_count: 0,
            initial,
            board: initial,
            show_hint: false,
        })
    }
}

struct Resource {
    goal: Board,
    hinter: Hinter<Board>,
    window: PistonWindow,
    glyphs: Glyphs,
}

impl Resource {
    fn write_white_text(
        glyphs: &mut Glyphs,
        c: Context,
        g: &mut G2d,
        x: math::Scalar,
        y: math::Scalar,
        size: types::FontSize,
        text: &str,
    ) {
        text::Text::new_color([1.0, 1.0, 1.0, 1.0], size)
            .draw(text, glyphs, &c.draw_state, c.transform.trans(x, y), g)
            .unwrap_or_else(|e| panic!("Failed to write text: {}", e))
    }
}

pub struct Game {
    resource: Resource,
    status: Status,
}

impl Game {
    pub fn new() -> Game {
        let mut window: PistonWindow = WindowSettings::new("Eight Puzzle", (640, 580))
            .samples(4)
            .opengl(OpenGL::V2_1)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

        let mut exe_folder = std::env::current_exe()
            .unwrap_or_else(|e| panic!("Failed to locate current file path: {}", e));
        exe_folder.pop();
        let assets = Search::KidsThenParents(1, 2)
            .of(exe_folder)
            .for_folder("Resources")
            .unwrap_or_else(|e| panic!("Resources not found: {}", e));

        let glyphs = window
            .load_font(assets.join("FiraSans-Regular.ttf"))
            .unwrap_or_else(|e| panic!("Failed to create glyphs: {}", e));

        let goal = Board::from_array(&[1, 2, 3, 4, 0, 5, 6, 7, 8]);
        let hinter: Hinter<Board> = Hinter::new(goal);

        window.set_lazy(true);
        Game {
            resource: Resource {
                goal,
                hinter,
                window,
                glyphs,
            },
            status: Status::Menu,
        }
    }

    pub fn next(&mut self) -> Option<()> {
        let Game { resource, status } = self;
        println!("{:?} status: {:?}.", Instant::now(), status);
        resource
            .window
            .next()
            .and_then(|e| {
                println!("{:?} event: {:?}.", Instant::now(), e);
                match status {
                    Status::Menu => Game::on_menu(resource, &e),
                    Status::InGame(in_game) => Game::in_game(resource, in_game, &e),
                    Status::Finish(finish) => Game::on_finish(resource, &e, finish),
                }
            })
            .map(|new_status| *status = new_status)
    }

    fn on_finish(resource: &mut Resource, e: &Event, finish: &Finish) -> Option<Status> {
        let Resource {
            window,
            glyphs,
            hinter,
            ..
        } = resource;
        let Finish {
            move_count,
            hint_count,
            board,
        } = finish;

        window.draw_2d(e, |c, g, device| {
            clear([0.0, 0.0, 0.0, 0.0], g);
            Resource::write_white_text(
                glyphs,
                c,
                g,
                0.0,
                25.0,
                25,
                &format!("Move: {}", move_count),
            );
            Resource::write_white_text(
                glyphs,
                c,
                g,
                0.0,
                50.0,
                25,
                &format!("Hint: {}", hint_count),
            );
            Game::draw_board(glyphs, c, g, board, 0.0, 350.0, 300.0, 100);
            Resource::write_white_text(glyphs, c, g, 0.0, 500.0, 150, "Success");
            Resource::write_white_text(glyphs, c, g, 300.0, 25.0, 25, "Restart: R");
            Resource::write_white_text(glyphs, c, g, 300.0, 50.0, 25, "Quit: ESC");
            glyphs.factory.encoder.flush(device);
        });
        match e.press_args() {
            Some(Button::Keyboard(Key::R)) => Some(Status::start(hinter)),
            Some(Button::Keyboard(Key::Escape)) => Some(Status::Menu),
            _ => Some(Status::Finish(*finish)),
        }
    }

    fn on_menu(resource: &mut Resource, e: &Event) -> Option<Status> {
        let Resource {
            window,
            glyphs,
            hinter,
            ..
        } = resource;

        window.draw_2d(e, |c, g, device| {
            clear([0.0, 0.0, 0.0, 0.0], g);
            Resource::write_white_text(glyphs, c, g, 60.0, 100.0, 100, "Eight Puzzle");
            Resource::write_white_text(glyphs, c, g, 140.0, 200.0, 50, "Press S to start");
            glyphs.factory.encoder.flush(device);
        });

        match e.press_args() {
            Some(Button::Keyboard(Key::S)) => Some(Status::start(hinter)),
            Some(Button::Keyboard(Key::Escape)) => None,
            _ => Some(Status::Menu),
        }
    }

    fn draw_board(
        glyphs: &mut Glyphs,
        c: Context,
        g: &mut G2d,
        board: &Board,
        x: math::Scalar,
        mut y: math::Scalar,
        draw_size: math::Scalar,
        font_size: types::FontSize,
    ) {
        let arr = board.to_array();
        let square_size = draw_size / 3.0;
        y -= square_size * 2.0;
        for i in 0..9 {
            if arr[i] != 0 {
                Resource::write_white_text(
                    glyphs,
                    c,
                    g,
                    x + ((i % 3) as Scalar * square_size),
                    y + ((i / 3) as Scalar * square_size),
                    font_size,
                    &arr[i].to_string(),
                );
            }
        }
    }

    fn in_game(resource: &mut Resource, in_game: &InGame, e: &Event) -> Option<Status> {
        let Resource {
            goal,
            window,
            glyphs,
            hinter,
            ..
        } = resource;

        if in_game.board == *goal {
            return Some(Status::Finish(Finish {
                move_count: in_game.move_count,
                hint_count: in_game.hint_count,
                board: in_game.board,
            }));
        }

        window.draw_2d(e, |c, g, device| {
            clear([0.0, 0.0, 0.0, 0.0], g);
            Resource::write_white_text(
                glyphs,
                c,
                g,
                0.0,
                25.0,
                25,
                &format!("Move: {}", in_game.move_count),
            );
            Resource::write_white_text(
                glyphs,
                c,
                g,
                0.0,
                50.0,
                25,
                &format!("Hint: {}", in_game.hint_count),
            );
            Game::draw_board(glyphs, c, g, &in_game.board, 0.0, 350.0, 300.0, 100);
            if in_game.show_hint {
                Resource::write_white_text(
                    glyphs,
                    c,
                    g,
                    0.0,
                    375.0,
                    25,
                    &format!("Hint: {}", hinter.hint(&in_game.board).unwrap()),
                )
            }
            Resource::write_white_text(glyphs, c, g, 300.0, 25.0, 25, "Move: Direction keys.");
            Resource::write_white_text(glyphs, c, g, 300.0, 50.0, 25, "Goal: ");
            Game::draw_board(glyphs, c, g, &goal, 425.0, 225.0, 200.0, 50);
            Resource::write_white_text(glyphs, c, g, 300.0, 250.0, 25, "Hint: H");
            Resource::write_white_text(glyphs, c, g, 300.0, 275.0, 25, "Reset: R");
            Resource::write_white_text(glyphs, c, g, 300.0, 300.0, 25, "Quit: ESC");
            glyphs.factory.encoder.flush(device);
        });

        match e.press_args() {
            Some(Button::Keyboard(Key::Up)) => Some(Status::InGame(in_game.move_board(|b| b.up()))),
            Some(Button::Keyboard(Key::Down)) => {
                Some(Status::InGame(in_game.move_board(|b| b.down())))
            }
            Some(Button::Keyboard(Key::Left)) => {
                Some(Status::InGame(in_game.move_board(|b| b.left())))
            }
            Some(Button::Keyboard(Key::Right)) => {
                Some(Status::InGame(in_game.move_board(|b| b.right())))
            }
            Some(Button::Keyboard(Key::Escape)) => Some(Status::Menu),
            Some(Button::Keyboard(Key::R)) => Some(Status::InGame(InGame {
                move_count: 0,
                hint_count: 0,
                initial: in_game.initial,
                board: in_game.initial,
                show_hint: false,
            })),
            Some(Button::Keyboard(Key::H)) => Some(Status::InGame(InGame {
                hint_count: in_game.hint_count + 1,
                show_hint: true,
                ..*in_game
            })),
            _ => Some(Status::InGame(*in_game)),
        }
    }
}
