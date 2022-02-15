use std::process::Command;

#[test]
fn test_backends() {
    // Test the use-default crate.
    assert!(
        !has_dependency("test-crates/use-default", &[], "libc"),
        "use-default depends on libc"
    );
    assert!(
        has_dependency("test-crates/use-default", &[], "linux-raw-sys"),
        "use-default does not depend on linux-raw-sys"
    );

    // Test the use-libc crate, which enables the "use-libc" cargo feature.
    assert!(
        has_dependency("test-crates/use-libc", &[], "libc"),
        "use-libc doesn't depend on libc"
    );

    // Test the use-default crate with --cfg=rustix_use_libc
    assert!(
        has_dependency("test-crates/use-default", &[("RUSTFLAGS", "--cfg=rustix_use_libc")], "libc"),
        "use-default with --cfg=rustix_use_libc does not depend on libc"
    );
    assert!(
        !has_dependency("test-crates/use-default", &[("RUSTFLAGS", "--cfg=rustix_use_libc")], "linux-raw-sys"),
        "use-default with --cfg=rustix_use_libc depends on linux-raw-sys"
    );
}

fn has_dependency(dir: &str, envs: &[(&str, &str)], dependency: &str) -> bool {
    let mut command = Command::new("cargo");

    command
        .arg("tree")
        .arg("--quiet")
        .arg(&format!("--invert={}", dependency))
        .current_dir(dir);

    for (key, value) in envs {
        command.env(key, value);
    }

    let child = command
        .output()
        .unwrap();

    child.status.success() && !child.stdout.is_empty()
}
