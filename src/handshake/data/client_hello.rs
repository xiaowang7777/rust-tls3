use crate::cipher_suite::CipherSuite;
use crate::compression_methods::CompressionMethod;
use crate::extension::Extension;
use crate::u8array2u16;

pub struct ClientHello {
    legacy_version: [u8; 2],
    random: [u8; 32],
    session_id_length: u8,
    legacy_session_id: Vec<u8>,
    cipher_suites_length: [u8; 2],
    cipher_suites: Vec<Box<dyn CipherSuite>>,
    legacy_compression_methods_length: u8,
    legacy_compression_methods:Vec<Box<dyn CompressionMethod>>,
    extensions_length: [u8; 2],
    extensions: Vec<Box<dyn Extension>>,
}

impl ClientHello {}

impl From<Vec<u8>> for ClientHello {
    fn from(value: Vec<u8>) -> Self {
        todo!()
    }
}

impl Into<Vec<u8>> for ClientHello {
    fn into(self) -> Vec<u8> {
        let len = 1 // Fixed Handshake Type
            + 3 // Message Length
            + 2 // Legacy Version
            + 32 // Random Length
            + 1 // Session ID Length
            + (self.session_id_length as usize) // Session Length
            + 2 // Cipher Suites Length
            + (u8array2u16(&self.cipher_suites_length) as usize) // Cipher Suites
            + 1 // compression_methods_length
            + (self.legacy_compression_methods_length as usize) // Compression Methods Length
            + 2 // Extensions Length
            + (u8array2u16(&self.extensions_length) as usize);

        let mut res = Vec::with_capacity(len);

        res[0] = crate::handshake::_type::HandshakeType::ClientHello as u8;
        // res[1..4] = crate::usize2u8array(len).into_vec();

        res
    }
}
