// Copyright (c) 2023 snipcola & elliottophellia
// SPDX-License-Identifier: MIT

extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.compile().unwrap();
    }
}