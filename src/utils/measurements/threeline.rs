use std::iter::Sum;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(PartialEq, Debug, Default)]
struct ThreeLine(f32, f32, f32);
impl ThreeLine {
    // NOTE: maybe it is useful to change this self instead of &self
    fn abs(&self) -> ThreeLine {
        ThreeLine(self.0, self.1, self.2)
    }
}

impl Add for ThreeLine {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for ThreeLine {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
    }
}

impl Sub for ThreeLine {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for ThreeLine {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2);
    }
}

impl Mul for ThreeLine {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f32> for ThreeLine {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign for ThreeLine {
    fn mul_assign(&mut self, rhs: ThreeLine) {
        *self = Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2);
    }
}

impl MulAssign<f32> for ThreeLine {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs);
    }
}

impl Sum for ThreeLine {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(ThreeLine::default(), |acc, ThreeLine(l1, l2, l3)| {
            ThreeLine(acc.0 + l1, acc.1 + l2, acc.2 + l3)
        })
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
    fn test_three_line_iadd() {
        let first = ThreeLine(2., 2., 2.);
        let mut result = ThreeLine(4., 6., 8.);
        let expected_result = ThreeLine(6., 8., 10.);

        result += first;

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

    #[test]
    fn test_three_line_sub_assign() {
        let first = ThreeLine(2., 2., 2.);
        let mut result = ThreeLine(4., 6., 8.);
        let expected_result = ThreeLine(2., 4., 6.);

        result -= first;

        assert_eq!(result, expected_result)
    }

    #[test]
    fn test_three_line_mul() {
        let first = ThreeLine(2., 2., 2.);
        let second = ThreeLine(2., 4., 8.);
        let expected_result = ThreeLine(4., 8., 16.);
        let result: ThreeLine = first * second;

        assert_eq!(result, expected_result)
    }

    #[test]
    fn test_three_line_mul_assign() {
        let first = ThreeLine(2., 2., 2.);
        let mut result = ThreeLine(2., 4., 8.);
        let expected_result = ThreeLine(4., 8., 16.);
        result *= first;

        assert_eq!(result, expected_result)
    }

    #[test]
    fn test_three_line_mul_f32() {
        let first = ThreeLine(2., 2., 2.);
        let expected_result = ThreeLine(4., 4., 4.);
        let result: ThreeLine = first * 2.;

        assert_eq!(result, expected_result)
    }

    #[test]
    fn test_three_line_mul_f32_assign() {
        let mut result = ThreeLine(2., 4., 8.);
        let expected_result = ThreeLine(4., 8., 16.);
        result *= 2.;

        assert_eq!(result, expected_result)
    }

    #[test]
    fn test_three_line_sum() {
        let vec = vec![
            ThreeLine(2., 2., 2.),
            ThreeLine(2., 2., 2.),
            ThreeLine(2., 2., 2.),
        ];
        let expected_result = ThreeLine(6., 6., 6.);

        let result: ThreeLine = vec.into_iter().sum();

        assert_eq!(result, expected_result)
    }

    #[test]
    fn test_three_line_abs() {
        let first = ThreeLine(2., -2., 2.);
        let expected_result = ThreeLine(2., 2., 2.);

        let result: ThreeLine = first.abs();

        assert_eq!(result, expected_result)
    }
}
