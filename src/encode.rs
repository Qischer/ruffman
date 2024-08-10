//Encoding module
use std::{collections::HashMap, fs, io::Write};

pub fn to_file(src: &str, dest: &str, dict: &HashMap<char, String>) -> std::io::Result<()> {
    let binding = fs::read_to_string(src).unwrap();
    let content = binding.chars();

    let mut outfile = fs::File::create(dest)?;

    let mut b = BitBuffer::default();

    println!("source text: {binding}");

    for c in content {
        let code = dict[&c].clone();
        print!("{code}");
        b.read_code(&code);

        if b.buffer.len() >= 32 {
            let bytes = b.consume();
            outfile.write_all(&bytes)?;
        }
    }
    println!();

    Ok(())
}

struct BitBuffer {
    size: usize,
    capacity: usize,
    buffer: String,
}

impl BitBuffer {
    fn default() -> Self {
        Self {
            size: 0,
            capacity: 64,
            buffer: String::from(""),
        }
    }

    fn read_code(&mut self, code: &str) {
        //assert!(self.size < self.capacity);

        self.buffer.push_str(code);
        self.size += code.len();
    }

    fn consume(&mut self) -> [u8; 4] {
        assert!(self.buffer.len() >= 32);
        let (left, right) = self.buffer.split_at(32);
        let bytes = str_to_i8(left).to_ne_bytes();

        self.buffer = String::from(right);

        bytes
    }
}

pub fn str_to_i8(s: &str) -> u32 {
    assert_eq!(s.len(), 32);
    let mut n: u32 = 0;

    for (i, c) in s.chars().enumerate() {
        n += c.to_digit(2).unwrap();
        if i != s.len() - 1 {
            n = n << 1;
        };
    }

    n
}
