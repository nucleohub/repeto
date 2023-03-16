use super::super::dsRNA;
use std::ops::Range;

pub struct IndexAnchor {
    pub pos: usize,
    pub rnas: Vec<usize>,
}

pub struct Index {
    // Sorted rnas based on their start or end positions
    starts: Vec<IndexAnchor>,
    ends: Vec<IndexAnchor>,

    // RNA id -> start & end index
    revstart: Vec<usize>,
    revend: Vec<usize>,

    // dsRNA blocks in each dsRNA
    blocks: Vec<Vec<Range<usize>>>,
}

impl Index {
    pub fn new(dsrna: &[dsRNA]) -> Self {
        let (starts, revstart) = Index::index(dsrna, |x| x.brange().start);
        let (ends, revend) = Index::index(dsrna, |x| x.brange().end);

        let blocks = dsrna
            .iter()
            .map(|x| {
                let mut blocks = x.blocks();
                blocks.sort_by_key(|x| x.start);
                blocks
            })
            .collect();

        Self {
            starts,
            ends,
            revstart,
            revend,
            blocks,
        }
    }

    pub fn ends(&self) -> &[IndexAnchor] {
        &self.ends
    }

    pub fn starts(&self) -> &[IndexAnchor] {
        &self.starts
    }

    pub fn revmap(&self, rnaid: usize) -> (usize, usize) {
        (self.revstart[rnaid], self.revend[rnaid])
    }

    pub fn blocks(&self, rnaid: usize) -> &[Range<usize>] {
        &self.blocks[rnaid]
    }

    fn index(
        rnas: &[dsRNA],
        key: impl for<'b> Fn(&'b dsRNA) -> usize,
    ) -> (Vec<IndexAnchor>, Vec<usize>) {
        let mut argsort = (0..rnas.len()).collect::<Vec<_>>();
        argsort.sort_by_key(|x| key(&rnas[*x]));

        let mut index = Vec::with_capacity(rnas.len());
        let mut revmap = vec![0; rnas.len()];

        let mut curkey = key(&rnas[argsort[0]]);
        let mut cache = vec![argsort[0]];

        for rnaind in argsort.into_iter().skip(1) {
            if key(&rnas[rnaind]) != curkey {
                for ind in &cache {
                    revmap[*ind] = index.len();
                }
                index.push(IndexAnchor {
                    pos: curkey,
                    rnas: cache,
                });

                curkey = key(&rnas[rnaind]);
                cache = vec![rnaind];
            } else {
                cache.push(rnaind);
            }
        }
        for ind in &cache {
            revmap[*ind] = index.len();
        }
        index.push(IndexAnchor {
            pos: curkey,
            rnas: cache,
        });
        (index, revmap)
    }
}

pub mod bisect {
    use super::IndexAnchor;

    pub fn right(data: &[IndexAnchor], pos: usize, mut lo: usize, mut hi: usize) -> usize {
        debug_assert!(lo <= hi && hi <= data.len());
        while lo < hi {
            let mid = (lo + hi) / 2;
            if pos < data[mid].pos {
                hi = mid
            } else {
                lo = mid + 1
            };
        }
        lo
    }

    pub fn left(data: &[IndexAnchor], pos: usize, mut lo: usize, mut hi: usize) -> usize {
        debug_assert!(lo <= hi && hi <= data.len());
        while lo < hi {
            let mid = (lo + hi) / 2;
            if data[mid].pos < pos {
                lo = mid + 1
            } else {
                hi = mid
            };
        }
        lo
    }
}
