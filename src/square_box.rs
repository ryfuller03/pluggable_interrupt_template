#![cfg_attr(not(test), no_std)]
use bare_metal_modulo::{ModNumC, MNum};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, };
use crate::random_ng;

#[derive(Copy,Debug,Clone,Eq,PartialEq)]
pub struct SquareBox {
    model: char,
    pub col: ModNumC<usize, BUFFER_WIDTH>,
    pub row: ModNumC<usize, BUFFER_HEIGHT>,
    rand: random_ng::Random,
    pub dead: bool
}

impl SquareBox {
    pub fn new(seed: usize, col: usize, row: usize) -> Self {
        SquareBox {
            model: 219 as char,
            col: ModNumC::new(col),
            row: ModNumC::new(row),
            rand: random_ng::Random::new(seed),
            dead: false
        }
    }

    pub fn tick(&mut self, current_tick: usize) {
        self.clear_box();
        if self.dead {
            self.model = 'X';
            self.draw_box();
        } else {
            if current_tick % 20 == 0 {
                self.move_randomly();
                
            }
            self.draw_box();
        }
    }

    fn clear_box(&self) {
        plot(' ', self.col.a(), self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
    }

    pub fn draw_box(&self) {
        plot(self.model, self.col.a(), self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
    }

    fn move_randomly(&mut self) {
        if self.rand.next() % 2 == 0 {
            if self.rand.next() % 2 == 0 {
                self.col += 1;
            } else {
                self.col -= 1;
            }
        } else {
            if self.rand.next() % 2 == 0 {
                self.row += 1;
            } else {
                self.row -= 1;
            }
        }
    }
}