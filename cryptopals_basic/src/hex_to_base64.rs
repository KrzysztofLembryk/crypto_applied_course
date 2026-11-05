use hex;

fn u8_to_base64(val: u8) -> String
{
    if val == 63 {return "/".to_string();}
    if val == 62 {return "+".to_string();}
    if val <= 61 && val >= 52 {return (val - 52).to_string();}
    if val <= 51 && val >= 26 {return ((val + 71) as char).to_string();}
    if val <= 25 && val >= 0 {return ((val + 65) as char).to_string()}

    panic!("unsupported value u8_to_base64");
}

// Take the bit representation of the hex string (each hex char is 4 bits), 
// Then take those bits 6 at time (values from 0 to 63) and convert them into an 
// ascii characters: 
//  - values 0-25 to uppercase characters, 
//  - 26-51 to lowercase characters, 
//  - 52-61 to digits, 
//  - 62 to + 
//  - 63 to / (or - and _ in urls) 
pub fn hex_to_base64(hex_input: &str) -> Result<String, String>
{
    let mut base64_val: Vec<String> = Vec::with_capacity(hex_input.len() / 2);
    let mut bits_in_carry: u8 = 0;
    // We ensure that bits in carry are always
    let mut carry: u8 = 0;
    let raw_hex_bytes = hex::decode(hex_input).unwrap();
    let mut i = 0;
    let n = raw_hex_bytes.len();

    while i < n
    {
        let byte = raw_hex_bytes.get(i).unwrap();
        let val: u8;
        if bits_in_carry == 0
        {
            let hh = hh_mask(4) & byte;
            let lh = lh_mask(2) & byte;
            val = (hh | lh) >> 2;
            carry = ll_mask(2) & byte;
            bits_in_carry = 2;
        }
        else if bits_in_carry == 2
        {
            // carry is 0000 0011
            // after shift is 0011 0000
            carry = carry << 4;

            // hh is: 1111 0000
            let mut hh = hh_mask(4) & byte; 
            // after shift will be 0000 1111
            hh = hh >> 4;

            val = carry | hh;
            carry = lh_mask(4) & byte;
            bits_in_carry = 4;
        }
        else if bits_in_carry == 4
        {
            carry = carry << 2;
            let mut hh = hh_mask(2) & byte;
            hh = hh >> 6;

            val = carry | hh;

            carry = (hl_mask(2) | ll_mask(4)) & byte;
            bits_in_carry = 6;
            i -= 1;
        }
        else if bits_in_carry == 6
        {
            val = carry;
            carry = 0;
            bits_in_carry = 0;
        }
        else
        {
            return Err(format!("hex_to_base64:: bits_in_carry should never have: {}", bits_in_carry));
        }

        base64_val.push(u8_to_base64(val));
        i += 1;
    }

    match bits_in_carry {
        0 => {}

        2 => {
            // Two remaining bits become the high two bits of a Base64 value.
            base64_val.push(u8_to_base64(carry << 4));
            base64_val.push("=".to_string());
            base64_val.push("=".to_string());
        }

        4 => {
            // Four remaining bits become the high four bits of a Base64 value.
            base64_val.push(u8_to_base64(carry << 2));
            base64_val.push("=".to_string());
        }

        6 => {
            // This should normally be consumed by the next loop iteration.
            return Err(format!("unexpected six bits remaining"));
        }

        _ => {
            return Err(format!("invalid bits_in_carry: {}", bits_in_carry));
        }
    }

    return Ok(base64_val.concat())
}


// Returns mask for N high bits, starting from the highest bit
fn hh_mask(n: usize) -> u8
{
    if n == 1 {return 0x80;}    // 1000 0000
    if n == 2 {return 0xc0;}    // 1100 0000
    if n == 3 {return 0xe0;}    // 1110 0000
    if n == 4 {return 0xf0;}    // 1111 0000

    panic!("hh_mask:: unsupported n: {}", n);
}

// Returns mask for N high bits, starting from the lowest high bit
fn hl_mask(n: usize) -> u8
{
    if n == 1 {return 0x10;}    // 0001 0000
    if n == 2 {return 0x30;}    // 0011 0000
    if n == 3 {return 0x70;}    // 0111 0000
    if n == 4 {return 0xf0;}    // 1111 0000

    panic!("hl_mask:: unsupported n: {}", n);
}

// Returns mask for N low bits, starting from the lowest bit
fn ll_mask(n: usize) -> u8
{
    if n == 1 {return 0x01;}    // 0000 0001 
    if n == 2 {return 0x03;}    // 0000 0011 
    if n == 3 {return 0x07;}    // 0000 0111 
    if n == 4 {return 0x0f;}    // 0000 1111 

    panic!("ll_mask:: unsupported n: {}", n);
}

// Returns mask for N low bits, starting from the highest bit
fn lh_mask(n: usize) -> u8
{
    if n == 1 {return 0x08;}    // 0000 1000 
    if n == 2 {return 0x0c;}    // 0000 1100 
    if n == 3 {return 0x0e;}    // 0000 1110 
    if n == 4 {return 0x0f;}    // 0000 1111 

    panic!("lh_mask:: unsupported n: {}", n);
}