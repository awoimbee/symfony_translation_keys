use ansi_term::Colour;
use std::error::Error;
use std::io::Read;
use std::path::Path;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;
use std::borrow::Cow;
use std::sync::atomic::AtomicUsize;

use super::file_finder::f_find;

#[derive(Debug)]
pub struct Key {
    pub uses: AtomicUsize,
    pub key: String,
    pub partial: bool,
}

impl Key {
    pub fn new(key: &str, partial: bool) -> Self {
        let key = key.to_owned();
        Key {
            uses: AtomicUsize::new(0),
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
    let is_end_of_branch = yaml.as_hash().is_none();
    keys.push(Key::new(key, !is_end_of_branch));
    if is_end_of_branch{ return };
    for (sub_key, sub_yaml) in yaml.as_hash().unwrap() {
        let sub_key = if let Some(s) = sub_key.as_str() {
            Cow::from(s)
        } else if let Some(i) = sub_key.as_i64() {
            Cow::from(i.to_string())
        } else {
            println!("Invalid key: {}.{:?}", key, sub_key);
            continue;
        };
        if key.len() != 0 {
            key.push('.');
        }
        key.push_str(&sub_key);
        yaml_to_vec(sub_yaml, key, keys);
        key.truncate(key.rfind('.').unwrap_or(0));
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
    let trans_files = f_find(&[&trans_roots], &[".fr.yaml"]);
    let mut keys = Vec::with_capacity(20000);
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
        yaml_to_vec(&yaml[0], &mut String::with_capacity(100), &mut keys);
    }
    keys.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap());
    keys.dedup_by(|a, b| a.key == b.key);
    keys
}
