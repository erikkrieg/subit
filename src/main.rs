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
    let mut n = 0;
    let mut history: Vec<Items> = Vec::new();
    let mut history_index = 0;
    loop {
        if history_index >= history.len() {
            n = rand::gen_range(1, max_n + 1);
            history.push(new_items(n));
        }
        clear_background(bg);
        draw_items(&history[history_index]);
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Right) {
            history_index += 1;
        }
        if is_key_pressed(KeyCode::Left) {
            if history_index > 0 {
                history_index -= 1;
            }
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
        let x = rand::gen_range(0.0 + size, screen_width() - size);
        let y = rand::gen_range(0.0 + size, screen_height() - size);
        items.push((x, y, size));
    }
    items
}

fn draw_items(items: &Items) {
    for (x, y, size) in items.iter() {
        draw_circle(*x, *y, *size, WHITE);
    }
}
