use cmd_lib::*;

fn main() {
    println!("Hello, world!");
    run_cmd!(echo "hi!").unwrap();
}
