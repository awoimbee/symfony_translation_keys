use std::path::PathBuf;
mod trad_key;

mod file_finder;

fn main() {
    let project_root = "../meero/master";

    let trad_keys = trad_key::load_trans_keys(&PathBuf::from(project_root));

    // vec of keys (app.form.recruitment.legal.company_id, ...)
    // vec of partial keys (admin., admin.form., admin.form.recruitment., ...)
    // regex all keys & partial keys everywhere & count occurences -> re.find_iter(txt).count()
    // print keys & occurences (w/ some processing to make things pretty & readable)

    // let c = s.matches(t).count();
}
