use cryptopals_basic::hex_to_base64::hex_to_base64;
use cryptopals_basic::hex_xor::{_xor_hex, xor_hex};
use cryptopals_basic::single_byte_xor_cipher::decrypt_byte_xor_cipher;

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

    println!("res: {}", res);
    println!("expected: {}", expected);

    if res == expected
    {
        println!("SUCCESS");
    }
    else 
    {
        println!("FAILURE");
    }
}

fn exmpl_decrypt_xor_byte_cipher()
{
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    decrypt_byte_xor_cipher(input);
}

fn main()
{
    // exmpl_hex_xor();
    exmpl_decrypt_xor_byte_cipher();
}