#[cfg(test)]
mod tests {
    use crate::gf256_mul;

    #[test]
    fn test_gf256_mul_standard() {
        let a: u8 = 0x57;
        let b: u8 = 0x83;
        let out: u8 = 0xC1;
        assert_eq!(gf256_mul(0x57, 0x83), 0xC1);
        println!("a = {}, b = {}, output = {}", a, b, out)
    }

    #[test]
    fn test_gf256_mul_zero() {
        let a: u8 = 0x00;
        let b: u8 = 0x00;
        let out: u8 = 0x00;
        assert_eq!(gf256_mul(0x00, 0x00), 0x00);
        println!("a = {}, b = {}, output = {}", a, b, out)
    }
}