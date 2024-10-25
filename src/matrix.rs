pub struct Matrix {
    // 4x4 byte Matrix
    data: [[u8; 4]; 4], 
}

impl Matrix {
    // Empty constructor
    pub fn new() -> Self {
        Matrix {
            data: [[0u8; 4]; 4],
        }
    }

    // Construct from array
    pub fn from_array(array: [u8; 16]) -> Self {
        let mut matrix = Self::new();
        for i in 0..4 {
            for j in 0..4 {
                matrix.data[j][i] = array[i * 4 + j];
            }
        }
        matrix
    }

    // Access element
    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.data[row][col]
    }

    // Set element
    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        self.data[row][col] = value;
    }

    // Convert matrix back to array (column major for AES)
    pub fn to_array(&self) -> [u8; 16] {
        let mut array = [0u8; 16];
        for col in 0..4 {
            for row in 0..4 {
                array[col * 4 + row] = self.data[row][col];
            }
        }
        array
    }

    // Print matrix
    pub fn print(&self) {
        for row in self.data.iter() {
            for &val in row.iter() {
                print!("{:02x} ", val);
            }
            println!();
        }
        println!();
    }

    // Rotate data to the left
    pub fn rotate_left(&mut self, row: usize, position: usize) {
        self.data[row].rotate_left(position);
    }
}