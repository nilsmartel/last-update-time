use std::fs::read_dir;
use std::time::SystemTime;

pub fn last_update_time(path: &str) -> Option<SystemTime> {
    // smallest thing I would ever need
    let mut smallest_time = SystemTime::UNIX_EPOCH;

    for entry in read_dir(path).ok()? {
        let entry = entry.ok()?;

        let m = entry.metadata().ok()?;

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
