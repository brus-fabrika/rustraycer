#[derive(Debug, PartialEq, Clone)]
pub struct Interval {
    pub min: f32,
    pub max: f32
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Interval {
        Interval{min, max}
    }

    pub fn from_intervals(a: &Interval, b: &Interval) -> Interval {
        let min = if a.min < b.min { a.min } else { b.min };
        let max = if a.max > b.max { a.max } else { b.max };
        Interval{min, max}
    }

    pub fn default() -> Interval {
        Interval::new(f32::NEG_INFINITY, f32::INFINITY)
    }

    pub fn empty() -> Interval {
        Interval::new(f32::INFINITY, f32::NEG_INFINITY)
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn _contains(&self, x: f32) -> bool {
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

    pub fn _expand(&self, delta: f32) -> Interval {
        let padding = delta / 2.0; 
        Interval { min: self.min - padding, max: self.max + padding }
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
        assert_eq!(true, i._contains(0.0));
        assert_eq!(true, i._contains(-1.0));
        assert_eq!(true, i._contains(1.0));
        assert_eq!(false, i._contains(1.1));
        assert_eq!(false, i._contains(-1.1));
    }
}
