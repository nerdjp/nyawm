mod client;
mod monitor;
mod rect;
mod workspace;

use client::Client;
use monitor::Monitor;
use rect::Rect;
use std::vec::Vec;
use workspace::Workspace;
use xcb::x::{self, Cw};

const GAPPX: i32 = 5;

fn setup() -> xcb::Connection {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();

    let screen = conn.get_setup().roots().nth(screen_num as usize).unwrap();

    let cookie = conn.send_request_checked(&x::ChangeWindowAttributes {
        window: screen.root(),
        value_list: &[Cw::EventMask(
            x::EventMask::BUTTON_PRESS
                | x::EventMask::ENTER_WINDOW
                | x::EventMask::KEY_PRESS
                | x::EventMask::LEAVE_WINDOW
                | x::EventMask::POINTER_MOTION
                | x::EventMask::PROPERTY_CHANGE
                | x::EventMask::STRUCTURE_NOTIFY
                | x::EventMask::SUBSTRUCTURE_NOTIFY
                | x::EventMask::SUBSTRUCTURE_REDIRECT,
        )],
    });

    match conn.check_request(cookie) {
        Err(val) => {
            println!("Error: {}", val.to_string());
        }
        _ => {}
    };

    conn
}

fn main() {
    let conn = setup();
    let mut monitors = Monitor::update_geometry(Vec::new(), &conn);
    monitors[0].workspaces.push(Workspace {
        num: 0,
        name: "".to_string(),
        clients: Vec::new(),
        sel_client: 0,
        gappx: 5,
        geometry: Rect {
            x: 0,
            y: 0,
            w: 1280,
            h: 720,
        },
        mfact: 0.55,
    });
    monitors[0].workspaces.push(Workspace {
        num: 0,
        name: "".to_string(),
        clients: Vec::new(),
        sel_client: 0,
        gappx: 5,
        geometry: Rect {
            x: 0,
            y: 0,
            w: 1280,
            h: 720,
        },
        mfact: 0.55,
    });
    monitors[0].sel_workspace = 0;

    loop {
        match conn.wait_for_event().expect("Erron in main event loop") {
            xcb::Event::X(x::Event::MapRequest(map)) => {
                if conn.wait_for_reply(conn.send_request(&x::GetWindowAttributes {
                    window: map.window(),
                })).unwrap().override_redirect() {
                    continue;
                }
                let client = Client::new(map.window());

                //TODO Select the sel_workspace
                let workspace = monitors[0].get_selected_workspace();

                workspace.add_client(client);
                workspace.tile(&conn);

                conn.send_request(&x::MapWindow {
                    window: monitors[0].workspaces[0].clients.last().unwrap().window,
                });

                conn.flush().unwrap();
            }
            xcb::Event::X(x::Event::UnmapNotify(map)) => {
                println!("UnmapNotify");
                let workspace = &mut monitors[0].workspaces[0];
                let clients = &mut workspace.clients;
                for i in 0..clients.len() {
                    if clients[i].window == map.window() {
                        clients.remove(i);
                        break;
                    }
                }
                workspace.tile(&conn);
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
                    let monitor = &mut monitors[0];
                    println!("Toggle");
                    if monitor.sel_workspace == 0 {
                        monitor.select_workspace(1, &conn);
                    } else {
                        monitor.select_workspace(0, &conn);
                    }
                }
            }
            _ => {}
        }
    }
}
