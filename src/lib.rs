#![allow(static_mut_refs)]
#![allow(dead_code)]

use hudhook::*;
use hudhook::hooks::dx9::ImguiDx9Hooks;
use crate::render_loop::RenderLoop;

mod memory;
mod render_loop;

hudhook!(ImguiDx9Hooks, RenderLoop::new());
