use chrono::{Duration, Utc};
use crossbeam::channel::Receiver;

trait Collectable<T> {
    /// Counts Recvs over duration with a 1 millisecond timeout on the recv call
    fn count_over_duration(&self, duration: Duration) -> i32;
    /// Counts all recvs until the channel is empty
    fn count_until_empty(&self) -> i64;
    /// Collects all values over the given duration
    fn collect_over_duration(&self, period: Duration) -> Vec<T>;
}

impl<T> Collectable<T> for Receiver<T> {
    fn count_over_duration(&self, duration: Duration) -> i32 {
        let start = Utc::now();
        let mut counter = 0;
        while Utc::now() < start + duration {
            if self
                .recv_timeout(Duration::milliseconds(1).to_std().unwrap())
                .is_ok()
            {
                counter += 1;
            } else {
                break;
            }
        }
        return counter;
    }

    fn count_until_empty(&self) -> i64 {
        let mut counter = 0;
        while !self.is_empty() {
            self.recv().unwrap();
            counter += 1;
        }
        return counter;
    }

    fn collect_over_duration(&self, duration: Duration) -> Vec<T> {
        let start = Utc::now();
        let mut inputs_in_period = vec![];
        while Utc::now() < start + duration {
            if let Ok(item) = self.recv_timeout(Duration::milliseconds(1).to_std().unwrap()) {
                inputs_in_period.push(item);
            } else {
                break;
            }
        }
        return inputs_in_period;
    }
}

#[cfg(test)]
mod test {
    use crate::crossbeam::collect::Collectable;
    use chrono::Duration;
    use crossbeam::channel;

    #[test]
    fn test_count_over_duration() {
        let (s, r) = channel::unbounded::<bool>();
        s.send(true).unwrap();
        s.send(true).unwrap();
        s.send(true).unwrap();
        s.send(true).unwrap();

        assert_eq!(4, r.count_over_duration(Duration::seconds(5)));
    }

    #[test]
    fn test_count_until_empty() {
        let (s, r) = channel::unbounded::<bool>();
        s.send(true).unwrap();
        s.send(true).unwrap();
        s.send(true).unwrap();
        s.send(true).unwrap();

        assert_eq!(4, r.count_until_empty());
    }

    #[test]
    fn test_collect_over_duration() {
        let (s, r) = channel::unbounded::<bool>();
        s.send(true).unwrap();
        s.send(true).unwrap();
        s.send(true).unwrap();
        s.send(true).unwrap();

        assert_eq!(
            [true; 4].to_vec(),
            r.collect_over_duration(Duration::seconds(5))
        );
    }
}
