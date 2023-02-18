use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included};

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn get_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Segment {
    pub start: isize,
    pub length: usize,
}

pub fn get_intersection_disk_row(center: &Pos, radius: usize, y: isize) -> Option<Segment> {
    let y_diff = center.y.abs_diff(y);
    if y_diff > radius {
        return None;
    }
    let length = 1 + (radius - y_diff) * 2;
    let start = center.x - length as isize / 2;
    Some(Segment { start, length })
}

pub struct Segments {
    pub map: BTreeMap<isize, usize>,
}

impl Default for Segments {
    fn default() -> Self {
        Self::new()
    }
}

impl Segments {
    pub fn new() -> Self {
        Segments {
            map: BTreeMap::new(),
        }
    }

    pub fn add_segment(&mut self, segment: Segment) {
        if let Some(length) = self.map.get(&segment.start) {
            if *length >= segment.length {
                return;
            } else {
                return self.try_extend_segment(segment.start, segment.length);
            }
        }

        let previous_segment = self
            .map
            .range((Included(isize::MIN), Excluded(segment.start)))
            .rev()
            .next();
        if let Some((start, length)) = previous_segment {
            let diff = (segment.start + segment.length as isize) - (start + *length as isize);
            let overlap = (start + *length as isize) - segment.start;

            if overlap >= 0 {
                if diff > 0 {
                    return self.try_extend_segment(*start, *length + diff as usize);
                } else {
                    return; // We are fully inside the existing segment
                }
            }
        }

        self.map.insert(segment.start, 1);
        self.try_extend_segment(segment.start, segment.length);
    }

    fn try_extend_segment(&mut self, start: isize, length: usize) {
        let additional_length = self.remove_overlapping_segments(start, length);
        let segment_length = self.map.get_mut(&start).unwrap();
        *segment_length = additional_length + length;
    }

    fn remove_overlapping_segments(&mut self, start: isize, length: usize) -> usize {
        let removed_starts: Vec<isize> = self
            .map
            .range((Excluded(start), Included(start + length as isize)))
            .map(|(start, _)| *start)
            .collect();

        let last_start = match removed_starts.last() {
            Some(val) => val,
            None => return 0,
        };
        let last_length = *self.map.get(last_start).unwrap();
        let overlap = (start + length as isize) - last_start;
        let overlap = overlap.min(last_length as isize);
        let additional_length = last_length - overlap as usize;
        removed_starts.iter().for_each(|start| {
            self.map.remove(start);
        });
        additional_length
    }

    pub fn remove_dot(&mut self, pos: isize) {
        let previous_segment = self
            .map
            .range((Included(isize::MIN), Included(pos)))
            .rev()
            .next();
        if let Some((start, length)) = previous_segment {
            self.split_segment(*start, *length, pos);
        }
    }

    fn split_segment(&mut self, start: isize, length: usize, pos: isize) {
        if start + (length as isize) <= pos {
            return; // point outside segment
        }
        if start == pos {
            let new_length = length - 1;
            self.map.remove(&start);
            if new_length > 0 {
                self.map.insert(start + 1, new_length);
            }
        } else if start + (length as isize) - 1 == pos {
            self.map.insert(start, length - 1);
        } else {
            let first_length = pos - start;
            let second_length = length - (pos - start) as usize - 1;
            self.map.insert(start, first_length as usize);
            self.map.insert(pos + 1, second_length);
        }
    }

    pub fn get_inverse_on_range(&self, start: isize, end: isize) -> Self {
        let mut segments = Segments::new();
        let previous = self
            .map
            .range((Included(isize::MIN), Excluded(start)))
            .rev()
            .next();

        let mut current_pos = start;
        if let Some((pos, size)) = previous {
            current_pos = current_pos.max(*pos + *size as isize);
        }

        self.map
            .range((Included(start), Included(end)))
            .for_each(|(pos, size)| {
                if *pos > current_pos {
                    let new_segment = Segment {
                        start: current_pos,
                        length: (pos - current_pos) as usize,
                    };
                    segments.add_segment(new_segment);
                }
                current_pos = pos + *size as isize;
            });

        if current_pos < end + 1 {
            let new_segment = Segment {
                start: current_pos,
                length: (end + 1 - current_pos) as usize,
            };
            segments.add_segment(new_segment);
        }

        segments
    }

    pub fn get_covered(&self) -> usize {
        self.map.values().sum()
    }
}

pub struct Couple {
    pub sensor: Pos,
    pub beacon: Pos,
    pub distance: usize,
}
