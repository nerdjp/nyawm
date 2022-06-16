mod client;
mod monitor;
mod workspace;
mod geometry;

use client::Client;
use monitor::Monitor;
use std::vec::Vec;

use xcb::x::{self, Cw};

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
                | x::EventMask::SUBSTRUCTURE_REDIRECT
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

    loop {
        match conn.wait_for_event().expect("Erron in main event loop") {
            xcb::Event::X(x::Event::MapRequest(map)) => {
                if conn.wait_for_reply(conn.send_request(&x::GetWindowAttributes {
                    window: map.window(),
                })).unwrap().override_redirect() {
                    continue;
                }

                let client = Client::new(map.window());
                conn.send_request(&xcb::x::MapWindow {
                    window: client.window,
                });
                monitors[0].add_client(client, &conn);
            }
            xcb::Event::X(x::Event::UnmapNotify(map)) => {
                println!("UnmapNotify");
                let client = Client::new(map.window());
                monitors[0].remove_client(client, &conn);
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
            xcb::Event::X(x::Event::KeyPress(_key)) => {
                println!("Key Press");
            }
            _ => {}
        }
    }
}
