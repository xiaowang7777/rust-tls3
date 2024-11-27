use std::ops::Deref;

mod cipher_suite;
mod compression_methods;
mod extension;
mod handshake;

trait ParentType: From<Vec<u8>> + Into<Vec<u8>> + Sized + Clone {}

pub struct VectorData<T>(Vec<Box<T>>)
where
    T: ParentType;

impl<T> VectorData<T>
where
    T: ParentType,
{
    pub(crate) fn as_slice(&self) -> Box<[u8]> {
        let mut res = Vec::new();

        let l = self
            .0
            .iter()
            .map(|item| (**item).clone().into())
            .collect::<Vec<Vec<u8>>>();
        for item in l.iter() {
            res.append(&mut item.clone());
        }

        res.into_boxed_slice()
    }

    pub(crate) fn new(data: Vec<Box<T>>) -> VectorData<T> {
        VectorData(data)
    }
}

// pub (crate) use handshake::_type;

fn u8array2u16(arr: &[u8; 2]) -> u16 {
    ((arr[0] as u16) << 8) | (arr[1] as u16)
}

fn u162u8array(data: u16) -> [u8; 2] {
    data.to_be_bytes()
}

fn usize2u8array(data: usize) -> [u8; 3] {
    let mut res: [u8; 3] = [0, 0, 0];
    res[0] = (data >> 16) as u8;
    res[1] = (data >> 8) as u8;
    res[2] = (data) as u8;
    res
}
