#[cfg(windows)]
fn main() {
    use std::{env, path::PathBuf};

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let icon_path = PathBuf::from(manifest_dir)
        .join("..") // up to workspace root if needed
        .join("assets")
        .join("icon.ico");

    let mut res = winres::WindowsResource::new();
    res.set_icon(icon_path.to_str().unwrap());
    res.set("ProductName", "DosDisassm");
    res.set("FileDescription", "DOS Disassembler");
    res.set("FileVersion", env!("CARGO_PKG_VERSION"));
    res.set("ProductVersion", env!("CARGO_PKG_VERSION"));
    res.set("CompanyName", "sk337 <me@pk3.zip>");
    res.set("OriginalFilename", "dosdisassm.exe");
    res.set("InternalName", "dosdisassm");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
