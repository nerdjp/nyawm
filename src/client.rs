use crate::geometry::Geometry;
use xcb::x::{self, Window};

pub struct Client {
    pub name: String,
    pub geometry: Geometry,
    pub window: Window,
    pub border: i32,
    pub isfloating: bool,
    pub isurgent: bool,
    pub isfullscreen: bool,
}

impl Client {
    pub fn new(win: Window) -> Client {
        Client {
            name: "Test".to_string(),
            geometry: Geometry {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
            },
            window: win,
            border: 2,
            isfloating: false,
            isurgent: false,
            isfullscreen: false,
        }
    }
    pub fn hide(&self, conn: &xcb::Connection) {
        conn.send_request(&x::ConfigureWindow {
            window: self.window,
            value_list: &[
                x::ConfigWindow::X(0 - self.geometry.w),
                x::ConfigWindow::Y(0 - self.geometry.h),
                x::ConfigWindow::Width(self.geometry.w.try_into().unwrap()),
                x::ConfigWindow::Height(self.geometry.h.try_into().unwrap()),
                x::ConfigWindow::BorderWidth(self.border.try_into().unwrap()),
            ],
        });
    }

    pub fn resize(&mut self, geometry: Geometry, conn: &xcb::Connection) {
        self.geometry = geometry;
        conn.send_request(&x::ConfigureWindow {
            window: self.window,
            value_list: &[
                x::ConfigWindow::X(self.geometry.x),
                x::ConfigWindow::Y(self.geometry.y),
                x::ConfigWindow::Width(self.geometry.w.try_into().unwrap()),
                x::ConfigWindow::Height(self.geometry.h.try_into().unwrap()),
                x::ConfigWindow::BorderWidth(self.border.try_into().unwrap()),
            ],
        });
        conn.send_request(&x::ChangeWindowAttributes {
            window: self.window,
            value_list: &[
                x::Cw::BorderPixel(0xFFFFFF),
            ],
        });
    }
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.window == other.window
    }
}
