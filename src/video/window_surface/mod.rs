pub use error::Error;
pub use event::{WindowEvent, WindowEventIterator};

use sdl3::{
    Sdl,
    pixels::PixelFormat,
    render::{Texture, TextureAccess, TextureCreator},
    video::WindowContext,
};

use crate::video::{Image, Surface};

pub mod error;
pub mod event;

pub struct WindowSurface {
    _sdl: Sdl,
    event_pump: sdl3::EventPump,
    _video: sdl3::VideoSubsystem,
    canvas: sdl3::render::Canvas<sdl3::video::Window>,
    texture_creator: TextureCreator<WindowContext>,
    size: [u32; 2],
}

impl WindowSurface {
    pub fn new(initial_size: [u32; 2]) -> Result<Self, Error> {
        let sdl = sdl3::init()?;
        let video = sdl.video()?;
        let canvas = video
            .window("Fraya", initial_size[0], initial_size[1])
            .build()
            .map_err(|e| Error::Sdl3(format!("{}", e)))?
            .into_canvas();
        let texture_creator = canvas.texture_creator();
        let event_pump = sdl.event_pump()?;

        Ok(Self {
            _sdl: sdl,
            _video: video,
            canvas,
            event_pump: event_pump,
            texture_creator,
            size: initial_size,
        })
    }

    pub fn event_iter(&mut self) -> WindowEventIterator<'_> {
        WindowEventIterator {
            inner: self.event_pump.poll_iter(),
        }
    }
}

impl Surface for WindowSurface {
    fn update_image(&mut self, img: &Image) -> Result<(), Box<dyn std::error::Error>> {
        // Resize if not the right size
        if self.size != img.size {
            self.canvas
                .window_mut()
                .set_size(img.size[0], img.size[1])?;
            self.size = img.size;
        }

        // TODO: Creating every time here may cost us quite a bit so rethink.
        let mut t = self.texture_creator.create_texture(
            Some(PixelFormat::BGRA8888),
            TextureAccess::Streaming,
            self.size[0],
            self.size[1],
        )?;

        t.with_lock(None, |pixels, _| {
            for (i, p) in img.pixels.iter().enumerate() {
                pixels[i * 4 + 0] = p.a;
                pixels[i * 4 + 1] = p.r;
                pixels[i * 4 + 2] = p.g;
                pixels[i * 4 + 3] = p.b;
            }
        })?;

        self.canvas.copy(&t, None, None)?;
        self.canvas.present();

        Ok(())
    }
}
