use jwalk::WalkDir;
use size_format::SizeFormatterBinary;
use std::time::Instant;

extern crate humantime;
extern crate jwalk;
extern crate size_format;

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
