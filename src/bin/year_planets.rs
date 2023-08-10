#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600.0
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600.0 / 0.2408467
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600.0 / 0.61519726
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        // 31557600 seconds
        d.seconds as f64 / 31557600.0
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600.0 / 1.8808158
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600.0 / 11.862615
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600.0 / 29.447498
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600.0 / 84.016846
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31557600.0 / 164.79132
    }
}

fn main() {
    let duration = Duration::from(1_000_000_000);
    println!("WTF: {:.2}", Earth::years_during(&duration));
}

#[cfg(test)]
fn assert_in_delta(expected: f64, actual: f64) {
    let diff: f64 = (expected - actual).abs();
    let delta: f64 = 0.01;
    if diff > delta {
        panic!("Your result of {actual} should be within {delta} of the expected result {expected}")
    }
}
#[test]
fn earth_age() {
    let duration = Duration::from(1_000_000_000);
    assert_in_delta(31.69, Earth::years_during(&duration));
}
#[test]
fn mercury_age() {
    let duration = Duration::from(2_134_835_688);
    assert_in_delta(280.88, Mercury::years_during(&duration));
}
#[test]
fn venus_age() {
    let duration = Duration::from(189_839_836);
    assert_in_delta(9.78, Venus::years_during(&duration));
}
#[test]
fn mars_age() {
    let duration = Duration::from(2_129_871_239);
    assert_in_delta(35.88, Mars::years_during(&duration));
}
#[test]
fn jupiter_age() {
    let duration = Duration::from(901_876_382);
    assert_in_delta(2.41, Jupiter::years_during(&duration));
}
#[test]
fn saturn_age() {
    let duration = Duration::from(2_000_000_000);
    assert_in_delta(2.15, Saturn::years_during(&duration));
}
#[test]
fn uranus_age() {
    let duration = Duration::from(1_210_123_456);
    assert_in_delta(0.46, Uranus::years_during(&duration));
}
#[test]
fn neptune_age() {
    let duration = Duration::from(1_821_023_456);
    assert_in_delta(0.35, Neptune::years_during(&duration));
}
