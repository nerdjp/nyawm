use xcb::{x::{self, Cw, Window}, xinerama};
use std::vec::Vec;

const GAPPX: i32 = 5;

struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

struct Layout {
    name: String,
    symbol: String,
    arrange: dyn FnMut(Vec<Client>) -> (),
}

struct Monitor {
    pub number: i32,
    pub geometry: Rect,
    pub window_area: Rect,
    pub mfact: f32,
    //pub layout: Layout,
}

impl Monitor {
    pub fn new(conn: &xcb::Connection, root: Window) -> Vec<Monitor> {
        let screens = conn.wait_for_reply(
            conn.send_request(
                &xinerama::QueryScreens{},
            ),
        ).unwrap();
        let mut monitors: Vec<Monitor> = Vec::new();
        let mut i = 0;
        for screen in screens.screen_info() {
            println!("x: {}, y: {}, w: {}, h: {}", screen.x_org, screen.y_org, screen.width, screen.height);
            monitors.push(Monitor { 
                number: i,
                geometry: Rect {
                    x: screen.x_org as i32,
                    y: screen.y_org as i32,
                    w: screen.width as i32,
                    h: screen.height as i32,
                },
                window_area: Rect {
                    x: screen.x_org as i32,
                    y: screen.y_org as i32,
                    w: screen.width as i32,
                    h: screen.height as i32,
                },
                mfact: 0.55, 
            });
            i += 1;
        }
        monitors
    }
}

struct Client {
    pub name: String,
    pub geometry: Rect,
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
            geometry: Rect { x: 0, y: 0, w: 0, h: 0 },
            window: win,
            border: 2,
            isfloating: false,
            isurgent: false,
            isfullscreen: false
        }
    }
}

fn setup() -> (xcb::Connection, Vec<Monitor>) {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();

    let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();

    let cookie = conn.send_request_checked(&x::ChangeWindowAttributes {
        window: screen.root(),
        value_list: &[
            Cw::EventMask(
                x::EventMask::BUTTON_PRESS |
                x::EventMask::ENTER_WINDOW |
                x::EventMask::KEY_PRESS |
                x::EventMask::LEAVE_WINDOW |
                x::EventMask::POINTER_MOTION |
                x::EventMask::PROPERTY_CHANGE |
                x::EventMask::STRUCTURE_NOTIFY |
                x::EventMask::SUBSTRUCTURE_NOTIFY |
                x::EventMask::SUBSTRUCTURE_REDIRECT
            ),
        ],
    });

    match conn.check_request(cookie) {
        Err(val) => {
            println!("Error: {}", val.to_string());
        },
        _ => {},
    };

    let monitor = Monitor::new(&conn, screen.root());

    (conn, monitor)
}

fn tile(monitor: &Monitor, clients: &mut Vec<Client>, conn: &xcb::Connection) {
    let n = clients.len();
    if n == 0 { return };

    let mut m_widht = monitor.window_area.w as f32 - GAPPX as f32;
    if n != 1 {
        m_widht = monitor.window_area.w as f32 * monitor.mfact;
    }

    let mut master_y = GAPPX;
    let mut tile_y = GAPPX;

    let mut i = 0;
    for client in clients {
        let mut rect = Rect{x: 0, y: 0, w: 0, h: 0 };
        if i < 1 {
            let h = (monitor.window_area.h - master_y) / (std::cmp::min(n, 1) as i32 - i) - GAPPX;
            rect.x = monitor.window_area.x + GAPPX;
            rect.y = monitor.window_area.y + master_y;
            rect.w = m_widht as i32 - (2 * client.border) - GAPPX;
            rect.h = h - (2 * client.border);
            resize(client, rect, conn);
            let h = client.geometry.h + (2 * client.border);
            if master_y + h < monitor.window_area.h {
                master_y += h + GAPPX;
            }
        } else {
            let h = (monitor.window_area.h - tile_y) / (n as i32 - i) - GAPPX;
            rect.x = monitor.window_area.x + m_widht as i32 + GAPPX;
            rect.y = monitor.window_area.y + tile_y;
            rect.w = monitor.window_area.w - m_widht as i32 - (2 * client.border) - 2 * GAPPX;
            rect.h = h - (2 * client.border);
            resize(client, rect, conn);
            let h = client.geometry.h + (2 * client.border);
            if tile_y + h < monitor.window_area.h {
                tile_y += h + GAPPX;
            }
        }
        i += 1;
    }
}

fn resize(client: &mut Client, geometry: Rect, conn: &xcb::Connection) {
    client.geometry = geometry;
    conn.send_request(&x::ConfigureWindow {
        window: client.window,
        value_list: &[
            x::ConfigWindow::X(client.geometry.x),
            x::ConfigWindow::Y(client.geometry.y),
            x::ConfigWindow::Width(client.geometry.w.try_into().unwrap()),
            x::ConfigWindow::Height(client.geometry.h.try_into().unwrap()),
            x::ConfigWindow::BorderWidth(client.border.try_into().unwrap()),
        ],
    });
}

fn main() {

    let (conn, monitor) = setup();
    let mut clients: Vec<Client> = Vec::new();
    loop {
        match conn.wait_for_event().expect("Erron in main event loop") {
            xcb::Event::X(x::Event::MapRequest(map)) => {
                let client = Client::new(map.window());
                clients.push(client);
                tile(&monitor[0], &mut clients, &conn);
                conn.send_request(&x::MapWindow {
                    window: clients.last().unwrap().window,
                });
                conn.flush().unwrap();

            }
            xcb::Event::X(x::Event::UnmapNotify(map)) => {
                println!("UnmapNotify");
                for i in 0..clients.len() - 1 {
                    if clients[i].window == map.window() {
                        clients.remove(i);
                        break;
                    }
                }
                tile(&monitor[0], &mut clients, &conn);
                conn.flush().unwrap();
            } 
            xcb::Event::X(x::Event::ClientMessage(_)) => {
                println!("ClientMessage");
            }
            xcb::Event::X(x::Event::ConfigureRequest(_)) => {
                println!("ConfigureRequest");
            }
            xcb::Event::X(x::Event::ConfigureNotify(_)) => {
                println!("ConfigureNotify");
            }
            xcb::Event::X(x::Event::DestroyNotify(_)) => {
                println!("DestroyNotify");
            }
            xcb::Event::X(x::Event::EnterNotify(_)) => {
                println!("EnterNotify");
            }
            xcb::Event::X(x::Event::Expose(_)) => {
                println!("Expose");
            }
            xcb::Event::X(x::Event::FocusIn(_)) => {
                println!("FocusIn");
            }
            xcb::Event::X(x::Event::MappingNotify(_)) => {
                println!("MappingNotify");
            }
            xcb::Event::X(x::Event::MotionNotify(_)) => {
                //println!("MotionNotify");
            }
            xcb::Event::X(x::Event::PropertyNotify(_)) => {
                println!("PropertyNotify");
            }
            xcb::Event::X(x::Event::ButtonPress(_)) => {
                println!("Button Press");
            }
            xcb::Event::X(x::Event::KeyPress(key)) => {
                println!("Key Press");
                if key.detail() == 0x18 {
                    break;
                }
            }
            _ => {println!("Event"); }
        }
    }
}
