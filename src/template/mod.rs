pub mod elements;

use std::path::PathBuf;
use crate::{
    SITE_ROOT
};

fn resolve_path<Path: AsRef<str>>(path: Path, current_file: PathBuf) -> PathBuf {
    let site_root = SITE_ROOT.get()
        .expect("Failed to acquire site root")
        .parent()
        .expect("Invalid Site root")
        .to_path_buf();
    let mut path = path.as_ref().to_owned();

    assert!(site_root.is_absolute());

    if path.starts_with("#") {
        path = path.replacen("#", &format!("{}/", site_root.to_string_lossy()), 1);
    }

    let mut path = PathBuf::from(path);

    if path.is_relative() {
        path = current_file.join(path);
    }

    return path;
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use crate::template::resolve_path;

    #[test]
    fn test_resolve() {
        let site_root = PathBuf::from("/home/jcake/Code/personal-website/site.toml");
        let current_file = PathBuf::from("/home/jcake/Code/personal-website/www/home.en.html");

        assert_eq!(resolve_path("#include/frame.html", current_file.clone()), PathBuf::from("/home/jcake/Code/personal-website/include/frame.html"));
        assert_eq!(resolve_path("./frame.html", current_file.clone()), PathBuf::from("/home/jcake/Code/personal-website/www/frame.html"));
        assert_eq!(resolve_path("frame.html", current_file.clone()), PathBuf::from("/home/jcake/Code/personal-website/www/frame.html"));
    }
}