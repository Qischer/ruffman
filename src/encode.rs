//Encoding module
use std::{collections::HashMap, fs, io::Write};

pub struct Encoder {
    bit_buffer: BitBuffer,
}

impl Encoder {
    pub fn default() -> Self {
        Self {
            bit_buffer: BitBuffer::default(),
        }
    }

    pub fn to_file(
        &mut self,
        src: &str,
        dest: &str,
        dict: &HashMap<char, String>,
        freq: &HashMap<char, usize>,
    ) -> std::io::Result<()> {
        let content = fs::read_to_string(src)?;
        let mut outfile = fs::File::create(dest)?;

        self.encode_header(&mut outfile, freq)?;
        self.encode_content(&mut outfile, &content, dict)?;

        Ok(())
    }

    fn encode_header(
        &self,
        file: &mut fs::File,
        freq: &HashMap<char, usize>,
    ) -> std::io::Result<()> {
        let n = freq.len();
        file.write(&n.to_ne_bytes())?;

        for (k, v) in freq.iter() {
            file.write(&k.to_string().as_bytes()[..1])?;
            file.write(&v.to_ne_bytes())?;
        }

        Ok(())
    }

    fn encode_content(
        &mut self,
        file: &mut fs::File,
        content: &str,
        dict: &HashMap<char, String>,
    ) -> std::io::Result<()> {
        for c in content.chars() {
            let code = dict[&c].clone();
            self.bit_buffer.read_code(&code);

            if self.bit_buffer.buffer.len() >= 32 {
                let bytes = self.bit_buffer.consume();
                file.write_all(&bytes)?;
            }
        }

        let bytes = self.bit_buffer.consume_remaining();
        file.write_all(&bytes)?;

        Ok(())
    }
}

pub struct BitBuffer {
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
        let bytes = str_to_u32(left).to_ne_bytes();

        self.buffer = String::from(right);

        bytes
    }

    fn consume_remaining(&mut self) -> [u8; 4] {
        assert!(self.buffer.len() < 32);
        let bytes = str_to_u32(&self.buffer).to_ne_bytes();

        self.buffer.clear();

        bytes
    }
}

pub fn str_to_u32(s: &str) -> u32 {
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
