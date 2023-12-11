use wasm_bindgen::prelude::*;
use web_sys::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct App {
    width: u32,
    height: u32,
    root: HtmlElement,
}

#[wasm_bindgen]
impl App {
    pub fn new(root: &HtmlElement) -> App {
        App {
            width: root.client_width() as u32,
            height: root.client_height() as u32,
            root: root.clone(),
        }
    }

    pub fn resize(&mut self, root: &HtmlElement) {
        self.width = root.client_width() as u32;
        self.height = root.client_height() as u32;
        self.render().unwrap();
    }

    pub fn render(&self) -> Result<(), JsValue> {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");

        let svg: SvgElement = match document.get_element_by_id("coord") {
            Some(svg) => svg
                .dyn_into::<web_sys::SvgElement>()
                .map_err(|_| ())
                .unwrap(),
            _ => document
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
                .unwrap()
                .dyn_into::<web_sys::SvgElement>()
                .map_err(|_| ())
                .unwrap(),
        };
        svg.set_attribute("id", "coord")?;
        svg.set_attribute("width", &format!("{}", self.width))?;
        svg.set_attribute("height", &format!("{}", self.height))?;
        svg.set_attribute("viewBox", &format!("0 0 {} {}", self.width, self.height))?;

        //clean svg
        while let Some(child) = svg.last_child() {
            svg.remove_child(&child)?;
        }

        let circle = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
        circle.set_attribute("cx", "100")?;
        circle.set_attribute("cy", "100")?;
        circle.set_attribute("r", "60")?;
        circle.set_attribute("stroke", "black")?;
        circle.set_attribute("fill", "blue")?;
        svg.append_child(&circle)?;

        self.root.append_child(&svg)?;

        Ok(())
    }

    pub fn run(&self) -> Result<(), JsValue> {
        #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();

        self.render()?;
        console::log_1(&JsValue::from_str("Hello from Rust!"));

        Ok(())
    }
}
