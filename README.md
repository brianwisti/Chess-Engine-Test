# Chess Engine Move Generator Comparison

This code is originally from "Coding with Tom" videos comparing implementations of a chess bitboard algorithm.

- [I Coded a Chess Engine in 7 Languages to test Performance!][cwt-1]
- [12 Programming Languages Tested! - Chess move generator test with updated results][cwt-2]

Cited source for the bitboard chess engine approach:

- [Bitboard CHESS ENGINE in C: intro][bitboard]

[cwt-1]: https://www.youtube.com/watch?v=cFNBIYwht8o
[cwt-2]: https://www.youtube.com/watch?v=m4c38NS43cE
[bitboard]: https://www.youtube.com/watch?v=QUNP-UjujBM

The constants are massive and almost everything is written in one function to maximize performance.

## Initial results

| Language | Reported Time |
| -------- | ------------- |
| C        | 339.4ms       |
| C#       | 683.2ms       |
| C++      | 331.2ms       |
| D        | 438ms         |
| Go       | 627.4ms       |
| Java     | 1988ms        |
| Nim      | 429ms         |
| Odin     | 398ms         |
| Python   | 51139ms       |
| Rust     | 463.03ms      |
| Swift    | 585ms         |
| Zig      | 335.8ms       |

Feel free to make improvements to any of the code. Some notes:

- test the opening chess position to depth 6. Target: 119,060,324 nodes
- The max moves in a chess position are 220. I made the *move list* 250 just for safety. The max moves
  reached from any chess position from the start is 46. So you can set the move_list to 47 elements
  without an index error but this will make the algorithm break in any other position.

Another approach is to make the *move list* global and use an index like this:

c# example:

```csharp
        static int[,] StartingSquares = new int[6, 50];
        static int[,] TargetSquares = new int[6, 50];
        static int[,] Tags = new int[6, 50];
        static int[,] Pieces = new int[6, 50];
        static int[,] move_counts = new int[6];
```

Function example:

```csharp
        static int Perft(int depth, int ply)
        {

            int move_count = GetMoves(ply);

            if (depth <= 1) {
                return move_count;
            }

            int nodes = 0;
            for (int i = 0; i < move_count; i++)
            {
                int startingSquare = StartingSquares[ply, i];
                //etc

                //make move
                nodes += Perft(depth - 1, ply + 1);
                //unmake move
            }

            return nodes;
        }
```

I might test all code examples with 50 size move_list and global move_lists later.

The original code was in CPP where I made everything in one big function and was testing loads of versions.
For that reason the code is very messy and not clean at all. I made a "clean code" version that I put in the C# folder.
This is where the code is extracted into functions and made clearer. However this approach is slower, as function calls have a cost.

I normally don't code this way. I extract functions and hate over nesting. I also rarely use else and prefer early returns.
I only did it here to maximize performance.

Another reason why I didn't extract functions and refactor is because you create the most amount of bugs imaginable with a project like this.
One small flaw in logic and you can break the entire thing:

- Did you change sides correctly?
- Should you use the `WHITE_OCCUPANCIES` or `BLACK_OCCUPANCIES` when searching for pins?
- Did you use `WHITE_PAWN_ATTACKS` to look for a black pawn check and BLACK_PAWN_ATTACKS to look for a
  white pawn check?
- Does en passant work?
- Does castling work correctly? Does the king go through check?
- Do pins work correctly? Was the king captured?

In the C# clean version folder I also added my debug perft.
This is what you can use to debug any changed you make to make sure you don't have bugs.
With so much debugging you can find almost any bug you create while refactoring.

