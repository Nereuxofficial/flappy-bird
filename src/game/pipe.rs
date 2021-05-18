use crate::game::game::Render;
use wasm_bindgen::JsValue;

pub const WIDTH: f64 = 100.0;
pub const HEIGHT: f64 = 750.0;
pub const HOLE_SIZE: f64 = 80.0;
pub const BORDER_WIDTH: f64 = 20.0;
pub const LINE_WIDTH: f64 = 5.0;

pub struct Pipe {
    pub x: f64,
    pub y: f64,
    pub hole: f64
}

impl Pipe {
    pub fn new(x: f64, y: f64) -> Pipe {
        Pipe { x, y , hole: y - HOLE_SIZE / 2.}
    }

    pub fn move_left(&mut self) {
        self.x -= 5.0;
    }
}

impl Render for Pipe {
    fn render(&self, canvas_ctx: &web_sys::CanvasRenderingContext2d) {
        canvas_ctx.begin_path();
        canvas_ctx.set_fill_style(&JsValue::from_str("#6ebb2d"));
        canvas_ctx.set_stroke_style(&JsValue::from_str("black"));
        canvas_ctx.set_line_width(LINE_WIDTH);
        canvas_ctx.rect(self.x, self.y + BORDER_WIDTH , WIDTH, HEIGHT);
        canvas_ctx.fill();
        canvas_ctx.rect(self.x, self.y + BORDER_WIDTH , WIDTH, HEIGHT);
        canvas_ctx.stroke();
        canvas_ctx.rect(self.x - 5.0 , self.y, WIDTH + 10.0, BORDER_WIDTH);
        canvas_ctx.fill();
        canvas_ctx.rect(self.x - 5.0, self.y, WIDTH + 10.0, BORDER_WIDTH);
        canvas_ctx.stroke();

        canvas_ctx.rect(self.x, self.y - HOLE_SIZE - BORDER_WIDTH, WIDTH, -HEIGHT);
        canvas_ctx.fill();
        canvas_ctx.rect(self.x, self.y - HOLE_SIZE - BORDER_WIDTH, WIDTH, -HEIGHT);
        canvas_ctx.stroke();
        canvas_ctx.rect(self.x - 5.0 , self.y - HOLE_SIZE , WIDTH + 10.0, -BORDER_WIDTH);
        canvas_ctx.fill();
        canvas_ctx.rect(self.x - 5.0, self.y - HOLE_SIZE , WIDTH + 10.0, -BORDER_WIDTH);
        canvas_ctx.stroke();
    }
}
