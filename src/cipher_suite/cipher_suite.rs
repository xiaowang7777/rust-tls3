use crate::cipher_suite::CipherSuite;

enum CipherSuiteType {
    TLS_AES_128_GCM_SHA256,
    TLS_AES_256_GCM_SHA384,
    TLS_CHACHA20_POLY1305_SHA256,
    TLS_AES_128_CCM_SHA256,
    TLS_AES_128_CCM_8_SHA256,
}

impl Into<Vec<u8>> for CipherSuiteType {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}

impl From<Vec<u8>> for CipherSuiteType {
    fn from(value: Vec<u8>) -> Self {
        todo!()
    }
}

impl CipherSuite for CipherSuiteType {
    fn from_vec(self: &Self, vec: Vec<u8>) -> Box<dyn CipherSuite> {
        Box::new(Self::from(vec))
    }

    fn to_vec(self) -> Vec<u8> {
        self.into()
    }
}
