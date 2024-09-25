
mod utils;
use std::env;
use utils::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    // ทดลองใช้งาน echo
    // utils::echo::main(args);

    // ทดลองใช้งาน cat กับ test.txt
    // utils::cat::main(args);

    // ทดลองใช้งาน ls
    // utils::ls::main(args);
}
