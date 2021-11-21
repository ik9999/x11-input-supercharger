mod config;
mod features;
mod gui;
mod x;

use lazy_panic::{formatter, set_panic_message};

use std::alloc::System;
use std::process::Command;

use crate::config::Config;
//use crate::features::keyboard_click::KeyboardClick;
use crate::features::scroll::Scroll;
use crate::x::xlib::XLib;

// Xlib sometimes chokes and crashes with jemalloc, while calling XNextEvent
// TODO: check whether necessary anymore - it probably happened due to double freeing memory
#[global_allocator]
static ALLOCATOR: System = System;

pub fn need_dep(name: &str) {
    Command::new(name)
        .arg("--version")
        .output()
        .unwrap_or_else(|_| panic!("Missing global binary: {}", name));
}

#[allow(clippy::option_map_unit_fn)]
fn main() {
    set_panic_message!(formatter::Simple);

    need_dep("xdotool");
    need_dep("xmodmap");

    let config = Config::load();
    if config.scroll.is_none() && config.keyboard_click.is_none() {
        panic!("Current configuration does nothing - all features disabled");
    }

    let mut x = XLib::new();

    let mut scroll = config.scroll.as_ref().map(|c| Scroll::new(c, &mut x));
    //let mut keyboard_click = config
        //.keyboard_click
        //.as_ref()
        //.map(|c| KeyboardClick::new(c, &mut x));

    let mut x = x.finish();
    loop {
        if let Some(ev) = x.poll() {
            scroll.as_mut().map(|o| o.handle(&ev));
            //keyboard_click.as_mut().map(|o| o.handle(&ev));
        }
    }
}
