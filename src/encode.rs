//Encoding module
use std::{collections::HashMap, fs, io::Write};

pub struct Encoder {
    bit_buffer: BitBuffer,
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            bit_buffer: BitBuffer::default(),
        }
    }

    pub fn to_file(
        &mut self,
        src: &str,
        dest: &str,
        dict: &HashMap<char, String>,
    ) -> std::io::Result<()> {
        let binding = fs::read_to_string(src).unwrap();
        let content = binding.chars();

        let mut outfile = fs::File::create(dest)?;

        for c in content {
            let code = dict[&c].clone();
            self.bit_buffer.read_code(&code);

            if self.bit_buffer.buffer.len() >= 32 {
                let bytes = self.bit_buffer.consume();
                outfile.write_all(&bytes)?;
            }
        }

        let bytes = self.bit_buffer.consume_remaining();
        outfile.write_all(&bytes)?;

        Ok(())
    }
}

struct BitBuffer {
    buffer: String,
}

impl BitBuffer {
    fn default() -> Self {
        Self {
            buffer: String::from(""),
        }
    }

    fn read_code(&mut self, code: &str) {
        //assert!(self.size < self.capacity);

        self.buffer.push_str(code);
    }

    fn consume(&mut self) -> [u8; 4] {
        assert!(self.buffer.len() >= 32);
        let (left, right) = self.buffer.split_at(32);
        let bytes = str_to_i8(left).to_ne_bytes();

        self.buffer = String::from(right);

        bytes
    }

    fn consume_remaining(&mut self) -> [u8; 4] {
        assert!(self.buffer.len() < 32);
        let bytes = str_to_i8(&self.buffer).to_ne_bytes();

        self.buffer.clear();

        bytes
    }
}

pub fn str_to_i8(s: &str) -> u32 {
    let mut n: u32 = 0;

    //assume str slice len is 32
    for (i, c) in s.chars().enumerate() {
        n += c.to_digit(2).unwrap();
        if i != s.len() - 1 {
            n = n << 1;
        };
    }

    //padding for trailing bits
    let pad = s.len() % 32;
    n = n << pad;

    n
}
