use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let dir = Path::read_dir(args[0].as_path()).unwrap();
    for file in dir {

    }
}
