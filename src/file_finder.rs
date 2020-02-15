use std::fs;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

// TODO: use gitignore
fn allowed_dir(dname: &Path) -> bool {
    !dname.ends_with("/.git") && !dname.ends_with("/vendor") && !dname.ends_with("/var/cache")
}

fn allowed_file(fname: &Path, allow_exts: &[&str]) -> bool {
    let fname = fname.as_os_str().as_bytes();
    allow_exts.iter().any(|ext| {
        // A hand-made 'ends_with()' implementation (/!\ Path::ends_with != str::ends_with)
        let ext = ext.as_bytes();
        let mut f_len = fname.len();
        let mut ext_len = ext.len();
        while f_len != 0 && ext_len != 0 {
            f_len -= 1;
            ext_len -= 1;
            if fname[f_len] as u8 != ext[ext_len] as u8 {
                return false;
            }
        }
        true
    })
}

/// finds files inside `root` w/ names that matches
/// Performance:
///   Good enough. It's not really slow and it permits the use of .into_par_iter()
pub fn f_find(root: &Path, allow_exts: &[&str]) -> Vec<PathBuf> {
    let mut file_stack = Vec::new();
    let mut dir_stack = Vec::new();
    let root = PathBuf::from(root);
    let root_meta = match fs::metadata(&root) {
        Ok(m) => m,
        Err(e) => panic!("Invalid root dir: {} ({})", root.to_string_lossy(), e),
    };
    if root_meta.is_dir() {
        dir_stack.push(root)
    } else {
        file_stack.push(root)
    };

    while let Some(dir) = dir_stack.pop() {
        let dir_reader = match fs::read_dir(&dir) {
            Ok(dr) => dr,
            Err(e) => {
                println!("Couldn't read {}: {}", dir.to_string_lossy(), e);
                continue;
            }
        };
        for f in dir_reader {
            let f = f.unwrap(); // I dont know/understand why unwrap is necessary here
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
