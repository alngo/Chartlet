use wasm_bindgen::prelude::*;
use web_sys::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Guidelines {
    width: u32,
    height: u32,
}

impl Guidelines {
    fn new(width: u32, height: u32) -> Guidelines {
        Guidelines { width, height }
    }

    fn render(&self) -> web_sys::SvgElement {
        let document = web_sys::window().unwrap().document().unwrap();
        let svg = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
            .unwrap()
            .dyn_into::<web_sys::SvgElement>()
            .map_err(|_| ())
            .unwrap();

        svg.set_attribute("width", &self.width.to_string()).unwrap();
        svg.set_attribute("height", &self.height.to_string())
            .unwrap();
        svg.set_attribute("viewBox", &format!("0 0 {} {}", self.width, self.height))
            .unwrap();

        let grid = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "g")
            .unwrap()
            .dyn_into::<web_sys::SvgElement>()
            .map_err(|_| ())
            .unwrap();

        let mut x = 0;
        let mut y = 0;
        while x < self.width {
            let line = document
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "line")
                .unwrap()
                .dyn_into::<web_sys::SvgElement>()
                .map_err(|_| ())
                .unwrap();
            line.set_attribute("x1", &x.to_string()).unwrap();
            line.set_attribute("y1", "0").unwrap();
            line.set_attribute("x2", &x.to_string()).unwrap();
            line.set_attribute("y2", &self.height.to_string()).unwrap();
            line.set_attribute("stroke", "grey").unwrap();
            line.set_attribute("stroke-width", "1").unwrap();
            grid.append_child(&line).unwrap();
            x += 10;
        }
        while y < self.height {
            let line = document
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "line")
                .unwrap()
                .dyn_into::<web_sys::SvgElement>()
                .map_err(|_| ())
                .unwrap();
            line.set_attribute("x1", "0").unwrap();
            line.set_attribute("y1", &y.to_string()).unwrap();
            line.set_attribute("x2", &self.width.to_string()).unwrap();
            line.set_attribute("y2", &y.to_string()).unwrap();
            line.set_attribute("stroke", "grey").unwrap();
            line.set_attribute("stroke-width", "1").unwrap();
            grid.append_child(&line).unwrap();
            y += 10;
        }
        svg.append_child(&grid).unwrap();
        svg
    }
}

#[wasm_bindgen]
pub fn run(width: u32, height: u32) -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let chartlet = document.get_element_by_id("chartlet").unwrap();

    let grid = Guidelines::new(width, height);
    let svg = grid.render();

    chartlet.append_child(&svg)?;

    Ok(())
}
