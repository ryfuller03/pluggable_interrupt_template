#![cfg_attr(not(test), no_std)]

use bare_metal_modulo::{ModNumC, MNum};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, plot_num};

#[derive(Copy,Debug,Clone,Eq,PartialEq)]
pub struct Timer {
    pub current_time: isize,
    col: ModNumC<usize, BUFFER_WIDTH>,
    row: ModNumC<usize, BUFFER_HEIGHT>,
    pub game_over: bool
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            current_time: 0,
            col: ModNumC::new(0),
            row: ModNumC::new(0),
            game_over: false
        }
    }

    fn clear(&self) {
        plot(' ', self.col.a(), self.row.a(), ColorCode::new(Color::White, Color::Black));
    }

    fn update(&mut self) {
        if !self.game_over {
            self.current_time += 1;
        }
    }

    fn display(&self) {
        plot_num(self.current_time, self.col.a(), self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
    }

    pub fn tick(&mut self, current_tick: usize) {
        self.clear();
        if current_tick % 20 == 0 {
            self.update();
        }
        self.display();
    }
}