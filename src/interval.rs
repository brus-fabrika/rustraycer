#[derive(Debug, PartialEq)]
pub struct Interval {
    pub min: f32,
    pub max: f32
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Interval {
        Interval{min, max}
    }

    pub fn default() -> Interval {
        Interval::new(f32::NEG_INFINITY, f32::INFINITY)
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::INFINITY;

    use crate::interval::Interval;

    #[test]
    fn interval_default() {
        let i = Interval::default();
        assert_eq!(i, Interval{min: f32::NEG_INFINITY, max: INFINITY});
    }

    #[test]
    fn interval_new() {
        let i = Interval::new(-1.0, 1.0);
        assert_eq!(i, Interval{min: -1.0, max: 1.0});
    }
    
    #[test]
    fn interval_surrounds() {
        let i = Interval::new(-1.0, 1.0);
        assert_eq!(true, i.surrounds(0.0));
        assert_eq!(false, i.surrounds(-1.0));
        assert_eq!(false, i.surrounds(1.0));
        assert_eq!(false, i.surrounds(1.1));
        assert_eq!(false, i.surrounds(-1.1));
    }
    
    #[test]
    fn interval_contains() {
        let i = Interval::new(-1.0, 1.0);
        assert_eq!(true, i.contains(0.0));
        assert_eq!(true, i.contains(-1.0));
        assert_eq!(true, i.contains(1.0));
        assert_eq!(false, i.contains(1.1));
        assert_eq!(false, i.contains(-1.1));
    }
}
