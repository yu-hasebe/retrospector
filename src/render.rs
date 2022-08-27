use std::rc::Rc;

/// draw_image depicts a given sprite at a specified location on the canvas.
pub fn draw_image(renderer: &Renderer, sprite: Sprite, location: Location) {
    let is_outside_of_canvas = location.dx() + sprite.width() < 0.0
        || location.dx() > renderer.canvas_width()
        || location.dy() + sprite.height() < 0.0
        || location.dy() > renderer.canvas_height();
    if is_outside_of_canvas {
        return;
    }

    renderer
        .context()
        .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &sprite.atlas(),
            sprite.sx(),
            sprite.sy(),
            sprite.width(),
            sprite.height(),
            location.dx(),
            location.dy(),
            sprite.width(),
            sprite.height(),
        )
        .unwrap();
}

/// clear clears the canvas.
pub fn clear(renderer: &Renderer) {
    renderer
        .context()
        .clear_rect(0.0, 0.0, renderer.canvas_width(), renderer.canvas_height());
}

/// Renderer is responsible for depiction on the canvas.
#[derive(Debug)]
pub struct Renderer {
    context: web_sys::CanvasRenderingContext2d,
    canvas_width: f64,
    canvas_height: f64,
}

impl Renderer {
    pub(crate) fn new(
        context: web_sys::CanvasRenderingContext2d,
        canvas_width: f64,
        canvas_height: f64,
    ) -> Self {
        Self {
            context,
            canvas_width,
            canvas_height,
        }
    }

    /// context enalbes you to use the rendering context on the canvas.
    pub fn context(&self) -> &web_sys::CanvasRenderingContext2d {
        &self.context
    }

    /// canvas_width enalbes you to refer to that.
    pub fn canvas_width(&self) -> f64 {
        self.canvas_width
    }

    /// canvas_height enalbes you to refer to that.
    pub fn canvas_height(&self) -> f64 {
        self.canvas_height
    }
}

/// Sprite is responsible for representing a sprite.
#[derive(Debug)]
pub struct Sprite {
    atlas: Rc<web_sys::HtmlImageElement>,
    sx: f64,
    sy: f64,
    width: f64,
    height: f64,
}

impl Sprite {
    fn new(
        atlas: Rc<web_sys::HtmlImageElement>,
        sx: f64,
        sy: f64,
        width: f64,
        height: f64,
    ) -> Self {
        Self {
            atlas,
            sx,
            sy,
            width,
            height,
        }
    }

    /// atlas is a set of sprites.
    pub fn atlas(&self) -> Rc<web_sys::HtmlImageElement> {
        Rc::clone(&self.atlas)
    }

    /// sx is a source x on the atlas.
    pub fn sx(&self) -> f64 {
        self.sx
    }

    /// sy is a source y on the atlas.
    pub fn sy(&self) -> f64 {
        self.sy
    }

    /// width is a length from sx on the atlas.
    pub fn width(&self) -> f64 {
        self.width
    }

    /// height is a length from sy on the atlas.
    pub fn height(&self) -> f64 {
        self.height
    }
}

/// SpriteBuilder builds Sprites with the same atlas, width, and height.
#[derive(Debug)]
pub struct SpriteBuilder {
    atlas: Rc<web_sys::HtmlImageElement>,
    width: f64,
    height: f64,
}

impl SpriteBuilder {
    /// new returns an instantiated SpriteBuilder
    pub fn new(bytes: &[u8], extension: &str, width: f64, height: f64) -> Self {
        let html_image_element = web_sys::HtmlImageElement::new().unwrap();
        let src = format!(
            "data:image/{};base64,{}",
            extension,
            base64::encode(&bytes.to_vec())
        );
        html_image_element.set_src(&src);
        let atlas = Rc::new(html_image_element);
        Self {
            atlas,
            width,
            height,
        }
    }

    /// sprite returns a specified Sprite on the atlas.
    pub fn sprite(&self, col: i32, row: i32) -> Sprite {
        let sx = col as f64 * self.width;
        let sy = row as f64 * self.height;
        Sprite::new(Rc::clone(&self.atlas), sx, sy, self.width, self.height)
    }
}

/// Location is responsible for specifing a location on a canvas.
#[derive(Debug)]
pub struct Location {
    dx: f64,
    dy: f64,
}

impl Location {
    /// new returns an initialized Location.
    pub fn new(dx: f64, dy: f64) -> Self {
        Self { dx, dy }
    }

    /// dx is a differential x from the left on the canvas.
    pub fn dx(&self) -> f64 {
        self.dx
    }

    /// dy is a differential y from the top on the canvas.
    pub fn dy(&self) -> f64 {
        self.dy
    }
}
