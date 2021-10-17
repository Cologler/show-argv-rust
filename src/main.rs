
fn main() {
    std::env::args().enumerate().for_each(|(idx, arg)| {
        println!("{}: {:?}", idx, arg);
    });
    std::io::stdin().read_line(&mut String::new()).unwrap_or_default();
}
