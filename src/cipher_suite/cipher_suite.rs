use crate::cipher_suite::CipherSuite;
use crate::ParentType;
use std::convert::Into;

#[derive(Debug, PartialEq)]
pub enum CipherSuiteType {
    TLS_AES_128_GCM_SHA256,
    TLS_AES_256_GCM_SHA384,
    TLS_CHACHA20_POLY1305_SHA256,
    TLS_AES_128_CCM_SHA256,
    TLS_AES_128_CCM_8_SHA256,
}

impl CipherSuiteType {
    fn get_vec(value:Vec<u8>)->Self{
        // match value {
        //     vec![0x13, 0x1] =>
        //         CipherSuiteType::TLS_AES_128_GCM_SHA256,
        //     vec![0x13, 0x2] =>
        //         CipherSuiteType::TLS_AES_256_GCM_SHA384,
        //     vec![0x13, 0x3] =>
        //         CipherSuiteType::TLS_CHACHA20_POLY1305_SHA256,
        //     vec![0x13, 0x4] =>
        //         CipherSuiteType::TLS_AES_128_CCM_SHA256,
        //     vec![0x13, 0x5] =>
        //         CipherSuiteType::TLS_AES_128_CCM_8_SHA256,
        // };


        CipherSuiteType::TLS_AES_128_CCM_8_SHA256
    }
}

impl Into<Vec<u8>> for CipherSuiteType {
    fn into(self) -> Vec<u8> {
        match self {
            CipherSuiteType::TLS_AES_128_GCM_SHA256 => {
                vec![0x13, 0x1]
            }
            CipherSuiteType::TLS_AES_256_GCM_SHA384 => {
                vec![0x13, 0x2]
            }
            CipherSuiteType::TLS_CHACHA20_POLY1305_SHA256 => {
                vec![0x13, 0x3]
            }
            CipherSuiteType::TLS_AES_128_CCM_SHA256 => {
                vec![0x13, 0x4]
            }
            CipherSuiteType::TLS_AES_128_CCM_8_SHA256 => {
                vec![0x13, 0x5]
            }
        }
    }
}

impl From<Vec<u8>> for CipherSuiteType {
    fn from(value: Vec<u8>) -> Self {
        Self::get_vec(value)
    }
}

impl Clone for CipherSuiteType {
    fn clone(&self) -> Self {
        match self {
            CipherSuiteType::TLS_AES_128_GCM_SHA256 => CipherSuiteType::TLS_AES_128_GCM_SHA256,
            CipherSuiteType::TLS_AES_256_GCM_SHA384 => CipherSuiteType::TLS_AES_256_GCM_SHA384,
            CipherSuiteType::TLS_CHACHA20_POLY1305_SHA256 => {
                CipherSuiteType::TLS_CHACHA20_POLY1305_SHA256
            }
            CipherSuiteType::TLS_AES_128_CCM_SHA256 => CipherSuiteType::TLS_AES_128_CCM_8_SHA256,
            CipherSuiteType::TLS_AES_128_CCM_8_SHA256 => CipherSuiteType::TLS_AES_128_CCM_8_SHA256,
        }
    }
}

impl ParentType for CipherSuiteType {}

impl CipherSuite for CipherSuiteType {
    type SelfFrom = Self;

    // fn from_vec(vec: Vec<u8>) -> Self::SelfFrom {
    //     Self::SelfFrom::from(vec)
    // }

    fn to_vec(self) -> Vec<u8> {
        Self::SelfFrom::into(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_cipher_suite() {
        let res = CipherSuiteType::from(vec![0x13, 0x5]);
        assert_eq!(res, CipherSuiteType::TLS_AES_256_GCM_SHA384);
    }
}
