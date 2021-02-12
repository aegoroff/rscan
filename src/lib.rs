use jwalk::WalkDir;
use std::env;
use std::time::Instant;

extern crate humantime;
extern crate jwalk;

pub fn execute() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let now = Instant::now();

    let mut files: i64 = 0;
    let mut folders: i64 = 0;
    let mut total_size: u64 = 0;
    for entry in WalkDir::new(path).follow_links(false) {
        let e = entry.unwrap();
        if e.file_type.is_file() {
            match e.metadata() {
                Ok(m) => total_size += m.len(),
                Err(_) => {}
            }
            files += 1;
        } else {
            folders += 1;
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
