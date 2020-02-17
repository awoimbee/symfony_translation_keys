use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

// TODO: use gitignore
fn allowed_dir(dname: &Path) -> bool {
    !dname.ends_with("/.git") && !dname.ends_with("/vendor") && !dname.ends_with("/var/cache")
}

#[cfg(unix)]
fn path_to_bytes<'a>(p: &'a Path) -> &'a [u8] {
    p.as_os_str().as_bytes()
}
#[cfg(windows)]
fn path_to_bytes<'a>(p: &'a Path) -> &'a [u8] {
    match p.to_str() {
        Some(s) => s.as_bytes(),
        None => {
            eprintln!("Invalide file name, skipping: {}", p.to_string_lossy());
            "".as_bytes()
        }
    }
}
#[cfg(all(not(unix), not(windows)))]
fn path_to_bytes<'a>(p: &'a Path) -> &'a [u8] {
    compile_error!("Not implemented for this target");
}

fn allowed_file(fname: &Path, allow_exts: &[&str]) -> bool {
    let fname = path_to_bytes(fname);
    allow_exts
        .iter()
        .any(|&ext| fname.ends_with(ext.as_bytes()))
}

/// finds files inside `root` w/ names that matches
/// Performance:
///   Good enough. It's not really slow and it permits the use of .into_par_iter()
pub fn f_find(roots: &[&Path], allow_exts: &[&str]) -> Vec<PathBuf> {
    let mut file_stack = Vec::new();
    let mut dir_stack = Vec::new();
    // handle roots
    for &r in roots {
        let meta = match fs::metadata(r) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Couldn't read: {} ({})", r.to_string_lossy(), e);
                continue;
            }
        };
        #[rustfmt::skip]
        let stack = if meta.is_dir() { &mut dir_stack } else { &mut file_stack };
        stack.push(r.to_owned());
    }
    // handle subdirs
    while let Some(dir) = dir_stack.pop() {
        let dir_reader = match fs::read_dir(&dir) {
            Ok(dr) => dr,
            Err(e) => {
                println!("Couldn't read: {} ({})", dir.to_string_lossy(), e);
                continue;
            }
        };
        for f in dir_reader {
            let f = f.unwrap();
            let f_path = f.path();
            let f_meta = f.metadata().unwrap();
            if f_meta.is_file() && allowed_file(&f_path, allow_exts) {
                file_stack.push(f_path);
            } else if !f_meta.is_file() && allowed_dir(&f_path) {
                dir_stack.push(f_path);
            }
        }
    }
    file_stack
}
