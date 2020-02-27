use std::path::{Path, PathBuf};

mod file_finder;
mod trad_key;

use rayon::prelude::*;
use std::sync::atomic::Ordering;

use std::fs::File;
use std::io::prelude::*;

use ansi_term::Colour;
use trad_key::Key;

use clap::{App, Arg};

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
    let args = App::new("translations checker")
        .version("0.2")
        .author("Arthur W. <arthur.woimbee@gmail.com>")
        .about("Find unused translations in symfony project")
        .arg(
            Arg::with_name("project_root")
                .short("p")
                .long("project_root")
                .value_name("FOLDER")
                .help("Where to work")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("translations")
                .long("trans_fd")
                .short("t")
                .value_name("FILE|FOLDER")
                .help("Where to load translation keys (rel. to p. root)")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("src")
                .short("s")
                .long("src")
                .value_name("FILE|FOLDER")
                .takes_value(true)
                .multiple(true)
                .help("where to search for translation keys usage (rel. to p. root)"),
        )
        .get_matches();

    let project_root = PathBuf::from(args.value_of("project_root").unwrap());
    let src_owned = match args.values_of("src") {
        Some(values) => values
            .map(|v| project_subfolder(&project_root, v))
            .collect(),
        None => vec![
            project_subfolder(&project_root, "src"),
            project_subfolder(&project_root, "templates"),
        ],
    };
    let translations_owned = match args.values_of("translations") {
        Some(values) => values
            .map(|v| project_subfolder(&project_root, v))
            .collect(),
        None => vec![project_subfolder(&project_root, "translations")],
    };
    let src = src_owned.iter().map(|p| p.as_ref()).collect::<Vec<&Path>>();
    let translations = translations_owned
        .iter()
        .map(|p| p.as_ref())
        .collect::<Vec<&Path>>();

    /* load translation keys */
    let (origins, mut trad_keys) = trad_key::load_yaml::load_trans_keys(&translations);

    /* search for usage of each translation key */
    let files = file_finder::f_find(&src, &[""]);
    files.into_par_iter().for_each(|file_path| {
        let contents = match read_file(&file_path) {
            Some(c) => c,
            None => return,
        };
        for t_k in trad_keys.iter() {
            let matches = contents.matches(&t_k.key).count();
            t_k.uses.fetch_add(matches, Ordering::Relaxed);
        }
    });

    /* "Algo" */
    let mut pretty_output: [(Colour, Vec<Key>); 7] = [
        (Colour::White, Vec::new()),
        (Colour::Blue, Vec::new()),
        (Colour::Cyan, Vec::new()),
        (Colour::Green, Vec::new()),
        (Colour::Yellow, Vec::new()),
        (Colour::Purple, Vec::new()),
        (Colour::Red, Vec::new()),
    ];

    for i in 0..trad_keys.len() {
        if trad_keys[i].partial == true {
            let mut calc_uses = 0;
            let mut j = i+1;
            while
                j < trad_keys.len() && trad_keys[j].key.starts_with(&trad_keys[i].key)
             {
                if !trad_keys[j].partial {
                    calc_uses += trad_keys[j].uses.load(Ordering::Relaxed);
                };
                j += 1;
            }
            if trad_keys[i].uses.load(Ordering::Relaxed) == calc_uses {
                trad_keys[i].trusted += 1;
                trad_keys[i..j].iter_mut().for_each(|k| k.trusted += 1);
            }
        } else if trad_keys[i].uses.load(Ordering::Relaxed) == 0 {
            let index = std::cmp::min(trad_keys[i].trusted as usize, pretty_output.len()-1);
            pretty_output[index].1.push(trad_keys[i].clone());
        }
    }

    /* Print */
    println!("All keys:");
    for k in trad_keys {
        println!(
            "occurences: {:5} trust: {:2} Key: '{:80}' Origin: {}",
            k.uses.load(Ordering::Relaxed), k.trusted, k.key, origins[k.origin as usize]
        );
    }

    println!("{}", Colour::Red.bold().paint("Keys to remove:"));
    for (trust_lvl, (color, contents)) in pretty_output.iter().enumerate() {
        let out = contents
            .iter()
            .map(|k| format!("\t{:80} origin: {}\n", k.key, origins[k.origin as usize]))
            .collect::<String>();
        if out.len() != 0 {
            println!("Trust: {}", trust_lvl);
            println!("{}", color.paint(out));
        }
    }
}
