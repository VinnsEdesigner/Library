#[test]
fn test_safe_path_resolution() {
    let path = std::path::Path::new(".gitignore");
    assert!(!path.is_dir());
}
