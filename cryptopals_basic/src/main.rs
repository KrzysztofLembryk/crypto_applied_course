use cryptopals_basic::hex_to_base64::hex_to_base64;

fn main()
{
    // should output: 
    // SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
    let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    hex_to_base64(hex_str);
}