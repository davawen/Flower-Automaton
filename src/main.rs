use sfml::graphics::*;
use sfml::window::*;
use sfml::system::*;

use rand::{thread_rng, Rng};

#[derive(Default, Debug, Clone, Copy)]
struct Flower(u8, u8, u8);

impl Flower {
    fn mutate(&self) -> Flower {
        let mut rng = thread_rng();
        let mut r = self.0 as i16;
        let mut g = self.1 as i16;
        let mut b = self.2 as i16;

        r += rng.gen_range(-5..=5);
        g += rng.gen_range(-5..=5);
        b += rng.gen_range(-5..=5);

        Flower(r.clamp(0, 255) as u8, g.clamp(0, 255) as u8, b.clamp(0, 255) as u8)
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Dead,
    Flower(Flower),
}

struct Board {
    cells: Vec<Cell>,
    width: isize,
    height: isize,
    len: isize
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Board {
            cells: vec![Cell::Dead; (width*height) as usize],
            width: width as isize,
            height: height as isize,
            len: width as isize * height as isize
        }
    }

    fn from_board(board: &Board) -> Self {
        Board {
            cells: board.cells.clone(),
            width: board.width,
            height: board.height,
            len: board.len
        }
    }

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }

    fn at(&self, x: isize, y: isize) -> Cell {
        if !self.in_bounds(x, y) {
            Cell::Dead
        }
        else {
            *self.cells.get(usize::try_from(y*self.width + x).unwrap()).unwrap()
        }
    }

    fn color_at(&self, x: isize, y: isize) -> [u8; 4] {
        match self.at(x, y) {
            Cell::Dead => [ 0, 0, 0, 255 ],
            Cell::Flower(f) => [ f.0, f.1, f.2, 255 ]
        }
    }

    fn render_at(&self, text: &mut Texture, x: isize, y: isize) {
        if !self.in_bounds(x, y) {
            return;
        }

        unsafe {
            text.update_from_pixels(&self.color_at(x, y), 1, 1, x as u32, y as u32);
        }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y*self.width as usize + x]
    }

    fn set(&mut self, x: usize, y: usize, value: Cell) {
        *self.get_mut(x, y) = value;
    }
}

fn update(board: &mut Board, text: &mut Texture) {
    let x_range = 0..board.width;
    let y_range = 0..board.height;

    let mut update_cell = |cx: isize, cy: isize| {
        if let Cell::Flower(f) = board.at(cx, cy) {
            let mut write_to = |cx, cy| {
                if !board.in_bounds(cx, cy) {
                    return;
                }

                let neighbour = board.get_mut(cx as usize, cy as usize);
                if let Cell::Dead = *neighbour {
                    *neighbour = Cell::Flower(f.mutate());

                    board.render_at(text, cx, cy);
                }
            };

            write_to(cx-1, cy);
            write_to(cx+1, cy);
            write_to(cx, cy-1);
            write_to(cx, cy+1);
        }
    };

    // Update random cells
    for _ in 0..8000 {
        update_cell( thread_rng().gen_range(x_range.clone()), thread_rng().gen_range(y_range.clone()) );
    }
}

fn main() {

    let mut window = RenderWindow::new(
        (800, 800),
        "Flower automaton",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_framerate_limit(240);

    let _font = Font::from_file("/usr/share/fonts/open-sans/OpenSans-Regular.ttf").unwrap();

    let mut text = Texture::new(800, 800).unwrap();

    let mut board = Board::new(800, 800);

    loop {
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => return,
                Event::MouseButtonPressed { button, x, y } => {
                    match button {  
                        mouse::Button::LEFT => {
                            let col = ||{ thread_rng().gen_range(0..255) };

                            board.set(x as usize, y as usize, Cell::Flower(Flower( col(), col(), col() )));

                            board.render_at(&mut text, x as isize, y as isize);
                        },
                        _ => ()
                    }
                },
                _ => {}
            }
        }

        if mouse::Button::is_pressed(mouse::Button::RIGHT) {
            let pos = window.mouse_position();
            let x = pos.x;
            let y = pos.y;

            for ny in -20..=20 {
                for nx in -20..=20 {
                    let x = isize::try_from(x + nx).unwrap();
                    let y = isize::try_from(y + ny).unwrap();

                    if board.in_bounds(x, y) {
                        board.set(x as usize, y as usize, Cell::Dead);
                        board.render_at(&mut text, x, y);
                    }
                }
            }
        };

        update(&mut board, &mut text);

        window.clear(Color::BLACK);

        let sprite = Sprite::with_texture(&text);

        window.draw(&sprite);

        window.display();
    }
}
