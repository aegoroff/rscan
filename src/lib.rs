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
    for entry in WalkDir::new(path) {
        let e = entry.unwrap();
        if e.file_type().is_dir() {
            folders += 1;
        } else {
            files += 1;
        }
    }
    println!(
        "files: {} folders: {} elapsed: {}",
        files,
        folders,
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
