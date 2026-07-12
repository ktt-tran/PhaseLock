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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_password_and_salt_produce_same_key() {
        let salt = generate_salt();

        let key1 =
            derive_password_key("test-password", &salt)
                .unwrap();

        let key2 =
            derive_password_key("test-password", &salt)
                .unwrap();

        assert_eq!(
            key1.as_bytes(),
            key2.as_bytes()
        );
    }

    #[test]
    fn different_passwords_produce_different_keys() {
        let salt = generate_salt();

        let key1 =
            derive_password_key("password-one", &salt)
                .unwrap();

        let key2 =
            derive_password_key("password-two", &salt)
                .unwrap();

        assert_ne!(
            key1.as_bytes(),
            key2.as_bytes()
        );
    }

    #[test]
    fn different_salts_produce_different_keys() {
        let salt1 = generate_salt();
        let salt2 = generate_salt();

        let key1 =
            derive_password_key("same-password", &salt1)
                .unwrap();

        let key2 =
            derive_password_key("same-password", &salt2)
                .unwrap();

        assert_ne!(
            key1.as_bytes(),
            key2.as_bytes()
        );
    }
}