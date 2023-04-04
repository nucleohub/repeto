use std::fmt::{Debug, Formatter};
use std::ops::Range;

use derive_getters::{Dissolve, Getters};
use itertools::Itertools;

#[derive(Eq, PartialEq, Hash, Clone, Getters, Dissolve)]
pub struct Segment {
    left: Range<isize>,
    right: Range<isize>,
}

impl Segment {
    pub fn new(left: Range<isize>, right: Range<isize>) -> Self {
        assert!(left.start < left.end, "Sequence range start must be < end: {:?}", left);
        assert!(right.start < right.end, "Sequence range start must be < end: {:?}", right);

        assert_eq!(left.end - left.start, right.end - right.start, "Repeat segments' length must be equal");
        assert!(
            left.start < left.end && left.end <= right.start && right.start < right.end,
            "Repeat segments must not overlap: {:?} vs {:?}", left, right
        );
        Self { left, right }
    }

    pub fn len(&self) -> usize { (self.left.end - self.left.start) as usize }

    pub fn shift(&mut self, shift: isize) {
        self.left.start += shift;
        self.left.end += shift;

        self.right.start += shift;
        self.right.end += shift;
    }

    pub fn brange(&self) -> Range<isize> { self.left.start..self.right.end }
}

impl Debug for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "inv::Repeat [{}, {}) <=> [{}, {})",
            self.left.start, self.left.end, self.right.start, self.right.end
        )
    }
}

impl From<(Range<isize>, Range<isize>)> for Segment {
    fn from(value: (Range<isize>, Range<isize>)) -> Self {
        Self { left: value.0, right: value.1 }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Getters, Dissolve)]
pub struct Repeat {
    segments: Vec<Segment>,
}

impl Repeat {
    pub fn new(segments: Vec<Segment>) -> Self {
        assert!(!segments.is_empty(), "Inverted repeat must have at least one segment");
        // segments.sort_by_key(|x| x.left.start);

        for (prev, nxt) in segments.iter().tuple_windows() {
            assert!(
                (prev.left.end <= nxt.left.start) && (prev.right.start >= nxt.right.end),
                "Segments shouldn't overlap: {:?} vs {:?}", prev, nxt
            );
        }

        Self { segments }
    }

    pub fn shift(&mut self, shift: isize) {
        for x in &mut self.segments { x.shift(shift) };
    }

    pub fn brange(&self) -> Range<isize> { self.segments[0].brange() }
}
