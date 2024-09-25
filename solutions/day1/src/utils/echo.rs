pub fn main(args: Vec<String>) {
    let args: Vec<String> = args.into_iter().skip(1).collect();
    println!("{}", args.join(" "));
}
