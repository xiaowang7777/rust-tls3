pub(crate) mod _type {

    pub enum HandshakeType {
        ClientHello,
        ServerHello,
        NewSessionTicket,
        EndOfEarlyData,
        EncryptedExtensions,
        Certificate,
        CertificateRequest,
        CertificateVerify,
        Finished,
        KeyUpdate,
        MessageHash,
        Other,
    }

    impl From<u8> for HandshakeType {
        fn from(val: u8) -> Self {
            match val {
                1 => HandshakeType::ClientHello,
                2 => HandshakeType::ServerHello,
                4 => HandshakeType::NewSessionTicket,
                5 => HandshakeType::EndOfEarlyData,
                8 => HandshakeType::EncryptedExtensions,
                11 => HandshakeType::Certificate,
                13 => HandshakeType::CertificateRequest,
                15 => HandshakeType::CertificateVerify,
                20 => HandshakeType::Finished,
                24 => HandshakeType::KeyUpdate,
                254 => HandshakeType::MessageHash,
                _ => HandshakeType::Other,
            }
        }
    }

    impl Into<u8> for HandshakeType {
        fn into(self) -> u8 {
            match self {
                HandshakeType::ClientHello => 1,
                HandshakeType::ServerHello => 2,
                HandshakeType::NewSessionTicket => 4,
                HandshakeType::EndOfEarlyData => 5,
                HandshakeType::EncryptedExtensions => 8,
                HandshakeType::Certificate => 11,
                HandshakeType::CertificateRequest => 13,
                HandshakeType::CertificateVerify => 15,
                HandshakeType::Finished => 20,
                HandshakeType::KeyUpdate => 24,
                HandshakeType::MessageHash => 254,
                HandshakeType::Other => 255,
            }
        }
    }
}

mod data;
