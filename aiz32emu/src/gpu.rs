pub struct GPU {
    vram: Vec<u32>, 
    width: usize,
    height: usize,
    command: u32,
    x: u32,
    y: u32,
    color: u32,
}

impl GPU {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            vram: vec![0; width * height],
            width,
            height,
            command: 0,
            x: 0,
            y: 0,
            color: 0,
        }
    }

    fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.vram[y * self.width + x] = color;
        }
    }
}
