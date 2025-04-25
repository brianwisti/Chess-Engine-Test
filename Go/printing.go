package main

import "fmt"

func PrintMoveNoNL(starting int, target_square int, tag int) { //starting
	if OutOfBounds(starting) {
		fmt.Printf("%d", starting)
	} else {
		fmt.Printf("%c", SQ_CHAR_X[starting])
		fmt.Printf("%c", SQ_CHAR_Y[starting])
	}

	//target
	if OutOfBounds(target_square) {
		fmt.Printf("%d", target_square)
	} else {
		fmt.Printf("%c", SQ_CHAR_X[target_square])
		fmt.Printf("%c", SQ_CHAR_Y[target_square])
	}

	switch tag {
		case TAG_BCaptureKnightPromotion, TAG_BKnightPromotion, TAG_WKnightPromotion, TAG_WCaptureKnightPromotion:
			fmt.Printf("n")
		case TAG_BCaptureRookPromotion, TAG_BRookPromotion, TAG_WRookPromotion, TAG_WCaptureRookPromotion:
			fmt.Printf("r")
		case TAG_BCaptureBishopPromotion, TAG_BBishopPromotion, TAG_WBishopPromotion, TAG_WCaptureBishopPromotion:
			fmt.Printf("b")
		case TAG_BCaptureQueenPromotion, TAG_BQueenPromotion, TAG_WQueenPromotion, TAG_WCaptureQueenPromotion:
			fmt.Printf("q")
	}
}
