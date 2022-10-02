use std::fs::read_dir;
use std::time::SystemTime;

pub fn last_update_time(path: &str) -> Option<SystemTime> {
    // smallest thing I would ever need
    let mut smallest_time = SystemTime::UNIX_EPOCH;

    let dir = if let Ok(d) = read_dir(path) {
        d
    } else {
        return None;
    };

    for entry in dir {
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
            .modified()
            .unwrap_or_else(|_| m.created().expect("read modified time or created time"));

        if m.is_dir() {
            let path = entry.path();
            let path = path.to_str().unwrap();
            if let Some(subtime) = last_update_time(path) {
                if subtime < time {
                    time = subtime;
                }
            }
        }

        if time < smallest_time {
            smallest_time = time;
        }
    }

    if smallest_time == SystemTime::UNIX_EPOCH {
        return None;
    }

    Some(smallest_time)
}
