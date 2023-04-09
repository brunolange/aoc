import pytest

from wrapping_paper import paper_needed, Box

@pytest.mark.parametrize("box,expected", [
    (Box(height=2, width=3, length=4), 58),
    (Box(height=1, width=1, length=10), 43),
])
def test_wrapping_paper(box, expected):
    assert paper_needed(box) == expected
