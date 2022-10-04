use chrono::{Duration, Utc};
use crate::time::elapsed::Elapsed;

/// Measures the time the function takes
pub fn measure_duration<F>(
    f: F
) -> Duration
    where F: FnOnce() -> () {
    let now = Utc::now();
    f();
    return now.elapsed();
}

/// Measures the time the function takes, also returns the value from the function
pub fn measure_operation<T, F>(
    f: F
) -> (Duration, T)
    where F: FnOnce() -> T {
    let now = Utc::now();
    let v = f();
    return (now.elapsed(), v);
}


#[cfg(test)]
mod test {
    use chrono::Duration;
    use crate::time::timing::{measure_duration, measure_operation};
    use crate::time::thread::sleep;

    #[test]
    fn test_measure_duration() {
        let d = measure_duration(|| {
            sleep(Duration::milliseconds(20));
            let _ = 1 + 2;
        });
        assert!(d > Duration::nanoseconds(0));
    }

    #[test]
    fn test_measure_operation() {
        let (d, x) = measure_operation(|| {
            sleep(Duration::milliseconds(20));
            1 + 2
        });

        assert_eq!(3, x);
        assert!(d > Duration::nanoseconds(0));
    }
}
