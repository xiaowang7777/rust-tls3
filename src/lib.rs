mod cipher_suite;
mod compression_methods;
mod extension;
mod handshake;

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
    res[2] = (data ) as u8;
    res
}
