use jwalk::WalkDir;
use std::collections::VecDeque;
use std::fs;
use std::io;
use std::time::Instant;
use size_format::{SizeFormatterBinary};

extern crate humantime;
extern crate jwalk;
extern crate size_format;

pub fn execute_single(path: &str) {
    let now = Instant::now();

    let mut files: i64 = 0;
    let mut folders: i64 = 0;
    let mut total_size: u64 = 0;

    let mut vector: VecDeque<String> = VecDeque::new();
    vector.push_back(String::from(path));

    loop {
        if let Some(p) = vector.pop_front() {
            folders += 1;
            match read_dir(&mut vector, &p, &mut files, &mut total_size) {
                Err(e) => println!("{} path: {}", e, p),
                Ok(_) => {}
            }
        } else {
            break;
        }
    }

    println!(
        "files: {} folders: {} size: {}B elapsed: {}",
        files,
        folders,
        SizeFormatterBinary::new(total_size),
        humantime::format_duration(now.elapsed()).to_string()
    );
}

fn read_dir(vector: &mut VecDeque<String>, path: &str, f: &mut i64, t: &mut u64) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let full = entry.path().into_os_string().into_string().unwrap();
        if entry.path().is_dir() {
            vector.push_back(full);
        } else {
            let m = entry.metadata()?;
            if m.is_file() {
                *t += m.len();
                *f += 1;
            }
        }
    }
    Ok(())
}

pub fn execute_parallel(path: &str) {
    let now = Instant::now();

    let mut files: i64 = 0;
    let mut folders: i64 = 0;
    let mut total_size: u64 = 0;
    for entry in WalkDir::new(path).skip_hidden(false).follow_links(false) {
        if let Ok(e) = entry {
            if e.file_type().is_file() {
                if let Ok(m) = e.metadata() {
                    total_size += m.len()
                }
                files += 1;
            } else if e.file_type().is_dir() {
                folders += 1;
            }
        }
    }
    println!(
        "files: {} folders: {} size: {}B elapsed: {}",
        files,
        folders,
        SizeFormatterBinary::new(total_size),
        humantime::format_duration(now.elapsed()).to_string()
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
