use crate::{client::Client, geometry::Geometry};

pub struct Workspace {
    name: String,
    clients: Vec<Client>,
    sel_client: Option<usize>,
    gappx: i32,
    mfact: f32,
}

impl Workspace {
    pub fn new(
        name: String,
    ) -> Self {
        Self {
            name,
            clients: Vec::new(),
            sel_client: Option::None,
            gappx: 5,
            mfact: 0.55,
        }
    }

    pub fn add_client(&mut self, client: Client) -> usize {
        self.clients.push(client);
        self.clients.len() - 1
    }

    pub fn remove_client(&mut self, client: &Client) -> bool {
        match self.clients.iter().position(|c| c == client) {
            Some(index) => {
                self.clients.remove(index);
                true
            },
            None => {
                false
            },
        }
    }

    pub fn get_client_by_index(&self, index: usize) -> &Client {
        self.clients.get(index).unwrap()
    }

    pub fn tile(&mut self, window_area: Geometry, conn: &xcb::Connection) {
        let n = self.clients.len();
        if n == 0 {
            return;
        };

        let mut m_widht = window_area.w as f32 - self.gappx as f32;
        if n != 1 {
            m_widht = window_area.w as f32 * self.mfact;
        }

        let mut master_y = self.gappx;
        let mut tile_y = self.gappx;

        let mut i = 0;
        for client in &mut self.clients {
            let mut rect = Geometry {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
            };
            if i < 1 {
                let h =
                    (window_area.h - master_y) / (std::cmp::min(n, 1) as i32 - i) - self.gappx;
                rect.x = window_area.x + self.gappx;
                rect.y = window_area.y + master_y;
                rect.w = m_widht as i32 - (2 * client.border) - self.gappx;
                rect.h = h - (2 * client.border);
                client.resize(rect, conn);
                let h = client.geometry.h + (2 * client.border);
                if master_y + h < window_area.h {
                    master_y += h + self.gappx;
                }
            } else {
                let h = (window_area.h - tile_y) / (n as i32 - i) - self.gappx;
                rect.x = window_area.x + m_widht as i32 + self.gappx;
                rect.y = window_area.y + tile_y;
                rect.w = window_area.w - m_widht as i32 - (2 * client.border) - 2 * self.gappx;
                rect.h = h - (2 * client.border);
                client.resize(rect, conn);
                let h = client.geometry.h + (2 * client.border);
                if tile_y + h < window_area.h {
                    tile_y += h + self.gappx;
                }
            }
            i += 1;
        }
    }

    pub fn hide(&mut self, conn: &xcb::Connection) {
        for client in &self.clients {
            client.hide(conn);
        }
        conn.flush().unwrap();
    }
}
