use chrono::Duration;

pub fn sleep(duration: Duration) {
    std::thread::sleep(duration.to_std().expect("Failed to convert "))
}
