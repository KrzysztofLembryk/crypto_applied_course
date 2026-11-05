use cryptopals_basic::hex_to_base64::hex_to_base64;
use cryptopals_basic::hex_xor::{_xor_hex, xor_hex};

fn exmpl_hex_to_base64()
{
    // should output: 
    // SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
    let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let res = hex_to_base64(hex_str).unwrap();

    if res == expected
    {
        println!("Equal")
    }
    else
    {
        println!("expected: {}", expected);
        println!("res: {}", res);
    }
}

fn exmpl_hex_xor()
{
    let hex1 = "1c0111001f010100061a024b53535009181c";
    let hex2 = "686974207468652062756c6c277320657965";
    let expected = "746865206b696420646f6e277420706c6179";

    let res = xor_hex(hex1, hex2).unwrap();

    let res_str = hex::encode(res);

    println!("res: {}", res_str);
    println!("expected: {}", expected);

    if res_str == expected
    {
        println!("SUCCESS");
    }
    else 
    {
        println!("FAILURE");
    }
}

fn main()
{
    exmpl_hex_xor();
}