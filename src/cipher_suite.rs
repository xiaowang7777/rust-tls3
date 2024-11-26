pub(crate) trait CipherSuite {
    fn from_vec(self: &Self, vec: Vec<u8>) -> Box<dyn CipherSuite>;
    fn to_vec(self) -> Vec<u8>;
}

mod cipher_suite;
