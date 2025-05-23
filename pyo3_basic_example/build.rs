use std::{env, path::Path, process::Command};

fn main() {
  println!("cargo:rerun-if-changed=build.rs");

  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let venv_path = Path::new(&manifest_dir).join(".env");

  // Check if virtual environment already exists
  if !venv_path.exists() {
    println!("cargo:warning=Creating Python virtual environment...");

    // Create virtual environment
    let output = Command::new("python3")
      .args(["-m", "venv", ".env"])
      .current_dir(&manifest_dir)
      .output()
      .expect("Failed to create virtual environment");

    if !output.status.success() {
      panic!(
        "Failed to create virtual environment: {}",
        String::from_utf8_lossy(&output.stderr)
      );
    }

    // Install maturin in the virtual environment
    println!("cargo:warning=Installing maturin...");

    #[cfg(target_os = "windows")]
    let pip_path = venv_path.join("Scripts").join("pip3.exe");
    #[cfg(not(target_os = "windows"))]
    let pip_path = venv_path.join("bin").join("pip3");

    let output = Command::new(&pip_path)
      .args(["install", "maturin"])
      .current_dir(&manifest_dir)
      .output()
      .expect("Failed to install maturin");

    if !output.status.success() {
      panic!(
        "Failed to install maturin: {}",
        String::from_utf8_lossy(&output.stderr)
      );
    }

    println!("cargo:warning=Virtual environment setup complete!");
  } else {
    println!("cargo:warning=Virtual environment already exists, skipping setup");
  }
}
