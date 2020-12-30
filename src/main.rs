use std::collections::HashSet;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    let target_dir = Path::new(&args[0]);

    // Make the map
    let name_edge_map: HashSet<&'static str> =
        ["4k", "1080p", "720p"].iter().cloned().collect();
    let capitalize_exclusion_map: HashSet<&'static str> =
        ["and", "at", "around", "by", "after", "along", "for", "from", "of", "on", "the", "to", "with", "without"]
        .iter().cloned().collect();

    let dir = Path::read_dir(target_dir).unwrap();
    for file_res in dir {
       let file = file_res.unwrap();
       if file.file_type().unwrap().is_file() {
            let name = file.file_name().into_string().unwrap();
            let split = name.split(".");
            let ext = Path::new(&name).extension().unwrap();
            let mut new_name_vec = Vec::new();
            for piece in split {
                if !name_edge_map.contains(piece) {
                    let mut do_cap = true;
                    if !new_name_vec.is_empty() {
                        new_name_vec.push(" ".to_owned());
                        if capitalize_exclusion_map.contains(piece) {
                            do_cap = false;
                        }
                    }

                    let word = if do_cap {
                        capitalize(piece)
                    } else {
                        piece.to_owned()
                    };
                    new_name_vec.push(word);
                }
                else {
                    // Hit our delimiter, break out.
                    break;
                }
            }
            new_name_vec.push(ext.to_str().unwrap().to_owned());

            let new_name: String = new_name_vec.iter().cloned().collect();
            let new_path_dir = target_dir.clone();
            let new_path = new_path_dir.join(new_name);

            let old_path_dir = target_dir.clone();
            let old_path = old_path_dir.join(name);

            // Rename the file
            std::fs::rename(old_path, new_path).unwrap();
       }
    }
}

fn capitalize(word: &str) -> String {
    let mut v: Vec<char> = word.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    v.into_iter().collect()
}
