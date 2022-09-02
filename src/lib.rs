//! Retrospector is a super simple 2D game engine for WebAssembly.
//! The minimum implementation is the following:
//! ```
//! use wasm_bindgen::prelude::*;
//!
//! use retrospector::app::{run, App, AppConfig};
//! use retrospector::render::{clear, draw_image, Location, Renderer, SpriteStore};
//! use retrospector::update::KeyEvent;
//!
//! #[wasm_bindgen(start)]
//! pub fn start() -> Result<(), JsValue> {
//!     let app = TestMock::new();
//!     let config = AppConfig::new(String::from("canvas"), 352.0, 352.0);
//!     run(app, config)
//! }
//!
//! struct TestMock {
//!     elapsed_time: f64,
//!     text: String,
//!     sprite_store: SpriteStore,
//! }
//!
//! impl TestMock {
//!     fn new() -> Self {
//!         let mock_bytes = vec![];
//!         let sprite_store = SpriteStore::new(&mock_bytes, "gif", 640, 640, 32, 32).unwrap();
//!         Self {
//!             elapsed_time: 0.0,
//!             text: String::from("test hello"),
//!             sprite_store,
//!         }
//!     }
//! }
//!
//! // Implement App trait for your game objects.
//! impl App for TestMock {
//!     fn update(&mut self, elapsed_time: f64, key_event: &KeyEvent) {
//!         self.elapsed_time = elapsed_time;
//!         if key_event.is_arrow_right_down() {
//!             self.text += "->";
//!         }
//!         if key_event.is_enter_down() {
//!             self.text = String::from("test hello");
//!         }
//!     }
//!
//!     fn render(&self, renderer: &Renderer) {
//!         // Before rendering, clear the canvas first.
//!         clear(renderer);
//!
//!         // You can get a sprite from a sprite_store.
//!         if let Ok(sprite) = self.sprite_store.sprite(0, 0) {
//!             // You can use the draw_image to draw sprites at the specified location on the canvas.
//!             draw_image(renderer, &sprite, Location::new(0.0, 0.0));
//!         }
//!
//!         let text = format!("elapsed time: {} ms", self.elapsed_time);
//!         // You can use your own rendering functions.
//!         fill_text(renderer, &text, Location::new(0.0, 50.0));
//!         fill_text(renderer, &self.text, Location::new(0.0, 100.0));
//!     }
//! }
//!
//! // You can define your own rendering functions like this.
//! fn fill_text(renderer: &Renderer, text: &str, location: Location) {
//!     renderer
//!         .context()
//!         .fill_text(text, location.dx(), location.dy())
//!         .unwrap();
//! }
//! ```

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

/// app is a core module.
/// It has App trait. Implement the App trait for your game objects.
/// It also has run function. That is an entry point for starting the game.
pub mod app;

/// render is about rendering module.
pub mod render;

/// update is about data-updating module. It is almost about KeyEvent.
pub mod update;
