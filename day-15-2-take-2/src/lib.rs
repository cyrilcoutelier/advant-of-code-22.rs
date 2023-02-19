#[derive(Hash, Clone, PartialEq, Eq, Debug)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn get_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn get_circle_iter(&self, radius: usize) -> CircleIterator {
        CircleIterator::new(self.clone(), radius)
    }
}

pub struct CircleIterator {
    center: Pos,
    radius: usize,
    x: isize,
    y: isize,
}

impl CircleIterator {
    fn new(center: Pos, radius: usize) -> Self {
        let x = center.x;
        let y = center.y + radius as isize;
        CircleIterator {
            center,
            radius,
            x,
            y,
        }
    }

    pub fn get_x_offset(&self) -> usize {
        self.radius - self.y.abs_diff(self.center.y)
    }
}

impl Iterator for CircleIterator {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y.abs_diff(self.center.y) > self.radius {
            return None;
        }
        let pos = Pos {
            x: self.x,
            y: self.y,
        };

        if self.x == self.center.x {
            self.x = self.center.x - 1;
            self.y -= 1;
        } else {
            let x_offset = self.get_x_offset();
            if self.x == self.center.x - x_offset as isize {
                self.x = self.center.x + x_offset as isize;
            } else if self.x == self.center.x + x_offset as isize {
                self.y -= 1;
                let x_offset = self.get_x_offset();
                self.x = self.center.x - x_offset as isize;
            } else {
                panic!("In an impossible state, x={}, y={}", self.x, self.y);
            }
        }
        Some(pos)
    }
}

pub struct Couple {
    pub sensor: Pos,
    pub beacon: Pos,
    pub distance: usize,
}
