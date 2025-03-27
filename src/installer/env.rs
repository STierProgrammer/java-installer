use std::path::Path;
use anyhow::Result;

pub fn set_environment_variables(java_home: &Path) -> Result<()> {
    let bin_path = java_home.join("bin");
    
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            use winreg::RegKey;
            use winreg::enums::HKEY_CURRENT_USER;
            use winapi::um::winnt::{KEY_READ, KEY_WRITE};

            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let environment = hkcu.open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)?;

            environment.set_value("JAVA_HOME", &java_home.to_string_lossy().to_string())?;

            let path_value: String = environment.get_value("Path").unwrap_or_default();
            let bin_path_str = bin_path.to_string_lossy();
            
            if !path_value.contains(&*bin_path_str) {
                let new_path = format!("{};{}", bin_path_str, path_value);
                environment.set_value("Path", &new_path)?;
            }

            unsafe {
                use winapi::um::winuser::SendMessageTimeoutW;
                use winapi::um::winuser::{HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE};

                SendMessageTimeoutW(
                    HWND_BROADCAST,
                    WM_SETTINGCHANGE,
                    0,
                    "Environment\0".as_ptr() as _,
                    SMTO_ABORTIFHUNG,
                    5000,
                    std::ptr::null_mut(),
                );
            }
        } else {
            let shell_config = dirs::home_dir()
                .unwrap()
                .join(if env::var("SHELL").unwrap_or_default().contains("zsh") {
                    ".zshrc"
                } else {
                    ".bashrc"
                });

            let content = fs::read_to_string(&shell_config).unwrap_or_default();
            if !content.contains("JAVA_HOME") {
                let mut file = fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&shell_config)?;

                writeln!(file, "\nexport JAVA_HOME={}", java_home.display())?;
                writeln!(file, "export PATH=\"$JAVA_HOME/bin:$PATH\"")?;
            }

            println!("You may need to run: source {}", shell_config.display());
        }
    }

    Ok(())
}
