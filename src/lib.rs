#![cfg_attr(not(test), no_std)]

use bare_metal_modulo::{ModNumC, MNum};
use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, };
use pc_keyboard::{DecodedKey, KeyCode};

#[derive(Copy,Debug,Clone,Eq,PartialEq)]
pub struct PlayerCharacter {
    player: [char; 2],
    player_letters: ModNumC<usize, 2>,
    pub col: ModNumC<usize, BUFFER_WIDTH>,
    pub row: ModNumC<usize, BUFFER_HEIGHT>,
    pub facing: char,
    pub striking: bool,
    striking_timer: usize
}

impl PlayerCharacter {
    pub fn new() -> Self {
        PlayerCharacter {
            player: ['O', '/'],
            player_letters: ModNumC::new(2),
            col: ModNumC::new(BUFFER_WIDTH / 2),
            row: ModNumC::new(BUFFER_HEIGHT / 2),
            facing: 'r',
            striking: false, // 0 = not striking, 1 = striking
            striking_timer: 0
        }
    }

    pub fn tick(&mut self) {
        self.clear_player();  // in case of striking
        if self.striking == true {
            self.striking_timer += 1;
            if self.striking_timer >= 2 {
                self.striking = false;
            }
        }
        self.draw_player();
    }

    fn clear_player(&self) {
        if self.facing == 'r' {  // facing right
            if !self.striking {
                let new_sword_position = (self.col.a() + 1) % BUFFER_WIDTH;
                plot(' ', self.col.a() % BUFFER_WIDTH, self.row.a() % BUFFER_HEIGHT, ColorCode::new(Color::Magenta, Color::Black));
                plot(' ', new_sword_position, self.row.a() % BUFFER_HEIGHT, ColorCode::new(Color::Magenta, Color::Black));
            } else {
                let new_sword_1 = (self.col.a() + 1) % BUFFER_WIDTH;
                let new_sword_2 = (self.col.a() + 2) % BUFFER_WIDTH;
                plot(' ', self.col.a(), self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                plot(' ', new_sword_1, self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                plot(' ', new_sword_2, self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
            }
        } else {  // facing left
            if !self.striking {
                let new_sword_position = (self.col.a() - 1) % BUFFER_WIDTH;
                plot(' ', self.col.a(), self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                plot(' ', new_sword_position, self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
            } else {
                let new_sword_1 = (self.col.a() - 1) % BUFFER_WIDTH;
                let new_sword_2 = (self.col.a() - 2) % BUFFER_WIDTH;
                plot(' ', self.col.a(), self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                plot(' ', new_sword_1, self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                plot(' ', new_sword_2, self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
            }
        }
    }

    pub fn draw_player(&mut self) {
        if self.facing == 'r' {
            if !self.striking {  // right and stagnant
                if self.col.a() + 1 >= BUFFER_WIDTH {
                    self.col = ModNumC::new(0);
                    plot('O', self.col.a(), self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                    plot('/', self.col.a() + 1, self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                } else {
                    let new_sword_position = (self.col.a() + 1) % BUFFER_WIDTH;
                    plot('O', self.col.a(), self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                    plot('/', new_sword_position, self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                }
            } else {  // right and striking
                let new_sword_1 = (self.col.a() + 1) % BUFFER_WIDTH;
                let new_sword_2 = (self.col.a() + 2) % BUFFER_WIDTH;
                plot('O', self.col.a(), self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                plot('-', new_sword_1, self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                plot('-', new_sword_2, self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
            }
        } else {
            if !self.striking {  // left and stagnant
                if (self.col.a() - 1) == 0 {
                    self.col = ModNumC::new(BUFFER_WIDTH - 1);
                    plot('\\', self.col.a() - 1, self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                    plot('O', self.col.a(), self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                } else {
                    let new_sword_position = (self.col.a() - 1) % BUFFER_WIDTH;
                    plot('\\', new_sword_position, self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                    plot('O', self.col.a(), self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                }
            } else {  // left and striking
                if self.col.a() - 1 == 0 {
                    self.col = ModNumC::new(BUFFER_WIDTH - 1);
                    plot('-', self.col.a() - 1, self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                    plot('-', self.col.a() - 2, self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                    plot('O', self.col.a(), self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                } else {
                    let new_sword_1 = (self.col.a() - 1) % BUFFER_WIDTH;
                    let new_sword_2 = (self.col.a() - 2) % BUFFER_WIDTH;
                    plot('-', new_sword_1, self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                    plot('-', new_sword_2, self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                    plot('O', self.col.a(), self.row.a(), ColorCode::new(Color::Magenta, Color::Black));
                }
            }
            
        }
    }

    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(code) => self.handle_raw(code),
            DecodedKey::Unicode(c) => self.handle_unicode(c)
        }
    }

    fn handle_raw(&mut self, key: KeyCode) {
        match key {
            KeyCode::ArrowLeft => {
                self.clear_player();
                self.facing = 'l';
                self.col -= 1;
            }
            KeyCode::ArrowRight => {
                self.clear_player();
                self.facing = 'r';
                self.col += 1;
            }
            KeyCode::ArrowUp => {
                self.clear_player();
                self.row -= 1;
            }
            KeyCode::ArrowDown => {
                self.clear_player();
                self.row += 1;
            }
            _ => {}
        }
    }

    fn handle_unicode(&mut self, key: char) {
        if key == 'z' {
            self.striking = true;
            self.striking_timer = 0;
        }
    }
}