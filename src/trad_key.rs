use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;
use std::collections::BTreeSet;
use std::error::Error;
use ansi_term::Colour;

use super::file_finder::f_find;

#[derive(PartialEq, Eq,PartialOrd,Ord)]
pub struct TradKey (String);

impl PartialEq<str> for TradKey {
    /// <str>: admin.global == <TradKey>: admin.global.true
    fn eq(&self, other: &str) -> bool {
        self.0.starts_with(other)
    }
}

pub fn read_to_yaml(file_path: &Path) -> Result<Vec<Yaml>, Box<dyn Error>> {
	let mut file = std::fs::File::open(file_path)?;
	let mut f_contents = vec![];
	file.read_to_end(&mut f_contents)?;
	let mut f_contents = String::from_utf8_lossy(&f_contents);
	let yaml = YamlLoader::load_from_str(&f_contents)?;
	return Ok(yaml);
}

pub fn load_trans_keys(proj_root: &Path) -> BTreeSet<TradKey> {
	let mut trans_roots = proj_root.to_owned();
	trans_roots.push("translations");
	println!("looking into: {:?}", trans_roots);
	let trans_files = f_find(&trans_roots, &[".fr.yaml"]);
	let mut set = BTreeSet::new();
	let mut test = vec![];
	for f in trans_files {
		println!("file: {:?}", f);
		let yaml = match read_to_yaml(&f) {
			Ok(yaml) => yaml,
			Err(e) => {
				eprintln!("Could not read {} ({})",
					Colour::Blue.paint(f.to_string_lossy()),
					Colour::Red.paint(e.to_string())
				);
				continue;
			}
		};
		test.push(yaml);
	}
	println!("{:?}", test);
	panic!("job done.");
	set
}
