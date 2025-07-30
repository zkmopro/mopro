use anyhow::Result;
use cli::create::{Create, Flutter};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_flutter_create_success() -> Result<()> {
    let temp = tempdir()?;
    let project_dir = temp.path().to_path_buf();

    // Mock empty bindings directories
    let ios_bindings = project_dir.join("ios_bindings");
    let android_bindings = project_dir.join("android_bindings");
    fs::create_dir_all(&ios_bindings)?;
    fs::create_dir_all(&android_bindings)?;

    // Call legacy create (both platforms)
    let result = Flutter::create(project_dir.clone());
    assert!(result.is_ok());

    let target_dir = project_dir.join("flutter");
    assert!(target_dir.exists());

    Ok(())
}

#[test]
fn test_flutter_create_with_ios() -> Result<()> {
    let temp = tempdir()?;
    let project_dir = temp.path().to_path_buf();

    let ios_bindings = project_dir.join("ios_bindings");
    fs::create_dir_all(&ios_bindings)?;

    let result = Flutter::create_with_platform(project_dir.clone(), "ios".to_string());
    assert!(result.is_ok());

    let target_dir = project_dir.join("flutter");
    assert!(target_dir.exists());

    Ok(())
}

#[test]
fn test_flutter_create_with_android() -> Result<()> {
    let temp = tempdir()?;
    let project_dir = temp.path().to_path_buf();

    let android_bindings = project_dir.join("android_bindings");
    fs::create_dir_all(&android_bindings)?;

    let result = Flutter::create_with_platform(project_dir.clone(), "android".to_string());
    assert!(result.is_ok());

    let target_dir = project_dir.join("flutter");
    assert!(target_dir.exists());

    Ok(())
}

#[test]
fn test_flutter_create_invalid_platform() {
    let temp = tempdir().unwrap();
    let project_dir = temp.path().to_path_buf();

    let result = Flutter::create_with_platform(project_dir, "windows".to_string());
    assert!(result.is_err());
}
