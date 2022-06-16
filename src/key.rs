use xcb::x::{ModMask, Keysym};

pub enum Arg {
    i(i32),
    f(f32),
    b(bool),
    s(String)
}

pub struct Keybind {
    pub modifier: ModMask,
    pub keysym: Keysym,
    pub func: Box<dyn FnMut(&mut Monitor, &Arg) -> ()>,
    pub arg: Arg,
}

impl Keybind {
    pub fn exec(&mut self) {
        (self.func)(&self.arg);
    }
}
