//! ***************************************************************************
//! Rust Rubiks Cube Solver <https://github.com/Thief3/RubiksCubeSolver>
//!
//! Copyright 2018 by Malik Kissarli <kissarlim@gmail.com>
//! Licensed under GNU General Public License 3.0 or later.
//! Some rights reserved. See COPYING, AUTHORS.
//!
//! @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>
//! ***************************************************************************

#[allow(unused_imports)]
use std::io;

extern crate imgui;
extern crate glium;
extern crate imgui_glium_renderer;
extern crate imgui_winit_support;
extern crate clipboard;

mod facelets;
mod physical;
mod solver;

mod console;
mod gui;
mod ui_support;

mod utility;
    
fn main() {
    gui::create_window();
    //console::create_terminal();
}
