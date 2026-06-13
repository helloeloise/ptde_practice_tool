use hudhook::inject::Process;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

fn candidate_dll_paths(exe_path: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();

    if let Some(dir) = exe_path.parent() {
        out.push(dir.join("dinput8.dll"));
        out.push(dir.join("hello_hud.dll"));

        if let Some(parent) = dir.parent() {
            out.push(parent.join("dinput8.dll"));
            out.push(parent.join("hello_hud.dll"));
        }
    }

    out
}

fn open_target_process() -> Result<Process, String> {
    let name_candidates = [
        "DARKSOULS.exe",
        "DarkSouls.exe",
        "darksouls.exe",
        "DARKSOULS",
        "DarkSouls",
        "darksouls",
    ];

    let title_candidates = [
        "DARK SOULS",
        "Dark Souls",
        "Dark Souls: Prepare to Die Edition",
    ];

    let attempts = 15;
    for _ in 0..attempts {
        for name in name_candidates {
            if let Ok(p) = Process::by_name(name) {
                println!("Opened target process by name: {name}");
                return Ok(p);
            }
        }

        for title in title_candidates {
            if let Ok(p) = Process::by_title(title) {
                println!("Opened target process by window title: {title}");
                return Ok(p);
            }
        }

        thread::sleep(Duration::from_millis(500));
    }

    Err("Could not find/open Dark Souls process by known names or titles".to_string())
}

fn main() {
    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to resolve injector executable path: {e}");
            std::process::exit(1);
        }
    };

    let mut resolved_dll: Option<PathBuf> = None;
    for candidate in candidate_dll_paths(&exe_path) {
        if candidate.exists() {
            match candidate.canonicalize() {
                Ok(path) => {
                    resolved_dll = Some(path);
                    break;
                }
                Err(e) => {
                    eprintln!("Found DLL candidate but canonicalize failed: {:?} ({e})", candidate);
                }
            }
        }
    }

    let cur_dll = match resolved_dll {
        Some(p) => p,
        None => {
            eprintln!("Could not find dinput8.dll or hello_hud.dll near injector executable.");
            eprintln!("Injector path: {:?}", exe_path);
            for p in candidate_dll_paths(&exe_path) {
                eprintln!("Tried: {:?}", p);
            }
            std::process::exit(1);
        }
    };

    println!("Injector executable: {:?}", exe_path);
    println!("DLL path: {:?}", cur_dll);

    let process = match open_target_process() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{e}");
            eprintln!("If running under Proton, launch this injector from the same Proton prefix/session after the game reaches the main menu.");
            std::process::exit(1);
        }
    };

    if let Err(e) = process.inject(cur_dll.into()) {
        eprintln!("DLL injection failed: {e:?}");
        std::process::exit(1);
    }

    println!("Injection succeeded.");
}
