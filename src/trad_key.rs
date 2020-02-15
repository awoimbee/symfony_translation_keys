use ansi_term::Colour;
use regex::Regex;
use std::error::Error;
use std::io::Read;
use std::path::Path;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

use super::file_finder::f_find;

#[derive(Debug)]
pub struct Key {
    reg: regex::Regex,
    count: usize,
    key: String,
    partial: bool,
}

impl Key {
    pub fn new(key: &str, partial: bool) -> Self {
        let key = key.to_owned();
        let reg = Regex::new(&regex::escape(&key)).unwrap();
        Key {
            reg,
            count: 0,
            key,
            partial,
        }
    }
}

fn yaml_to_vec(yaml: &Yaml, key: &mut String, keys: &mut Vec<Key>) {
    if yaml.is_badvalue() {
        eprintln!("Bad value: {}", key);
        return;
    }
    if yaml.as_hash().is_none() {
        keys.push(Key::new(key, false));
        return;
    }
    keys.push(Key::new(key, true));
    for (sub_key, sub_yaml) in yaml.as_hash().unwrap() {
        let sub_key = if let Some(s) = sub_key.as_str() {
            s.to_owned()
        } else if let Some(i) = sub_key.as_i64() {
            i.to_string()
        } else {
            println!("Invalid key: {}.{:?}", key, sub_key);
            continue;
        };
        key.push_str(&format!(".{}", sub_key));
        yaml_to_vec(sub_yaml, key, keys);
        key.truncate(key.rfind('.').unwrap());
    }
}

fn read_to_yaml(file_path: &Path) -> Result<Vec<Yaml>, Box<dyn Error>> {
    let mut file = std::fs::File::open(file_path)?;
    let mut f_contents = vec![];
    file.read_to_end(&mut f_contents)?;
    let f_contents = String::from_utf8_lossy(&f_contents);
    let yaml = YamlLoader::load_from_str(&f_contents)?;
    Ok(yaml)
}

pub fn load_trans_keys(proj_root: &Path) -> Vec<Key> {
    let mut trans_roots = proj_root.to_owned();
    trans_roots.push("translations");
    println!("looking into: {:?}", trans_roots);
    let trans_files = f_find(&trans_roots, &[".fr.yaml"]);
    let mut keys = vec![];
    for f in trans_files {
        println!("file: {:?}", f);
        let yaml = match read_to_yaml(&f) {
            Ok(yaml) => yaml,
            Err(e) => {
                eprintln!(
                    "Could not read {} ({})",
                    Colour::Blue.paint(f.to_string_lossy()),
                    Colour::Red.paint(e.to_string())
                );
                continue;
            }
        };
        // iterate over root keys before delegating
        for (root_key, yaml) in yaml[0].as_hash().unwrap().iter() {
            let mut root_key = if let Some(s) = root_key.as_str() {
                s.to_owned()
            } else if let Some(i) = root_key.as_i64() {
                i.to_string()
            } else {
                println!("Invalid root key: {:?}", root_key);
                continue;
            };
            yaml_to_vec(yaml, &mut root_key, &mut keys);
        }
    }
    // keys.iter().for_each(|k| println!("{}", k.key));
    keys
}
