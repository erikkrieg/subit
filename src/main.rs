use macroquad::prelude::*;

const ITEM_SIZE: f32 = 50.0;

#[macroquad::main("subit")]
async fn main() {
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64;
    rand::srand(seed);
    // TODO: ensure good background color contrast with white items
    let bg = Color::new(
        rand::gen_range(0.0, 1.0),
        rand::gen_range(0.0, 1.0),
        rand::gen_range(0.0, 1.0),
        1.0,
    );
    // TODO: have user provide min and max number of items to draw
    let max_n = 5;
    let mut history: Vec<Items> = Vec::new();
    let mut history_index = 0;
    loop {
        if history_index >= history.len() {
            history.push(new_items(rand::gen_range(1, max_n + 1)));
        }
        clear_background(bg);
        draw_items(&history[history_index]);
        match get_last_key_pressed() {
            Some(KeyCode::Space) | Some(KeyCode::Right) => history_index += 1,
            Some(KeyCode::Left) =>  history_index = history_index.saturating_sub(1),
            Some(KeyCode::Escape) | Some(KeyCode::Q) => break,
            Some(k) if key_to_digit(k).is_some() => {
                let guessed_n = key_to_digit(k).unwrap();
                let actual_n = history[history_index].len();
                if guessed_n == actual_n {
                    history_index += 1;
                }
            }
            _ => {}
        }
        next_frame().await
    }
}

type Item = (f32, f32, f32); // x, y, size
type Items = Vec<Item>;

fn new_items(n: usize) -> Items {
    let mut items = Vec::new();
    for _ in 0..n {
        let size = ITEM_SIZE;
        // TODO: ensure items do not overlap
        let x = rand::gen_range(size, screen_width() - size);
        let y = rand::gen_range(size, screen_height() - size);
        items.push((x, y, size));
    }
    items
}

fn draw_items(items: &[Item]) {
    for (x, y, size) in items.iter() {
        draw_circle(*x, *y, *size, WHITE);
    }
}

fn key_to_digit(k: KeyCode) -> Option<usize> {
    match k {
        KeyCode::Key1 | KeyCode::Kp1 => Some(1),
        KeyCode::Key2 | KeyCode::Kp2 => Some(2),
        KeyCode::Key3 | KeyCode::Kp3 => Some(3),
        KeyCode::Key4 | KeyCode::Kp4 => Some(4),
        KeyCode::Key5 | KeyCode::Kp5 => Some(5),
        KeyCode::Key6 | KeyCode::Kp6 => Some(6),
        KeyCode::Key7 | KeyCode::Kp7 => Some(7),
        KeyCode::Key8 | KeyCode::Kp8 => Some(8),
        KeyCode::Key9 | KeyCode::Kp9 => Some(9),
        _ => None,
    }
}
