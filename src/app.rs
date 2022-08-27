use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::render::Renderer;
use crate::update::KeyEvent;

/// App trait should be implemented by all game objects.
pub trait App {
    fn update(&mut self, elapsed_time: f64, key_event: &KeyEvent);
    fn render(&self, renderer: &Renderer);
}

/// run is an entry point for starting the game.
pub fn run<T: App + 'static>(app: T, config: AppConfig) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let shared_key_event = Rc::new(RefCell::new(KeyEvent::new()));
    {
        let keydown_event = Rc::clone(&shared_key_event);
        let keydown_handler = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            keydown_event.borrow_mut().update_on_keydown(event);
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback(
            "keydown",
            keydown_handler.as_ref().unchecked_ref(),
        )?;
        keydown_handler.forget();
    }
    {
        let keyup_event = Rc::clone(&shared_key_event);
        let keyup_handler = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            keyup_event.borrow_mut().update_on_keyup(event);
        }) as Box<dyn FnMut(_)>);
        document
            .add_event_listener_with_callback("keyup", keyup_handler.as_ref().unchecked_ref())?;
        keyup_handler.forget();
    }

    let canvas = document
        .get_element_by_id(&config.canvas_id)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_width(config.canvas_width as u32);
    canvas.set_height(config.canvas_height as u32);
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let renderer = Renderer::new(context, config.canvas_width, config.canvas_height);

    let shared_app = Rc::new(RefCell::new(app));
    let f = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);
    {
        let app_cloned = Rc::clone(&shared_app);
        g.replace(Some(Closure::wrap(Box::new(move |time: f64| {
            app_cloned
                .borrow_mut()
                .update(time, &shared_key_event.borrow());
            app_cloned.borrow().render(&renderer);
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut(f64)>)));
        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

/// AppConfig is a configuration for starting the game.
pub struct AppConfig {
    canvas_id: String,
    canvas_width: f64,
    canvas_height: f64,
}

impl AppConfig {
    /// new returns an instantiated AppConfig.
    pub fn new(canvas_id: String, canvas_width: f64, canvas_height: f64) -> Self {
        Self {
            canvas_id,
            canvas_width,
            canvas_height,
        }
    }
}
