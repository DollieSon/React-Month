fn main() {
    let temp_str: String = "1 2".to_string();
    let something = temp_str.split(' ');
    let s2 = something.map(|str| str.parse::<u32>().unwrap());
    let s3 = s2.collect::<Vec<u32>>();
    println!("Hello, world!");
}
