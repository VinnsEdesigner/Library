use sha2::{Sha256, Digest};

#[test]
fn test_hashing_consistency() {
    let mut hasher = Sha256::new();
    hasher.update(b"vyzorix");
    let hash = hex::encode(hasher.finalize());
    assert_eq!(hash.len(), 64);
}
