use crate::{rect::Rect, workspace::Workspace};
use xcb::{x::Window, xinerama};

pub struct Monitor {
    pub number: i32,
    pub geometry: Rect,
    pub window_area: Rect,
    pub mfact: f32,
    pub workspaces: Vec<Workspace>,
    pub sel_workspace: i32,
    //pub layout: Layout,
}

impl Monitor {
    fn new(num: i32, geometry: Rect, window_area: Rect) -> Monitor {
        Monitor {
            number: num,
            geometry: geometry,
            window_area: window_area,
            mfact: 0.55,
            workspaces: Vec::new(),
            sel_workspace: -1,
        }
    }

    pub fn update_geometry(monitors: Vec<Monitor>, conn: &xcb::Connection) -> Vec<Monitor> {
        let screens = conn
            .wait_for_reply(conn.send_request(&xinerama::QueryScreens {}))
            .unwrap();
        let mut monitors: Vec<Monitor> = Vec::new();
        let mut i = 0;
        for screen in screens.screen_info() {
            println!(
                "x: {}, y: {}, w: {}, h: {}",
                screen.x_org, screen.y_org, screen.width, screen.height
            );
            let geometry = Rect {
                x: screen.x_org as i32,
                y: screen.y_org as i32,
                w: screen.width as i32,
                h: screen.height as i32,
            };
            let window_area = Rect {
                x: screen.x_org as i32,
                y: screen.y_org as i32,
                w: screen.width as i32,
                h: screen.height as i32,
            };
            monitors.push(Monitor::new(i, geometry, window_area));
            i += 1;
        }
        monitors
    }
    pub fn get_selected_workspace(&mut self) -> &mut Workspace {
        &mut self.workspaces[self.sel_workspace as usize]
    }
    pub fn select_workspace(&mut self, index: i32, conn: &xcb::Connection) {
        self.workspaces[self.sel_workspace as usize].unfocus(conn);
        self.sel_workspace = index;
        self.get_selected_workspace().focus(conn);
    }
}
