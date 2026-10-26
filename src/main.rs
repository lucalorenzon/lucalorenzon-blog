mod home_page;

use std::{env, fs, io, path::PathBuf};

fn main() -> io::Result<()> {
    let out_dir = env::args_os()
        .nth(1)
        .map_or_else(|| PathBuf::from("target/site"), PathBuf::from);
    fs::create_dir_all(&out_dir)?;
    fs::write(out_dir.join("index.html"), home_page::render_home_page())
}
