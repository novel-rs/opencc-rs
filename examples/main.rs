use opencc_rs::{Config, OpenCC};

fn main() {
    let opencc = OpenCC::new(vec![Config::S2T]).unwrap();
    println!("{}", opencc.convert("汉字").unwrap());
}
