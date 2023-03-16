#![no_std]
#![no_main]

use pc_keyboard::DecodedKey;
use pluggable_interrupt_os::HandlerTable;
use pluggable_interrupt_os::vga_buffer::{clear_screen, BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, plot_num};
use pluggable_interrupt_template::PlayerCharacter;
use crossbeam::atomic::AtomicCell;
mod square_box;
mod timer;
use timer::Timer;
use square_box::SquareBox;
mod random_ng;

/* FOR ENEMIES AND HIT DETECTION:
1. Create an array of enemies in main
2. Give access to PlayerCharacter to the array
3. Change the array if necessary
*/


#[no_mangle]
pub extern "C" fn _start() -> ! {
    HandlerTable::new()
        .keyboard(key)
        .timer(tick)
        .startup(startup)
        .cpu_loop(cpu_loop)
        .start()
}

static LAST_KEY: AtomicCell<Option<DecodedKey>> = AtomicCell::new(None);
static TICKS: AtomicCell<usize> = AtomicCell::new(0);

fn cpu_loop() -> ! {
    let mut player = PlayerCharacter::new();
    let mut timer = Timer::new();
    let mut enemy_array = [SquareBox::new(0, 0, 0); 10];
    let mut random_num = random_ng::Random::new(20);
    for i in 0..10 {
        enemy_array[i] = SquareBox::new(i + 1, random_num.next() % BUFFER_WIDTH, random_num.next() % BUFFER_HEIGHT);
    }
    let mut last_tick = 0;
    loop {
        if let Some(key) = LAST_KEY.load() {
            if key == DecodedKey::Unicode('r') {
                clear_screen();
                cpu_loop();
            }
            LAST_KEY.store(None);
            player.key(key);
        }
        let current_tick = TICKS.load();
        let mut dead_counter = 0;
        for i in 0..10 {
            if enemy_array[i].dead {
                dead_counter += 1;
            }
        }
        if current_tick > last_tick {
            if dead_counter >= 10 {
                timer.game_over = true;
                timer.tick(current_tick);
                win_screen(timer.current_time);
            } else {
                last_tick = current_tick;
                timer.tick(current_tick);
                player.tick();
                for i in 0..10 {
                    collision_check(&player, &mut enemy_array[i]);
                    enemy_array[i].tick(current_tick);
                }
            }
        }
    }
}

fn tick() {
    TICKS.fetch_add(1);
}

fn key(key: DecodedKey) {
    LAST_KEY.store(Some(key));
}

fn startup() {
    clear_screen();
    let mut player = PlayerCharacter::new();
    player.draw_player();
}

fn collision_check(player: &PlayerCharacter, enemy: &mut SquareBox) {
    if (player.facing == 'r') && (player.row == enemy.row) && ((player.col + 2 == enemy.col) || (player.col + 1 == enemy.col)) && (player.striking) {
        enemy.dead = true;
    }

    if (player.facing == 'l') && (player.row == enemy.row) && ((player.col - 2 == enemy.col) || (player.col - 1 == enemy.col)) && (player.striking) {
        enemy.dead = true;
    }
}

fn win_screen(current_time: isize) {
    let you_win_your_score = ['Y', 'O', 'U', ' ', 'W', 'I', 'N'];
    let score_string = ['S', 'C', 'O', 'R', 'E', ':', ' '];
    let mut separator_string = 3;
    let mut separator_string_2 = 0;
    let restart_string = ['P', 'R', 'E', 'S', 'S', ' ', 'r', ' ', 'T', 'O', ' ', 'R', 'E', 'S', 'T', 'A', 'R', 'T'];
    for letter in 0..7 {
        if letter < 3 {
            plot(you_win_your_score[letter], (BUFFER_WIDTH / 2) - separator_string, (BUFFER_HEIGHT / 2) - 1, ColorCode::new(Color::White, Color::Black));
            separator_string -= 1;
        } else {
            plot(you_win_your_score[letter], (BUFFER_WIDTH / 2) + separator_string_2, (BUFFER_HEIGHT / 2) - 1, ColorCode::new(Color::White, Color::Black));
            separator_string_2 += 1;
        }
    }

    separator_string = 3;
    separator_string_2 = 0;
    for letter in 0..7 {
        if letter < 3 {
            plot(score_string[letter], (BUFFER_WIDTH / 2) - separator_string, BUFFER_HEIGHT / 2, ColorCode::new(Color::White, Color::Black));
            separator_string -= 1;
        } else {
            plot(score_string[letter], (BUFFER_WIDTH / 2) + separator_string_2, BUFFER_HEIGHT / 2, ColorCode::new(Color::White, Color::Black));
            separator_string_2 += 1;
        }
    }
    plot_num(current_time, (BUFFER_WIDTH / 2) + 7, BUFFER_HEIGHT / 2, ColorCode::new(Color::White, Color::Black));

    separator_string = 9;
    separator_string_2 = 0;
    for letter in 0..18 {
        if letter < 9 {
            plot(restart_string[letter], (BUFFER_WIDTH / 2) - separator_string, (BUFFER_HEIGHT / 2) + 1, ColorCode::new(Color::White, Color::Black));
            separator_string -= 1;
        } else {
            plot(restart_string[letter], (BUFFER_WIDTH / 2) + separator_string_2, (BUFFER_HEIGHT / 2) + 1, ColorCode::new(Color::White, Color::Black));
            separator_string_2 += 1;
        }
    }
}