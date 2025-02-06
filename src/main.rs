use macroquad::prelude::*;
pub mod glass_tank;

#[macroquad::main("Falling Sand")]
async fn main() {
    let screen_width = 150;
    let screen_height = 200;

                                                         // Magic number to fix window height
    request_new_screen_size(screen_width as f32, screen_height as f32 + 28.0);
    next_frame().await;

    let mut tank = glass_tank::Tank::new(screen_width, screen_height, 5);

    loop {
        clear_background(BLACK);

        let mouse_pos = mouse_position();
        
        if mouse_pos.0 > 0.0 && mouse_pos.0 < screen_width as f32 &&
           mouse_pos.1 > 0.0 && mouse_pos.1 < screen_height as f32 {
            if is_key_down(KeyCode::G) {
                tank.drop_sand_in_column(mouse_pos.0 as usize, 'G'); // Green
            } else if is_key_down(KeyCode::R) {
                tank.drop_sand_in_column(mouse_pos.0 as usize, 'R'); // Red
            } else if is_key_down(KeyCode::B) {
                tank.drop_sand_in_column(mouse_pos.0 as usize, 'B'); // Blue
            } else if is_key_down(KeyCode::P) {
                tank.drop_sand_in_column(mouse_pos.0 as usize, 'P'); // Pink
            } else if is_key_down(KeyCode::U) {
                tank.drop_sand_in_column(mouse_pos.0 as usize, 'U'); // Purple
            } else if is_key_down(KeyCode::C) {
                tank.drop_sand_in_column(mouse_pos.0 as usize, 'C'); // Cyan
            } else if is_key_down(KeyCode::Y) {
                tank.drop_sand_in_column(mouse_pos.0 as usize, 'Y'); // Yellow
            } else if is_key_down(KeyCode::O) {
                tank.drop_sand_in_column(mouse_pos.0 as usize, 'O'); // Orange
            } else if is_key_down(KeyCode::M) {
                tank.drop_sand_in_column(mouse_pos.0 as usize, 'M'); // Maroon
            } else if is_key_down(KeyCode::W) {
                tank.drop_sand_in_column(mouse_pos.0 as usize, 'W'); // White
            } else if is_key_down(KeyCode::F) {
                tank.drop_sand_in_column(mouse_pos.0 as usize, 'F'); // Fuscia
            } else if is_key_down(KeyCode::S) {
                tank.drop_sand_in_column(mouse_pos.0 as usize, 'S'); // Sand Color (Beige)
            } 
        }

        for row in 0..tank.grains.len() {
            for column in 0..tank.grains[row].len() {
                if tank.grains[row][column] == '-' {
                    continue;
                }

                let color = match tank.grains[row][column] {
                    'P' => PINK,
                    'F' => MAGENTA,
                    'R' => RED,
                    'M' => MAROON,
                    'O' => ORANGE,
                    'S' => BEIGE,
                    'Y' => YELLOW,
                    'G' => GREEN,
                    'C' => SKYBLUE,
                    'B' => BLUE,
                    'U' => PURPLE,
                    _ => BLACK
                };

                draw_rectangle(column as f32, row as f32, 1.0, 1.0, color);
            }
        }

        if is_key_down(KeyCode::Semicolon) {
            let screenshot: Image = get_screen_data();
            screenshot.export_png("my_screenshot.png");
        }

        tank.advance_frame();
        next_frame().await
    }
}