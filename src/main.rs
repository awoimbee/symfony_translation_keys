use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
mod trad_key;
use trad_key::TradKey;

mod file_finder;

fn main() {
    let project_root = "../meero/master";

    let test = trad_key::load_trans_keys(&PathBuf::from(project_root));

    // for (&key, &value) in t.range::<&str, _>("tes"..="tes"..) {
    //     println!("{}: {}", key, value);
    // }

    // let mut map = BTreeMap::new();
    // map.insert(3, "a");
    // map.insert(5, "b");
    // map.insert(8, "c");
    // for (&key, &value) in map.range((Included(&4), Included(&8))) {
    //     println!("{}: {}", key, value);
    // }

}
