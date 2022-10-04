use chrono::{DateTime, Duration, Utc};

pub trait Elapsed {
    /// Elapsed but for chrono duration
    fn elapsed(&self) -> Duration;
}

impl Elapsed for DateTime<Utc> {
    fn elapsed(&self) -> Duration {
        Utc::now() - *self
    }
}


#[cfg(test)]
mod test {
    use chrono::{Duration, Utc};
    use crate::time::elapsed::Elapsed;
    use crate::time::thread::sleep;

    #[test]
    fn test_elapsed() {
        let x = Utc::now();
        sleep(Duration::milliseconds(20));
        assert!(x.elapsed() > Duration::nanoseconds(0));
    }

}
