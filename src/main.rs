use std::collections::HashMap;
use std::env;
use std::fs::read_dir;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("card-counter may have only 1 argument (the directory that contains a number of gemp deck files) or 0 (use the current directory.)");
        return;
    }

    if args.len() == 2 && (args[1] == "help" || args[1] == "-h" || args[1] == "--help") {
        println!("card-counter counts the max number of any one card across all decks. It then outputs a new deck file with the max number of each card.");
        println!("an optional argument specifies a directory to read deck files from. If no argument is specified, the current directory is used.");
        return;
    }

    let directory_name = if args.len() == 1 { "." } else { &args[1] };

    let mut file_maps: Vec<HashMap<String, i32>> = Vec::new();
    match read_dir(directory_name) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                if let Some(extension) = entry.path().extension() {
                    if extension == "txt" {
                        let mut file = File::open(entry.path()).expect("Unable to open file");
                        let mut contents = String::new();
                        file.read_to_string(&mut contents)
                            .expect("Unable to read file");
                        let mut file_map: HashMap<String, i32> = HashMap::new();
                        for line in contents.lines() {
                            if !(line.trim_start().starts_with("<?xml")
                                && line.trim_end().ends_with(">"))
                                && !(line.starts_with("<deck>") || line.starts_with("</deck>"))
                                && !line.trim_start().starts_with("<cardOutsideDeck")
                            {
                                file_map
                                    .entry(line.to_string())
                                    .and_modify(|e| *e += 1)
                                    .or_insert(1);
                            }
                        }
                        file_maps.push(file_map);
                    }
                }
            }
        }
        Err(err) => println!("Error reading directory: {}", err),
    }

    let mut final_map: HashMap<String, i32> = HashMap::new();
    let file_maps_clone = file_maps.clone();

    for file_map in file_maps {
        for (key, value) in file_map {
            let counter = final_map.entry(key).or_insert(0);
            *counter = i32::max(*counter, value);
        }
    }

    use std::fs::OpenOptions;
    let mut file_number = 0;
    let file;
    loop {
        let filename = if file_number == 0 {
            "output.txt".to_string()
        } else {
            format!("output-{}.txt", file_number)
        };

        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&filename)
        {
            Ok(opened_file) => {
                file = opened_file;
                break;
            }
            Err(_) => {
                file_number += 1;
            }
        }
    }

    let mut writer = std::io::BufWriter::new(&file);
    write!(
        writer,
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n<deck>\n"
    )
    .expect("Unable to write to output.txt");

    for (key, value) in &final_map {
        for _ in 0..*value {
            writeln!(writer, "{}", key).expect("Unable to write to output.txt");
        }
    }
    writeln!(writer, "</deck>").expect("Unable to write to output.txt");

    let number_of_files = file_maps_clone.len();
    let number_of_keys = final_map.len();
    let sum_values: i32 = final_map.values().sum();

    println!("Number of decks processed: {}", number_of_files);
    println!("Number of unique cards: {}", number_of_keys);
    println!("Total number of cards: {}", sum_values);
}
