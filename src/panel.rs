use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};

#[derive(Debug)]
pub struct Panel {
    pub canvas: HtmlCanvasElement,
    pub ratio: f64,
    pub label: String,
    pub foreground_color: String,
    pub background_color: String,
}

impl Panel {
    pub fn create(
        window: &Window,
        label: String,
        foreground_color: String,
        background_color: String,
    ) -> Self {
        let ratio = window.device_pixel_ratio();
        let document = window.document().unwrap();
        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let width = 80.0 * ratio;
        let height = 48.0 * ratio;
        let text_x = 3.0 * ratio;
        let text_y = 2.0 * ratio;
        let graph_x = 3.0 * ratio;
        let graph_y = 15.0 * ratio;
        let graph_width = 74.0 * ratio;
        let graph_height = 30.0 * ratio;

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
        canvas.style().set_css_text(&*format!(
            "width: {}px; height: {}px",
            width as u32 / 2,
            height as u32 / 2
        ));

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        context.set_font(&*format!(
            "bold {}px Helvetica,Arial,sans-serif",
            (9.0 * ratio) as u32
        ));
        context.set_text_baseline("top");

        context.set_fill_style(&JsValue::from_str(&background_color));
        context.fill_rect(0.0, 0.0, width, height);

        context.set_fill_style(&JsValue::from_str(&foreground_color));
        context.fill_text(&*label, text_x, text_y).unwrap();
        context.fill_rect(graph_x, graph_y, graph_width, graph_height);

        context.set_fill_style(&JsValue::from_str(&background_color));
        context.set_global_alpha(0.9);
        context.fill_rect(graph_x, graph_y, graph_width, graph_height);

        Self {
            canvas,
            ratio,
            label,
            foreground_color,
            background_color,
        }
    }

    pub fn update(&mut self, value: f64, max_value: f64) {
        let context = self
            .canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let width = 80.0 * self.ratio;
        let text_x = 3.0 * self.ratio;
        let text_y = 2.0 * self.ratio;
        let graph_x = 3.0 * self.ratio;
        let graph_y = 15.0 * self.ratio;
        let graph_width = 74.0 * self.ratio;
        let graph_height = 30.0 * self.ratio;

        context.set_fill_style(&JsValue::from_str(&self.background_color));
        context.set_global_alpha(1.0);
        context.fill_rect(0.0, 0.0, width, graph_y);

        context.set_fill_style(&JsValue::from_str(&self.foreground_color));
        context
            .fill_text(&*format!("{} {}", value, self.label), text_x, text_y)
            .unwrap();

        context
            .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &self.canvas,
                graph_x + self.ratio,
                graph_y,
                graph_width - self.ratio,
                graph_height,
                graph_x,
                graph_y,
                graph_width - self.ratio,
                graph_height,
            )
            .unwrap();

        context.fill_rect(
            graph_x + graph_width - self.ratio,
            graph_y,
            self.ratio,
            graph_height,
        );

        context.set_fill_style(&JsValue::from_str(&self.background_color));
        context.set_global_alpha(0.9);
        context.fill_rect(
            graph_x + graph_width - self.ratio,
            graph_y,
            self.ratio,
            (1.0 - (value / max_value)) * graph_height,
        );
    }
}
