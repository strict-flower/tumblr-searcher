use regex::Regex;
use std::io;
use std::path::{Path, PathBuf};

pub struct FoundItem<'a> {
    pub path: &'a Path,
    pub hit_lines: Vec<String>,
}

impl<'a> FoundItem<'a> {
    pub fn new(path: &'a Path, hit_lines: Vec<String>) -> FoundItem<'a> {
        FoundItem { path, hit_lines }
    }
}

pub fn search_linear<'a, F>(
    keyword: &'a String,
    target_files: &'a Vec<PathBuf>,
    read_func: F,
) -> io::Result<Vec<FoundItem<'a>>>
where
    F: Fn(&Path) -> String,
{
    let re = Regex::new(keyword).unwrap();
    let mut res = vec![];
    for entry in target_files {
        let path = &entry.as_path();
        let data = read_func(path);
        if !re.is_match(&data) {
            continue;
        }
        let lines: Vec<String> = data
            .lines()
            .filter(|x| re.is_match(&x))
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();
        res.push(FoundItem::new(&path, lines));
    }

    Ok(res)
}
