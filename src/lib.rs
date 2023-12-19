use minifb::*;
#[cfg(test)]
mod test;

struct Point {
    x: f64,
    y: f64,
}
pub struct Window {
    window: minifb::Window,
    pub framebuffer: FrameBuffer,
}

impl Window {
    pub fn new(name: &str, width: usize, height: usize) -> Self {
        let mut window = minifb::Window::new(name, 
                                             width, 
                                             height, 
                                             minifb::WindowOptions::default())
                                             .expect("failed to create a window");
        let framebuffer = FrameBuffer::new(width, height);
        window.limit_update_rate(Some(std::time::Duration::from_micros(1000000 / 60)));
        Window {
            window,
            framebuffer,
        }
    }
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
    pub fn display(&mut self) {
        self.window
            .update_with_buffer(
                &self.framebuffer.data,
                self.framebuffer.width,
                self.framebuffer.height,
            )
            .unwrap();
    }
    pub fn update(&mut self) {
        self.window.update();
    }
    pub fn get_mouse_pos(&self, mode: MouseMode) -> Option<(f32, f32)> {
        self.window.get_mouse_pos(mode)
    }
    pub fn get_mouse_down(&self, button: MouseButton) -> bool {
        self.window.get_mouse_down(button)
    }
}

pub struct FrameBuffer {
    data: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![0; width * height];
        FrameBuffer {
            data,
            width,
            height,
        }
    }

    fn to_rgb(color: u32) -> (u8, u8, u8) {
        todo!();
    }

    fn from_rgb(color: (u8, u8, u8)) -> u32 {
        ((color.0 as u32) << 16) | ((color.1 as u32) << 8) | color.2 as u32
    }

    fn pixel_fits(&self, pixel_x: i64, pixel_y: i64) -> bool {
        !(pixel_y < 0
            || pixel_x < 0
            || pixel_y >= self.height as i64
            || pixel_x >= self.width as i64)
    }

    pub fn set_pixel(&mut self, x: i64, y: i64, color: (u8, u8, u8)) {
        // x:50 y:0 = 50
        // x: 50 y:2 = width*2 + 50
        if self.pixel_fits(x, y) {
            self.data[self.width * y as usize + x as usize] = Self::from_rgb(color);
        }
    }

    pub fn rectangle(&mut self, p1: (i64, i64), p2: (i64, i64), color: (u8, u8, u8)) {
        for i in p1.0..p2.0 {
            for j in p1.1..p2.1 {
                self.set_pixel(i, j, color)
            }
        }
    }

    pub fn line(&mut self, p1: (i64, i64), p2: (i64, i64), color: (u8, u8, u8)) {
        //can B in a linear function be a fraction if the arguments are intigers??
        //idk i assume so ;)

        let p1 = Point {
            x: p1.0 as f64,
            y: p1.1 as f64,
        };
        let p2 = Point {
            x: p2.0 as f64,
            y: p2.1 as f64,
        };

        let a = (p2.y - p1.y) / (p2.x - p1.x);
        let b = p1.y - (p1.x * a);

        //println!("a: {} b: {}", a, b);
        for x in p1.x as i64..p2.x as i64 {
            //println!("x: {} y: {}", x, a * x + b);
            let y = (a * x as f64 + b) as i64;

            self.set_pixel(x, y, color);
        }
    }

    pub fn parabola(
        &mut self,
        p1: (f32, f32),
        p2: (f32, f32),
        p3: (f32, f32),
        color: (u8, u8, u8),
    ) {
        let mut p = vec![p1, p2, p3];
        p.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let p1 = Point {
            x: p[0].0 as f64,
            y: p[0].1 as f64,
        };
        let p2 = Point {
            x: p[1].0 as f64,
            y: p[1].1 as f64,
        };
        let p3 = Point {
            x: p[2].0 as f64,
            y: p[2].1 as f64,
        };

        let b: f64 = ((p3.y - p2.y) * (p2.x - p1.x) * (p2.x + p1.x)
            - (p2.y - p1.y) * (p3.x + p2.x) * (p3.x - p2.x))
            / ((p3.x - p2.x) * (p2.x - p1.x) * (p1.x - p3.x));

        let a: f64 = (p2.y - p1.y - b * (p2.x - p1.x)) / (p2.x * p2.x - p1.x * p1.x);
        let c: f64 = p1.y - a * (p1.x * p1.x) - b * p1.x;

        //println!("a: {} b: {} c: {}", a, b, c);
        let mut y = p1.y as i64;
        for x in (p1.x) as i64..(p3.x) as i64 {
            let next_y = (a * ((x + 1) * (x + 1)) as f64 + b * (x + 1) as f64 + c) as i64;

            if next_y > y {
                for v_y in y..next_y {
                    self.set_pixel(x, v_y, color);
                }
            } else if next_y < y {
                for v_y in next_y..y {
                    self.set_pixel(x, v_y, color);
                }
            } else {
                self.set_pixel(x, y, color);
            }
            y = next_y;
            /*
            self.set_pixel(
                x,
                y,
                color
            );
            */
        }
    }

    pub fn triangle() {
        todo!();
    }
}
/*
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test",
        WIDTH,
        HEIGHT,
        WindowOptions::default()
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.limit_update_rate(Some(std::time::Duration::from_micros(1000000/60)));
        let mut color:u32 = 0;

        while window.is_open() {
            for i in buffer.iter_mut() {
                *i = color;
            }
            color = color + 16;
            println!("r: {} g: {} b: {}",(color << 8)>>24,(color<<16)>>24,(color<<24)>>24);

            window
                .update_with_buffer(&buffer, WIDTH, HEIGHT)
                .unwrap_or_else(|e| {
                    panic!("{}", e);
                });
*/
