use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mode = &args[1];
    let path = &args[2];

    if mode == "p" {
        rscan::execute_parallel(path);
    } else if mode == "s" {
        rscan::execute_single(path);
    }


}
