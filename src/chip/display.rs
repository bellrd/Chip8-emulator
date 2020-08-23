use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::{Window, WindowContext};

pub struct Display {
    renderer: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    pub buffer: Vec<u8>,
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl, title: &str, scale: u32, width: u32, height: u32) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, width * scale, height * scale)
            .position_centered()
            .build()
            .unwrap();

        let mut renderer = window.into_canvas().accelerated().build().unwrap();
        let texture_creator = renderer.texture_creator();
        renderer.set_scale(scale as f32, scale as f32).unwrap();
        renderer.clear();
        Self {
            renderer,
            texture_creator,
            buffer: vec![0; width as usize * height as usize * 4],
        }
    }
}

impl Display {
    // it toggle pixel and return if after toggling pixel
    pub fn set_pixel(&mut self, x: usize, y: usize) -> u8 {
        let x = if x >= 64 { x % 64 } else { x };
        let y = if y >= 32 { y % 32 } else { y };
        let position = (y * 64 + x) * 4; // Since Each pixel occupy 4 byte in vec

        // pixel is already set
        if (self.buffer[position]
            | self.buffer[position + 1]
            | self.buffer[position + 2]
            | self.buffer[position + 3])
            != 0
        {
            // unset pixel
            self.buffer[position] = 0; // unset A (alpha)
            self.buffer[position + 1] = 0; // unset R
            self.buffer[position + 2] = 0; // unset G
            self.buffer[position + 3] = 0; // unset B
            return 1;
        } else {
            // else set pixel
            self.buffer[position] = 255; //set  A (alpha)
            self.buffer[position + 1] = 100; // set R
            self.buffer[position + 2] = 255; // set G
            self.buffer[position + 3] = 100; // set B
            return 0;
        }
    }

    pub fn render(&mut self) {
        //self.renderer.set_draw_color(Color::BLACK);
        self.renderer.clear();

        let surface = Surface::from_data(
            self.buffer.as_mut(),
            64,
            32,
            64 * 4,
            PixelFormatEnum::ARGB8888,
        )
        .unwrap();
        let texture = self
            .texture_creator
            .create_texture_from_surface(surface)
            .unwrap();
        self.renderer.copy(&texture, None, None).unwrap();
        self.renderer.present();
    }

    pub fn clear(&mut self) {
        self.buffer = vec![0; self.buffer.len()];
        self.renderer.set_draw_color(Color::BLACK);
        self.renderer.clear();
        self.renderer.present();
    }
}
