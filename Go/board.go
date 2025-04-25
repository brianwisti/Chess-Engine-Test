package main

import "fmt"

var PieceArray [12]uint64 = [12]uint64{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}
var whiteToPlay bool = true
var CastleRights [4]bool = [4]bool{true, true, true, true}
var ep uint8 = NO_SQUARE
var BoardPly uint8 = 0

func SetStartingPosition() {
	ep = NO_SQUARE
	whiteToPlay = true
	CastleRights[0] = true
	CastleRights[1] = true
	CastleRights[2] = true
	CastleRights[3] = true

	PieceArray[WP] = WP_STARTING_POSITIONS
	PieceArray[WN] = WN_STARTING_POSITIONS
	PieceArray[WB] = WB_STARTING_POSITIONS
	PieceArray[WR] = WR_STARTING_POSITIONS
	PieceArray[WQ] = WQ_STARTING_POSITION
	PieceArray[WK] = WK_STARTING_POSITION
	PieceArray[BP] = BP_STARTING_POSITIONS
	PieceArray[BN] = BN_STARTING_POSITIONS
	PieceArray[BB] = BB_STARTING_POSITIONS
	PieceArray[BR] = BR_STARTING_POSITIONS
	PieceArray[BQ] = BQ_STARTING_POSITION
	PieceArray[BK] = BK_STARTING_POSITION
}

func PrintBoard() {
	fmt.Println("Board:")

	var boardArray = [64]int(FillBoardArray())

	for rank := range 8 {
		fmt.Print("   ")

		for file := range 8 {
			var square = (rank * 8) + file

			fmt.Printf("%c%c ", PieceColours[boardArray[square]], PieceNames[boardArray[square]])
		}

		fmt.Println()
	}

	fmt.Println()

	fmt.Printf("White to play: %t\n", whiteToPlay)

	fmt.Printf("Castle: %t %t %t %t\n", CastleRights[0], CastleRights[1], CastleRights[2], CastleRights[3])
	fmt.Printf("ep: %d\n", ep)
	fmt.Printf("ply: %d\n", BoardPly)
	fmt.Println()
	fmt.Println()
}

func PrintAllBitboards() {
	for i := range 12 {
		fmt.Println(PieceArray[i])
	}
}

func FillBoardArray() []int {
	var boardArray [64]int
	for i := range 64 {
		boardArray[i] = GetOccupiedIndex(i)
	}

	return boardArray[:]
}

func IsBoardArraySame(copy [12]uint64) bool {
	for i := range 12 {
		if PieceArray[i] != copy[i] {
			fmt.Printf("ERROR piece not same: %d\n", i)

			return false
		}
	}

	return true
}

func PrintBitboard(bitboard uint64) {
	for rank := range 8 {
		for file := range 8 {
			var square = (rank * 8) + file
			if (bitboard & (ONE_U64 << square)) != 0 {
				fmt.Print("X ")
				continue
			}

			fmt.Print("_ ")
		}

		fmt.Println()
	}

	fmt.Println(bitboard)
}

func ResetBoard() {
	for i := range 12 {
		PieceArray[i] = EMPTY_BITBOARD
	}

	whiteToPlay = true

	for i := range 4 {
		CastleRights[i] = true
	}

	ep = NO_SQUARE
	BoardPly = 0
}
