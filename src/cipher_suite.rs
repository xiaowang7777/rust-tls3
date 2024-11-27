// pub(crate) trait Into {
//     fn to_vec(self) -> Vec<u8>;
// }
//
// pub(crate) trait From<T>
// where
//     T: Sized,
// {
//     fn from_vec(vec: Vec<u8>) -> T;
// }

pub(crate) trait CipherSuite {
    type SelfFrom: Sized + From<Vec<u8>>;
    fn from_vec(vec: Vec<u8>) -> Self::SelfFrom
    where
        Self: Sized,
    {
        Self::SelfFrom::from(vec)
    }
    fn to_vec(self) -> Vec<u8>;
}

mod cipher_suite;
pub use cipher_suite::*;
