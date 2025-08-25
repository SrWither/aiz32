use aiz32core::peripheral::Peripheral;

pub struct GPU {
    pub width: usize,
    pub height: usize,
    pub front_buffer: Vec<u32>,
    pub back_buffer: Vec<u32>,
    pub frame_dirty: bool,

    pub command: u32,
    pub x: u32,
    pub y: u32,
    pub color: u32,
    pub color_end: u32,
    pub color_mid: u32,
    pub tile_index: u32,
    pub angle: u32,
    pub w: u32,
    pub h: u32,
    pub rom: Vec<u32>,
}

impl GPU {
    pub fn new(width: usize, height: usize, rom: Vec<u32>) -> Self {
        Self {
            width,
            height,
            front_buffer: vec![0; width * height],
            back_buffer: vec![0; width * height],
            frame_dirty: false,
            command: 0,
            x: 0,
            y: 0,
            color: 0,
            color_end: 0,
            color_mid: 0,
            tile_index: 0,
            angle: 90,
            w: 0,
            h: 0,
            rom,
        }
    }

    pub fn framebuffer(&self) -> &[u32] {
        &self.front_buffer
    }

    pub fn present(&mut self) {
        if self.frame_dirty {
            self.front_buffer.copy_from_slice(&self.back_buffer);
            self.frame_dirty = false;
        }
    }
    

    fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.back_buffer[y * self.width + x] = color;
            self.frame_dirty = true;
        }
    }

    fn fill_rect(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
        for y in y0..=y1.min(self.height - 1) {
            for x in x0..=x1.min(self.width - 1) {
                self.back_buffer[y * self.width + x] = color;
            }
        }
        self.frame_dirty = true;
    }

    fn fill_linear_gradient(&mut self, color_start: u32, color_end: u32, _angle: u32) {
        for y in 0..self.height {
            let t = y as f32 / (self.height - 1) as f32;
            let color = lerp_color(color_start, color_end, t);
            let row_start = y * self.width;
            let row_end = row_start + self.width;
            for pixel in &mut self.back_buffer[row_start..row_end] {
                *pixel = color;
            }
        }
        self.frame_dirty = true;
    }

    fn fill_multi_linear_gradient(&mut self, color_start: u32, color_mid: u32, color_end: u32, _angle: u32) {
        for y in 0..self.height {
            let t = y as f32 / (self.height - 1) as f32;
            let color = if t < 0.5 {
                lerp_color(color_start, color_mid, t * 2.0)
            } else {
                lerp_color(color_mid, color_end, (t - 0.5) * 2.0)
            };
            let row_start = y * self.width;
            let row_end = row_start + self.width;
            for pixel in &mut self.back_buffer[row_start..row_end] {
                *pixel = color;
            }
        }
        self.frame_dirty = true;
    }

    fn fill_radial_gradient(&mut self, cx: usize, cy: usize, color_center: u32, color_outer: u32) {
        let mut max_r = 0.0;
        for &(px, py) in &[(0, 0), (self.width - 1, 0), (0, self.height - 1), (self.width - 1, self.height - 1)] {
            let dx = px as f32 - cx as f32;
            let dy = py as f32 - cy as f32;
            let r = (dx*dx + dy*dy).sqrt();
            if r > max_r { max_r = r; }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let dx = x as f32 - cx as f32;
                let dy = y as f32 - cy as f32;
                let r = (dx*dx + dy*dy).sqrt();
                let t = if max_r != 0.0 { (r / max_r).clamp(0.0, 1.0) } else { 0.0 };
                self.back_buffer[y * self.width + x] = lerp_color(color_center, color_outer, t);
            }
        }
        self.frame_dirty = true;
    }

    fn draw_tile(&mut self, tile_index: usize, x0: usize, y0: usize) {
        let tile_w = if self.w == 0 { 8 } else { self.w as usize };
        let tile_h = if self.h == 0 { 8 } else { self.h as usize };
        let start = tile_index * tile_w * tile_h;
    
        for ty in 0..tile_h {
            for tx in 0..tile_w {
                let rom_idx = start + ty * tile_w + tx;
                if rom_idx < self.rom.len() {
                    let color = self.rom[rom_idx];
                    self.draw_pixel(x0 + tx, y0 + ty, color);
                }
            }
        }
    }
    
    pub fn execute_command(&mut self) {
        match self.command {
            1 => self.draw_pixel(self.x as usize, self.y as usize, self.color),
            2 => {
                let x1 = if self.w == 0 { self.width - 1 } else { (self.x + self.w - 1) as usize };
                let y1 = if self.h == 0 { self.height - 1 } else { (self.y + self.h - 1) as usize };
                self.fill_rect(self.x as usize, self.y as usize, x1, y1, self.color);
            }
            3 => self.fill_linear_gradient(self.color, self.color_end, self.angle),
            4 => self.draw_tile(self.tile_index as usize, self.x as usize, self.y as usize),
            5 => self.fill_radial_gradient(self.x as usize, self.y as usize, self.color, self.color_end),
            6 => self.fill_multi_linear_gradient(self.color, self.color_mid, self.color_end, self.angle),
            _ => {}
        }
    }
}

fn lerp_color(c0: u32, c1: u32, t: f32) -> u32 {
    let r0 = ((c0 >> 16) & 0xFF) as f32;
    let g0 = ((c0 >> 8) & 0xFF) as f32;
    let b0 = (c0 & 0xFF) as f32;

    let r1 = ((c1 >> 16) & 0xFF) as f32;
    let g1 = ((c1 >> 8) & 0xFF) as f32;
    let b1 = (c1 & 0xFF) as f32;

    let r = (r0 + (r1 - r0) * t) as u32;
    let g = (g0 + (g1 - g0) * t) as u32;
    let b = (b0 + (b1 - b0) * t) as u32;

    (r << 16) | (g << 8) | b
}

impl Peripheral for GPU {
    fn handles_port(&self, port: u16) -> bool {
        (0x2000..=0x20FF).contains(&port)
    }

    fn read(&self, port: u16) -> u32 {
        match port {
            0x2000 => self.command,
            0x2001 => self.x,
            0x2002 => self.y,
            0x2003 => self.color,
            0x2004 => self.color_end,
            0x2005 => self.tile_index,
            0x2006 => self.angle,
            0x2007 => self.color_mid,
            0x2008 => self.w,
            0x2009 => self.h,
            _ => 0,
        }
    }

    fn write(&mut self, port: u16, value: u32) {
        match port {
            0x2000 => {
                self.command = value;
                self.execute_command();
            }
            0x2001 => self.x = value,
            0x2002 => self.y = value,
            0x2003 => self.color = value,
            0x2004 => self.color_end = value,
            0x2005 => self.tile_index = value,
            0x2006 => self.angle = value,
            0x2007 => self.color_mid = value,
            0x2008 => self.w = value,
            0x2009 => self.h = value,
            _ => {}
        }
    }
}
