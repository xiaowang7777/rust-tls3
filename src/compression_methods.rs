pub(crate) trait CompressionMethod {
    fn to_vec(self) -> Vec<u8>;
}

mod methods;
