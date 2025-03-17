use core::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

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

    pub fn draw(&mut self) {
        // This canvas element is set by the dom element in React/JS
        let width = self.ctx.canvas().unwrap().width() as f64;
        let height = self.ctx.canvas().unwrap().height() as f64;

        let r0 = width / 2.0;

        self.ctx.begin_path();
        self.ctx.move_to(width, height / 2.0);

        for i in 0..1000 {
            let t = (2.0 * f64::consts::PI / 1000.0) * i as f64;
            let x = r0 * t.cos() + width / 2.0;
            let y = r0 * t.sin() + height / 2.0;

            self.ctx.line_to(x, y);
        }
        self.ctx.stroke();
    }
}
#[wasm_bindgen]
pub fn generate_svg() -> String {
    let mut svg = String::from(
        r#"<svg width="500" height="500" xmlns="http://www.w3.org/2000/svg">
        <path d="M"#,
    );

    for t in (0..=1000).map(|x| x as f64 * 0.01) {
        let x = 250.0 + 100.0 * (t * 3.0).cos() + 50.0 * (t * 2.0).cos();
        let y = 250.0 + 100.0 * (t * 3.0).sin() + 50.0 * (t * 2.0).sin();
        svg.push_str(&format!("{},{} ", x, y));
    }

    svg.push_str(r#"" stroke="black" fill="none" stroke-width="2"/></svg>"#);
    svg
}
