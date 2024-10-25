mod tests;
mod matrix;
mod aes;

use aes::AES;
use matrix::Matrix;

fn main() {
    // Plaintext and roundkey
    // Student ID 020176095A
    // Last four bytes: 01 33 DC DF (ignore A)
    let plaintext_array: [u8; 16] = [
        0x01, 0x23, 0x45, 0x67, 
        0x89, 0xab, 0xcd, 0xef, 
        0xfe, 0xdc, 0xba, 0x98, 
        0x76, 0x54, 0x32, 0x10
    ];
    let round_key_array: [u8; 16] = [
        0x0f, 0x15, 0x71, 0xc9, 
        0x47, 0xd9, 0xe8, 0x59, 
        0x0c, 0xb7, 0xad, 0xd6, 
        0x01, 0x33, 0xdc, 0xdf
    ];
    // Start AES operations
    let plaintext_matrix = Matrix::from_array(plaintext_array);
    let roundkey_matrix = Matrix::from_array(round_key_array);
    let mut aes = AES::from_matrix(plaintext_matrix);
    aes.round_trans(&roundkey_matrix);
}
