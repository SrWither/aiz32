pub struct Register {
    pub value: u32,
}

impl Register {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn from(value: u32) -> Self {
        Self { value }
    }
}

pub struct RegisterBank {
    pub general: [Register; 32],
    pub pc: Register,
    pub sp: Register,
    pub flags: Register,
    pub zero: Register,
    pub fregs: [f32; 32],
}

impl RegisterBank {
    pub fn new(pc: u32, sp: u32) -> Self {
        Self {
            general: std::array::from_fn(|_| Register::new()),
            pc: Register::from(pc),
            sp: Register::from(sp),
            flags: Register::new(),
            zero: Register::new(),
            fregs: [0.0; 32],
        }
    }

    #[inline]
    pub fn get(&self, idx: u8) -> u32 {
        let i = idx as usize;
        debug_assert!(i < self.general.len(), "register index out of bounds");
        self.general[i].value
    }

    #[inline]
    pub fn set(&mut self, idx: u8, val: u32) {
        if idx == 0 {
            return;
        }
        let i = idx as usize;
        debug_assert!(i < self.general.len(), "register index out of bounds");
        self.general[i].value = val;
    }

    #[inline]
    pub fn pc(&self) -> u32 {
        self.pc.value
    }
    #[inline]
    pub fn set_pc(&mut self, v: u32) {
        self.pc.value = v
    }

    #[inline]
    pub fn sp(&self) -> u32 {
        self.sp.value
    }
    #[inline]
    pub fn set_sp(&mut self, v: u32) {
        self.sp.value = v
    }

    #[inline]
    pub fn flags(&self) -> u32 {
        self.flags.value
    }
    #[inline]
    pub fn set_flags(&mut self, v: u32) {
        self.flags.value = v
    }

    #[inline]
    pub fn fget(&self, idx: u8) -> f32 {
        let i = idx as usize;
        debug_assert!(i < self.fregs.len(), "floating register index out of bounds");
        self.fregs[i]
    }

    #[inline]
    pub fn fset(&mut self, idx: u8, val: f32) {
        let i = idx as usize;
        debug_assert!(i < self.fregs.len(), "floating register index out of bounds");
        self.fregs[i] = val;
    }
}

