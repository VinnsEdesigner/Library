use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key
};

pub fn encrypt_secret(key: &[u8; 32], plaintext: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits entropy buffer
    if let Ok(ciphertext) = cipher.encrypt(&nonce, plaintext) {
        Some((ciphertext, nonce.to_vec()))
    } else {
        None
    }
}
