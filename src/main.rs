use std::collections::HashSet;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    assert!(args.len() > 1);
    let target_dir = Path::new(&args[1]);
    println!("target_dir is {:?}", target_dir);

    // Make the sets
    let name_edge_map: HashSet<String> =
        ["4k", "1080p", "720p"].iter().map(|val| String::from(*val)).collect();
    let capitalize_exclusion_map: HashSet<String> =
        ["and", "at", "around", "by", "after", "along", "for", "from", "of", "on", "the", "to", "with", "without"]
        .iter().map(|val| String::from(*val)).collect();

    let dir = Path::read_dir(target_dir).unwrap();
    for file_res in dir {
       let file = file_res.unwrap();
       let mut do_rename = false;
       if file.file_type().unwrap().is_file() {
            let name = file.file_name().into_string().unwrap();
            let split = name.split(".");
            let ext = Path::new(&name).extension().unwrap();
            let mut new_name_vec = Vec::new();
            for piece in split {
                if !name_edge_map.contains(&piece.to_lowercase()) {
                    let mut do_cap = true;
                    if !new_name_vec.is_empty() {
                        new_name_vec.push(" ".to_owned());
                        if capitalize_exclusion_map.contains(&piece.to_lowercase()) {
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
                    do_rename = true;
                    break;
                }
            }
            // Wrap the last value (the year) in parenthesis
            let year = format!("({})", new_name_vec.pop().unwrap());
            new_name_vec.push(year);

            // Put the extension back
            new_name_vec.push(format!(".{}", ext.to_str().unwrap()));

            // If we didn't hit an expected delimiter, skip out on renaming
            if do_rename {
                new_name_vec.push(ext.to_str().unwrap().to_owned());

                let new_name: String = new_name_vec.iter().cloned().collect();
                let new_path_dir = target_dir.clone();
                let new_path = new_path_dir.join(new_name);

                let old_path_dir = target_dir.clone();
                let old_path = old_path_dir.join(name);

                // Rename the file
                std::fs::rename(old_path, new_path).unwrap();
            }
            else {
                println!("Not renaming file {}, not all criteria met", name);
            }
       }
    }
}

fn capitalize(word: &str) -> String {
    let mut v: Vec<char> = word.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    v.into_iter().collect()
}
