
fn main() {
    std::env::args().enumerate().for_each(|(idx, arg)| {
        println!("{}: {:?}", idx, arg);
    });
}
