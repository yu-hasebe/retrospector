use std::rc::Rc;

use anyhow::{anyhow, ensure, Context, Result};

/// draw_image depicts a given sprite at a specified position on the canvas.
pub fn draw_image(renderer: &Renderer, sprite: &Sprite, position: Position) -> Result<()> {
    ensure!(
        0.0 <= position.dx() + sprite.width()
            && position.dx() <= renderer.canvas_width()
            && 0.0 <= position.dy() + sprite.height()
            && position.dy() <= renderer.canvas_height(),
        "the sprite to draw is out of canvas"
    );

    renderer
        .context()
        .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &sprite.atlas(),
            sprite.sx(),
            sprite.sy(),
            sprite.width(),
            sprite.height(),
            position.dx(),
            position.dy(),
            sprite.width(),
            sprite.height(),
        )
        .map_err(|e| anyhow!("failed to draw image: {:?}", e))?;

    Ok(())
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

/// SpriteStore builds Sprites with the same atlas, width, and height.
#[derive(Debug)]
pub struct SpriteStore {
    store: Vec<Sprite>,
    width_in_tile: usize,
    height_in_tile: usize,
}

impl SpriteStore {
    /// new returns an instantiated SpriteStore
    pub fn new(
        bytes: &[u8],
        extension: &str,
        width: u32,
        height: u32,
        tile_width: u32,
        tile_height: u32,
    ) -> Result<Self> {
        ensure!(
            width % tile_width == 0,
            "width: {} should be divisible by tile_width: {}",
            width,
            tile_width,
        );

        ensure!(
            height % tile_height == 0,
            "height: {} should be divisible by tile_height: {}",
            height,
            tile_height
        );

        let html_image_element = web_sys::HtmlImageElement::new()
            .map_err(|e| anyhow!("failed to create a new html image element: {:?}", e))?;
        let src = format!(
            "data:image/{};base64,{}",
            extension,
            base64::encode(&bytes.to_vec())
        );
        html_image_element.set_src(&src);
        let atlas = Rc::new(html_image_element);

        let width_in_tile = width / tile_width;
        let height_in_tile = height / tile_height;
        let mut store = vec![];
        for y in 0..height_in_tile {
            for x in 0..width_in_tile {
                let sprite = Sprite::new(
                    Rc::clone(&atlas),
                    (x * tile_width) as f64,
                    (y * tile_height) as f64,
                    tile_width as f64,
                    tile_height as f64,
                );
                store.push(sprite);
            }
        }

        Ok(Self {
            store,
            width_in_tile: width_in_tile as usize,
            height_in_tile: height_in_tile as usize,
        })
    }

    /// sprite returns a specified Sprite on the atlas.
    pub fn sprite(&self, index: usize) -> Result<&Sprite> {
        self.store.get(index).with_context(|| {
            format!(
                "failed to get the sprite from the atlas(width_in_tile: {}, height_in_tile: {}) by index: {}",
                self.width_in_tile, self.height_in_tile, index
            )
        })
    }

    /// sprite_by_col_and_row returns a specified Sprite on the atlas.
    pub fn sprite_by_col_and_row(&self, col: usize, row: usize) -> Result<&Sprite> {
        ensure!(
            col < self.width_in_tile,
            "col: {} should be less than width_in_tile: {}",
            col,
            self.width_in_tile
        );
        ensure!(
            row < self.height_in_tile,
            "row: {} should be less than height_in_tile: {}",
            row,
            self.height_in_tile,
        );
        let index = col + row * self.width_in_tile;
        self.sprite(index)
    }
}

/// Position is responsible for specifing a position on a canvas.
#[derive(Debug)]
pub struct Position {
    dx: f64,
    dy: f64,
}

impl Position {
    /// new returns an initialized Position.
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
