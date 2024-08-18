//Rust Huffman Encoding
use std::{env, io::prelude::*, process};

mod decode;
mod encode;
mod tree;
fn print_usage() {
    println!("Usage: ruffman [mode] [arguments]");
    println!("-p\t[src]\t: Print huffman tree of file");
    println!("-pc\t[src]\t: Print huffman tree of encoded file");
    println!("-pn\t[src]\t: Print node array of file");
    println!("-e\t[src] [dest]\t: Encode source file to a specified destination");
    println!(
        "-ep\t[src] [dest]\t: Encode source file to a specified destination and print huffman tree"
    );
    println!("-d\t[src] [dest]\t: Decode source file to a specified destination");
    println!("-dp\t[src] [dest]\t: Decode source file to a specified destination and print huffmand tree");
    process::exit(0)
}

//Implement -p
fn print_huffman(src: &str) {
    println!("Print huffman tree from {src}");

    let mut huf = tree::Huffman::new_from_file(src);
    huf.translate();

    println!("{huf}");
}

//Implement -pc
fn print_huffman_encoded(src: &str) {
    println!("Print huffman tree from encoded file {src}");
}

//Implement -pn
fn print_node_array(src: &str) {
    println!("Print node array from {src}");

    let huf = tree::Huffman::new_from_file(src);
    let freq = huf.get_freq().unwrap();

    for (k, v) in freq.iter() {
        println!("{k}: {v}");
    }
}

//Implement -e
fn encode(src: &str, dest: &str) {
    println!("Encode {src} into {dest}");

    let mut huf = tree::Huffman::new_from_file(src);
    huf.translate();

    let dict = huf.get_dict().unwrap();
    let freq = huf.get_freq().unwrap();

    let mut encoder = encode::Encoder::default();
    let res = encoder.to_file(src, dest, &dict, &freq);

    let n = freq.len();
    println!("Unique characters: {:?}", n);
    match res {
        Ok(_) => println!("Successfully compressed!"),
        Err(e) => println!("Something went wrong. Error: {e:?}"),
    }
}

//Implement -ep
fn encode_and_print(src: &str, dest: &str) {
    println!("Encode {src} into {dest} and Print")
}

//Implement -d
fn decode(src: &str, dest: &str) {
    println!("Decode {src} into {dest}");

    let decoder = decode::Decoder::default();
    let res = decoder.from_file(src);

    match res {
        Ok(_) => println!("Successfully decoded!"),
        Err(e) => println!("Something went wrong. Error: {e:?}"),
    }
}

//Implement -dp
fn decode_and_print(src: &str, dest: &str) {
    println!("Decode {src} into {dest} and Print")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => match args[1].as_str() {
            "-p" => print_huffman(&args[2]),
            "-pc" => print_huffman_encoded(&args[2]),
            "-pn" => print_node_array(&args[2]),
            _ => print_usage(),
        },
        4 => match args[1].as_str() {
            "-e" => encode(&args[2], &args[3]),
            "-ep" => encode_and_print(&args[2], &args[3]),
            "-d" => decode(&args[2], &args[3]),
            "-dp" => decode_and_print(&args[2], &args[3]),
            _ => print_usage(),
        },
        _ => print_usage(),
    }
}
