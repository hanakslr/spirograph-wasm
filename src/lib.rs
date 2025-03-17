use wasm_bindgen::prelude::*;

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
