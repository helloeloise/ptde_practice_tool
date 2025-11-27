#![allow(static_mut_refs)]
#![allow(dead_code)]

use crate::render_loop::RenderLoop;
use hudhook::hooks::dx9::ImguiDx9Hooks;
use hudhook::*;

mod memory;
mod render_loop;
mod ui;

hudhook!(ImguiDx9Hooks, RenderLoop::new());
