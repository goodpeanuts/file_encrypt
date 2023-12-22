use std::fs::File;
use std::io::{BufWriter, Write};

pub fn output_file() {
    let file = File::create("file.txt").expect("file not found");
    let mut writer = BufWriter::new(file);
    
    print!("Please input file size: ");
    std::io::stdout().flush().unwrap();

    // 输入一个数字
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("read failed");
    let input: usize = input.trim().parse().expect("parse failed");

    for i in 0..input {
        writer.write_all(format!("{}       good for test\n", i + 1).as_bytes()).expect("write failed");
    }
}