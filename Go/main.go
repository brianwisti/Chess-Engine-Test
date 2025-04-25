package main

func main() {
	testDepth := 6

	LoadFen(FEN_STARTING_POSITION)
	PrintBoard()
	RunPerftInline(testDepth)
}
