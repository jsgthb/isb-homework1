#[cfg(test)]
use crate::aes::AES;
#[cfg(test)]
use crate::matrix::Matrix;

mod tests {
    #[cfg(test)]
    use super::*;
    
    #[test]
    fn test_aes_gf256_mul_standard() {
        let a: u8 = 0x57;
        let b: u8 = 0x83;
        let out: u8 = 0xC1;
        assert_eq!(AES::gf256_mul(0x57, 0x83), 0xC1);
        println!("a = {}, b = {}, output = {}", a, b, out)
    }

    #[test]
    fn test_aes_gf256_mul_zero() {
        let a: u8 = 0x00;
        let b: u8 = 0x00;
        let out: u8 = 0x00;
        assert_eq!(AES::gf256_mul(0x00, 0x00), 0x00);
        println!("a = {}, b = {}, output = {}", a, b, out)
    }

    #[test]
    fn test_matrix_from_array() {
        let array_last_element = 0xF2;
        let array: [u8; 16] = [
            0x23, 0x00, 0x00, 0xBB,
            0x00, 0x3C, 0x21, 0x00,
            0x00, 0x3D, 0x44, 0x00,
            0x20, 0x00, 0x00, array_last_element
        ];
        let matrix = Matrix::from_array(array);
        let matrix_last_element = matrix.get(3, 3);
        assert_eq!(matrix_last_element, array_last_element);
        println!("Array = {:x?}\nMatrix =", array);
        matrix.print();
    }

    #[test]
    fn test_aes_add_round_key() {
        let plaintext_array: [u8; 16] = [
            0x23, 0x00, 0x00, 0x00,
            0x00, 0x3C, 0x00, 0x00,
            0x00, 0x00, 0x44, 0x00,
            0x00, 0x00, 0x00, 0xF2
        ];
        let round_key_array: [u8; 16] = [
            0x12, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x45
        ];
        let plaintext_matrix = Matrix::from_array(plaintext_array);
        let round_key_matrix = Matrix::from_array(round_key_array);
        let mut aes = AES::from_matrix(plaintext_matrix);
        aes.add_round_key(&round_key_matrix);
        let matrix_last_element = aes.state.get(3, 3);
        assert_eq!(matrix_last_element, 0xb7);
        println!("Plaintext = {:x?}\nRound key = {:x?}", plaintext_array, round_key_array);
        println!("AES state =");
        aes.print_state();
    }

    #[test]
    fn test_aes_sub_bytes() {
        let plaintext_array: [u8; 16] = [
            0x23, 0x00, 0x00, 0x00,
            0x00, 0x3C, 0x00, 0x00,
            0x00, 0x00, 0x44, 0x00,
            0x00, 0x00, 0x00, 0xF2
        ];
        let plaintext_matrix = Matrix::from_array(plaintext_array);
        let mut aes = AES::from_matrix(plaintext_matrix);
        aes.sub_bytes();
        assert_eq!(aes.state.get(0, 0), 0x26);
        println!("Plaintext = {:x?}\nSubstituted AES state =", plaintext_array);
        aes.print_state();
    }

    #[test]
    fn test_matrix_rotate_left() {
        let plaintext_array: [u8; 16] = [
            0x23, 0x00, 0x00, 0x00,
            0x00, 0x3C, 0x00, 0x00,
            0x00, 0x00, 0x44, 0x00,
            0x00, 0x00, 0x00, 0xF2
        ];
        let mut plaintext_matrix = Matrix::from_array(plaintext_array);
        println!("Matrix before rotation =");
        plaintext_matrix.print();
        plaintext_matrix.rotate_left(3, 3);
        println!("Matrix after rotation =");
        plaintext_matrix.print();
        assert_eq!(plaintext_matrix.get(3, 0), 0xF2);
    }

    #[test]
    fn test_aes_mix_columns() {
        let plaintext_array: [u8; 16] = [
            0x23, 0x00, 0x00, 0x00,
            0x00, 0x3C, 0x00, 0x00,
            0x00, 0x00, 0x44, 0x00,
            0x00, 0x00, 0x00, 0xF2
        ];
        let plaintext_matrix = Matrix::from_array(plaintext_array);
        let mut aes = AES::from_matrix(plaintext_matrix);
        println!("AES state before column mixing =");
        aes.print_state();
        aes.mix_columns();
        println!("AES state after column mixing =");
        aes.print_state();
        assert_eq!(aes.state.get(3, 3), 0xFF);
    }

    #[test]
    fn test_aes_round_trans() {
        // Plaintext and roundkey
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
            0xaf, 0x7f, 0x67, 0x98
        ];
        // Start AES operations
        let plaintext_matrix = Matrix::from_array(plaintext_array);
        let roundkey_matrix = Matrix::from_array(round_key_array);
        let mut aes = AES::from_matrix(plaintext_matrix);
        aes.round_trans(&roundkey_matrix);
        let ciphertext_generated = aes.state.to_array();
        let ciphertext_expected = [
            0xb9, 0xe4, 0x47, 0xc5, 
            0x94, 0x8e, 0x20, 0xd6, 
            0x57, 0x16, 0x9a, 0xf5, 
            0x75, 0x51, 0x3f, 0x3b
        ];
        println!("Expected ciphertext: {:x?}\n", ciphertext_expected);
        assert_eq!(ciphertext_generated, ciphertext_expected)
    }
}