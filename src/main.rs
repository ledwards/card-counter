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
                    let mut name = String::new();

                    for e in parser {
                        match e {
                            Ok(XmlEvent::StartElement {
                                name, attributes, ..
                            }) => {
                                depth += 1;
                                if depth == 2 {
                                    if name.local_name == "card" {
                                        for attr in attributes {
                                            if attr.name.local_name == "title" {
                                                *map.entry(attr.value.clone()).or_insert(0) += 1;
                                            }
                                        }
                                    }
                                }
                            }
                            Ok(XmlEvent::EndElement { .. }) => {
                                depth -= 1;
                                if depth == 1 {
                                    name.clear();
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

    Ok(())
}
