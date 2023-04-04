import pickle

import pytest
import repeto

SHIFTS = [0, -1, 1, -10, 10]


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
        assert repeat == repeto.Range(start, end)
        assert repeat == repeat

    r = repeto.Range(start, end)
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
        assert segment.brange() == repeto.Range(left.start, right.end)
        assert segment.left == left and segment.right == right
        assert str(segment) == f"RepeatSegment {{ {left} <=> {right} }}"
        assert segment == repeto.RepeatSegment(left, right)
        assert segment == segment

    left = repeto.Range(left[0], left[1])
    right = repeto.Range(right[0], right[1])
    segment = repeto.RepeatSegment(left, right)
    dotest(segment, left, right)

    for shift in SHIFTS:
        segment.shift(shift)
        left.shift(shift)
        right.shift(shift)

        dotest(segment, left, right)


@pytest.mark.parametrize(
    "segments",
    [
        ([(0, 10), (20, 30)],),
        ([(0, 10), (20, 30)], [(12, 14), (15, 17)]),
        ([(-20, -15), (120, 125)], [(-5, 0), (0, 5)]),
        pytest.param(([(-5, 0), (0, 5)], [(-10, -5), (5, 10)]), marks=pytest.mark.xfail),
        pytest.param(([(-5, 0), (0, 5)], [(4, 6), (8, 10)]), marks=pytest.mark.xfail),
        pytest.param(([],), marks=pytest.mark.xfail),
    ]
)
def test_inverted_repeat(segments):
    def dotest(repeat, segments):
        assert repeat.brange() == repeto.Range(segments[0].left.start, segments[0].right.end)
        assert repeat.segments == segments
        assert repeat == repeto.InvertedRepeat(segments)
        assert repeat == repeat

    segments = [
        repeto.RepeatSegment(repeto.Range(r[0], r[1]), repeto.Range(l[0], l[1]))
        for r, l in segments
    ]
    repeat = repeto.InvertedRepeat(segments)
    dotest(repeat, segments)

    for shift in SHIFTS:
        for s in segments:
            s.shift(shift)
        repeat.shift(shift)


def test_pickle():
    def dotest(obj):
        pickled = pickle.dumps(obj)
        assert obj == pickle.loads(pickled)

    dotest(repeto.Range(0, 10))
    dotest(repeto.RepeatSegment(repeto.Range(0, 10), repeto.Range(20, 30)))

    segments = [repeto.RepeatSegment(repeto.Range(0, 1), repeto.Range(2, 3))]
    dotest(repeto.InvertedRepeat(segments))

    segments = [repeto.RepeatSegment(repeto.Range(0, 10), repeto.Range(90, 100)),
                repeto.RepeatSegment(repeto.Range(25, 30), repeto.Range(75, 80)),
                repeto.RepeatSegment(repeto.Range(50, 60), repeto.Range(60, 70))]
    dotest(repeto.InvertedRepeat(segments))
