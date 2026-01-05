fn sierpinski(draw: &Draw, x: f32, y: f32, size: f32, depth: u32) {
    // base case: draw a triangle
    if depth == 0 {
        let p1 = pt2(x, y + size / 2.0);
        let p2 = pt2(x - size / 3.0_f32.sqrt(), y - size / 2.0);
        let p3 = pt2(x + size / 3.0_f32.sqrt(), y - size / 2.0);
        draw.tri().points(p1, p2, p3).color(WHITE);
    }
    // recursive case: call sierpinski three times at half size
    else {
        let new_x1 = x;
        let new_y1 = y + size / 4.0;
        sierpinski(draw, new_x1, new_y1, size / 2.0, depth - 1);

        let new_x2 = x - size / (2.0 * 3.0_f32.sqrt());
        let new_y2 = y - size / 4.0;
        sierpinski(draw, new_x2, new_y2, size / 2.0, depth - 1);

        let new_x3 = x + size / (2.0 * 3.0_f32.sqrt());
        let new_y3 = y - size / 4.0;
        sierpinski(draw, new_x3, new_y3, size / 2.0, depth - 1);
    }
}

use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    sierpinski(&draw, 0.0, 0.0, 500.0, 7);

    draw.to_frame(app, &frame).unwrap();
}
