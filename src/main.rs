use ::rand::rngs::SmallRng;
use ::rand::{Rng, SeedableRng};
use macroquad::prelude::*;

// One star in 3D space, plus a few values for motion and brightness.
struct Star {
    x: f32,
    y: f32,
    z: f32,
    speed: f32,
    brightness: f32,
}

const STAR_DEPTH: f32 = 32.0;
const STAR_SPREAD: f32 = 20.0;
const TITLE_TEXT: &str = "RUST // WASM";

fn init_stars(count: usize) -> Vec<Star> {
    // Fixed seed so the initial field is repeatable while developing.
    let mut rng = SmallRng::seed_from_u64(0);
    (0..count)
        .map(|_| Star {
            // Spread stars across a wider 3D volume so the field fills the screen.
            x: rng.gen_range(-STAR_SPREAD..STAR_SPREAD),
            y: rng.gen_range(-STAR_SPREAD..STAR_SPREAD),
            z: rng.gen_range(1.0..STAR_DEPTH),
            speed: rng.gen_range(0.012..0.032),
            brightness: rng.gen_range(0.6..1.0),
        })
        .collect()
}

#[macroquad::main("Starfield")]
async fn main() {
    // Create the initial field once at startup.
    let mut stars = init_stars(650);
    // Time-based seed so recycled stars re-enter less predictably.
    let mut rng = SmallRng::seed_from_u64(macroquad::miniquad::date::now() as u64);

    loop {
        // Recompute dimensions every frame so the effect stays centered.
        let screen_w = screen_width();
        let screen_h = screen_height();
        let center_x = screen_w * 0.5;
        let center_y = screen_h * 0.5;
        let time = get_time() as f32;

        // Paint the background before drawing the stars on top.
        clear_background(Color::from_rgba(3, 4, 10, 255));
        draw_rectangle(
            0.0,
            0.0,
            screen_w,
            screen_h,
            Color::from_rgba(18, 10, 36, 90),
        );

        for star in stars.iter_mut() {
            // 1. Move the star forward along the Z-axis
            star.z -= star.speed;

            // 2. Recycle the star if it gets too close to the "windshield"
            if star.z < 0.2 {
                star.z = STAR_DEPTH;
                // Respawn across the full width of the field, not just the center.
                star.x = rng.gen_range(-STAR_SPREAD..STAR_SPREAD);
                star.y = rng.gen_range(-STAR_SPREAD..STAR_SPREAD);
            }

            // 3. Project to 2D screen: dividing by 'z' is the key.
            let sx = (star.x / star.z) * screen_w * 0.5 + center_x;
            let sy = (star.y / star.z) * screen_h * 0.5 + center_y;

            // 4. Scale size and brightness based on depth
            let depth_factor = 1.0 - (star.z / STAR_DEPTH);
            let size = 1.0 + 2.0 * depth_factor;
            let brightness = star.brightness * depth_factor;

            // Draw brighter, larger stars as they get closer to the camera.
            draw_circle(
                sx,
                sy,
                size,
                Color::new(brightness, brightness, brightness, 1.0),
            );
        }

        // Smoothly oscillate between 0.0 and 1.0 for the title pulse
        let pulse = 0.5 + 0.5 * (time * 2.0).sin();
        let title_color = Color::new(0.5 + 0.5 * pulse, 0.8, 1.0, 1.0);
        let title_size = 84.0;
        let title_metrics = measure_text(TITLE_TEXT, None, title_size as u16, 1.0);
        let title_x = center_x - title_metrics.width * 0.5;
        let title_y = center_y - title_metrics.height * 0.5;
        draw_text(TITLE_TEXT, title_x, title_y, title_size, title_color);

        // Present the current frame, then continue the loop
        next_frame().await;
    }
}
