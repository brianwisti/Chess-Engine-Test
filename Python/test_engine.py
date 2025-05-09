"""Check that I haven't messed up the original implementation."""

import pytest

from engine import DEFAULT_DEPTH, NO_SQUARE, Board, perft_inline, set_starting_position


@pytest.fixture
def board():
    return Board()


EXPECTED_OUTPUT = """b1a3: 4856835
b1c3: 5708064
g1f3: 5723523
g1h3: 4877234
a2a3: 4463267
a2a4: 5363555
b2b3: 5310358
b2b4: 5293555
c2c3: 5417640
c2c4: 5866666
d2d3: 8073082
d2d4: 8879566
e2e3: 9726018
e2e4: 9771632
f2f3: 4404141
f2f4: 4890429
g2g3: 5346260
g2g4: 5239875
h2h3: 4463070
h2h4: 5385554
"""


class TestBoard:
    def test_clear_ep(self, board):
        board.ep = 1
        board.clear_ep()
        assert board.ep == NO_SQUARE


def test_perft_inline(capsys):
    initial_ply = 0
    depth = DEFAULT_DEPTH
    expected_node_count = 119_060_324
    board = set_starting_position()

    node_count = perft_inline(board, depth, initial_ply)
    assert node_count == expected_node_count

    captured = capsys.readouterr()
    assert captured.out == EXPECTED_OUTPUT
