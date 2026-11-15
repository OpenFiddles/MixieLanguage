use std::{
    env,
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};

#[derive(Clone)]
struct Environment {
    label: &'static str,
    manager: Option<&'static str>,
    packages: &'static [&'static str],
}

fn command_exists(name: &str) -> bool {
    let paths = env::var_os("PATH").unwrap_or_default();
    env::split_paths(&paths).any(|p| {
        let candidate = p.join(name);
        candidate.is_file()
            || (cfg!(windows) && p.join(format!("{name}.exe")).is_file())
    })
}

fn environment() -> Environment {
    if env::var_os("TERMUX_VERSION").is_some()
        || env::var("PREFIX").map(|p| p.contains("com.termux")).unwrap_or(false)
    {
        Environment {
            label: "Termux (Android)",
            manager: Some("pkg"),
            packages: &["rust", "git", "make", "clang", "dialog"],
        }
    } else if cfg!(target_os = "linux") && command_exists("apt-get") {
        Environment { label: "Debian / Ubuntu", manager: Some("apt-get"), packages: &["rustc", "cargo", "git", "make", "dialog"] }
    } else if cfg!(target_os = "linux") && command_exists("dnf") {
        Environment { label: "Fedora", manager: Some("dnf"), packages: &["rust", "cargo", "git", "make", "dialog"] }
    } else if cfg!(target_os = "linux") && command_exists("pacman") {
        Environment { label: "Arch Linux", manager: Some("pacman"), packages: &["rust", "git", "base-devel", "dialog"] }
    } else if cfg!(target_os = "linux") && command_exists("apk") {
        Environment { label: "Alpine Linux", manager: Some("apk"), packages: &["rust", "cargo", "git", "make", "dialog"] }
    } else if cfg!(target_os = "linux") && command_exists("zypper") {
        Environment { label: "openSUSE", manager: Some("zypper"), packages: &["rust", "cargo", "git", "make", "dialog"] }
    } else if cfg!(target_os = "macos") && command_exists("brew") {
        Environment { label: "macOS (Homebrew)", manager: Some("brew"), packages: &["rust", "git", "make", "dialog"] }
    } else if cfg!(windows) {
        Environment { label: "Windows", manager: None, packages: &[] }
    } else {
        Environment { label: "Unknown Unix-like environment", manager: None, packages: &[] }
    }
}

fn dialog(args: &[&str]) -> Option<String> {
    let output = Command::new("dialog").args(args).arg("--output-fd").arg("1")
        .stdout(Stdio::piped()).stderr(Stdio::null()).output().ok()?;
    if !output.status.success() { return None; }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn message(title: &str, body: &str) {
    if dialog(&["--title", title, "--msgbox", body, "12", "68"]).is_none() {
        println!("\n== {title} ==\n{body}\n");
        let _ = pause();
    }
}

fn pause() -> io::Result<()> {
    print!("Press Enter to continue...");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(())
}

fn dependency_report() -> String {
    ["rustc", "cargo", "git", "make", "dialog"]
        .iter()
        .map(|name| format!("{:8} {}", name, if command_exists(name) { "[OK] found" } else { "[MISSING]" }))
        .collect::<Vec<_>>().join("\n")
}

fn install_packages(env_info: &Environment) -> Result<(), String> {
    let manager = env_info.manager.ok_or_else(|| {
        "No supported package manager was detected. Install Rust/Cargo, Git, Make, and dialog using your environment's official package manager.".to_string()
    })?;
    let (program, args): (&str, Vec<&str>) = match manager {
        "pkg" => ("pkg", vec!["install", "-y"]),
        "apt-get" => {
            if !is_root() && command_exists("sudo") {
                ("sudo", vec!["apt-get", "update"])
            } else { ("apt-get", vec!["update"]) }
        }
        "dnf" => ("dnf", vec!["install", "-y"]),
        "pacman" => ("pacman", vec!["-S", "--needed", "--noconfirm"]),
        "apk" => ("apk", vec!["add"]),
        "zypper" => ("zypper", vec!["--non-interactive", "install"]),
        "brew" => ("brew", vec!["install"]),
        _ => return Err("Unsupported package manager.".into()),
    };
    if manager == "apt-get" {
        let status = Command::new(program).args(&args).status().map_err(|e| e.to_string())?;
        if !status.success() { return Err("Package index update failed.".into()); }
        let mut install_args = vec!["install", "-y"];
        install_args.extend(env_info.packages.iter().copied());
        let (program, args) = if !is_root() && command_exists("sudo") {
            ("sudo", [vec!["apt-get"], install_args].concat())
        } else { ("apt-get", install_args) };
        let status = Command::new(program).args(args).status().map_err(|e| e.to_string())?;
        return if status.success() { Ok(()) } else { Err("Package installation failed.".into()) };
    }
    let mut full_args = args;
    full_args.extend(env_info.packages.iter().copied());
    let (program, full_args) = if !matches!(manager, "pkg" | "brew") && !is_root() && command_exists("sudo") {
        ("sudo", [vec![program], full_args].concat())
    } else { (program, full_args) };
    let status = Command::new(program).args(full_args).status().map_err(|e| e.to_string())?;
    if status.success() { Ok(()) } else { Err("Package installation failed.".into()) }
}

// Detect root without adding a libc dependency.
#[cfg(unix)]
fn is_root() -> bool {
    Command::new("id").arg("-u").output().ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .is_some_and(|s| s.trim() == "0")
}
#[cfg(not(unix))]
fn is_root() -> bool { false }

fn build_project() -> Result<(), String> {
    let status = Command::new("cargo").args(["build", "--release"]).status()
        .map_err(|e| format!("could not start Cargo: {e}"))?;
    if status.success() { Ok(()) } else { Err("Cargo build failed; see the compiler output above.".into()) }
}

fn run_example() -> Result<(), String> {
    let status = Command::new("cargo").args(["run", "--release", "--", "-f", "examples/hello.mx"]).status()
        .map_err(|e| format!("could not start Cargo: {e}"))?;
    if status.success() { Ok(()) } else { Err("Example run failed.".into()) }
}

fn text_menu(env_info: &Environment) -> Result<(), String> {
    loop {
        println!("\nMixieLand setup — {}\n1) Check dependencies\n2) Install prerequisites\n3) Build MixieLang\n4) Run hello-world example\n5) Show environment\n0) Exit", env_info.label);
        print!("Choose: "); io::stdout().flush().map_err(|e| e.to_string())?;
        let mut choice=String::new(); io::stdin().read_line(&mut choice).map_err(|e| e.to_string())?;
        match choice.trim() {
            "1" => message("Dependency check", &dependency_report()),
            "2" => { if env_info.manager.is_some() { install_packages(env_info)?; } else { message("Manual setup", "Install Rust/Cargo, Git, Make, and dialog using your platform's official instructions."); } },
            "3" => build_project()?,
            "4" => run_example()?,
            "5" => message("Environment", &format!("Detected: {}\nPackage manager: {}\nPackages: {}",env_info.label,env_info.manager.unwrap_or("manual"),env_info.packages.join(", "))),
            "0" => break,
            _ => println!("Invalid choice."),
        }
    }
    Ok(())
}

pub fn run_setup() -> Result<(), String> {
    let env_info=environment();
    if !command_exists("dialog") {
        println!("The dialog utility is required for the full setup menu.");
        println!("Detected environment: {}",env_info.label);
        if env_info.manager == Some("pkg") {
            println!("Install it first with: pkg install dialog");
        } else {
            println!("Install 'dialog' with your system package manager, then run: mixie setup");
        }
        return text_menu(&env_info);
    }
    loop {
        let choice=dialog(&[
            "--clear","--title","MixieLand Setup",
            "--menu",&format!("Environment: {}\nChoose a setup task:",env_info.label),
            "18","72","8",
            "check","Check dependencies",
            "install","Install prerequisites",
            "build","Build MixieLang (release)",
            "example","Run hello-world example",
            "environment","Show environment details",
            "exit","Exit setup",
        ]);
        match choice.as_deref() {
            Some("check") => message("Dependency check",&dependency_report()),
            Some("install") => {
                let prompt=format!("Install the recommended packages for {}?\n\n{}\n\nThis runs your system package manager.",env_info.label,env_info.packages.join(", "));
                if dialog(&["--title","Confirm installation","--yesno",&prompt,"12","68"]).is_some() {
                    if let Err(e)=install_packages(&env_info) { message("Installation error",&e); }
                    else { message("Installation complete","Package installation finished. Reopen setup and check dependencies."); }
                }
            }
            Some("build") => if let Err(e)=build_project(){message("Build failed",&e)} else {message("Build complete","MixieLang was built in target/release.")},
            Some("example") => if let Err(e)=run_example(){message("Example failed",&e)} else {message("Example complete","The hello-world example finished successfully.")},
            Some("environment") => message("Environment details",&format!("Detected: {}\nPackage manager: {}\nRecommended packages: {}",env_info.label,env_info.manager.unwrap_or("manual"),env_info.packages.join(", "))),
            Some("exit") | None => break,
            _ => break,
        }
    }
    Ok(())
}
