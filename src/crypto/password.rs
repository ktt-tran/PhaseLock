use argon2::{
    Algorithm,
    Argon2,
    Params,
    Version,
};
use rand::RngCore;
use zeroize::{Zeroize, ZeroizeOnDrop};

pub const SALT_SIZE: usize = 16;
pub const KEY_SIZE: usize = 32;

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct PasswordKey {
    bytes: [u8; KEY_SIZE],
}

impl PasswordKey {
    pub fn as_bytes(&self) -> &[u8; KEY_SIZE] {
        &self.bytes
    }
}

pub fn generate_salt() -> [u8; SALT_SIZE] {
    let mut salt = [0u8; SALT_SIZE];
    rand::rng().fill_bytes(&mut salt);
    salt
}

pub fn derive_password_key(
    password: &str,
    salt: &[u8; SALT_SIZE],
) -> Result<PasswordKey, argon2::Error> {

    let params = Params::new(
        64 * 1024, // 64 MiB memory
        3,         // 3 iterations
        1,         // 1 parallel lane
        Some(KEY_SIZE),
    )?;

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        params,
    );

    let mut salt_key_material = [0u8; KEY_SIZE];

    argon2.hash_password_into(
        password.as_bytes(),
        salt,
        &mut salt_key_material,
    )?;

    Ok(PasswordKey { bytes: salt_key_material })
}