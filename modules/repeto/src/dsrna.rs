use std::cmp::{max, min};
use std::ops::Range;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Segment {
    top: Range<usize>,
    bottom: Range<usize>,
}

impl Segment {
    pub fn new(top: Range<usize>, bottom: Range<usize>) -> Self {
        assert_eq!(top.len(), bottom.len());
        assert!(top.start < top.end && top.end <= bottom.start && bottom.start < bottom.end);
        Self { top, bottom }
    }

    pub fn top(&self) -> &Range<usize> {
        &self.top
    }

    pub fn bottom(&self) -> &Range<usize> {
        &self.bottom
    }
}

impl From<(Range<usize>, Range<usize>)> for Segment {
    fn from(value: (Range<usize>, Range<usize>)) -> Self {
        Self {
            top: value.0,
            bottom: value.1,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct dsRNA {
    segments: Vec<Segment>,
    brange: Range<usize>,
}

impl dsRNA {
    pub fn new(segments: Vec<Segment>) -> Self {
        assert!(!segments.is_empty());

        // Derive the bounding range
        let (mut start, mut end) = (segments[0].top.start, segments[0].top.end);
        for segment in &segments {
            for s in [&segment.top, &segment.bottom] {
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
            blocks.push(segment.top.clone());
            blocks.push(segment.bottom.clone());
        }
        blocks
    }
}
