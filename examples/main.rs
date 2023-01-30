use opencc_rs::{Config, OpenCC};

fn main() {
    let opencc = OpenCC::new([Config::S2T]).unwrap();
    println!("{}", opencc.convert("汉字").unwrap());
}
