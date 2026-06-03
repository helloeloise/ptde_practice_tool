#![allow(static_mut_refs)]
#![allow(dead_code)]

#[cfg(windows)]
use crate::render_loop::RenderLoop;
#[cfg(windows)]
use hudhook::hooks::dx9::ImguiDx9Hooks;
#[cfg(windows)]
use hudhook::*;

#[cfg(windows)]
mod config;
#[cfg(windows)]
mod memory;
#[cfg(windows)]
mod render_loop;
#[cfg(windows)]
mod ui;

#[cfg(windows)]
hudhook!(ImguiDx9Hooks, RenderLoop::new());
