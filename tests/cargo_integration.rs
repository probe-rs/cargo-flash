use cargo_flash;

use std::path::PathBuf;

// Test reading metadata from
// the [package.metadata] section of
// Cargo.toml
#[test]
fn read_chip_metadata() {
    let work_dir = test_project_dir("binary_project");

    let metadata = cargo_flash::read_metadata(&work_dir).expect("Failed to read metadata.");

    assert_eq!(metadata.chip, Some("nrf51822".to_owned()));
}

#[test]
fn get_binary_artifact() {
    let work_dir = test_project_dir("binary_project");
    let expected_path = work_dir.join("target/debug/binary_project");

    let args = [];

    let binary_path =
        cargo_flash::build_artifact(&work_dir, &args).expect("Failed to read artifact path.");

    assert_eq!(binary_path, expected_path);
}

#[test]
fn get_binary_artifact_with_cargo_config() {
    let work_dir = test_project_dir("binary_cargo_config");
    let expected_path = work_dir.join("target/thumbv7m-none-eabi/debug/binary_cargo_config");

    let args = [];

    let binary_path =
        cargo_flash::build_artifact(&work_dir, &args).expect("Failed to read artifact path.");

    assert_eq!(binary_path, expected_path);
}

#[test]
fn get_binary_artifact_with_cargo_config_toml() {
    let work_dir = test_project_dir("binary_cargo_config_toml");
    let expected_path = work_dir.join("target/thumbv7m-none-eabi/debug/binary_cargo_config_toml");

    let args = [];

    let binary_path =
        cargo_flash::build_artifact(&work_dir, &args).expect("Failed to read artifact path.");

    assert_eq!(binary_path, expected_path);
}

#[test]
fn get_library_artifact_fails() {
    let work_dir = test_project_dir("library_project");

    let args = ["--release".to_owned()];

    let binary_path = cargo_flash::build_artifact(&work_dir, &args);

    assert!(
        binary_path.is_err(),
        "Library project should not return a path to a binary, but got {}",
        binary_path.unwrap().display()
    );
}

#[test]
fn workspace_root() {
    // In a workspace with a single binary crate,
    // we should be able to find the binary for that crate.

    let work_dir = test_project_dir("workspace_project");

    let expected_path = work_dir.join("target/release/workspace_bin");

    let args = ["--release".to_owned()];

    let binary_path =
        cargo_flash::build_artifact(&work_dir, &args).expect("Failed to read artifact path.");

    assert_eq!(binary_path, expected_path);
}

#[test]
fn workspace_binary_package() {
    // In a binary crate which is a member of a workspace,
    // we should be able to find the binary for that crate.

    let workspace_dir = test_project_dir("workspace_project");
    let work_dir = workspace_dir.join("workspace_bin");

    let expected_path = workspace_dir.join("target/release/workspace_bin");

    let args = ["--release".to_owned()];

    let binary_path =
        cargo_flash::build_artifact(&work_dir, &args).expect("Failed to read artifact path.");

    assert_eq!(binary_path, expected_path);
}

#[test]
fn workspace_library_package() {
    // In a library crate which is a member of a workspace,
    // we should show an error message.

    let work_dir = test_project_dir("workspace_project/workspace_lib");

    let args = ["--release".to_owned()];

    let binary_path = cargo_flash::build_artifact(&work_dir, &args);

    assert!(
        binary_path.is_err(),
        "Library project should not return a path to a binary, but got {}",
        binary_path.unwrap().display()
    );
}

#[test]
fn multiple_binaries_in_crate() {
    // With multiple binaries in a crate,
    // we should show an error message if no binary is specified
    let work_dir = test_project_dir("multiple_binary_project");

    let args = [];

    let binary_path = cargo_flash::build_artifact(&work_dir, &args);

    assert!(
        binary_path.is_err(),
        "With multiple binaries, an error message should be shown. Got path '{}' instead.",
        binary_path.unwrap().display()
    );
}

#[test]
fn multiple_binaries_in_crate_select_binary() {
    // With multiple binaries in a crate,
    // we should show an error message if no binary is specified
    let work_dir = test_project_dir("multiple_binary_project");
    let expected_path = work_dir.join("target/debug/bin_a");

    let args = ["--bin".to_owned(), "bin_a".to_owned()];

    let binary_path =
        cargo_flash::build_artifact(&work_dir, &args).expect("Failed to get artifact path.");

    assert_eq!(binary_path, expected_path);
}

#[test]
fn library_with_example() {
    // In a library with no binary target, but with an example,
    // we should return an error. (Same behaviour as cargo run)
    let work_dir = test_project_dir("library_with_example_project");

    let args = [];

    let binary_path = cargo_flash::build_artifact(&work_dir, &args);

    assert!(binary_path.is_err())
}

#[test]
fn library_with_example_specified() {
    // When the example flag is specified, we should flash that example
    let work_dir = test_project_dir("library_with_example_project");
    let expected_path = work_dir.join("target/debug/examples/example");

    let args = owned_args(&["--example", "example"]);

    let binary_path =
        cargo_flash::build_artifact(&work_dir, &args).expect("Failed to get artifact path.");

    assert_eq!(binary_path, expected_path);
}

/// Return the path to a test project, located in
/// tests/data.
fn test_project_dir(test_name: &str) -> PathBuf {
    let mut manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    manifest_dir.push("tests");
    manifest_dir.push("data");

    manifest_dir.join(test_name)
}

fn owned_args(args: &[&str]) -> Vec<String> {
    args.iter().map(|s| (*s).to_owned()).collect()
}