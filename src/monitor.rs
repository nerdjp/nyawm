use crate::{geometry::Geometry, workspace::Workspace, client::Client};
use xcb::xinerama;

pub struct Monitor {
    pub number: i32,
    pub geometry: Geometry,
    pub window_area: Geometry,
    pub workspaces: Vec<Workspace>,
    pub sel_workspace: Option<usize>,
}

impl Monitor {
    fn new(num: i32, geometry: Geometry, window_area: Geometry) -> Monitor {
        Monitor {
            number: num,
            geometry,
            window_area,
            workspaces: Vec::new(),
            sel_workspace: Option::None,
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
            let geometry = Geometry {
                x: screen.x_org as i32,
                y: screen.y_org as i32,
                w: screen.width as i32,
                h: screen.height as i32,
            };
            monitors.push(Monitor::new(i, geometry, geometry));
            i += 1;
        }
        monitors
    }

    pub fn get_selected_workspace(&mut self) -> &mut Workspace {
        match self.sel_workspace {
            None => {
                if self.workspaces.is_empty() {
                    self.workspaces.push(Workspace::new("".to_string()));
                    self.sel_workspace = Some(0);

                    self.workspaces.last_mut().unwrap()
                } else {
                    self.sel_workspace = Some(self.workspaces.len() - 1);
                    self.workspaces.last_mut().unwrap()
                }
            }
            Some(index) => {
                self.workspaces.get_mut(index).unwrap()
            }
        }
    }

    pub fn get_workspace_by_index(&mut self, index: usize) -> Option<&mut Workspace> {
        self.workspaces.get_mut(index)
    }

    pub fn get_workspace_by_name(&self, name: String) -> &mut Workspace {
        todo!();
    }

    pub fn swap_workspaces(&mut self, workspace: &mut Workspace) {
        todo!();
    }

    pub fn select_workspace(&mut self, index: Option<usize>, conn: &xcb::Connection) {
        self.get_selected_workspace().hide(conn);
        match index {
            Some(index) => {
                self.sel_workspace = Some(index);
                let window_area = self.window_area;
                self.get_selected_workspace().tile(window_area, &conn);

            }
            None => {}
        }
    }

    pub fn add_client(&mut self, client: Client, conn: &xcb::Connection) {
        let window_area = self.window_area;
        let workspace = self.get_selected_workspace();
        workspace.add_client(client);
        workspace.tile(window_area, &conn);
        conn.flush().unwrap();
    }

    pub fn remove_client(&mut self, client: Client, conn: &xcb::Connection) {
        let window_area = self.window_area;
        for workspace in &mut self.workspaces {
            if workspace.remove_client(&client) {
                break;
            };
        }
        self.get_selected_workspace().tile(window_area, &conn);
        conn.flush();
    }
}
