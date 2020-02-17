use std::path::{PathBuf, Path};

mod trad_key;
mod file_finder;

use std::sync::atomic::{AtomicUsize, Ordering};
use rayon::prelude::*;

use std::fs::File;
use std::io::prelude::*;
// use std::path::Path;

pub fn read_file(file_name: &Path) -> Option<String> {
    let mut contents = String::new();
    let mut f = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Could not open {} ({})", file_name.to_string_lossy(), e);
            return None;
        }
    };
    match f.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            if e.kind() != std::io::ErrorKind::InvalidData {
                eprintln!("Could not read {} ({})", file_name.to_string_lossy(), e);
            }
            return None;
        }
    };
    Some(contents)
}



fn project_subfolder(p_root: &Path, sub_d: &str) -> PathBuf {
    let mut new = p_root.to_owned();
    new.push(sub_d);
    new
}

fn main() {
    let project_root = PathBuf::from("../meero/master");
    let src = project_subfolder(&project_root, "src");
    let templates = project_subfolder(&project_root, "templates");

    let trad_keys = trad_key::load_trans_keys(&PathBuf::from(project_root));
    let files = file_finder::f_find(&[&src, &templates], &[""]);

    files.into_par_iter().for_each(|file_path|{
        let contents = match read_file(&file_path) {
            Some(c) => c,
            None => return,
        };
        for t_k in trad_keys.iter() {
            let matches = contents.matches(&t_k.key).count();
            t_k.uses.fetch_add(matches, Ordering::Relaxed);
        }
    });
    trad_keys.iter().for_each(|k| println!("{}: {}", k.key, k.uses.load(Ordering::Relaxed)));

    // vec of keys (app.form.recruitment.legal.company_id, ...)
    // vec of partial keys (admin., admin.form., admin.form.recruitment., ...)
    // let occurences = s.matches(t).count();
    // print keys & occurences (w/ some processing to make things pretty & readable)

}
