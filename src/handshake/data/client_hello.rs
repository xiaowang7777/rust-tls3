use crate::{u8array2u16, ParentType, VectorData};
use rand::{RngCore, SeedableRng};

pub struct ClientHello<C, D, E>
where
    C: ParentType,
    D: ParentType,
    E: ParentType,
{
    legacy_version: [u8; 2],
    random: [u8; 32],
    session_id_length: u8,
    legacy_session_id: Vec<u8>,
    cipher_suites_length: [u8; 2],
    cipher_suites: VectorData<C>,
    legacy_compression_methods_length: u8,
    legacy_compression_methods: VectorData<D>,
    extensions_length: [u8; 2],
    extensions: VectorData<E>,
}

impl<C, D, E> ClientHello<C, D, E>
where
    C: ParentType,
    D: ParentType,
    E: ParentType,
{
    pub fn new(
        random: [u8; 32],
        session_id_length: u8,
        session_id: Vec<u8>,
        cipher_suites_length: [u8; 2],
        cipher_suites: Vec<Box<C>>,
        compression_methods_length: u8,
        legacy_compression_methods: Vec<Box<D>>,
        extensions_length: [u8; 2],
        extensions: Vec<Box<E>>,
    ) -> ClientHello<C, D, E> {
        ClientHello {
            legacy_version: [0x03, 0x03],
            random,
            session_id_length,
            legacy_session_id: session_id,
            cipher_suites_length,
            cipher_suites:VectorData::new(cipher_suites),
            legacy_compression_methods_length: compression_methods_length,
            legacy_compression_methods:VectorData::new(legacy_compression_methods),
            extensions_length,
            extensions:VectorData::new(extensions),
        }
    }

    pub fn new_from_extensions_with_cipher_suites(
        cipher_suite_list: Vec<Box<C>>,
        extensions: Vec<Box<E>>,
    ) -> ClientHello<C, D, E> {
        let mut random = rand::rngs::StdRng::from_entropy();
        let mut random_bytes = [0; 32];
        random.fill_bytes(&mut random_bytes);

        let mut session_id = [0; 32]; // 兼容，随机生成32字节随机数数
        random.fill_bytes(&mut session_id);

        let cipher_suite_count = cipher_suite_list.len() as u16;
        let extensions_count = extensions.len() as u16;

        ClientHello::new(
            random_bytes,
            32,
            session_id.to_vec(),
            cipher_suite_count.to_be_bytes(),
            cipher_suite_list,
            0,
            Vec::new(),
            extensions_count.to_be_bytes(),
            extensions,
        )
    }
}

impl<C, D, E> From<Vec<u8>> for ClientHello<C, D, E>
where
    C: ParentType,
    D: ParentType,
    E: ParentType,
{
    fn from(value: Vec<u8>) -> Self {
        todo!()
    }
}

impl<C, D, E> Into<Vec<u8>> for ClientHello<C, D, E>
where
    C: ParentType,
    D: ParentType,
    E: ParentType,
{
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
        res[1..4].copy_from_slice(crate::usize2u8array(len).as_slice());
        res[4..6].copy_from_slice(&self.legacy_version);
        res[6..38].copy_from_slice(&self.random);
        res[38] = self.session_id_length;

        let session_id_end = 39 + self.session_id_length as usize;
        res[39..session_id_end].copy_from_slice(&self.legacy_session_id);
        res[session_id_end..session_id_end + 2]
            .copy_from_slice(self.cipher_suites_length.as_slice());

        let cipher_suites_end =
            session_id_end + 2 + u8array2u16(&self.cipher_suites_length) as usize;
        res[session_id_end + 2..cipher_suites_end].copy_from_slice(&*self.cipher_suites.as_slice());

        res[cipher_suites_end] = self.legacy_compression_methods_length;
        let compression_methods_end = cipher_suites_end + 1 + self.legacy_compression_methods_length as usize;
        res[cipher_suites_end + 1..compression_methods_end].copy_from_slice(&*self.legacy_compression_methods.as_slice());

        res[compression_methods_end..compression_methods_end+2].copy_from_slice(self.extensions_length.as_slice());
        res[compression_methods_end+2..].copy_from_slice(&*self.extensions.as_slice());

        res
    }
}
