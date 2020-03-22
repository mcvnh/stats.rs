//! Webassembly Performance Monitor
//!
//! Stats is a webassembly version of [stats.js](https://github.com/mrdoob/stats.js) written in Rust.
//!
//! This crate provides a simple info box that will help you monitor your code performance,
//! and can be used either Javascript code or in Rust code.
//!
//! * **FPS** Frames rendered in the last second. The higher the number the better.
//! * **MS** Milliseconds needed to render a frame. The lower the number the better.
//!
//! #### Examples
//!
//! In Javascript
//! ```js
//! const wasm = import('./pkg/stats_rs');
//!
//! wasm.then(({ Stats }) => {
//!   const stats = Stats.init();
//!   stats.attach(document.body);
//!
//!   function render() {
//!     stats.update();
//!     requestAnimationFrame(render);
//!   }
//!
//!   render();
//! });
//! ```
//!
//! ### Rust
//! ```
//! // TODO
//! ```
//!

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

mod monitor;
mod panel;

/// The Stats struct represents for the monitor graphs
#[wasm_bindgen]
pub struct Stats {
    container: HtmlElement,
    fps: panel::Panel,
    ms: panel::Panel,
    monitor: monitor::CanvasPerformance,
}

#[wasm_bindgen]
impl Stats {
    /// Constructs a Stats
    ///
    /// When the init method is called, the function itself will create a div contains two panels,
    /// one for FPS and the another for MS; however, the div does not attached or inserted to any dom
    /// until the `attach` method called.
    pub fn init() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let container = document
            .create_element("div")
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();

        container.style().set_css_text(
            r#"
      position: fixed;
      top: 0;
      left: 0;
      cursor: pointer;
      opacity: 0.9;
      z-index: 999;
    "#,
        );

        let fps = panel::Panel::create(
            &window,
            "FPS".to_string(),
            "#0ff".to_string(),
            "#002".to_string(),
        );
        let ms = panel::Panel::create(
            &window,
            "MS".to_string(),
            "#0f0".to_string(),
            "#020".to_string(),
        );
        let monitor = monitor::CanvasPerformance::init(&window);

        container.append_child(&fps.canvas).unwrap();
        container.append_child(&ms.canvas).unwrap();

        Self {
            container,
            fps,
            ms,
            monitor,
        }
    }

    /// Updates the newest value of FPS and MS
    ///
    /// This method should be called/executed in the render function where the `requestAnimationFrame` function called
    ///
    /// ```
    /// function render() {
    ///   stats.update();
    ///   requestAnimationFrame(render);
    /// }
    /// ```
    ///
    pub fn update(&mut self) {
        self.monitor.recalculate();

        self.fps.update(self.monitor.fps, 100.0);
        self.ms.update(self.monitor.ms, 200.0);
    }

    /// Attachs me at the end of a specific DOM
    ///
    /// ```
    /// stats.attach(document.body);
    /// ```
    ///
    pub fn attach(&mut self, dom: &HtmlElement) {
        dom.append_child(&self.container)
            .expect("unable to attach the stats to the DOM");
    }

    /// Detachs me from a specific DOM
    ///
    /// ```
    /// stats.detach(document.body);
    /// ```
    ///
    pub fn detach(&mut self, dom: &HtmlElement) {
        dom.remove_child(&self.container)
            .expect("unable to detach the stats from the DOM");
    }
}
