
pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Display {
    screen: [[u8; WIDTH]; HEIGHT]
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [[0; WIDTH]; HEIGHT]
        }
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, heigth: u8) -> bool {
        let width = 8;
        let mut flipped = false;
        for i in 0..heigth {
            for j in 0..width {
                let pixel = self.screen[x + j as usize][y + i as usize];
                if pixel == 1 {
                    self.screen[x + j as usize][y + i as usize] = 0;
                    flipped = true;
                } else {
                    self.screen[x + j as usize][y + i as usize] = 1;
                }
            }
        }

        flipped
    }

    pub fn clear(&mut self) {
        self.screen = [[0; WIDTH]; HEIGHT];
    }

    pub fn display(&self) {
        println!("-----");
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if self.screen[i][j] == 1 {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
