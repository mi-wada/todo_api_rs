pub(crate) fn hash_password(password: &str) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}

pub(crate) fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "password";
        let hash = hash_password(password);
        assert!(verify_password(password, &hash));
    }

    #[test]
    fn test_hash_password_use_salt() {
        let password = "password";
        assert_ne!(hash_password(password), hash_password(password));
    }

    #[test]
    fn test_verify_password_success() {
        let password = "password";
        let hash = hash_password(password);
        assert!(verify_password(password, &hash));
    }

    #[test]
    fn test_verify_password_fail() {
        let password = "password";
        let hash = hash_password(password);

        assert!(!verify_password("wrong_password", &hash));
        assert!(!verify_password("Password", &hash));
    }
}
