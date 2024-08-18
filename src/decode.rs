use super::tree;
use std::{collections::HashMap, fs::File, io::prelude::*};

pub struct Decoder {}

impl Decoder {
    pub fn default() -> Self {
        Self {}
    }

    pub fn from_file(&self, src: &str) -> std::io::Result<()> {
        let mut infile = std::fs::File::open(src)?;

        let freq = self.decode_header(&mut infile)?;

        for (k, v) in freq {
            println!("{k} : {v}");
        }

        Ok(())
    }

    fn decode_header(&self, file: &mut File) -> std::io::Result<HashMap<char, usize>> {
        const NUM: usize = 1;
        let mut buf = [0; NUM];
        let mut freq = HashMap::new();

        let _ = file.read(&mut buf)?;
        let n = &buf[0];

        for _ in 0..*n {
            let mut cbuf = [0; 8];
            let mut nbuf = [0; 1];

            let _ = file.read(&mut cbuf)?;
            let _ = file.read(&mut nbuf)?;

            let ch = String::from_utf8_lossy(&cbuf);
            let fr = nbuf[0];

            freq.insert(
                ch.chars()
                    .last()
                    .expect("Can't convert byte buffer to char"),
                fr.into(),
            );
        }

        Ok(freq)
    }
    fn decode_content(&self) -> std::io::Result<()> {
        todo!();
    }
}
