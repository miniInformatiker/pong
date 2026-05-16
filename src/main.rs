use macroquad::prelude::*;
use macroquad::rand::gen_range;

struct Ball {
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
    radius: f32,
}

struct Paddle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    speed: f32,
}

fn ball_hits_paddle(ball: &Ball, paddle: &Paddle) -> bool {
    ball.x + ball.radius >= paddle.x
        && ball.x - ball.radius <= paddle.x + paddle.width
        && ball.y + ball.radius >= paddle.y
        && ball.y - ball.radius <= paddle.y + paddle.height
}

fn reset_ball(ball: &mut Ball) {
    ball.x = screen_width() / 2.0;
    ball.y = screen_height() / 2.0;
    
    let direction = if gen_range(0, 2) == 0 {-1.0} else {1.0};
    
    ball.vel_x = 250.0 * direction;
    ball.vel_y = gen_range(-200.0, 200.0);
}

#[macroquad::main("Pong")]
async fn main() {
    let mut ball = Ball {
        x: 400.0,
        y: 300.0,
        vel_x: 250.0,
        vel_y: 180.0,
        radius: 10.0,
    };

    let mut left_paddle = Paddle {
        x: 30.0,
        y: 250.0,
        width: 20.0,
        height: 100.0,
        speed: 400.0,
    };

    let mut right_paddle = Paddle {
        x: 750.0,
        y: 250.0,
        width: 20.0,
        height: 100.0,
        speed: 400.0,
    };

    let mut left_score = 0;
    let mut right_score = 0;

    loop {
        let dt = get_frame_time();

        if is_key_down(KeyCode::W) {
            left_paddle.y -= left_paddle.speed * dt;
        }

        if is_key_down(KeyCode::S) {
            left_paddle.y += left_paddle.speed * dt;
        }

        if is_key_down(KeyCode::Up) {
            right_paddle.y -= right_paddle.speed * dt;
        }

        if is_key_down(KeyCode::Down) {
            right_paddle.y += right_paddle.speed * dt;
        }

        // Bewegung
        ball.x += ball.vel_x * dt;
        ball.y += ball.vel_y * dt;

        if ball_hits_paddle(&ball, &left_paddle) && ball.vel_x < 0.0 {
            ball.vel_x *= -1.05;

            let paddle_center = left_paddle.y + left_paddle.height / 2.0;

            let hit_pos = (ball.y - paddle_center) / (left_paddle.height / 2.0);

            ball.vel_y = hit_pos * 300.0;

            ball.x = left_paddle.x + left_paddle.width + ball.radius;
        }

        if ball_hits_paddle(&ball, &right_paddle) && ball.vel_x > 0.0 {
            ball.vel_x *= -1.05;

            let paddle_center = right_paddle.y + right_paddle.height / 2.0;

            let hit_pos = (ball.y - paddle_center) / (right_paddle.height / 2.0);

            ball.vel_y = hit_pos * 300.0;

            ball.x = right_paddle.x - ball.radius;
        }

        if ball.x < 0.0 {
            right_score += 1;
            reset_ball(&mut ball);
        }

        if ball.x > screen_width() {
            left_score += 1;
            reset_ball(&mut ball);
        }

        // Wandkollision
        if ball.y <= 0.0 || ball.y >= screen_height() {
            ball.vel_y *= -1.0;
        }

        clear_background(BLACK);

        draw_line(screen_width() / 2.0, 0.0,screen_width() /2.0, screen_height(),2.0, GRAY);

        draw_text(
            &format!("{} : {}", left_score, right_score),
            screen_width() / 2.0 - 40.0,
            50.0,
            40.0,
            WHITE,
        );

        draw_circle(ball.x, ball.y, ball.radius, WHITE);

        draw_rectangle(
            left_paddle.x,
            left_paddle.y,
            left_paddle.width,
            left_paddle.height,
            WHITE,
        );

        draw_rectangle(
            right_paddle.x,
            right_paddle.y,
            right_paddle.width,
            right_paddle.height,
            WHITE,
        );

        left_paddle.y = left_paddle
            .y
            .clamp(0.0, screen_height() - left_paddle.height);
        right_paddle.y = right_paddle
            .y
            .clamp(0.0, screen_height() - right_paddle.height);

        next_frame().await;
    }
}
