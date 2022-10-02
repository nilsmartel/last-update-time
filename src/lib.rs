use std::fs::read_dir;
use std::io;
use std::time::SystemTime;
use std::path::Path;

#[cfg(test)]
mod tests {
    use crate::last_update_time;

    #[test]
    fn file() {
        let result = last_update_time("./src/lib.rs");
        assert!(result.is_ok());
    }
    #[test]
    fn dir() {
        let result = last_update_time(".");
        assert!(result.is_ok());
    }
}

pub fn last_update_time(path: &str) -> io::Result<SystemTime> {
    // first, check if this is a normal file etc.
    let mut smallest_time = {
        let path = Path::new(path);
        let m = path.metadata()?;
        let time = m
            .modified()?;

        if !m.is_dir() {
            return Ok(time);
        }
        time
    };

    for entry in read_dir(path)? {
        if entry.is_err() {
            continue;
        }
        let entry = entry.unwrap();

        let m = entry.metadata();
        if m.is_err() {
            continue;
        }
        let m = m.unwrap();

        let mut time = m
            .modified()?;

        if m.is_dir() {
            let path = entry.path();
            let path = path.to_str().unwrap();
            if let Ok(subtime) = last_update_time(path) {
                if subtime < time {
                    time = subtime;
                }
            }
        }

        if time < smallest_time {
            smallest_time = time;
        }
    }

    Ok(smallest_time)
}
