use crate::{client::Client, rect::Rect};

struct Layout {
    name: String,
    symbol: String,
    arrange: dyn FnMut(&mut Workspace) -> (),
}

pub struct Workspace {
    pub num: i32,
    pub name: String,
    pub clients: Vec<Client>,
    pub sel_client: i32,
    pub gappx: i32,
    pub geometry: Rect,
    pub mfact: f32,
    //pub layout: Box<Layout>,
}

impl Workspace {
    pub fn new(
        num: i32,
        name: String,
        clients: Vec<Client>,
        sel_client: i32,
        geometry: Rect,
        mfact: f32,
        //layout: Layout
    ) -> Self {
        Self {
            num,
            name,
            clients,
            sel_client,
            gappx: 5,
            geometry,
            mfact,
            //layout
        }
    }

    pub fn add_client(&mut self, client: Client) {
        self.clients.push(client);
    }

    pub fn tile(&mut self, conn: &xcb::Connection) {
        let n = self.clients.len();
        if n == 0 {
            return;
        };

        let mut m_widht = self.geometry.w as f32 - self.gappx as f32;
        if n != 1 {
            m_widht = self.geometry.w as f32 * self.mfact;
        }

        let mut master_y = self.gappx;
        let mut tile_y = self.gappx;

        let mut i = 0;
        for client in &mut self.clients {
            let mut rect = Rect {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
            };
            if i < 1 {
                let h =
                    (self.geometry.h - master_y) / (std::cmp::min(n, 1) as i32 - i) - self.gappx;
                rect.x = self.geometry.x + self.gappx;
                rect.y = self.geometry.y + master_y;
                rect.w = m_widht as i32 - (2 * client.border) - self.gappx;
                rect.h = h - (2 * client.border);
                client.resize(rect, conn);
                let h = client.geometry.h + (2 * client.border);
                if master_y + h < self.geometry.h {
                    master_y += h + self.gappx;
                }
            } else {
                let h = (self.geometry.h - tile_y) / (n as i32 - i) - self.gappx;
                rect.x = self.geometry.x + m_widht as i32 + self.gappx;
                rect.y = self.geometry.y + tile_y;
                rect.w = self.geometry.w - m_widht as i32 - (2 * client.border) - 2 * self.gappx;
                rect.h = h - (2 * client.border);
                client.resize(rect, conn);
                let h = client.geometry.h + (2 * client.border);
                if tile_y + h < self.geometry.h {
                    tile_y += h + self.gappx;
                }
            }
            i += 1;
        }
    }

    pub fn focus(&mut self, conn: &xcb::Connection) {
        self.tile(conn);
        conn.flush().unwrap();
    }

    pub fn unfocus(&mut self, conn: &xcb::Connection) {
        for client in &self.clients {
            client.hide(conn);
        }
        conn.flush().unwrap();
    }
}
