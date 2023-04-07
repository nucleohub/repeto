from typing import Sequence, List, Tuple


class Range:
    """
    Half-open interval, i.e. [), in sequence coordinates.
    """
    start: int
    end: int

    __hash__ = None

    def __init__(self, start: int, end: int): ...

    def shift(self, shift: int):
        """
        Shift the entire range by the given value(in place). Useful for mapping coordinates.
        """
        pass

    def __repr__(self) -> str: ...

    def __len__(self) -> int: ...

    def __eq__(self, other) -> bool: ...


class RepeatSegment:
    """
    Complementary sequence segments that are part of a larger inverted repeat.

    It's guaranteed that:
     * left segment doesn't overlap the right segment
     * left segment coordinates < right segment coordinates
    """
    left: Range
    right: Range

    def __init__(self, left: Range, right: Range): ...

    def brange(self) -> Range:
        """
        A bounding range of the segment - minimum range that contains both the left and right arms of the segment.
        """
        pass

    def shift(self, shift: int):
        """
        Shift the entire segment by the given value(in place). Useful for mapping coordinates.
        """
        pass

    def __repr__(self) -> str: ...

    def __len__(self) -> int: ...

    def __eq__(self, other) -> bool: ...


class InvertedRepeat:
    """
    Inverted repeats composed of complementary segments separated by variable gaps.

    In terms of alignment scores, inverted repeat is locally optimal. That is, it can't be extended or shrunk to
    get a higher alignment score.
    """
    segments: Sequence[RepeatSegment]

    def __init__(self, segments: List[RepeatSegment]):
        """
        Construct a new inverted repeat from the given segments.
        Segments must not overlap and must be sorted by starting position (eg segment.brange().start).
        """
        pass

    def brange(self) -> Range:
        """
        A bounding range of the repeat - minimum range that contains all its segments.
        """
        pass

    def shift(self, offset: int):
        """
        Shift the entire repeat by the given value(in place). Useful for mapping coordinates.
        """
        pass

    def seqranges(self) -> List[Range]:
        """
        Ordered sequence blocks, i.e. sequence ranges, that underlay the inverted repeat.
        """
        pass

    def to_bed12(self, contig: str, *args,
                 name: str = ".", score: int = 0, strand: str = ".", color: str = "0,0,0") -> str:
        """
        Convert inverted repeat to a BED12 record. All arguments except the contig should be passed as kwargs
        """
        pass

    def __len__(self) -> int:
        """
        The length of inverted repeats is defined as the total number of base pairs of the underlying segments.
        """
        pass

    def __eq__(self, other) -> bool: ...


def predict(seq: bytes, min_score: int, min_matches_run: int) -> Tuple[List[InvertedRepeat], List[int]]:
    """
    Predict inverted repeats in the given nucleic acid sequence.

    :param seq: raw ASCII string, DNA or RNA sequence
    :param min_score: min self-alignment score for predict inverted repeats
    :param min_matches_run: min number of continuous matches (complementary base pairs) in predicted inverted repeats
    :return: list of inverted repeats satisfying given constraints and their alignment scores
    """
    pass


def optimize(ir: List[InvertedRepeat], scores: List[int]) -> Tuple[List[InvertedRepeat], int]:
    """
    Find score-maximal and coherent set of inverted nucleic acid repeats.

    Here, coherent matching means that there are no "interfering" inverted repeats in the final set.
    That is, there are no i and j where:
        left(i) <= left(j) < right(i) <= right(j)
    Visually (- is gap, * is matched nucleotide):
        *****--------*****
               ****---------****

    The following combinations are allowed:
        left(i) < right(i) < left(j) < right(j)
        *****--------*****    ****---------****

        left(i) < left(j) < right(j) < right(i)
        *****----------------------*****
               ****---------****

    Such coherent matching also represents a formally valid (but very rough) RNA secondary structure.

    :param ir: list of InvertedRepeat objects
    :param scores: numerical score for each InvertedRepeat
    :return: Tuple containing an optimal set of inverted repeats and the associated total score
    """
    pass
