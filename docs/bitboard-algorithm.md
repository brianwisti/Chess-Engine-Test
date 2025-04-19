# Bitboard algorithm

Cited source for the bitboard chess engine approach:

- [Bitboard CHESS ENGINE in C: intro][bitboard-video]
- [Wikipedia entry on bitboards][bitboard-wiki]

[bitboard-video]: https://www.youtube.com/watch?v=QUNP-UjujBM
[bitboard-wiki]: https://en.wikipedia.org/wiki/Bitboard

## Bitwise operations

| Operator | Name        | What it does                                      |
| -------- | ----------- | ------------------------------------------------- |
| `|`      | OR          | set a bit to 1 if either bit in two integers is 1 |
| `&`      | AND         | set a bit to 1 if both bits in two integers is 1  |
| `~`      | NOT         | invert all bits to the opposite                   |
| `>>`     | SHIFT RIGHT | move all bits right; right means a lower value    |
| `<<`     | SHIFT LEFT  | move all bits left; left means a higher value     |
| `^`      | XOR         | set `1` if bit is set in both values, else `0`    |

## Adding and removing bits

All pieces are represented by an unsigned 64 bit integer.

```text
unsigned long long bitboard_array[12];
const int WP = 0, WN = 1, WB = 2, WR = 3, WQ = 4, WK = 5, BP = 6, BN = 7, BB = 8, BR = 9, BQ = 10, BK = 11;
```

You can add a bit to a bitboard using a bitwise or: `|`

```text
unsigned long long bitboard = 0;
bitboard |= 1ULL << 50; //places 1 bit in square 50, c2
```

The bitshift operator will move all bits by that amount.
1 shifted by 50 simply puts one bit in "square" 50.
To avoid bitshifting I mostly use the `SQUARE_BBS` array.
This simply indexes all of those values in a constant.
So instead we write:

```text
unsigned long long bitboard = 0;
bitboard |= SQUARE_BBS[50]; //places 1 bit in square 50, c2
```

These are the same. To make this a bit more human I also made square constants:

```text
const int A8 = 0, B8 = 1, C8 = 2 // etc.
```

Normally integers use Big Endian Format, meaning the highest numbers at the start.
Print bitboards the opposite to be more like a chess board.
So in this order: 1,2,4,8,16,32,64,128 etc.
To remove a bit we need to set that bit on a ulong and then invert the bits:

```text
unsigned long long bitboard = 0;
bitboard |= SQUARE_BBS[50]; //places 1 bit in square 50, c2
bitboard &= ~SQUARE_BBS[50]; //bitboard now = 0 again

```mermaidjs
block-beta
  columns 1

  label["SQUARE_BBS[50]"]

  block
    columns 8

    a8["_"]  b8["_"] c8["_"] d8["_"] e8["_"] f8["_"] g8["_"] h8["_"]
    a7["_"]  b7["_"] c7["_"] d7["_"] e7["_"] f7["_"] g7["_"] h7["_"]
    a6["_"]  b6["_"] c6["_"] d6["_"] e6["_"] f6["_"] g6["_"] h6["_"]
    a5["_"]  b5["_"] c5["_"] d5["_"] e5["_"] f5["_"] g5["_"] h5["_"]
    a4["_"]  b4["_"] c4["_"] d4["_"] e4["_"] f4["_"] g4["_"] h4["_"]
    a3["_"]  b3["_"] c3["_"] d3["_"] e3["_"] f3["_"] g3["_"] h3["_"]
    a2["_"]  b2["_"] c2["1"] d2["_"] e2["_"] f2["_"] g2["_"] h2["_"]
    a1["_"]  b1["_"] c1["_"] d1["_"] e1["_"] f1["_"] g1["_"] h1["_"]
  end
```

```mermaidjs
block-beta
  columns 1

  label["~SQUARE_BBS[50]"]

  block
    columns 8

    a8["1"]  b8["1"] c8["1"] d8["1"] e8["1"] f8["1"] g8["1"] h8["1"]
    a7["1"]  b7["1"] c7["1"] d7["1"] e7["1"] f7["1"] g7["1"] h7["1"]
    a6["1"]  b6["1"] c6["1"] d6["1"] e6["1"] f6["1"] g6["1"] h6["1"]
    a5["1"]  b5["1"] c5["1"] d5["1"] e5["1"] f5["1"] g5["1"] h5["1"]
    a4["1"]  b4["1"] c4["1"] d4["1"] e4["1"] f4["1"] g4["1"] h4["1"]
    a3["1"]  b3["1"] c3["1"] d3["1"] e3["1"] f3["1"] g3["1"] h3["1"]
    a2["1"]  b2["1"] c2["_"] d2["1"] e2["1"] f2["1"] g2["1"] h2["1"]
    a1["1"]  b1["1"] c1["1"] d1["1"] e1["1"] f1["1"] g1["1"] h1["1"]
  end

If you bitwise AND (`&`) with the inverted bitboard it will keep all bits the same except the empty one.
Let's say we want to move a pawn:

```text
unsigned long long black_pawns = 65280;
```

in bits:

```mermaidjs
block-beta
  columns: 8
  a8["_"]  b8["_"] c8["_"] d8["_"] e8["_"] f8["_"] g8["_"] h8["_"]
  a7["1"]  b7["1"] c7["1"] d7["1"] e7["1"] f7["1"] g7["1"] h7["1"]
  a6["_"]  b6["_"] c6["_"] d6["_"] e6["_"] f6["_"] g6["_"] h6["_"]
  a5["_"]  b5["_"] c5["_"] d5["_"] e5["_"] f5["_"] g5["_"] h5["_"]
  a4["_"]  b4["_"] c4["_"] d4["_"] e4["_"] f4["_"] g4["_"] h4["_"]
  a3["_"]  b3["_"] c3["_"] d3["_"] e3["_"] f3["_"] g3["_"] h3["_"]
  a2["_"]  b2["_"] c2["_"] d2["_"] e2["_"] f2["_"] g2["_"] h2["_"]
  a1["_"]  b1["_"] c1["_"] d1["_"] e1["_"] f1["_"] g1["_"] h1["_"]
```

We need to remove the bit we want and place it somewhere else. Let's e7 to e5:

```text
black_pawns |= SQUARE_BBS[E5]; //add the bit
_  _  _  _  _  _  _  _
1  1  1  1  1  1  1  1
_  _  _  _  _  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
```

This puts a bit on e5. Now we remove e7 like so:

```text
black_pawns &= ~SQUARE_BBS[E7]; //remove the bit
_  _  _  _  _  _  _  _
1  1  1  1  _  1  1  1
_  _  _  _  _  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
```

That's how we move pieces.

## Generate moves

## Occupancies

We track white and black pieces together using occupancies.
One approach is to make them global:

```text
unsigned long long occupancies[3];
const int WHITE_OCCUPANCIES = 0, BLACK_OCCUPANCIES = 1, COMBINED_OCCUPANCIES = 2;
```

The fastest approach is to update the occupancies with every move. However this gave me the most headaches
imaginable with debugging. For example Castling:

```text
case TAG_W_CASTLE_KS: //it's a castle move
  bitboard_array_global[WK] |= Constants.SQUARE_BBS[G1]; //move the white king to G1
  bitboard_array_global[WK] &= ~Constants.SQUARE_BBS[E1];
  bitboard_array_global[WR] |= Constants.SQUARE_BBS[F1]; //move the white rook to F1
  bitboard_array_global[WR] &= ~Constants.SQUARE_BBS[H1];
  //We do the same for the occupancies
  occupancies[WHITE_OCCUPANCIES] |= Constants.SQUARE_BBS[G1];
  occupancies[WHITE_OCCUPANCIES] &= ~Constants.SQUARE_BBS[E1];
  occupancies[WHITE_OCCUPANCIES] |= Constants.SQUARE_BBS[F1];
  occupancies[WHITE_OCCUPANCIES] &= ~Constants.SQUARE_BBS[H1];
  occupancies[COMBINED_OCCUPANCIES] |= Constants.SQUARE_BBS[G1];
  occupancies[COMBINED_OCCUPANCIES] &= ~Constants.SQUARE_BBS[E1];
  occupancies[COMBINED_OCCUPANCIES] |= Constants.SQUARE_BBS[F1];
  occupancies[COMBINED_OCCUPANCIES] &= ~Constants.SQUARE_BBS[H1];

  castle_rights_global[WKS_CASTLE_RIGHTS] = false;
  castle_rights_global[WQS_CASTLE_RIGHTS] = false;
  ep_global = NO_SQUARE;
```

The amount of bug potential this creates is crazy. The safest thing to do is bitwise OR
the boards together for each position:

```text
const unsigned long long WHITE_OCCUPANCIES = bitboard_array[0] |
                                             bitboard_array[1] |
                                             bitboard_array[2] |
                                             bitboard_array[3] |
                                             bitboard_array[4] |
                                             bitboard_array[5];

const unsigned long long BLACK_OCCUPANCIES = bitboard_array[6] |
                                             bitboard_array[7] |
                                             bitboard_array[8] |
                                             bitboard_array[9] |
                                             bitboard_array[10] |
                                             bitboard_array[11];

const unsigned long long COMBINED_OCCUPANCIES = WHITE_OCCUPANCIES | BLACK_OCCUPANCIES;
const unsigned long long EMPTY_OCCUPANCIES = ~COMBINED_OCCUPANCIES;
```

This simply joins the white and black pieces together respectively. We can then use the occupancies for moves.
To find white capture moves we bitwise AND the attacks with BLACK_OCCUPANCIES and viceversa for black occupancies.
To get regular moves we bitwise AND attacks with EMPTY_OCCUPANCIES.

## Pins and check

First we need to work out pins and checks. The tutorial I followed didn't do this
and simply played each move and unmade the move if the king was attacked. This is
a lot slower and I found an explanation online of how to work this out.

First we get the king position, let's say it's white to play:

```text
const int whiteKingPosition = BitscanForward(bitboard_array[WK]);
```

From the king square we need to use the piece moves to see if there is a check or pin:

```text
int whiteKingCheckCount = 0; //We track this for double check and castling

//pawns
tempBitboard = bitboard_array[BP] & WHITE_PAWN_ATTACKS[whiteKingPosition]; //Here we are checking if there is a pawn diagonal from the king

if (tempBitboard != 0) //if it's not zero then there is a pawn
{
  int pawnSquare = (DEBRUIJN64[MAGIC * (tempBitboard ^ (tempBitboard - 1)) >> 58]); //This inlines BitscanForward
  checkBitboard = SQUARE_BBS[pawnSquare]; //We then set the checkbitboard with that square

  whiteKingCheckCount++;
}
```

For checks all we simply do is bitwise AND any potential moves with the checkbitboard. If there is no check then
we set checkBitboard to a MAX_ULONG which simply means all bits are set.

```text
if (whiteKingCheckCount == 0) {
  checkBitboard = MAX_ULONG;
}
```

This just avoids a conditional. So if there is a pawn attacking the white king then only moves that hit that square
will be stored.

```text
tempBitboard = bitboard_array[WN];

while (tempBitboard != 0) { //if there are still knights
   const int knightSquare = BitscanForward(tempBitboard);
   ulong knightAttacks = (KNIGHT_ATTACKS[knightSquare] & BLACK_OCCUPANCIES) & checkBitboard;
   //if the knight attacks from this square intersect with a black piece.
   //If there is a piece attacking the king, does this move attack that piece?

   while (knightAttacks != 0) {
       //save the move
   }
}
```

For pins it's a bit more complicated. We first need INBETWEEN_BITBOARDS.
This is a multi dimensional array that simply has all of this bits inbetween every square combination.
It also includes the last square.
Example:

```text
INBETWEEN_BITBOARD[E1][E8] =
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  _  _  _  _

INBETWEEN_BITBOARD[B7][G2] =
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  1  _  _  _  _  _
_  _  _  1  _  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  _  1  _  _
_  _  _  _  _  _  1  _
_  _  _  _  _  _  _  _
```

We use these for pins and check for rooks, queens and bishops.

```text
int pinArray[8][2] =
{
  { -1, -1 },
  { -1, -1 },
  { -1, -1 },
  { -1, -1 },
  { -1, -1 },
  { -1, -1 },
  { -1, -1 },
  { -1, -1 },
};
```

We create a pin array. The first index is the square being pinned. The second index is piece that is pinning the square.

```text
//bishops
//From the white king, get the bishop moves but only including the black occupancies
const unsigned long long bishopAttacksChecks = GetBishopAttacksFast(whiteKingPosition, BLACK_OCCUPANCIES_LOCAL);

temp_bitboard = bitboard_array_global[BB] & bishopAttacksChecks;  //See if there is a black bishop there
while (temp_bitboard != 0) //if there is a black bishop there
{
    const int piece_square = (DEBRUIJN64[MAGIC * (temp_bitboard ^ (temp_bitboard - 1)) >> 58]); //find the square
    temp_pin_bitboard = INBETWEEN_BITBOARDS[whiteKingPosition][piece_square] & WHITE_OCCUPANCIES_LOCAL;
    //for the squares inbetween, is there a white piece there

    if (temp_pin_bitboard == 0) //if there is no white piece, the bishop is attacking the king
    {
        if (check_bitboard == 0)
        {
            check_bitboard = INBETWEEN_BITBOARDS[whiteKingPosition][piece_square]; //add the check
        }
        whiteKingCheckCount++;
    }
    else //if there is a white piece inbetween, there is a potential pin
    {
        const int pinned_square = (DEBRUIJN64[MAGIC * (temp_pin_bitboard ^ (temp_pin_bitboard - 1)) >> 58]); //get the square
        temp_pin_bitboard &= temp_pin_bitboard - 1; //remove one bit from bitboard

        if (temp_pin_bitboard == 0) //if the bitboard is now empty then that piece was pinned.
        {
            pinArray[pinNumber][PINNED_SQUARE_INDEX] = pinned_square; //add the pinned square
            pinArray[pinNumber][PINNING_PIECE_INDEX] = piece_square; //add the pinning piece
            pinNumber++; //increase the pin number
        }
    }
    temp_bitboard &= temp_bitboard - 1; //remove the bit to stop infinitive loop
}
```

We do the same for queen and rook.
We then check if a piece is pinned like so:

```text
temp_bitboard = bitboard_array_global[WN];

while (temp_bitboard != 0)
{
  starting_square = (DEBRUIJN64[MAGIC * (temp_bitboard ^ (temp_bitboard - 1)) >> 58]);
  temp_bitboard &= temp_bitboard - 1; //removes the knight from that square to not infinitely loop

  temp_pin_bitboard = MAX_ULONG; //set it max, so allows all moves first
  if (pinNumber != 0) //if there is a pin somewhere
  {
    for (int i = 0; i < pinNumber; i++) //loop through them
    {
      if (pinArray[i][PINNED_SQUARE_INDEX] == starting_square) //if this piece is pinned
      {
        temp_pin_bitboard = INBETWEEN_BITBOARDS[whiteKingPosition][pinArray[i][PINNING_PIECE_INDEX]];
        //set the temp_pin_bitboard to the bits in between the king and pinning piece
        //This means the knight can only move
      }
    }
 }

  temp_attack = ((KNIGHT_ATTACKS[starting_square] & BLACK_OCCUPANCIES_LOCAL) & check_bitboard) & temp_pin_bitboard;
  //Then you AND the attacks with the check and pins

  //add moves if temp_attack isn't zero etc...
  while (temp_attack != 0) ...
```

Let's say in this position:

```text
__ __ __ __ BK __ __ __
__ __ __ __ BR __ __ __
__ __ __ __ __ __ __ __
__ __ __ __ __ __ __ __
__ __ __ __ __ __ __ __
__ __ __ __ __ __ __ __
__ __ __ __ WR __ __ __
__ __ __ __ WK __ __ __
```

Here both rooks are pinning each other.
The white rook bitboard looks like this:

```text
tempBitboard = bitboard_array[WR];
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  _  _  _  _
```

The rook moves will look like this:

```text
rook_attacks = GetRookAttackFast(rook_square, COMBINED_OCCUPANCIES);
_  _  _  _  _  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
1  1  1  1  _  1  1  1
_  _  _  _  1  _  _  _
```

We AND this with empty squares to get non capture moves:

```text
EMPTY_OCCUPANCIES:
1  1  1  1  _  1  1  1
1  1  1  1  _  1  1  1
1  1  1  1  1  1  1  1
1  1  1  1  1  1  1  1
1  1  1  1  1  1  1  1
1  1  1  1  1  1  1  1
1  1  1  1  _  1  1  1
1  1  1  1  _  1  1  1
```

```text
unsigned long long non_capture_moves = EMPTY_OCCUPANCIES & rook_attacks;
```

```text
non_capture_moves:
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
1  1  1  1  _  1  1  1
_  _  _  _  _  _  _  _
```

We get the capture moves by ANDing the attacks with black occupancies:

```text
BLACK_OCCUPANCIES:
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
```

```text
unsigned long long capture_moves = BLACK_OCCUPANCIES & rook_attacks;
```

```text
capture_moves:
_  _  _  _  _  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
```

The pin bitboard will look like this:

```text
_  _  _  _  _  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
```

We then AND that with captures and non captures. So if there is a pin, only these squares are valid.
Similar with check:

```text
 __ __ __ __ BK __ __ __
 __ __ __ __ BR __ __ __
 __ __ __ __ __ __ __ __
 __ __ __ __ __ __ __ __
 __ __ __ __ __ __ __ __
 __ __ __ __ __ __ __ __
 WR __ __ __ __ __ __ __
 __ __ __ __ WK __ __ __
```

The check bitboard here:

```text
_  _  _  _  _  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  _  _  _  _
```

The white rook attacks:

```text
1  _  _  _  _  _  _  _
1  _  _  _  _  _  _  _
1  _  _  _  _  _  _  _
1  _  _  _  _  _  _  _
1  _  _  _  _  _  _  _
1  _  _  _  _  _  _  _
_  1  1  1  1  1  1  1
1  _  _  _  _  _  _  _
```

AND then together:

```text
unsigned long long valid_rook_moves = checkBitboard & rook_attacks;
```

```text
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  1  _  _  _
_  _  _  _  _  _  _  _
```

Only these moves are valid.

The lazy and slow way to do pins and check:

```text
for (int i = 0; i < move_count; ++i) {

    MakeMove(move_list[i]);
    if (isWhiteToMove)
    {
      if (is_attacked_by_black(white_king_position, COMBINED_OCCUPANCIES) == true) {
        UnmakeMove(move_list[i]);
        continue;
      }
    } else {
      if (is_attacked_by_white(black_king_position, COMBINED_OCCUPANCIES) == true) {
        UnmakeMove(move_list[i]);
        continue;
      }
    }
}
```

## Finding Pieces

I didn't write BitscanForward. It uses the DEBRUJN method to get "the least significant bit".
That means the first smallest bit you find. There is a very simple way to do this but it's slower:

```text
int BitScanForwardSlow(const unsigned long long bitboard)
{
    if (bitboard == 0) { //no bits
        return -1;
    }
    for (size_t i = 0; i < 64; ++i) { //loop through the squares
        if ((bitboard & SQUARE_BBS[i]) != 0) { //We compare the bit for that square, != 0 means it is 1 in that square
            return i;
        }
    }
    return -1;
}
```

The original looks like this:

```text
inline int BitScanForward(unsigned long long bitboard)
{
  return (DEBRUIJN64[MAGIC * (bitboard ^ (bitboard - 1)) >> 58]);
}

const unsigned long long MAGIC = 0x03f79d71b4cb0a89;
const int DEBRUIJN64[64] =
{
  0, 47,  1, 56, 48, 27,  2, 60,
  57, 49, 41, 37, 28, 16,  3, 61,
  54, 58, 35, 52, 50, 42, 21, 44,
  38, 32, 29, 23, 17, 11,  4, 62,
  46, 55, 26, 59, 40, 36, 15, 53,
  34, 51, 20, 43, 31, 22, 10, 45,
  25, 39, 14, 33, 19, 30,  9, 24,
  13, 18,  8, 12,  7,  6,  5, 63
};
```

I can't explain this algorithm to you. I just copied the code but it's faster
than looping the squares. There is an integer overflow in this algorithm when you multiply so it
will cause problems in some languages.

To remove the least significant bit we use this method:

```text
tempBitboard &= tempBitboard - 1;
```

Let's say we have the black pawns in the starting position:

```text
tempBitboard = 65280:
_  _  _  _  _  _  _  _
1  1  1  1  1  1  1  1
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
_  _  _  _  _  _  _  _
```

Inside a loop we get the square and then need to remove that.
We could do this:

```text
int square = BitscanForward(tempBitboard);
tempBitboard & ~SQUARE_BBS[square];
```

This can be a problem if "square" is not valid though. I do it this way:

```mermaidjs
block-beta
  columns 8
  action["tempBitboard - 1"]:8

  a8["1"]  b8["1"] c8["1"] d8["1"] e8["1"] f8["1"] g8["1"] h8["1"]
  a7["_"]  b7["1"] c7["1"] d7["1"] e7["1"] f7["1"] g7["1"] h7["1"]
  a6["_"]  b6["_"] c6["_"] d6["_"] e6["_"] f6["_"] g6["_"] h6["_"]
  a5["_"]  b5["_"] c5["_"] d5["_"] e5["_"] f5["_"] g5["_"] h5["_"]
  a4["_"]  b4["_"] c4["_"] d4["_"] e4["_"] f4["_"] g4["_"] h4["_"]
  a3["_"]  b3["_"] c3["_"] d3["_"] e3["_"] f3["_"] g3["_"] h3["_"]
  a2["_"]  b2["_"] c2["_"] d2["_"] e2["_"] f2["_"] g2["_"] h2["_"]
  a1["_"]  b1["_"] c1["_"] d1["_"] e1["_"] f1["_"] g1["_"] h1["_"]
```

When you minus 1 bitboard, it removes the smallest bit and sets all bits below it.
We then just AND this with the bitboard to remove the smallest bit.

```text
block-beta
  columns 1
  action["tempBitboard - 1"]:8

  block
    columns 8
    a8["1"]  b8["1"] c8["1"] d8["1"] e8["1"] f8["1"] g8["1"] h8["1"]
    a7["_"]  b7["_"] c7["_"] d7["_"] e7["_"] f7["_"] g7["_"] h7["_"]
    a6["_"]  b6["_"] c6["_"] d6["_"] e6["_"] f6["_"] g6["_"] h6["_"]
    a5["_"]  b5["_"] c5["_"] d5["_"] e5["_"] f5["_"] g5["_"] h5["_"]
    a4["_"]  b4["_"] c4["_"] d4["_"] e4["_"] f4["_"] g4["_"] h4["_"]
    a3["_"]  b3["_"] c3["_"] d3["_"] e3["_"] f3["_"] g3["_"] h3["_"]
    a2["_"]  b2["_"] c2["_"] d2["_"] e2["_"] f2["_"] g2["_"] h2["_"]
    a1["_"]  b1["_"] c1["_"] d1["_"] e1["_"] f1["_"] g1["_"] h1["_"]
  end

  and["&"]

  block
    columns 8

    a8["1"]  b8["1"] c8["1"] d8["1"] e8["1"] f8["1"] g8["1"] h8["1"]
    a7["_"]  b7["1"] c7["1"] d7["1"] e7["1"] f7["1"] g7["1"] h7["1"]
    a6["_"]  b6["_"] c6["_"] d6["_"] e6["_"] f6["_"] g6["_"] h6["_"]
    a5["_"]  b5["_"] c5["_"] d5["_"] e5["_"] f5["_"] g5["_"] h5["_"]
    a4["_"]  b4["_"] c4["_"] d4["_"] e4["_"] f4["_"] g4["_"] h4["_"]
    a3["_"]  b3["_"] c3["_"] d3["_"] e3["_"] f3["_"] g3["_"] h3["_"]
    a2["_"]  b2["_"] c2["_"] d2["_"] e2["_"] f2["_"] g2["_"] h2["_"]
    a1["_"]  b1["_"] c1["_"] d1["_"] e1["_"] f1["_"] g1["_"] h1["_"]
  end

  eq["="]

  block
    columns 8

    a8["_"]  b8["_"] c8["_"] d8["_"] e8["_"] f8["_"] g8["_"] h8["_"]
    a7["_"]  b7["1"] c7["1"] d7["1"] e7["1"] f7["1"] g7["1"] h7["1"]
    a6["_"]  b6["_"] c6["_"] d6["_"] e6["_"] f6["_"] g6["_"] h6["_"]
    a5["_"]  b5["_"] c5["_"] d5["_"] e5["_"] f5["_"] g5["_"] h5["_"]
    a4["_"]  b4["_"] c4["_"] d4["_"] e4["_"] f4["_"] g4["_"] h4["_"]
    a3["_"]  b3["_"] c3["_"] d3["_"] e3["_"] f3["_"] g3["_"] h3["_"]
    a2["_"]  b2["_"] c2["_"] d2["_"] e2["_"] f2["_"] g2["_"] h2["_"]
    a1["_"]  b1["_"] c1["_"] d1["_"] e1["_"] f1["_"] g1["_"] h1["_"]
  end
```

We removed the first piece.
