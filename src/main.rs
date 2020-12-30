use std::collections::HashSet;
use std::path::Path;
use std::fs::File;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    // Make the map
    let name_edge_map: HashSet<&'static str> =
        ["4k", "1080p", "720p"].iter().collect();

    let dir = Path::read_dir(Path::new(&args[0])).unwrap();
    for file_res in dir {
       let file = file_res.unwrap();
       if file.file_type().unwrap().is_file() {
            let name = file.file_name().into_string().unwrap();
            let split = name.split(".");
            let ext = split.back();
            let new_name_vec = Vec::new();
            for piece in split {
                if !name_edge_map.contains(piece) {
                    if !new_name_vec.is_empty() {
                        new_name_vec.push(" ");
                    }
                    new_name_vec.push(piece);
                }
                else {
                    break;
                }
            }
            new_name_vec.push(ext);

            let new_name: String = new_name_vec.iter().collect();
       }
    }
}
