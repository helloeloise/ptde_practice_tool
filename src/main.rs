use hudhook::inject::Process;

fn main() {
    let mut cur_exe = std::env::current_exe().unwrap();
    cur_exe.push("..");
    cur_exe.push("hello_hud.dll");

    let cur_dll = cur_exe.canonicalize().unwrap();
    print!("{:?}", cur_dll);
    print!("test");
    Process::by_name("DARKSOULS.exe")
        .unwrap()
        .inject(
            cur_dll
                .into(),
        )
        .unwrap();
}
