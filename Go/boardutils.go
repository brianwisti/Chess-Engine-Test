package main

func IsOccupied(bitboard uint64, square int) bool {
	return (bitboard & SQUARE_BBS[square]) != 0
}

func GetOccupiedIndex(square int) int {
	for i := range 12 {
		if IsOccupied(PieceArray[i], square) {
			return i
		}
	}

	return EMPTY
}

func OutOfBounds(move int) bool {
	const lastTile = 64

	if move < 0 {
		return true
	}

	if move > lastTile {
		return true
	}

	return false
}
