use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);

    // ファイルが見つかりませんでした
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        // ファイルの読み込み中に問題がありました
        .expect("something went wrong reading the file");

    // テキストは\n{}です
    println!("文字数は{}です。", contents.chars().count());
    
    let mut new_line_count = 0;
    for c in contents.chars() {
        if c == '\n' {
            new_line_count += 1;
        }
    }
    println!("{}行の文章です。", new_line_count+1);
}
