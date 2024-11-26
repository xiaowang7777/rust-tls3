pub(crate) trait CipherSuite {
    fn to_vec(self) -> Vec<u8>;
}

mod cipher_suite;