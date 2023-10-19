use std::env;


fn main() {
    let args: Vec<_> = env::args().skip(1).collect::<Vec<_>>();
    println!("{}", args.join(" "));
}
