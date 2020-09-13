use chrono::Local;

fn main() {
    println!("cargo:rustc-env=DOOMWADDIR=/usr/share/games/doom");

    println!(
        "cargo:rustc-env=CURRENT_DATE={}",
        Local::now().date().naive_local().to_string()
    );

    if cfg!(windows) {
        println!(r"cargo:rustc-link-search=C:\dev\libs\SDL2\lib\x64");
    }
}
