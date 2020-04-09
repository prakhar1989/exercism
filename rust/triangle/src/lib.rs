use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: PartialEq + PartialOrd + Add<Output = T> + Default + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let [a, b, c] = sides;
        if a + b > c && a + c > b && b + c > a && sides.iter().all(|s| *s != T::default()) {
            return Some(Triangle { sides });
        }
        None
    }

    pub fn is_equilateral(&self) -> bool {
        let [a, b, c] = self.sides;
        a == b && b == c
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        let [a, b, c] = self.sides;
        a == b || b == c || c == a
    }

    pub fn is_degenerate(&self) -> bool {
        let [a, b, c] = self.sides;
        a + b == c && a + c == b && b + c == a
    }
}
