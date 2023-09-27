use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let directory = &args[1];

    let mut map_arr = Vec::new();

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension() == Some(std::ffi::OsStr::new("txt")) {
                    let mut file = File::open(&path)?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?;

                    let parser = EventReader::from_str(&contents);
                    let mut map = HashMap::new();
                    let mut depth = 0;
                    let mut line = String::new();

                    for e in parser {
                        match e {
                            Ok(XmlEvent::StartElement {
                                name, attributes, ..
                            }) => {
                                depth += 1;
                                if depth == 2 {
                                    if name.local_name == "card" {
                                        line = format!(
                                            "<{} {}/>",
                                            name.local_name.clone(),
                                            attributes
                                                .iter()
                                                .map(|attr| format!(
                                                    "{}=\"{}\"",
                                                    attr.name.local_name,
                                                    attr.value.replace("&", "&amp;")
                                                ))
                                                .collect::<Vec<String>>()
                                                .join(" ")
                                        );
                                        *map.entry(line.clone()).or_insert(0) += 1;
                                    }
                                }
                            }
                            Ok(XmlEvent::EndElement { .. }) => {
                                depth -= 1;
                                if depth == 1 {
                                    line.clear();
                                }
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                                break;
                            }
                            _ => {}
                        }
                    }
                    map_arr.push(map);
                }
            }
        }
    }

    let mut final_map: HashMap<String, i32> = HashMap::new();
    for map in map_arr {
        for (key, value) in map {
            final_map
                .entry(key)
                .and_modify(|v: &mut i32| *v = (*v).max(value))
                .or_insert(value);
        }
    }

    println!("{:?}", final_map);
    use std::io::Write;

    let master_file_path = "master.txt";
    let mut master_file = File::create(master_file_path)?;
    master_file.set_len(0)?;

    write!(
        master_file,
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n<deck>\n"
    )?;
    for (key, value) in final_map.iter() {
        for _ in 0..*value {
            write!(master_file, "{}\n", key)?;
        }
    }
    write!(master_file, "</deck>\n")?;

    Ok(())
}
