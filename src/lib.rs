use wasm_bindgen::prelude::*;
use web_sys::*;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod chart;
mod component;

mod context;
use context::canvas_context::CanvasContext;
use context::svg_context::SvgContext;
use context::Context;

#[derive(Copy, Clone, Debug)]
struct World {
    pub x_min: u32,
    pub x_max: u32,
    pub y_min: u32,
    pub y_max: u32,
}

impl World {
    fn new(x_min: u32, x_max: u32, y_min: u32, y_max: u32) -> World {
        World {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct ViewPort {
    pub x_min: u32,
    pub x_max: u32,
    pub y_min: u32,
    pub y_max: u32,
}

impl ViewPort {
    fn new(x_min: u32, x_max: u32, y_min: u32, y_max: u32) -> ViewPort {
        ViewPort {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}

fn world_to_viewport(world: World, viewport: ViewPort, point: (u32, u32)) -> (u32, u32) {
    let (x, y) = point;
    // scaling factors for x coordinate and y coordinate
    let (x_scale, y_scale) = (
        (viewport.x_max - viewport.x_min) / (world.x_max - world.x_min),
        (viewport.y_max - viewport.y_min) / (world.y_max - world.y_min),
    );
    // calculate the new x and y coordinates
    let x = (x - world.x_min) * x_scale + viewport.x_min;
    let y = (y - world.y_min) * y_scale + viewport.y_min;
    (x, y)
}

#[wasm_bindgen]
pub struct App {
    root: HtmlElement,
    world: World,
    viewPort: ViewPort,
}

type Candle = (u32, u32, u32, u32);

#[wasm_bindgen]
impl App {
    pub fn new(root: HtmlElement) -> App {
        let world = World::new(
            0,
            root.client_width() as u32,
            0,
            root.client_height() as u32,
        );
        let viewPort = ViewPort::new(
            0,
            root.client_width() as u32,
            0,
            root.client_height() as u32,
        );
        App {
            root,
            world,
            viewPort,
        }
    }

    fn init(&self) {
        #[cfg(debug_assertions)]
        console_error_panic_hook::set_once();
    }

    pub fn run(&self) -> Result<(), JsValue> {
        self.init();

        // svg is the viewPort
        let svg_context = SvgContext::new(
            self.root.client_width() as u32,
            self.root.client_height() as u32,
        )?;

        // Draw a grid on the world dimension every 10 units
        let step: u32 = 10;

        for x in (0..self.world.x_max).step_by(step as usize) {
            svg_context.draw_line(
                world_to_viewport(self.world, self.viewPort, (x, 0)),
                world_to_viewport(self.world, self.viewPort, (x, self.world.y_max)),
                "grey",
            )?;
        }
        for y in (0..self.world.y_max).step_by(step as usize) {
            svg_context.draw_line(
                world_to_viewport(self.world, self.viewPort, (0, y)),
                world_to_viewport(self.world, self.viewPort, (self.world.x_max, y)),
                "grey",
            )?;
        }

        let mut candles = Vec::<Candle>::new();
        candles.push((10, 100, 20, 0));
        candles.push((100, 190, 80, 40));
        candles.push((110, 200, 90, 50));
        candles.push((120, 210, 100, 60));
        candles.push((130, 220, 110, 70));
        candles.push((140, 230, 120, 80));
        candles.push((150, 240, 130, 90));

        for (index, candle) in candles.iter().enumerate() {
            if index > 1 {
                svg_context.draw_line(
                    world_to_viewport(self.world, self.viewPort, (index as u32 * step, candle.1)),
                    world_to_viewport(self.world, self.viewPort, (index as u32 * step, candle.2)),
                    "white",
                )?;
                svg_context.draw_rect(
                    world_to_viewport(
                        self.world,
                        self.viewPort,
                        ((index as u32 * step) - step / 2, candle.0),
                    ),
                    world_to_viewport(self.world, self.viewPort, (step, candle.3)),
                    "red",
                )?;
            }
        }

        self.root.append_child(&svg_context.svg)?;

        // let svg_context = SvgContext::new(100, 100)?;
        // svg_context.draw_pixel((50, 50), "red")?;
        // svg_context.draw_line((0, 0), (100, 100), "red")?;
        // svg_context.draw_rect((0, 0), (100, 100), "red")?;
        // svg_context.draw_circle((50, 50), 50, "red")?;
        // self.root.append_child(&svg_context.svg)?;
        // let canvas_context = CanvasContext::new(100, 100)?;
        // canvas_context.draw_pixel((50, 50), "blue")?;
        // canvas_context.draw_line((0, 0), (100, 100), "blue")?;
        // canvas_context.draw_rect((0, 0), (100, 100), "blue")?;
        // canvas_context.draw_circle((50, 50), 50, "blue")?;
        // self.root.append_child(&canvas_context.canvas)?;

        console::log_1(&JsValue::from_str("App is running!"));

        Ok(())
    }
}
