use ansi_term::Colour;
use std::borrow::Cow;
use std::error::Error;
use std::io::Read;
use std::path::Path;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

use super::Key;
use crate::file_finder::f_find;

fn yaml_to_vec<'a>(yaml: &'a Yaml, key: &mut Vec<Cow<'a, str>>, keys: &mut Vec<Key>, origin: u8) {
    if yaml.is_badvalue() {
        eprintln!("Bad value: {}", key.join("."));
        return;
    }
    let is_end_of_branch = yaml.as_hash().is_none();
    keys.push(Key::new(key.join("."), !is_end_of_branch, origin));
    if is_end_of_branch {
        return;
    };
    for (sub_key, sub_yaml) in yaml.as_hash().unwrap() {
        let sub_key = if let Some(s) = sub_key.as_str() {
            Cow::from(s)
        } else if let Some(i) = sub_key.as_i64() {
            Cow::from(i.to_string())
        } else {
            println!("Invalid key: {}.{:?}", key.join("."), sub_key);
            continue;
        };
        key.push(sub_key);
        yaml_to_vec(sub_yaml, key, keys, origin);
        key.pop().unwrap();
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

pub fn load_trans_keys(wher: &[&Path]) -> (Vec<String>, Vec<Key>) {
    let trans_files = f_find(wher, &[".fr.yaml"]);
    let mut keys = Vec::with_capacity(20000);
    let mut origins = Vec::new();
    for f in trans_files {
        let f_str = f.to_string_lossy().to_string();
        origins.push(f_str);
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
        yaml_to_vec(&yaml[0], &mut Vec::with_capacity(20), &mut keys, (origins.len() - 1) as u8);
    }
    keys.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap());
    // should I warn about duplicates ?
    // btw this is full yolo, maybe `partial` is different
    keys.dedup_by(|a, b| a.key == b.key);
    (origins, keys)
}
