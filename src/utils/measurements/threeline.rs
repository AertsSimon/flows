use std::ops::Add;
use std::ops::Sub;

#[derive(PartialEq, Debug)]
struct ThreeLine(f32, f32, f32);

impl Add for ThreeLine {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for ThreeLine {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_line_add() {
        let first = ThreeLine(2., 2., 2.);
        let second = ThreeLine(4., 6., 8.);
        let expected_result = ThreeLine(6., 8., 10.);

        let result = first + second;

        assert_eq!(result, expected_result)
    }

    #[test]
    fn test_three_line_sub() {
        let first = ThreeLine(4., 6., 8.);
        let second = ThreeLine(2., 2., 2.);
        let expected_result = ThreeLine(2., 4., 6.);

        let result = first - second;

        assert_eq!(result, expected_result)
    }
}
