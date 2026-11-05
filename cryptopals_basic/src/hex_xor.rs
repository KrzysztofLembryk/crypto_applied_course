use hex;

pub fn xor_hex(hex_1: &str, hex_2: &str) -> Result<Vec<u8>, String>
{
    let raw_hex1 = hex::decode(hex_1).unwrap();
    let raw_hex2 = hex::decode(hex_2).unwrap();

    return _xor_hex(&raw_hex1, &raw_hex2);
}

pub fn _xor_hex(bytes_1: &[u8], bytes_2: &[u8]) -> Result<Vec<u8>, String>
{

    if bytes_1.len() != bytes_2.len()
    {
        return Err(format!("bytes_1.len() != bytes_2.len(), {} != {}", bytes_1.len(), bytes_2.len()));
    }
    let n = bytes_1.len();
    let mut res = Vec::with_capacity(n);

    for i in 0..n
    {
        let xored = bytes_1[i] ^ bytes_2[i];
        res.push(xored);
    }

    return Ok(res);
}