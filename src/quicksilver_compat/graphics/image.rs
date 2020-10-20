use web_sys::{HtmlImageElement, WebGlRenderingContext, WebGlTexture};

use crate::{
    quicksilver_compat::geom::{Rectangle, Transform, Vector},
    ErrorMessage, JsError, PaddleResult,
};
use std::{error::Error, fmt, io::Error as IOError, path::Path, rc::Rc};

///Pixel formats for use with loading raw images
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PixelFormat {
    /// Red, Green, and Blue
    RGB,
    /// Red, Green, Blue, and Alpha
    RGBA,
}

#[derive(Clone, Debug)]
///An image that can be drawn to the screen
pub struct Image {
    source: Rc<ImageData>,
    region: Rectangle,
}

#[derive(Debug)]
pub struct ImageData {
    pub tex: WebGlTexture,
    pub el: HtmlImageElement,
    pub width: u32,
    pub height: u32,
}

// impl Drop for ImageData {
//     fn drop(&mut self) {
//         unsafe { instance().destroy_texture(self) };
//     }
// }

impl ImageData {
    fn new(gl: &WebGlRenderingContext, el: HtmlImageElement) -> PaddleResult<Self> {
        let width = el.width();
        let height = el.height();
        let tex = gl.create_texture().ok_or(ErrorMessage::technical(
            "Failed to create texture".to_owned(),
        ))?;
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&tex));
        // JS equivalent: texImage2D()
        gl.tex_image_2d_with_u32_and_u32_and_image(
            WebGlRenderingContext::TEXTURE_2D,
            0,
            WebGlRenderingContext::RGBA as i32,
            WebGlRenderingContext::RGBA,
            WebGlRenderingContext::UNSIGNED_BYTE,
            &el,
        )
        .map_err(JsError::from_js_value)?;
        Ok(Self {
            tex,
            el,
            width,
            height,
        })
    }
}

impl Image {
    pub async fn load(gl: &WebGlRenderingContext, src: &str) -> PaddleResult<Self> {
        // Let the browser handle the image loading
        let el = HtmlImageElement::new().map_err(JsError::from_js_value)?;
        el.set_src(src);
        // asynchronously load data and block the future
        let promise = el.decode();
        wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .map_err(JsError::from_js_value)?;

        // When the image is ready, create a WebGL texture from it
        let data = ImageData::new(gl, el)?;

        let region = Rectangle::new_sized((128, 128));
        Ok(Image {
            source: Rc::new(data),
            region,
        })
    }
    // pub(crate) fn new(data: ImageData) -> Image {
    //     let region = Rectangle::new_sized((data.width, data.height));
    //     Image {
    //         source: Rc::new(data),
    //         region
    //     }
    // }

    // /// Start loading a texture from a given path
    // pub fn load<P: AsRef<Path>>(path: P) -> impl Future<Item = Image, Error = QuicksilverError> {
    //     load_file(path)
    //         .map(|data| Image::from_bytes(data.as_slice()))
    //         .and_then(future::result)
    // }

    // pub(crate) fn new_null(width: u32, height: u32, format: PixelFormat) -> Result<Image> {
    //     Image::from_raw(&[], width, height, format)
    // }

    // /// Load an image from pixel values in a byte array
    // pub fn from_raw(data: &[u8], width: u32, height: u32, format: PixelFormat) -> Result<Image> {
    //     Ok(unsafe {
    //         Image::new(instance().create_texture(data, width, height, format)?)
    //     })
    // }

    // /// Load an image directly from an encoded byte array
    // pub fn from_bytes(raw: &[u8]) -> Result<Image> {
    //     let img = image::load_from_memory(raw)?.to_rgba();
    //     let width = img.width();
    //     let height = img.height();
    //     Image::from_raw(img.into_raw().as_slice(), width, height, PixelFormat::RGBA)
    // }

    // pub(crate) fn get_id(&self) -> u32 {
    //     self.source.id
    // }

    pub(crate) fn source_width(&self) -> u32 {
        self.source.width
    }

    pub(crate) fn source_height(&self) -> u32 {
        self.source.height
    }

    ///The area of the source image this subimage takes up
    pub fn area(&self) -> Rectangle {
        self.region
    }

    ///Find a subimage of a larger image
    pub fn subimage(&self, rect: Rectangle) -> Image {
        Image {
            source: self.source.clone(),
            region: Rectangle::new(
                (
                    self.region.pos.x + rect.pos.x,
                    self.region.pos.y + rect.pos.y,
                ),
                (rect.width(), rect.height()),
            ),
        }
    }

    /// Create a projection matrix for a given region onto the Image
    pub fn projection(&self, region: Rectangle) -> Transform {
        let source_size: Vector = (self.source_width(), self.source_height()).into();
        let recip_size = source_size.recip();
        let normalized_pos = self.region.top_left().times(recip_size);
        let normalized_size = self.region.size().times(recip_size);
        Transform::translate(normalized_pos)
            * Transform::scale(normalized_size)
            * Transform::scale(region.size().recip())
            * Transform::translate(-region.top_left())
    }

    pub(crate) fn texture(&self) -> &WebGlTexture {
        &self.source.tex
    }
}

#[derive(Debug)]
///An error generated while loading an image
pub enum ImageError {
    // /// There was an error decoding the bytes of the image
    // DecodingError(image::ImageError),
    ///There was some error reading the image file
    IOError(IOError),
}

#[doc(hidden)]
impl From<IOError> for ImageError {
    fn from(err: IOError) -> ImageError {
        ImageError::IOError(err)
    }
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ImageError {
    fn description(&self) -> &str {
        match self {
            // &ImageError::DecodingError(ref err) => err.description(),
            &ImageError::IOError(ref err) => err.description(),
        }
    }
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            // &ImageError::DecodingError(ref err) => Some(err),
            &ImageError::IOError(ref err) => Some(err),
        }
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source) && self.region == other.region
    }
}
impl Eq for Image {}
