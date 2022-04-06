pub mod encryption_util {
    use bcrypt::{verify, hash, DEFAULT_COST};

    pub fn encrypt(password: &str) -> String {
        return hash(password, DEFAULT_COST).unwrap()
    }

    pub fn verify_pass(password: &str, hash: &str) -> bool {
        return verify(password, hash).unwrap()
    }

}