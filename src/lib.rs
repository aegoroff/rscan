use std::time::Instant;
use walkdir::WalkDir;

extern crate humantime;
extern crate jwalk;
extern crate walkdir;

pub fn execute_single(path: &str) {
    let now = Instant::now();

    let mut files: i64 = 0;
    let mut folders: i64 = 0;
    let mut total_size: u64 = 0;
    for entry in WalkDir::new(path).follow_links(false) {
        if let Ok(e) = entry {
            if e.file_type().is_file() {
                if let Ok(m) = e.metadata() {
                    total_size += m.len()
                }
                files += 1;
            } else {
                folders += 1;
            }
        }
    }
    println!(
        "files: {} folders: {} size: {} elapsed: {}",
        files,
        folders,
        total_size,
        humantime::format_duration(now.elapsed()).to_string()
    );
}

pub fn execute_parallel(path: &str) {
    let now = Instant::now();

    let mut files: i64 = 0;
    let mut folders: i64 = 0;
    let mut total_size: u64 = 0;
    for entry in jwalk::WalkDir::new(path).follow_links(false) {
        if let Ok(e) = entry {
            if e.file_type().is_file() {
                if let Ok(m) = e.metadata() {
                    total_size += m.len()
                }
                files += 1;
            } else {
                folders += 1;
            }
        }
    }
    println!(
        "files: {} folders: {} size: {} elapsed: {}",
        files,
        folders,
        total_size,
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
