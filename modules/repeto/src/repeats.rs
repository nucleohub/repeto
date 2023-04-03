use std::cmp::{max, min};
use std::ops::Range;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Segment {
    left: Range<usize>,
    right: Range<usize>,
}

impl Segment {
    pub fn new(left: Range<usize>, right: Range<usize>) -> Self {
        assert_eq!(left.len(), right.len());
        assert!(left.start < left.end && left.end <= right.start && right.start < right.end);
        Self { left, right }
    }

    pub fn left(&self) -> &Range<usize> {
        &self.left
    }

    pub fn right(&self) -> &Range<usize> {
        &self.right
    }
}

impl From<(Range<usize>, Range<usize>)> for Segment {
    fn from(value: (Range<usize>, Range<usize>)) -> Self {
        Self {
            left: value.0,
            right: value.1,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct InvertedRepeat {
    segments: Vec<Segment>,
    brange: Range<usize>,
}

impl InvertedRepeat {
    pub fn new(segments: Vec<Segment>) -> Self {
        assert!(!segments.is_empty());

        // Derive the bounding range
        let (mut start, mut end) = (segments[0].left.start, segments[0].left.end);
        for segment in &segments {
            for s in [&segment.left, &segment.right] {
                start = min(start, s.start);
                end = max(end, s.end);
            }
        }
        let brange = Range { start, end };

        Self { segments, brange }
    }

    #[inline(always)]
    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    #[inline(always)]
    pub fn brange(&self) -> &Range<usize> {
        &self.brange
    }

    pub fn blocks(&self) -> Vec<Range<usize>> {
        let mut blocks = Vec::with_capacity(self.segments.len() * 2);
        for segment in &self.segments {
            blocks.push(segment.left.clone());
            blocks.push(segment.right.clone());
        }
        blocks
    }
}
