use std::path::PathBuf;

#[macro_export]
macro_rules! path {
    ($first:expr $(, $rest:expr)* $(,)?) => {{
        use std::path::PathBuf;
        let mut p = PathBuf::from($first);
        $(
            p.push($rest);
        )*
        p
    }};
}

#[cfg(windows)]
pub fn is_elevated() -> bool {
    use std::ptr;
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::processthreadsapi::GetCurrentProcess;
    use winapi::um::{
        processthreadsapi::OpenProcessToken,
        securitybaseapi::GetTokenInformation,
        winnt::{HANDLE, TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation},
    };

    unsafe {
        let mut token_handle: HANDLE = ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) == 0 {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

        let result = GetTokenInformation(
            token_handle,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            size,
            &mut size,
        );

        if result == 0 {
            eprintln!("Error: {}", GetLastError());
            return false;
        }

        elevation.TokenIsElevated != 0
    }
}

#[cfg(unix)]
pub fn is_elevated() -> bool {
    unsafe { libc::geteuid() == 0 }
}

#[cfg(windows)]
pub fn get_default_installation_path() -> PathBuf {
    let program_files =
        std::env::var("ProgramFiles").unwrap_or_else(|_| "C:\\Program Files".to_string());
    let username = std::env::var("USERNAME").expect("Failed to get USERNAME");
    let app_data =
        std::env::var("APPDATA").unwrap_or(format!("C:\\Users\\{username}\\AppData\\Roaming"));

    if is_elevated() {
        path!(program_files, "DOSDisassm")
    } else {
        path!(app_data, "DOSDisassm")
    }
}

#[cfg(unix)]
pub fn get_default_installation_path() -> PathBuf {
    let home = std::env::var("HOME").expect("Failed to get HOME");
    if is_elevated() {
        path!("/usr", "local", "share", "DOSDisassm")
    } else {
        path!(home, ".local", "share", "DOSDisassm")
    }
}

pub fn create_temp_dir() -> PathBuf {
    let temp_dir = std::env::temp_dir();
    let random_string = rand::random::<u64>().to_string();
    let temp_dir = temp_dir.join(format!("DOSDisassm-{random_string}"));
    if !temp_dir.exists() {
        std::fs::create_dir_all(&temp_dir).expect("Failed to create temp directory");
    }
    temp_dir
}

pub fn mkdir_all(path: &PathBuf) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

#[cfg(unix)]
pub fn add_to_path(path: &PathBuf) -> std::io::Result<()> {
    println!(
        "To add to path, add the following line to your ~/.bashrc or ~/.zshrc file:
export PATH=\"$PATH:{path:?}\""
    );
    Ok(())
}

#[cfg(windows)]
pub fn add_to_path(path: &PathBuf) -> std::io::Result<()> {
    use winreg::RegKey;
    use winreg::enums::*;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path_key = hkcu.open_subkey_with_flags("Environment", KEY_SET_VALUE)?;

    let mut current_path: String = path_key.get_value("Path")?;
    if !current_path.contains(path.to_str().unwrap()) {
        current_path.push(';');
        current_path.push_str(path.to_str().unwrap());
        path_key.set_value("Path", &current_path)?;
    }

    Ok(())
}
