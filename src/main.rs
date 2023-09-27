use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    // if args.len() > 1 {
    let input = &args[1];
    // }
    // Open and read the file
    let mut file = File::open(input)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Init the XML parser
    let parser = EventReader::from_str(&contents);
    let mut depth = 0;
    let mut name = String::new();
    let mut map = HashMap::new();

    // Parse XML file
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
                                println!("{}", attr.value);
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

    println!("{:?}", map);

    Ok(())
}
