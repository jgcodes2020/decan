use std::{path::{Path, PathBuf}, process::Command};

use decan::{can::Can, SymbolGroup};

#[derive(SymbolGroup)]
pub struct DecanTestlib {
    pub print_message: extern "C" fn(),
    pub square_int: Option<extern "C" fn(i32) -> i32>,
}

#[test]
fn test_load() {
    let testlib_path = compile_testlib();

    let can = unsafe { Can::<_, DecanTestlib>::load(testlib_path).unwrap() };

    (can.print_message)();
    assert_eq!((can.square_int.unwrap())(2), 4);
}

fn compute_dll_name(name: &str) -> String {
    #[cfg(any(target_os = "windows"))]
    return format!("{name}.dll");
    #[cfg(any(target_os = "macos"))]
    return format!("lib{name}.dylib");
    #[cfg(any(target_os = "linux"))]
    return format!("lib{name}.so");

}

fn compile_testlib() -> PathBuf {
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let testlib_dir = project_root.join("decan-testlib");

    let status = Command::new(env!("CARGO"))
        .args(["build"])
        .current_dir(&testlib_dir)
        .status()
        .unwrap();
    assert!(status.success(), "Failed to compile decan-testlib!");

    project_root.join("target/debug").join(compute_dll_name("decan_testlib"))
}
