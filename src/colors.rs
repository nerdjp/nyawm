static COLORS: Colors = Colors {
    active_border: 0xFFFFFF,
    inactive_border: 0x000000,
};
pub struct Colors {
    pub active_border: i32,
    pub inactive_border: i32,
}

pub fn get_colors() -> Colors {
    COLORS
}
