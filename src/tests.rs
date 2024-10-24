#[cfg(test)]
mod tests {
    use crate::Matrix;
    use crate::AES;

    #[test]
    fn test_gf256_mul_standard() {
        let a: u8 = 0x57;
        let b: u8 = 0x83;
        let out: u8 = 0xC1;
        assert_eq!(AES::gf256_mul(0x57, 0x83), 0xC1);
        println!("a = {}, b = {}, output = {}", a, b, out)
    }

    #[test]
    fn test_gf256_mul_zero() {
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
}