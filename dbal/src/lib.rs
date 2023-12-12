use chacha20poly1305::aead::{Aead, OsRng};
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, Key, KeyInit, Nonce};
use pbkdf2::hmac::Hmac;
use sha2::Sha512;

use bamboo_error::*;

mod authentication;
mod character;
mod character_housing;
mod crafter;
mod custom_field;
mod event;
mod fighter;
mod free_company;
mod my;
mod user;

pub mod prelude {
    pub mod dbal {
        pub use crate::authentication::*;
        pub use crate::character::*;
        pub use crate::character_housing::*;
        pub use crate::crafter::*;
        pub use crate::custom_field::*;
        pub use crate::event::*;
        pub use crate::fighter::*;
        pub use crate::free_company::*;
        pub use crate::my::*;
        pub use crate::user::*;
    }
}

fn get_passphrase(passphrase: &[u8]) -> BambooResult<Key> {
    let mut key = [0_u8; 32];
    pbkdf2::pbkdf2::<Hmac<Sha512>>(
        passphrase,
        std::env::var("DATABASE_URL")
            .unwrap_or("f47ac10b-58cc-4372-a567-0e02b2c3d479".into())
            .as_bytes(),
        12,
        &mut key,
    )
    .map_err(|_| BambooError::crypto("encryption", "Failed to create pbkdf2 key"))?;

    Ok(Key::from(key))
}

pub(crate) fn decrypt_string(encrypted: Vec<u8>, passphrase: String) -> BambooResult<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(&get_passphrase(passphrase.as_bytes())?);
    let nonce = Nonce::from_slice(&encrypted[..12]);

    let decrypted = cipher
        .decrypt(nonce, encrypted[12..].as_ref())
        .map_err(|_| BambooError::crypto("encryption", "Failed to decrypt"))?;

    Ok(decrypted)
}

pub(crate) fn encrypt_string(plain: Vec<u8>, passphrase: String) -> BambooResult<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(&get_passphrase(passphrase.as_bytes())?);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let encrypted = cipher
        .encrypt(&nonce, plain.as_ref())
        .map_err(|_| BambooError::crypto("encryption", "Failed to encrypt"))?;

    let mut data = vec![];
    data.extend_from_slice(&nonce);
    data.extend(encrypted);

    Ok(data)
}
