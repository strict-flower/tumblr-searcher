use crate::searcher::backend;
use json;
use regex::{Captures, Regex};
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut base_path_str: &String = &".".to_owned();

    if args.len() == 3 {
        base_path_str = &args[2];
    } else if args.len() != 2 {
        println!("Usage: {} KEYWORD [DIR]", args[0]);
        return Ok(());
    }

    let keyword = &args[1];
    let re = Regex::new(keyword).unwrap();
    let file_list = gen_file_list(base_path_str).unwrap();

    let search_result = backend::search_parallel(&keyword, &file_list, &read_tumblr_file)?;

    for item in search_result {
        let data = read_file(item.path);
        let url = parse_tumblr_file(&data).0;
        println!("\x1b[31;1m{}\x1b[0m", url);
        for line in item.hit_lines {
            let decorated = re.replace_all(&line, |caps: &Captures| {
                format!("\x1b[4;1m{}\x1b[0m", &caps[0])
            });
            println!("\x1b[31;1m*\x1b[0m {}", decorated);
        }
        println!("");
    }

    Ok(())
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}

fn read_tumblr_file(path: &Path) -> String {
    let data = read_file(path);
    parse_tumblr_file(&data).1
}

fn parse_tumblr_file(data: &String) -> (String, String) {
    let mut parsed = json::parse(&data).unwrap();
    let url = parsed["url-with-slug"].take_string().unwrap();
    if parsed.has_key("regular-body") {
        (url, parsed["regular-body"].take_string().unwrap())
    } else {
        (url, "".to_owned())
    }
}

fn gen_file_list(base_path: &String) -> Option<Vec<PathBuf>> {
    if !fs::metadata(&base_path).unwrap().is_dir() {
        return None;
    }

    let entries = fs::read_dir(base_path).unwrap();
    Some(
        entries
            .filter_map(|entry| {
                let pb: PathBuf = entry.unwrap().path();
                if fs::metadata(&pb).unwrap().is_file() {
                    Some(pb)
                } else {
                    None
                }
            })
            .collect(),
    )
}
