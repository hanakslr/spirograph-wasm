use core::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct Spirograph {
    ctx: CanvasRenderingContext2d,
    t: f64,
}

#[wasm_bindgen]
impl Spirograph {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Self { ctx, t: 0.0 }
    }

    // Helper function to calculate the number of rotations needed
    fn calculate_rotations(&self, ratio: f64) -> i32 {
        // Convert the ratio to a fraction in lowest terms
        // We'll use a simple continued fraction approximation
        let mut n = ratio;
        let mut d = 1.0;
        let mut prev_n = 1.0;
        let mut prev_d = 0.0;

        // Use continued fraction expansion to find a rational approximation
        for _ in 0..10 {
            // Limit iterations to avoid infinite loops
            let whole = n.floor();
            let new_n = prev_n + whole * n;
            let new_d = prev_d + whole * d;

            prev_n = n;
            prev_d = d;
            n = new_n;
            d = new_d;

            // If we've found a good approximation, return the denominator
            if (ratio - n / d).abs() < 0.0001 {
                return d.round() as i32;
            }
        }

        // If we couldn't find a good approximation, return a reasonable default
        100
    }

    pub fn clear(&mut self) {
        let width = self.ctx.canvas().unwrap().width() as f64;
        let height = self.ctx.canvas().unwrap().height() as f64;
        self.ctx.clear_rect(0.0, 0.0, width, height);
    }

    pub fn draw_single(
        &mut self,
        inner_r: f64,
        offset: f64,
        phase_angle: Option<f64>,
        stroke_color: Option<String>,
    ) {
        let width = self.ctx.canvas().unwrap().width() as f64;
        let height = self.ctx.canvas().unwrap().height() as f64;
        let phase_angle = phase_angle.unwrap_or(0.0);

        let r_fixed = width / 4.0;
        let r = inner_r;
        let p = offset;

        // Calculate the number of rotations needed
        // The pattern repeats when the inner circle has completed an integer number of rotations
        // relative to the outer circle. This happens when:
        // (r_fixed + r) / r is a rational number
        // The number of rotations is the denominator of this ratio in lowest terms
        let ratio = (r_fixed + r) / r;
        let rotations = self.calculate_rotations(ratio);

        let step_size = 5000;
        self.ctx.begin_path();

        for i in 0..(step_size * rotations) {
            let t = (2.0 * f64::consts::PI / step_size as f64) * i as f64;
            let x = ((r_fixed + r) * t.cos())
                - ((r + p) * (((r_fixed + r) / r) * (t + phase_angle)).cos());
            let y = (r_fixed + r) * t.sin()
                - ((r + p) * ((t + phase_angle) * ((r_fixed + r) / r)).sin());

            self.ctx.line_to(x + width / 2.0, y + height / 2.0);
        }

        if let Some(color) = stroke_color {
            self.ctx.set_stroke_style_str(&color);
        }
        self.ctx.stroke();
    }

    pub fn draw(&mut self) {
        // This canvas element is set by the dom element in React/JS
        let width = self.ctx.canvas().unwrap().width() as f64;
        let height = self.ctx.canvas().unwrap().height() as f64;

        // This just designates resolution
        let step_size = 3000;

        let r0 = width / 2.0;
        let n0 = 1.0;

        self.ctx.begin_path();
        self.ctx.move_to(width, height / 2.0);

        let mut pos_x0 = vec![0.0; step_size];
        let mut pos_y0 = vec![0.0; step_size];

        for i in 0..step_size {
            let t = (2.0 * f64::consts::PI / step_size as f64) * i as f64 * n0;
            let x = r0 * t.cos() + width / 2.0;
            let y = r0 * t.sin() + height / 2.0;
            pos_x0[i] = x;
            pos_y0[i] = y;

            console::log_1(&JsValue::from_str(&format!("0 - x {} y {}", x, y)));

            self.ctx.line_to(x, y);
        }

        self.ctx.stroke();

        // Now lets draw another one
        let r1 = width / 8.0;
        let n1 = 32.0;

        self.ctx.begin_path();
        self.ctx.move_to(width, height / 2.0);

        let mut pos_x1 = vec![0.0; step_size];
        let mut pos_y1 = vec![0.0; step_size];

        for i in 0..step_size {
            let t = (2.0 * f64::consts::PI / step_size as f64) * i as f64 * n1;
            let x = r1 * t.cos() + pos_x0[i];
            let y = r1 * t.sin() + pos_y0[i];

            pos_x1[i] = x;
            pos_y1[i] = y;

            console::log_1(&JsValue::from_str(&format!("1 - x {} y {}", x, y)));

            self.ctx.line_to(x, y);
        }

        self.ctx.set_stroke_style_str("#0000FF");
        self.ctx.stroke();

        // And lets draw another one
        let r1 = width / 21.0;
        let n1 = 15.0;

        self.ctx.begin_path();
        self.ctx.move_to(width, height / 2.0);

        for i in 0..step_size {
            let t = (2.0 * f64::consts::PI / step_size as f64) * i as f64 * n1;
            let x = r1 * t.cos() + pos_x1[i];
            let y = r1 * t.sin() + pos_y1[i];

            self.ctx.line_to(x, y);
        }

        self.ctx.set_stroke_style_str("#800080");
        self.ctx.stroke();
    }
}
