import pickle

import pytest
import repeto as rpt

SHIFTS = [0, -1, 1, -10, 10]

IRS = {
    "single-block": ([(0, 10), (20, 30)],),
    "two-blocks": ([(0, 10), (20, 30)], [(12, 14), (15, 17)]),
    "three-blocks": ([(0, 10), (90, 100)], [(25, 30), (75, 80)], [(50, 60), (60, 70)]),
    "negative-coords": ([(-20, -15), (120, 125)], [(-5, 0), (0, 5)]),
}


def _make_ir(segments):
    segments = [
        rpt.RepeatSegment(rpt.Range(r[0], r[1]), rpt.Range(l[0], l[1]))
        for r, l in segments
    ]
    return rpt.InvertedRepeat(segments), segments


@pytest.mark.parametrize(
    ("start", "end"),
    [
        (1, 2), (-10, 20), (3, 4),
        pytest.param(1, 1, marks=pytest.mark.xfail),
        pytest.param(1, -1, marks=pytest.mark.xfail),
        pytest.param(-10, -20, marks=pytest.mark.xfail),
        pytest.param(20, 10, marks=pytest.mark.xfail),
    ]
)
def test_range(start, end):
    def dotest(repeat, start, end):
        assert repeat.start == start and repeat.end == end
        assert str(repeat) == f"{start}-{end}"
        assert len(repeat) == end - start
        assert repeat == rpt.Range(start, end)
        assert repeat == repeat

    r = rpt.Range(start, end)
    dotest(r, start, end)

    for shift in SHIFTS:
        r.shift(shift)
        start, end = start + shift, end + shift
        dotest(r, start, end)


@pytest.mark.parametrize(
    ("left", "right"),
    [
        ((0, 10), (20, 30)), ((-10, -5), (5, 10)), ((0, 100), (100, 200)),
        pytest.param((10, 20), (15, 25), marks=pytest.mark.xfail),
        pytest.param((15, 25), (10, 20), marks=pytest.mark.xfail),
        pytest.param((5, 10), (-5, 0), marks=pytest.mark.xfail),
    ]
)
def test_repeat_segment(left, right):
    def dotest(segment, left, right):
        assert segment.brange() == rpt.Range(left.start, right.end)
        assert segment.left == left and segment.right == right
        assert str(segment) == f"RepeatSegment {{ {left} <=> {right} }}"
        assert segment == rpt.RepeatSegment(left, right)
        assert segment == segment

    left = rpt.Range(left[0], left[1])
    right = rpt.Range(right[0], right[1])
    segment = rpt.RepeatSegment(left, right)
    dotest(segment, left, right)

    for shift in SHIFTS:
        segment.shift(shift)
        left.shift(shift)
        right.shift(shift)

        dotest(segment, left, right)


@pytest.mark.parametrize(
    "segments",
    [
        IRS['single-block'], IRS['two-blocks'], IRS['negative-coords'], IRS['three-blocks'],
        pytest.param(([(-5, 0), (0, 5)], [(-10, -5), (5, 10)]), marks=pytest.mark.xfail),
        pytest.param(([(-5, 0), (0, 5)], [(4, 6), (8, 10)]), marks=pytest.mark.xfail),
        pytest.param(([],), marks=pytest.mark.xfail),
    ]
)
def test_inverted_repeat(segments):
    def dotest(repeat, segments):
        assert repeat.brange() == rpt.Range(segments[0].left.start, segments[0].right.end)
        assert repeat.seqranges() == \
               sorted([x.left for x in segments] + [x.right for x in segments], key=lambda x: x.start)
        assert repeat.segments == segments
        assert repeat == rpt.InvertedRepeat(segments)
        assert repeat == repeat
        assert len(repeat) == sum(len(x) for x in segments)

    repeat, segments = _make_ir(segments)
    dotest(repeat, segments)

    for shift in SHIFTS:
        for s in segments:
            s.shift(shift)
        repeat.shift(shift)
        dotest(repeat, segments)


def test_pickle():
    def dotest(obj):
        pickled = pickle.dumps(obj)
        assert obj == pickle.loads(pickled)

    dotest(rpt.Range(0, 10))
    dotest(rpt.RepeatSegment(rpt.Range(0, 10), rpt.Range(20, 30)))

    segments = [rpt.RepeatSegment(rpt.Range(0, 1), rpt.Range(2, 3))]
    dotest(rpt.InvertedRepeat(segments))

    for segments in IRS.values():
        repeat, _ = _make_ir(segments)
        dotest(repeat)


# "negative-coords": ([(-20, -15), (120, 125)], [(-5, 0), (0, 5)]),
@pytest.mark.parametrize(
    ("segments", "start", "end", "blocks", "sizes", "starts"),
    [
        (IRS["single-block"], 0, 30, 2, "10,10", "0,20"),
        (IRS["two-blocks"], 0, 30, 4, "10,2,2,10", "0,12,15,20"),
        (IRS["three-blocks"], 0, 100, 6, "10,5,10,10,5,10", "0,25,50,60,75,90"),
        (IRS["negative-coords"], -20, 125, 4, "5,5,5,5", "0,15,20,140"),
    ]
)
def test_bed12(segments, start, end, blocks, sizes, starts):
    repeat, _ = _make_ir(segments)

    bed12 = repeat.to_bed12(".")
    assert bed12 == f".\t{start}\t{end}\t.\t0\t.\t{start}\t{end}\t0,0,0\t{blocks}\t{sizes}\t{starts}"

    for shift in SHIFTS:
        start, end = start + shift, end + shift
        repeat.shift(shift)
        for contig in "1", "2", "X":
            for name in ".", "NAME":
                for score in 0, 100, 1000:
                    for strand in "A", ".", "+", "-":
                        for color in "0,0,0", "255,0,0", "ASD":
                            bed12 = repeat.to_bed12(contig, name=name, score=score, strand=strand, color=color)

                            expected = f"{contig}\t{start}\t{end}\t{name}\t{score}\t{strand}\t{start}\t{end}\t{color}" \
                                       f"\t{blocks}\t{sizes}\t{starts}"

                            assert bed12 == expected, (name, score, strand, color)
