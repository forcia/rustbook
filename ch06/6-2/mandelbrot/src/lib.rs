mod logic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::ImageData;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

macro_rules! measure_elapsed_time {
    ($t:tt,$s:block) => {{
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");
        let start = performance.now();
        let result = { $s };
        let end = performance.now();
        console_log!("{}:{}[ms]", $t, end - start);
        result
    }};
}

#[wasm_bindgen]
pub fn generate_mandelbrot_set(
    canvas_w: usize,
    canvas_h: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    max_iter: usize,
) -> Vec<u8> {
    // measure_elapsed_time!("generate:wasm\telapsed:", {
    logic::generate_mandelbrot_set(canvas_w, canvas_h, x_min, x_max, y_min, y_max, max_iter)
    // })
}

#[wasm_bindgen]
pub fn draw_mandelbrot_set() {
    const CANVAS_ID: &str = "canvas_wasm";
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let canvas_w = canvas.width() as usize;
    let canvas_h = canvas.height() as usize;
    const X_MIN: f64 = -1.5;
    const X_MAX: f64 = 0.5;
    const Y_MIN: f64 = -1.0;
    const Y_MAX: f64 = 1.0;
    const MAX_ITER: usize = 1 << 7 - 1;

    let mut result = measure_elapsed_time!("\tgenerate:wasm\telapsed:", {
        logic::generate_mandelbrot_set(canvas_w, canvas_h, X_MIN, X_MAX, Y_MIN, Y_MAX, MAX_ITER)
    });
    measure_elapsed_time!("\tdraw:wasm\telapsed:", {
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut result),
            canvas.width(),
            canvas.height(),
        );
        if let Ok(data) = data {
            let _ = context.put_image_data(&data, 0.0, 0.0);
        }
    })
}
