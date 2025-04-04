use core::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct Spirograph {
    ctx: CanvasRenderingContext2d,
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

        Self { ctx }
    }

    // Helper function to calculate the number of rotations needed
    fn calculate_rotations(&self, ratio: f64) -> i32 {
        // Convert the ratio to a fraction in lowest terms.
        // The drawing is complete, and repeats on itself when the denominator
        // is a whole number.
        let mut n = ratio;
        let mut d = 1.0;
        let mut prev_n = 1.0;
        let mut prev_d = 0.0;

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
        // Convert phase angle from degrees to radians
        let phase_angle = phase_angle.unwrap_or(0.0) * f64::consts::PI / 180.0;

        console::log_1(&JsValue::from_str(&format!(
            "Parameters - inner_r: {}, offset: {}, phase_angle: {}",
            inner_r, offset, phase_angle
        )));

        let r_fixed = width / 4.0;
        let r = inner_r;
        let p = offset;

        // Calculate the number of rotations needed until the pattern repeats
        let ratio = (r_fixed + r) / r;
        let rotations = self.calculate_rotations(ratio);

        let step_size = 10000;
        self.ctx.begin_path();

        for i in 0..(step_size * rotations) {
            let t = (2.0 * f64::consts::PI / step_size as f64) * i as f64;
            let x = (r_fixed - r) * t.cos() + p * (((r_fixed - r) / r) * (t + phase_angle)).cos();
            let y = (r_fixed - r) * t.sin() + p * ((t + phase_angle) * ((r_fixed - r) / r)).sin();

            self.ctx.line_to(x + width / 2.0, y + height / 2.0);
        }

        if let Some(color) = stroke_color {
            self.ctx.set_stroke_style_str(&color);
        }
        self.ctx.set_line_width(1.5);
        self.ctx.stroke();
    }
}
