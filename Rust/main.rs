
use std::time::Instant;
mod constants;

const NO_SQUARE:usize = 64;

static mut PIECE_ARRAY: [u64; 12] = [0; 12];
static mut WHITE_TO_PLAY: bool = true;
static mut CASTLE_RIGHTS: [bool; 4] = [true, true, true, true];
static mut EP: usize = NO_SQUARE;

const MAGIC:u64 = 0x03f79d71b4cb0a89;

fn get_rook_attacks_fast(starting_square: usize, mut occupancy: u64) -> u64 {
    occupancy &= constants::ROOK_MASKS[starting_square];
    occupancy *= constants::ROOK_MAGIC_NUMBERS[starting_square];
    occupancy >>= 64 - constants::ROOK_REL_BITS[starting_square];
    let converted_occupancy: usize = occupancy as usize;
    return constants::ROOK_ATTACKS[starting_square][converted_occupancy];
}

fn get_bishop_attacks_fast(starting_square: usize, occupancy: u64) -> u64 {
    let mut mutable_occupancy = occupancy;

    mutable_occupancy &= constants::BISHOP_MASKS[starting_square];
    mutable_occupancy *= constants::BISHOP_MAGIC_NUMBERS[starting_square];
    mutable_occupancy >>= 64 - constants::BISHOP_REL_BITS[starting_square];
    return constants::BISHOP_ATTACKS[starting_square][mutable_occupancy as usize];
}

fn bitscan_forward_separate(bitboard: u64) -> usize {
    let bitboard_combined: u64 = bitboard ^ (bitboard - 1);
    let calculation: u128 = 0x03f79d71b4cb0a89 * bitboard_combined as u128;
    let calc_truncated: u64 = calculation as u64;
    let index: usize = (calc_truncated >> 58) as usize;
    return constants::DEBRUIJN64[index];
}

const PINNED_SQUARE_INDEX:usize = 0;
const PINNING_PIECE_INDEX:usize = 1;


const TAG_NONE: usize = 0;
const TAG_CAPTURE: usize = 1;
const TAG_WHITEEP: usize = 2;
const TAG_BLACKEP: usize = 3;
const TAG_WCASTLEKS: usize = 4;
const TAG_WCASTLEQS: usize = 5;
const TAG_BCASTLEKS: usize = 6;
const TAG_BCASTLEQS: usize = 7;
const TAG_B_KNIGHT_PROMOTION: usize = 8;
const TAG_B_BISHOP_PROMOTION: usize = 9;
const TAG_B_QUEEN_PROMOTION: usize = 10;
const TAG_B_ROOK_PROMOTION: usize = 11;
const TAG_W_KNIGHT_PROMOTION: usize = 12;
const TAG_W_BISHOP_PROMOTION: usize = 13;
const TAG_W_QUEEN_PROMOTION: usize = 14;
const TAG_W_ROOK_PROMOTION: usize = 15;
const TAG_B_CAPTURE_KNIGHT_PROMOTION: usize = 16;
const TAG_B_CAPTURE_BISHOP_PROMOTION: usize = 17;
const TAG_B_CAPTURE_QUEEN_PROMOTION: usize = 18;
const TAG_B_CAPTURE_ROOK_PROMOTION: usize = 19;
const TAG_W_CAPTURE_KNIGHT_PROMOTION: usize = 20;
const TAG_W_CAPTURE_BISHOP_PROMOTION: usize = 21;
const TAG_W_CAPTURE_QUEEN_PROMOTION: usize = 22;
const TAG_W_CAPTURE_ROOK_PROMOTION: usize = 23;
const TAG_DOUBLE_PAWN_WHITE: usize = 24;
const TAG_DOUBLE_PAWN_BLACK: usize = 25;

const MOVE_STARTING: usize = 0;
const MOVE_TARGET: usize = 1;
const MOVE_PIECE: usize = 2;
const MOVE_TAG: usize = 3;
const MAX_ULONG: u64 = 18446744073709551615;

const RANK_1_BITBOARD: u64 = 18374686479671623680;
const RANK_2_BITBOARD: u64 = 71776119061217280;
const RANK_3_BITBOARD: u64 = 280375465082880;
const RANK_4_BITBOARD: u64 = 1095216660480;
const RANK_5_BITBOARD: u64 = 4278190080;
const RANK_6_BITBOARD: u64 = 16711680;
const RANK_7_BITBOARD: u64 = 65280;
const RANK_8_BITBOARD: u64 = 255;

const WKS_CASTLE_RIGHTS: usize = 0;
const WQS_CASTLE_RIGHTS: usize = 1;
const BKS_CASTLE_RIGHTS: usize = 2;
const BQS_CASTLE_RIGHTS: usize = 3;

const WKS_EMPTY_BITBOARD: u64 = 6917529027641081856;
const WQS_EMPTY_BITBOARD: u64 = 1008806316530991104;
const BKS_EMPTY_BITBOARD: u64 = 96;
const BQS_EMPTY_BITBOARD: u64 = 14;

const EMPTY_BITBOARD: u64 = 0;

const A8: usize = 0;
const B8: usize = 1;
const C8: usize = 2;
const D8: usize = 3;
const E8: usize = 4;
const F8: usize = 5;
const G8: usize = 6;
const H8: usize = 7;
const A7: usize = 8;
const B7: usize = 9;
const C7: usize = 10;
const D7: usize = 11;
const E7: usize = 12;
const F7: usize = 13;
const G7: usize = 14;
const H7: usize = 15;
const A6: usize = 16;
const B6: usize = 17;
const C6: usize = 18;
const D6: usize = 19;
const E6: usize = 20;
const F6: usize = 21;
const G6: usize = 22;
const H6: usize = 23;
const A5: usize = 24;
const B5: usize = 25;
const C5: usize = 26;
const D5: usize = 27;
const E5: usize = 28;
const F5: usize = 29;
const G5: usize = 30;
const H5: usize = 31;
const A4: usize = 32;
const B4: usize = 33;
const C4: usize = 34;
const D4: usize = 35;
const E4: usize = 36;
const F4: usize = 37;
const G4: usize = 38;
const H4: usize = 39;
const A3: usize = 40;
const B3: usize = 41;
const C3: usize = 42;
const D3: usize = 43;
const E3: usize = 44;
const F3: usize = 45;
const G3: usize = 46;
const H3: usize = 47;
const A2: usize = 48;
const B2: usize = 49;
const C2: usize = 50;
const D2: usize = 51;
const E2: usize = 52;
const F2: usize = 53;
const G2: usize = 54;
const H2: usize = 55;
const A1: usize = 56;
const B1: usize = 57;
const C1: usize = 58;
const D1: usize = 59;
const E1: usize = 60;
const F1: usize = 61;
const G1: usize = 62;
const H1: usize = 63;


const SQ_CHAR_Y : [char;65] = [
    '8','8','8','8','8','8','8','8',
    '7','7','7','7','7','7','7','7',
    '6','6','6','6','6','6','6','6',
    '5','5','5','5','5','5','5','5',
    '4','4','4','4','4','4','4','4',
    '3','3','3','3','3','3','3','3',
    '2','2','2','2','2','2','2','2',
    '1','1','1','1','1','1','1','1','A'
];

const SQ_CHAR_X : [char;65] = [
    'a','b','c','d','e','f','g','h',
    'a','b','c','d','e','f','g','h',
    'a','b','c','d','e','f','g','h',
    'a','b','c','d','e','f','g','h',
    'a','b','c','d','e','f','g','h',
    'a','b','c','d','e','f','g','h',
    'a','b','c','d','e','f','g','h',
    'a','b','c','d','e','f','g','h','N'
];

const WP:usize = 0;
const WN:usize = 1;
const WB:usize = 2;
const WR:usize = 3;
const WQ:usize = 4;
const WK:usize = 5;
const BP:usize = 6;
const BN:usize = 7;
const BB:usize = 8;
const BR:usize = 9;
const BQ:usize = 10;
const BK:usize = 11;

fn Is_Square_Attacked_By_Black_Global(square:usize, occupancy: u64) -> bool {
unsafe{ 
    if ((PIECE_ARRAY[BP] & constants::WHITE_PAWN_ATTACKS[square]) != 0) {
        return true;
    }
    if ((PIECE_ARRAY[BN] & constants::KNIGHT_ATTACKS[square]) != 0) {
        return true;
    }
    if ((PIECE_ARRAY[BK] & constants::KING_ATTACKS[square]) != 0) {
        return true;
    }
    let bishopAttacks = get_bishop_attacks_fast(square, occupancy);
    if ((PIECE_ARRAY[BB] & bishopAttacks) != 0) {
        return true;
    }
    if ((PIECE_ARRAY[BQ] & bishopAttacks) != 0) {
        return true;
    }
    let rookAttacks = get_rook_attacks_fast(square, occupancy);
    if ((PIECE_ARRAY[BR] & rookAttacks) != 0) {
        return true;
    }
    if ((PIECE_ARRAY[BQ] & rookAttacks) != 0) {
        return true;
    }
    return false;
}
}

fn Is_Square_Attacked_By_White_Global(square:usize, occupancy:u64) -> bool {
    unsafe{
    if ((PIECE_ARRAY[WP] & constants::BLACK_PAWN_ATTACKS[square]) != 0) {
        return true;
    }
    if ((PIECE_ARRAY[WN] & constants::KNIGHT_ATTACKS[square]) != 0) {
        return true;
    }
    if ((PIECE_ARRAY[WK] & constants::KING_ATTACKS[square]) != 0) {
        return true;
    }
    let bishopAttacks = get_bishop_attacks_fast(square, occupancy);
    if ((PIECE_ARRAY[WB] & bishopAttacks) != 0) {
        return true;
    }
    if ((PIECE_ARRAY[WQ] & bishopAttacks) != 0) {
        return true;
    }
    let rookAttacks = get_rook_attacks_fast(square, occupancy);
    if ((PIECE_ARRAY[WR] & rookAttacks) != 0) {
        return true;
    }
    if ((PIECE_ARRAY[WQ] & rookAttacks) != 0) {
        return true;
    }
    return false;
}
}

fn OutOfBounds(input:usize) -> bool {
    if input > 63 {
        return true;
    }
    return false;
}

fn PrMoveNoNL(starting_square:usize, target_square:usize) {    //starting
    if (OutOfBounds(starting_square) == true) {
         print!("{}", starting_square);
    } else {
        print!("{}{}", SQ_CHAR_X[starting_square], SQ_CHAR_Y[starting_square]); 
    }
    if (OutOfBounds(target_square) == true) {
        print!("{}", target_square);
    } else {
        print!("{}{}", SQ_CHAR_X[target_square], SQ_CHAR_Y[target_square]); 
    }
}


fn set_starting_position() {
    unsafe {
        EP = 65;
        WHITE_TO_PLAY = true;
        CASTLE_RIGHTS[0] = true;
        CASTLE_RIGHTS[1] = true;
        CASTLE_RIGHTS[2] = true;
        CASTLE_RIGHTS[3] = true;

        PIECE_ARRAY[0] = 71776119061217280;
        PIECE_ARRAY[1] = 4755801206503243776;
        PIECE_ARRAY[2] = 2594073385365405696;
        PIECE_ARRAY[3] = 9295429630892703744;
        PIECE_ARRAY[4] = 576460752303423488;
        PIECE_ARRAY[5] = 1152921504606846976;
        PIECE_ARRAY[6] = 65280;
        PIECE_ARRAY[7] = 66;
        PIECE_ARRAY[8] = 36;
        PIECE_ARRAY[9] = 129;
        PIECE_ARRAY[10] = 8;
        PIECE_ARRAY[11] = 16;
    }
}

fn is_occupied(bitboard: u64, square: usize) -> bool {
    return (bitboard & constants::SQUARE_BBS[square]) != 0;
}

fn get_occupied_index(square: usize) -> usize {
    for i in 0..12 {
        unsafe {
            if is_occupied(PIECE_ARRAY[i], square) {
                return i;
            }
        }
    }
    return 12;
}

fn fill_board_array() -> [usize; 64] {
    let mut board_array: [usize; 64] = [0; 64];

    for i in 0..64 {
        board_array[i] = get_occupied_index(i);
    }
    return board_array;
}

fn print_board() {
    unsafe {
        const PIECE_NAMES: [u8; 13] = [
            b'P', b'N', b'B', b'R', b'Q', b'K', b'P', b'N', b'B', b'R', b'Q', b'K', b'_',
        ];
        const PIECE_COLOURS: [u8; 13] = [
            b'W', b'W', b'W', b'W', b'W', b'W', b'B', b'B', b'B', b'B', b'B', b'B', b'_',
        ];

        println!("Board:");
        let board_array = fill_board_array();

        for rank in 0..8 {
            print!("   ");

            for file in 0..8 {
                let square: usize = (rank * 8) + file;
                print!(
                    "{}{} ",
                    PIECE_COLOURS[board_array[square]] as char,
                    PIECE_NAMES[board_array[square]] as char
                );
            }

            println!();
        }
        println!();

        println!("White to play: {}", WHITE_TO_PLAY);

        println!(
            "Castle: {} {} {} {}",
            CASTLE_RIGHTS[0], CASTLE_RIGHTS[1], CASTLE_RIGHTS[2], CASTLE_RIGHTS[3]
        );
        println!("ep: {}\n", EP);
        println!();
        println!();
    }
}

static mut STARTING_SQUARES: [[usize; 50]; 6] = [[0; 50]; 6];
static mut TARGET_SQUARES: [[usize; 50]; 6] = [[0; 50]; 6];
static mut TAGS: [[usize; 50]; 6] = [[0; 50]; 6];
static mut PIECES: [[usize; 50]; 6] = [[0; 50]; 6];
static mut PIN_ARRAY_SQUARES: [usize; 8] = [NO_SQUARE; 8];
static mut PIN_ARRAY_PIECES: [usize; 8] = [NO_SQUARE; 8];

fn perft_inline(depth:i8, ply:usize) -> usize {
unsafe {
    //if (depth == 0)
    //{
    //    return 1;
    //}

    let mut move_count: usize = 0;

    //Move generating variables
    let white_occupancies:u64 = PIECE_ARRAY[0] | PIECE_ARRAY[1] | PIECE_ARRAY[2] | PIECE_ARRAY[3] | PIECE_ARRAY[4] | PIECE_ARRAY[5];
    let black_occupancies:u64 = PIECE_ARRAY[6] | PIECE_ARRAY[7] | PIECE_ARRAY[8] | PIECE_ARRAY[9] | PIECE_ARRAY[10] | PIECE_ARRAY[11];
    let combined_occupancies:u64 = white_occupancies | black_occupancies;
    let EMPTY_OCCUPANCIES:u64 = !combined_occupancies;
    let mut temp_bitboard:u64;
    let mut check_bitboard:u64 = 0;
    let mut temp_pin_bitboard:u64; 
    let mut temp_attack:u64; 
    let mut temp_empty:u64; 
    let mut temp_captures:u64;
    let mut starting_square:usize = NO_SQUARE;
    let mut target_square:usize = NO_SQUARE;

    let mut pin_number: usize = 0;

    if WHITE_TO_PLAY == true
    {
        let mut white_king_check_count: usize = 0;
        let white_king_position: usize = bitscan_forward_separate(PIECE_ARRAY[WK]);

        //pawns
        temp_bitboard = PIECE_ARRAY[BP] & constants::WHITE_PAWN_ATTACKS[white_king_position];
        if (temp_bitboard != 0) {
            let pawn_square: usize = bitscan_forward_separate(temp_bitboard);
            check_bitboard = constants::SQUARE_BBS[pawn_square];
          
            white_king_check_count+=1;
        }

        //knights
        temp_bitboard = PIECE_ARRAY[BN] & constants::KNIGHT_ATTACKS[white_king_position];
        if (temp_bitboard != 0) {
            let knight_square: usize = bitscan_forward_separate(temp_bitboard);

            check_bitboard = constants::SQUARE_BBS[knight_square];
           
            white_king_check_count+=1;
        }

        //bishops
        let bishop_attacks_checks: u64 =
        get_bishop_attacks_fast(white_king_position, black_occupancies);
        temp_bitboard = PIECE_ARRAY[BB] & bishop_attacks_checks;
        while temp_bitboard != 0 {
            let piece_square: usize = bitscan_forward_separate(temp_bitboard);
            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[white_king_position][piece_square] & white_occupancies;

            if temp_pin_bitboard == 0 {
                check_bitboard = constants::INBETWEEN_BITBOARDS[white_king_position][piece_square];
                white_king_check_count+=1;
            } else {
                let pinned_square: usize = bitscan_forward_separate(temp_pin_bitboard);
                temp_pin_bitboard &= temp_pin_bitboard - 1;

                if temp_pin_bitboard == 0 {
                    PIN_ARRAY_SQUARES[pin_number] = pinned_square;
                    PIN_ARRAY_PIECES[pin_number] = piece_square;
                    pin_number+=1;
                }
            }
            temp_bitboard &= temp_bitboard - 1;
        }

        //queen
        temp_bitboard = PIECE_ARRAY[BQ] & bishop_attacks_checks;
        while (temp_bitboard != 0) {
            let piece_square: usize = bitscan_forward_separate(temp_bitboard);

            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[white_king_position][piece_square] & white_occupancies;

            if temp_pin_bitboard == 0 {
                check_bitboard = constants::INBETWEEN_BITBOARDS[white_king_position][piece_square];
                white_king_check_count+=1;
            } else {
                let pinned_square: usize = bitscan_forward_separate(temp_pin_bitboard);
                temp_pin_bitboard &= temp_pin_bitboard - 1;

                if temp_pin_bitboard == 0 {
                    PIN_ARRAY_SQUARES[pin_number] = pinned_square;
                    PIN_ARRAY_PIECES[pin_number] = piece_square;
                    pin_number+=1;
                }
            }
            temp_bitboard &= temp_bitboard - 1;
        }

        //rook
        let rook_attacks: u64 = get_rook_attacks_fast(white_king_position, black_occupancies);
        temp_bitboard = PIECE_ARRAY[BR] & rook_attacks;
        while temp_bitboard != 0 {
            let piece_square: usize = bitscan_forward_separate(temp_bitboard);
            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[white_king_position][piece_square] & white_occupancies;

            if temp_pin_bitboard == 0 {
                check_bitboard = constants::INBETWEEN_BITBOARDS[white_king_position][piece_square];
                white_king_check_count+=1;
            } else {
                let pinned_square: usize = bitscan_forward_separate(temp_pin_bitboard);
                temp_pin_bitboard &= temp_pin_bitboard - 1;

                if temp_pin_bitboard == 0 {
                    PIN_ARRAY_SQUARES[pin_number] = pinned_square;
                    PIN_ARRAY_PIECES[pin_number] = piece_square;
                    pin_number+=1;
                }
            }
            temp_bitboard &= temp_bitboard - 1;
        }

        //queen
        temp_bitboard = PIECE_ARRAY[BQ] & rook_attacks;
        while temp_bitboard != 0 {
            let piece_square: usize = bitscan_forward_separate(temp_bitboard);
            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[white_king_position][piece_square] & white_occupancies;

            if temp_pin_bitboard == 0 {
                check_bitboard = constants::INBETWEEN_BITBOARDS[white_king_position][piece_square];
                white_king_check_count+=1;
            } else {
                let pinned_square: usize = bitscan_forward_separate(temp_pin_bitboard);
                temp_pin_bitboard &= temp_pin_bitboard - 1;

                if temp_pin_bitboard == 0 {
                    PIN_ARRAY_SQUARES[pin_number] = pinned_square;
                    PIN_ARRAY_PIECES[pin_number] = piece_square;
                    pin_number+=1;
                }
            }
            temp_bitboard &= temp_bitboard - 1;
        }

        if white_king_check_count > 1 {
            let occupancies_without_white_king: u64 = combined_occupancies & (!PIECE_ARRAY[WK]);
            temp_attack = constants::KING_ATTACKS[white_king_position];
            temp_empty = temp_attack & EMPTY_OCCUPANCIES;
            while (temp_empty != 0)
            {
                target_square = bitscan_forward_separate(temp_empty);
                temp_empty &= temp_empty - 1;

                if (PIECE_ARRAY[BP] & constants::WHITE_PAWN_ATTACKS[target_square]) != 0 {
                    continue;
                }
                if (PIECE_ARRAY[BN] & constants::KNIGHT_ATTACKS[target_square]) != 0 {
                    continue;
                }
                if (PIECE_ARRAY[BK] & constants::KING_ATTACKS[target_square]) != 0 {
                    continue;
                }
                let bishop_attacks: u64 =
                get_bishop_attacks_fast(target_square, occupancies_without_white_king);
                if (PIECE_ARRAY[BB] & bishop_attacks) != 0 {
                    continue;
                }
                if (PIECE_ARRAY[BQ] & bishop_attacks) != 0 {
                    continue;
                }
                let rook_attacks: u64 =
                get_rook_attacks_fast(target_square, occupancies_without_white_king);
                if (PIECE_ARRAY[BR] & rook_attacks) != 0 {
                    continue;
                }
                if (PIECE_ARRAY[BQ] & rook_attacks) != 0 {
                    continue;
                }

                STARTING_SQUARES[ply][move_count] = white_king_position;
                TARGET_SQUARES[ply][move_count] = target_square;
                TAGS[ply][move_count] = TAG_NONE;
                PIECES[ply][move_count] = WK;
                move_count+=1;
            }

            //captures
            temp_captures = temp_attack & black_occupancies;
            while (temp_captures != 0)
            {
                target_square = bitscan_forward_separate(temp_captures);
                temp_captures &= temp_captures - 1;

                if (PIECE_ARRAY[BP] & constants::WHITE_PAWN_ATTACKS[target_square]) != 0 {
                    continue;
                }
                if (PIECE_ARRAY[BN] & constants::KNIGHT_ATTACKS[target_square]) != 0 {
                    continue;
                }
                if (PIECE_ARRAY[BK] & constants::KING_ATTACKS[target_square]) != 0 {
                    continue;
                }
                let bishop_attacks: u64 =
                get_bishop_attacks_fast(target_square, occupancies_without_white_king);
                if (PIECE_ARRAY[BB] & bishop_attacks) != 0 {
                    continue;
                }
                if (PIECE_ARRAY[BQ] & bishop_attacks) != 0 {
                    continue;
                }
                let rook_attacks: u64 =
                get_rook_attacks_fast(target_square, occupancies_without_white_king);
                if (PIECE_ARRAY[BR] & rook_attacks) != 0 {
                    continue;
                }
                if (PIECE_ARRAY[BQ] & rook_attacks) != 0 {
                    continue;
                }

                STARTING_SQUARES[ply][move_count] = white_king_position;
                TARGET_SQUARES[ply][move_count] = target_square;
                TAGS[ply][move_count] = TAG_CAPTURE;
                PIECES[ply][move_count] = WK;
                move_count+=1;
            }
        } else {

            if white_king_check_count == 0 {
                check_bitboard = MAX_ULONG;
            }

            let occupancies_without_white_king: u64 = combined_occupancies & (!PIECE_ARRAY[WK]);
            temp_attack = constants::KING_ATTACKS[white_king_position];
            temp_empty = temp_attack & EMPTY_OCCUPANCIES;
            while temp_empty != 0 {
                target_square = bitscan_forward_separate(temp_empty);
                temp_empty &= temp_empty - 1;

                if (PIECE_ARRAY[BP] & constants::WHITE_PAWN_ATTACKS[target_square]) != 0 {
                    continue;
                }
                if (PIECE_ARRAY[BN] & constants::KNIGHT_ATTACKS[target_square]) != 0 {
                    continue;
                }
                if (PIECE_ARRAY[BK] & constants::KING_ATTACKS[target_square]) != 0
                {
                    continue;
                }
                let bishop_attacks: u64 =
                get_bishop_attacks_fast(target_square, occupancies_without_white_king);
                if (PIECE_ARRAY[BB] & bishop_attacks) != 0 {
                    continue;
                }
                if (PIECE_ARRAY[BQ] & bishop_attacks) != 0 {
                    continue;
                }
                let rook_attacks: u64 =
                get_rook_attacks_fast(target_square, occupancies_without_white_king);
                if (PIECE_ARRAY[BR] & rook_attacks) != 0 {
                    continue;
                }
                if (PIECE_ARRAY[BQ] & rook_attacks) != 0 {
                    continue;
                }

                STARTING_SQUARES[ply][move_count] = white_king_position;
                TARGET_SQUARES[ply][move_count] = target_square;
                TAGS[ply][move_count] = TAG_NONE;
                PIECES[ply][move_count] = WK;
                move_count+=1;
            }

            //captures
            temp_captures = temp_attack & black_occupancies;
            while temp_captures != 0 {
                target_square = bitscan_forward_separate(temp_captures);
                temp_captures &= temp_captures - 1;

                if ((PIECE_ARRAY[BP] & constants::WHITE_PAWN_ATTACKS[target_square]) != 0) {
                    continue;
                }
                if ((PIECE_ARRAY[BN] & constants::KNIGHT_ATTACKS[target_square]) != 0) {
                    continue;
                }
                if ((PIECE_ARRAY[BK] & constants::KING_ATTACKS[target_square]) != 0) {
                    continue;
                }
                let bishop_attacks: u64 =
                get_bishop_attacks_fast(target_square, occupancies_without_white_king);
                if ((PIECE_ARRAY[BB] & bishop_attacks) != 0) {
                    continue;
                } 
                if ((PIECE_ARRAY[BQ] & bishop_attacks) != 0) {
                    continue;
                }
                let rook_attacks: u64 =
                get_rook_attacks_fast(target_square, occupancies_without_white_king);
                if ((PIECE_ARRAY[BR] & rook_attacks) != 0) {
                    continue;
                }
                if ((PIECE_ARRAY[BQ] & rook_attacks) != 0) {
                    continue;
                }

                STARTING_SQUARES[ply][move_count] = white_king_position;
                TARGET_SQUARES[ply][move_count] = target_square;
                TAGS[ply][move_count] = TAG_CAPTURE;
                PIECES[ply][move_count] = WK;
                move_count+=1;
            }

            if (white_king_check_count == 0)
            {
                if (CASTLE_RIGHTS[WKS_CASTLE_RIGHTS] == true)
                {
                    if (white_king_position == E1) //king on e1
                    {
                        if ((WKS_EMPTY_BITBOARD & combined_occupancies) == 0) //f1 and g1 empty
                        {
                            if ((PIECE_ARRAY[WR] & constants::SQUARE_BBS[H1]) != 0) //rook on h1
                            {
                                if (Is_Square_Attacked_By_Black_Global(F1, combined_occupancies) == false)
                                {
                                    if (Is_Square_Attacked_By_Black_Global(G1, combined_occupancies) == false)
                                    {
                                        STARTING_SQUARES[ply][move_count] = E1;
                                        TARGET_SQUARES[ply][move_count] = G1;
                                        TAGS[ply][move_count] = TAG_WCASTLEKS;
                                        PIECES[ply][move_count] = WK;
                                        move_count+=1;
                                    }
                                }
                            }
                        }
                    }
                }
                if (CASTLE_RIGHTS[WQS_CASTLE_RIGHTS] == true)
                {
                    if (white_king_position == E1) //king on e1
                    {
                        if ((WQS_EMPTY_BITBOARD & combined_occupancies) == 0) //f1 and g1 empty
                        {
                            if ((PIECE_ARRAY[WR] & constants::SQUARE_BBS[A1]) != 0) //rook on h1
                            {
                                if (Is_Square_Attacked_By_Black_Global(C1, combined_occupancies) == false)
                                {
                                    if (Is_Square_Attacked_By_Black_Global(D1, combined_occupancies) == false)
                                    {
                                        STARTING_SQUARES[ply][move_count] = E1;
                                        TARGET_SQUARES[ply][move_count] = C1;
                                        TAGS[ply][move_count] = TAG_WCASTLEQS;
                                        PIECES[ply][move_count] = WK;
                                        move_count+=1;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            temp_bitboard = PIECE_ARRAY[WN];

            while (temp_bitboard != 0)
            {
                starting_square = bitscan_forward_separate(temp_bitboard);
                temp_bitboard &= temp_bitboard - 1; //removes the knight from that square to not infinitely loop

                temp_pin_bitboard = MAX_ULONG;
                if (pin_number != 0) {
                    for i in 0..pin_number {
                        if (PIN_ARRAY_SQUARES[i] == starting_square)
                        {
                            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[white_king_position][PIN_ARRAY_PIECES[i]];
                        }
                    }
                }

                temp_attack = ((constants::KNIGHT_ATTACKS[starting_square] & black_occupancies) & check_bitboard) & temp_pin_bitboard; //gets knight captures
                while (temp_attack != 0) {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    STARTING_SQUARES[ply][move_count] = starting_square;
                    TARGET_SQUARES[ply][move_count] = target_square;
                    TAGS[ply][move_count] = TAG_CAPTURE;
                    PIECES[ply][move_count] = WN;
                    move_count+=1;
                }

                temp_attack = ((constants::KNIGHT_ATTACKS[starting_square] & EMPTY_OCCUPANCIES) & check_bitboard) & temp_pin_bitboard;

                while temp_attack != 0 {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    STARTING_SQUARES[ply][move_count] = starting_square;
                    TARGET_SQUARES[ply][move_count] = target_square;
                    TAGS[ply][move_count] = TAG_NONE;
                    PIECES[ply][move_count] = WN;
                    move_count+=1;
                }
            }

            temp_bitboard = PIECE_ARRAY[WP];

            while (temp_bitboard != 0)
            {
                starting_square = bitscan_forward_separate(temp_bitboard);
                temp_bitboard &= temp_bitboard - 1;

                temp_pin_bitboard = MAX_ULONG;
                if (pin_number != 0) {
                    for i in 0..pin_number {
                        if (PIN_ARRAY_SQUARES[i] == starting_square) {
                            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[white_king_position][PIN_ARRAY_PIECES[i]];
                        }
                    }
                }

                if ((constants::SQUARE_BBS[starting_square - 8] & combined_occupancies) == 0) { //if up one square is empty 
                    if (((constants::SQUARE_BBS[starting_square - 8] & check_bitboard) & temp_pin_bitboard) != 0) {
                        if ((constants::SQUARE_BBS[starting_square] & RANK_7_BITBOARD) != 0) { //if promotion
                            STARTING_SQUARES[ply][move_count] = starting_square;
                            TARGET_SQUARES[ply][move_count] = starting_square - 8;
                            TAGS[ply][move_count] = TAG_W_QUEEN_PROMOTION;
                            PIECES[ply][move_count] = WP;
                            move_count+=1;

                            STARTING_SQUARES[ply][move_count] = starting_square;
                            TARGET_SQUARES[ply][move_count] = starting_square - 8;
                            TAGS[ply][move_count] = TAG_W_ROOK_PROMOTION;
                            PIECES[ply][move_count] = WP;
                            move_count+=1;

                            STARTING_SQUARES[ply][move_count] = starting_square;
                            TARGET_SQUARES[ply][move_count] = starting_square - 8;
                            TAGS[ply][move_count] = TAG_W_BISHOP_PROMOTION;
                            PIECES[ply][move_count] = WP;
                            move_count+=1;

                            STARTING_SQUARES[ply][move_count] = starting_square;
                            TARGET_SQUARES[ply][move_count] = starting_square - 8;
                            TAGS[ply][move_count] = TAG_W_KNIGHT_PROMOTION;
                            PIECES[ply][move_count] = WP;
                            move_count+=1;

                        } else {
                            STARTING_SQUARES[ply][move_count] = starting_square;
                            TARGET_SQUARES[ply][move_count] = starting_square - 8;
                            TAGS[ply][move_count] = TAG_NONE;
                            PIECES[ply][move_count] = WP;
                            move_count+=1;
                        }
                    }

                    if ((constants::SQUARE_BBS[starting_square] & RANK_2_BITBOARD) != 0) { //if on rank 2
                        if (((constants::SQUARE_BBS[starting_square - 16] & check_bitboard) & temp_pin_bitboard) != 0) { //if not pinned or 
                            if (((constants::SQUARE_BBS[starting_square - 16]) & combined_occupancies) == 0) { //if up two squares and one square are empty
                                STARTING_SQUARES[ply][move_count] = starting_square;
                                TARGET_SQUARES[ply][move_count] = starting_square - 16;
                                TAGS[ply][move_count] = TAG_DOUBLE_PAWN_WHITE;
                                PIECES[ply][move_count] = WP;
                                move_count+=1;
                            }
                        }
                    }
                }

                temp_attack = ((constants::WHITE_PAWN_ATTACKS[starting_square] & black_occupancies) & check_bitboard) & temp_pin_bitboard; //if black piece diagonal to pawn

                while (temp_attack != 0)
                {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    if ((constants::SQUARE_BBS[starting_square] & RANK_7_BITBOARD) != 0) //if promotion
                    {
                        STARTING_SQUARES[ply][move_count] = starting_square;
                        TARGET_SQUARES[ply][move_count] = target_square;
                        TAGS[ply][move_count] = TAG_W_CAPTURE_BISHOP_PROMOTION;
                        PIECES[ply][move_count] = WP;
                        move_count+=1;

                        STARTING_SQUARES[ply][move_count] = starting_square;
                        TARGET_SQUARES[ply][move_count] = target_square;
                        TAGS[ply][move_count] = TAG_W_CAPTURE_KNIGHT_PROMOTION;
                        PIECES[ply][move_count] = WP;
                        move_count+=1;

                        STARTING_SQUARES[ply][move_count] = starting_square;
                        TARGET_SQUARES[ply][move_count] = target_square;
                        TAGS[ply][move_count] = TAG_W_CAPTURE_QUEEN_PROMOTION;
                        PIECES[ply][move_count] = WP;
                        move_count+=1;

                        STARTING_SQUARES[ply][move_count] = starting_square;
                        TARGET_SQUARES[ply][move_count] = target_square;
                        TAGS[ply][move_count] = TAG_W_CAPTURE_ROOK_PROMOTION;
                        PIECES[ply][move_count] = WP;
                        move_count+=1;
                    }
                    else
                    {
                        STARTING_SQUARES[ply][move_count] = starting_square;
                        TARGET_SQUARES[ply][move_count] = target_square;
                        TAGS[ply][move_count] = TAG_CAPTURE;
                        PIECES[ply][move_count] = WP;
                        move_count+=1;
                    }
                }

                if ((constants::SQUARE_BBS[starting_square] & RANK_5_BITBOARD) != 0) //check rank for ep
                {
                    if (EP != NO_SQUARE)
                    {
                        if ((((constants::WHITE_PAWN_ATTACKS[starting_square] & constants::SQUARE_BBS[EP]) & check_bitboard) & temp_pin_bitboard) != 0)
                        {
                            if ((PIECE_ARRAY[WK] & RANK_5_BITBOARD) == 0) //if no king on rank 5
                            {
                                STARTING_SQUARES[ply][move_count] = starting_square;
                                TARGET_SQUARES[ply][move_count] = EP;
                                TAGS[ply][move_count] = TAG_WHITEEP;
                                PIECES[ply][move_count] = WP;
                                move_count+=1;
                            }
                            else if ((PIECE_ARRAY[BR] & RANK_5_BITBOARD) == 0 && (PIECE_ARRAY[BQ] & RANK_5_BITBOARD) == 0) // if no b rook or queen on rank 5
                            {
                                STARTING_SQUARES[ply][move_count] = starting_square;
                                TARGET_SQUARES[ply][move_count] = EP;
                                TAGS[ply][move_count] = TAG_WHITEEP;
                                PIECES[ply][move_count] = WP;
                                move_count+=1;
                            }
                            else //wk and br or bq on rank 5
                            {
                                let mut occupancy_without_ep_pawns: u64 = combined_occupancies
                                & (!constants::SQUARE_BBS[starting_square ]);
                            occupancy_without_ep_pawns &=
                                !constants::SQUARE_BBS[(EP + 8) ];

                            let rook_attacks_from_king: u64 = get_rook_attacks_fast(
                                white_king_position,
                                occupancy_without_ep_pawns,
                            );
                                if ((rook_attacks_from_king & PIECE_ARRAY[BR]) == 0)
                                {
                                    if ((rook_attacks_from_king & PIECE_ARRAY[BQ]) == 0)
                                    {
                                        STARTING_SQUARES[ply][move_count] = starting_square;
                                        TARGET_SQUARES[ply][move_count] = EP;
                                        TAGS[ply][move_count] = TAG_WHITEEP;
                                        PIECES[ply][move_count] = WP;
                                        move_count+=1;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            temp_bitboard = PIECE_ARRAY[WR];
            while (temp_bitboard != 0)
            {
                starting_square = bitscan_forward_separate(temp_bitboard);
                temp_bitboard &= temp_bitboard - 1;

                temp_pin_bitboard = MAX_ULONG;
                if (pin_number != 0) {
                    for i in 0..pin_number {
                    
                        if (PIN_ARRAY_SQUARES[i] == starting_square)
                        {
                            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[white_king_position][PIN_ARRAY_PIECES[i]];
                        }
                    }
                }

                let rook_attacks = get_rook_attacks_fast(starting_square, combined_occupancies);

                temp_attack = ((rook_attacks & black_occupancies) & check_bitboard) & temp_pin_bitboard;
                while (temp_attack != 0)
                {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    STARTING_SQUARES[ply][move_count] = starting_square;
                    TARGET_SQUARES[ply][move_count] = target_square;
                    TAGS[ply][move_count] = TAG_CAPTURE;
                    PIECES[ply][move_count] = WR;
                    move_count+=1;
                }

                temp_attack = ((rook_attacks & EMPTY_OCCUPANCIES) & check_bitboard) & temp_pin_bitboard;
                while (temp_attack != 0)
                {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    STARTING_SQUARES[ply][move_count] = starting_square;
                    TARGET_SQUARES[ply][move_count] = target_square;
                    TAGS[ply][move_count] = TAG_NONE;
                    PIECES[ply][move_count] = WR;
                    move_count+=1;
                }
            }

            temp_bitboard = PIECE_ARRAY[WB];
            while (temp_bitboard != 0)
            {
                starting_square = bitscan_forward_separate(temp_bitboard);
                temp_bitboard &= temp_bitboard - 1;

                temp_pin_bitboard = MAX_ULONG;
                if (pin_number != 0) {
                    for i in 0..pin_number {
                        if (PIN_ARRAY_SQUARES[i] == starting_square) {
                            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[white_king_position][PIN_ARRAY_PIECES[i]];
                        }
                    }
                }

                let bishop_attacks: u64 =
                get_bishop_attacks_fast(starting_square, combined_occupancies);

                temp_attack = ((bishop_attacks & black_occupancies) & check_bitboard) & temp_pin_bitboard;
                while (temp_attack != 0)
                {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    STARTING_SQUARES[ply][move_count] = starting_square;
                    TARGET_SQUARES[ply][move_count] = target_square;
                    TAGS[ply][move_count] = TAG_CAPTURE;
                    PIECES[ply][move_count] = WB;
                    move_count+=1;
                }

                temp_attack = ((bishop_attacks & EMPTY_OCCUPANCIES) & check_bitboard) & temp_pin_bitboard;
                while (temp_attack != 0)
                {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    STARTING_SQUARES[ply][move_count] = starting_square;
                    TARGET_SQUARES[ply][move_count] = target_square;
                    TAGS[ply][move_count] = TAG_NONE;
                    PIECES[ply][move_count] = WB;
                    move_count+=1;
                }
            }

            temp_bitboard = PIECE_ARRAY[WQ];
            while (temp_bitboard != 0) {
                starting_square = bitscan_forward_separate(temp_bitboard);
                temp_bitboard &= temp_bitboard - 1;

                temp_pin_bitboard = MAX_ULONG;
                if (pin_number != 0) {
                    for i in 0..pin_number {
                        if (PIN_ARRAY_SQUARES[i] == starting_square) {
                            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[white_king_position][PIN_ARRAY_PIECES[i]];
                        }
                    }
                }

                let mut queen_attacks =
                get_rook_attacks_fast(starting_square, combined_occupancies);
            queen_attacks |= get_bishop_attacks_fast(starting_square, combined_occupancies);

                temp_attack = ((queen_attacks & black_occupancies) & check_bitboard) & temp_pin_bitboard;

                while (temp_attack != 0)
                {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    STARTING_SQUARES[ply][move_count] = starting_square;
                    TARGET_SQUARES[ply][move_count] = target_square;
                    TAGS[ply][move_count] = TAG_CAPTURE;
                    PIECES[ply][move_count] = WQ;
                    move_count+=1;
                }

                temp_attack = ((queen_attacks & EMPTY_OCCUPANCIES) & check_bitboard) & temp_pin_bitboard;
                while (temp_attack != 0)
                {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    STARTING_SQUARES[ply][move_count] = starting_square;
                    TARGET_SQUARES[ply][move_count] = target_square;
                    TAGS[ply][move_count] = TAG_NONE;
                    PIECES[ply][move_count] = WQ;
                    move_count+=1;
                }
            }
        }
    } else { //black move
        let mut black_king_check_count: u8 = 0;
        let black_king_position: usize = bitscan_forward_separate(PIECE_ARRAY[BK]);
        //pawns
        temp_bitboard = PIECE_ARRAY[WP] & constants::BLACK_PAWN_ATTACKS[black_king_position];
        if (temp_bitboard != 0) {
            let pawn_square = bitscan_forward_separate(temp_bitboard);
                if (check_bitboard == 0) {
                    check_bitboard = constants::SQUARE_BBS[pawn_square];
                }
            
            black_king_check_count+=1;
        }

        //knights
        temp_bitboard = PIECE_ARRAY[WN] & constants::KNIGHT_ATTACKS[black_king_position];
        if (temp_bitboard != 0) {
            let knight_square: usize = bitscan_forward_separate(temp_bitboard);

            check_bitboard = constants::SQUARE_BBS[knight_square];
           
            black_king_check_count+=1;
        }

        //bishops
        let bishop_attacks_checks: u64 =
        get_bishop_attacks_fast(black_king_position, white_occupancies);
        temp_bitboard = PIECE_ARRAY[WB] & bishop_attacks_checks;
        while (temp_bitboard != 0) {
            let piece_square: usize = bitscan_forward_separate(temp_bitboard);
            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[black_king_position][piece_square] & black_occupancies;

            if (temp_pin_bitboard == 0) {
                if (check_bitboard == 0) {
                    check_bitboard = constants::INBETWEEN_BITBOARDS[black_king_position][piece_square];
                }
                black_king_check_count+=1;
            } else {
                let pinned_square: usize = bitscan_forward_separate(temp_pin_bitboard);
                temp_pin_bitboard &= temp_pin_bitboard - 1;

                if (temp_pin_bitboard == 0) {
                    PIN_ARRAY_SQUARES[pin_number] = pinned_square;
                    PIN_ARRAY_PIECES[pin_number] = piece_square;
                    pin_number+=1;
                }
            }
            temp_bitboard &= temp_bitboard - 1;
        }

        //queen
        temp_bitboard = PIECE_ARRAY[WQ] & bishop_attacks_checks;
        while (temp_bitboard != 0) {
            let piece_square: usize = bitscan_forward_separate(temp_bitboard);
            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[black_king_position][piece_square] & black_occupancies;

            if (temp_pin_bitboard == 0) {
                if (check_bitboard == 0) {
                    check_bitboard = constants::INBETWEEN_BITBOARDS[black_king_position][piece_square];
                }
                black_king_check_count+=1;
            } else {
                let pinned_square: usize = bitscan_forward_separate(temp_pin_bitboard);
                temp_pin_bitboard &= temp_pin_bitboard - 1;

                if (temp_pin_bitboard == 0) {
                    PIN_ARRAY_SQUARES[pin_number] = pinned_square;
                    PIN_ARRAY_PIECES[pin_number] = piece_square;
                    pin_number+=1;
                }
            }
            temp_bitboard &= temp_bitboard - 1;
        }

        //rook
        let rook_attacks = get_rook_attacks_fast(black_king_position, white_occupancies);
        temp_bitboard = PIECE_ARRAY[WR] & rook_attacks;
        while (temp_bitboard != 0) {
            let piece_square: usize = bitscan_forward_separate(temp_bitboard);
            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[black_king_position][piece_square] & black_occupancies;

            if (temp_pin_bitboard == 0) {
                if (check_bitboard == 0) {
                    check_bitboard = constants::INBETWEEN_BITBOARDS[black_king_position][piece_square];
                }
                black_king_check_count+=1;
            } else {
                let pinned_square: usize = bitscan_forward_separate(temp_pin_bitboard);
                temp_pin_bitboard &= temp_pin_bitboard - 1;

                if (temp_pin_bitboard == 0) {
                    PIN_ARRAY_SQUARES[pin_number] = pinned_square;
                    PIN_ARRAY_PIECES[pin_number] = piece_square;
                    pin_number+=1;
                }
            }
            temp_bitboard &= temp_bitboard - 1;
        }

        //queen
        temp_bitboard = PIECE_ARRAY[WQ] & rook_attacks;
        while (temp_bitboard != 0) {
            let piece_square: usize = bitscan_forward_separate(temp_bitboard);

            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[black_king_position][piece_square] & black_occupancies;

            if (temp_pin_bitboard == 0) {
                if (check_bitboard == 0) {
                    check_bitboard = constants::INBETWEEN_BITBOARDS[black_king_position][piece_square];
                }
                black_king_check_count+=1;
            } else {
                let pinned_square: usize = bitscan_forward_separate(temp_pin_bitboard);
                temp_pin_bitboard &= temp_pin_bitboard - 1;

                if (temp_pin_bitboard == 0) {
                    PIN_ARRAY_SQUARES[pin_number] = pinned_square;
                    PIN_ARRAY_PIECES[pin_number] = piece_square;
                    pin_number+=1;
                }
            }
            temp_bitboard &= temp_bitboard - 1;
        }


        if (black_king_check_count > 1) {
            let occupancy_without_black_king = combined_occupancies & (!PIECE_ARRAY[BK]);
            temp_attack =
                constants::KING_ATTACKS[black_king_position ] & white_occupancies;

            temp_attack = constants::KING_ATTACKS[black_king_position] & white_occupancies;

            while (temp_attack != 0) {
                target_square = bitscan_forward_separate(temp_attack);
                temp_attack &= temp_attack - 1;

                if ((PIECE_ARRAY[WP] & constants::BLACK_PAWN_ATTACKS[target_square]) != 0) {
                    continue;
                }
                if ((PIECE_ARRAY[WN] & constants::KNIGHT_ATTACKS[target_square]) != 0) {
                    continue;
                }
                if ((PIECE_ARRAY[WK] & constants::KING_ATTACKS[target_square]) != 0) {
                    continue;
                }
                let bishop_attacks =
                get_bishop_attacks_fast(target_square, occupancy_without_black_king);
                if ((PIECE_ARRAY[WB] & bishop_attacks) != 0) {
                    continue;
                }
                if ((PIECE_ARRAY[WQ] & bishop_attacks) != 0)
                {
                    continue;
                }
                let rook_attacks =
                get_rook_attacks_fast(target_square, occupancy_without_black_king);
                if ((PIECE_ARRAY[WR] & rook_attacks) != 0)
                {
                    continue;
                }
                if ((PIECE_ARRAY[WQ] & rook_attacks) != 0)
                {
                    continue;
                }

                STARTING_SQUARES[ply][move_count] = starting_square;
                TARGET_SQUARES[ply][move_count] = target_square;
                TAGS[ply][move_count] = TAG_CAPTURE;
                PIECES[ply][move_count] = BK;
                move_count+=1;
            }

            temp_attack = constants::KING_ATTACKS[black_king_position] & !combined_occupancies;

            while (temp_attack != 0) {
                target_square = bitscan_forward_separate(temp_attack);
                temp_attack &= temp_attack - 1;

                if ((PIECE_ARRAY[WP] & constants::WHITE_PAWN_ATTACKS[target_square]) != 0) {
                    continue;
                }
                if ((PIECE_ARRAY[WN] & constants::KNIGHT_ATTACKS[target_square]) != 0) {
                    continue;
                }
                if ((PIECE_ARRAY[WK] & constants::KING_ATTACKS[target_square]) != 0) {
                    continue;
                }
                let bishop_attacks =
                get_bishop_attacks_fast(target_square, occupancy_without_black_king);
                if ((PIECE_ARRAY[WB] & bishop_attacks) != 0) {
                    continue;
                }
                if ((PIECE_ARRAY[WQ] & bishop_attacks) != 0) {
                    continue;
                }
                let rook_attacks =
                get_rook_attacks_fast(target_square, occupancy_without_black_king);
                if ((PIECE_ARRAY[WR] & rook_attacks) != 0) {
                    continue;
                }
                if ((PIECE_ARRAY[WQ] & rook_attacks) != 0) {
                    continue;
                }

                STARTING_SQUARES[ply][move_count] = starting_square;
                TARGET_SQUARES[ply][move_count] = target_square;
                TAGS[ply][move_count] = TAG_NONE;
                PIECES[ply][move_count] = BK;
                move_count+=1;
            }
        }
        else
        {
            if (black_king_check_count == 0) {
                check_bitboard = MAX_ULONG;
            }

            temp_bitboard = PIECE_ARRAY[BP];

            while (temp_bitboard != 0)
            {
                starting_square = bitscan_forward_separate(temp_bitboard);
                temp_bitboard &= temp_bitboard - 1;

                temp_pin_bitboard = MAX_ULONG;
                if (pin_number != 0) {
                    for i in 0..pin_number {
                        if (PIN_ARRAY_SQUARES[i] == starting_square) {
                            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[black_king_position][PIN_ARRAY_PIECES[i]];
                        }
                    }
                }

                if ((constants::SQUARE_BBS[starting_square + 8] & combined_occupancies) == 0) //if up one square is empty
                {
                    if (((constants::SQUARE_BBS[starting_square + 8] & check_bitboard) & temp_pin_bitboard) != 0)
                    {
                        if ((constants::SQUARE_BBS[starting_square] & RANK_2_BITBOARD) != 0) //if promotion
                        {
                            STARTING_SQUARES[ply][move_count] = starting_square;
                            TARGET_SQUARES[ply][move_count] = starting_square + 8;
                            TAGS[ply][move_count] = TAG_B_BISHOP_PROMOTION;
                            PIECES[ply][move_count] = BP;
                            move_count+=1;

                            STARTING_SQUARES[ply][move_count] = starting_square;
                            TARGET_SQUARES[ply][move_count] = starting_square + 8;
                            TAGS[ply][move_count] = TAG_B_KNIGHT_PROMOTION;
                            PIECES[ply][move_count] = BP;
                            move_count+=1;

                            STARTING_SQUARES[ply][move_count] = starting_square;
                            TARGET_SQUARES[ply][move_count] = starting_square + 8;
                            TAGS[ply][move_count] = TAG_B_ROOK_PROMOTION;
                            PIECES[ply][move_count] = BP;
                            move_count+=1;

                            STARTING_SQUARES[ply][move_count] = starting_square;
                            TARGET_SQUARES[ply][move_count] = starting_square + 8;
                            TAGS[ply][move_count] = TAG_B_QUEEN_PROMOTION;
                            PIECES[ply][move_count] = BP;
                            move_count+=1;
                        }
                        else
                        {
                            STARTING_SQUARES[ply][move_count] = starting_square;
                            TARGET_SQUARES[ply][move_count] = starting_square + 8;
                            TAGS[ply][move_count] = TAG_NONE;
                            PIECES[ply][move_count] = BP;
                            move_count+=1;
                        }
                    }

                    if ((constants::SQUARE_BBS[starting_square] & RANK_7_BITBOARD) != 0) //if on rank 2
                    {
                        if (((constants::SQUARE_BBS[starting_square + 16] & check_bitboard) & temp_pin_bitboard) != 0)
                        {
                            if (((constants::SQUARE_BBS[starting_square + 16]) & combined_occupancies) == 0) //if up two squares and one square are empty
                            {
                                STARTING_SQUARES[ply][move_count] = starting_square;
                                TARGET_SQUARES[ply][move_count] = starting_square + 16;
                                TAGS[ply][move_count] = TAG_DOUBLE_PAWN_BLACK;
                                PIECES[ply][move_count] = BP;
                                move_count+=1;
                            }
                        }
                    }
                }

                temp_attack = ((constants::BLACK_PAWN_ATTACKS[starting_square] & white_occupancies) & check_bitboard) & temp_pin_bitboard; //if black piece diagonal to pawn

                while (temp_attack != 0)
                {
                    target_square = bitscan_forward_separate(temp_attack); //find the bit
                    temp_attack &= temp_attack - 1;

                    if ((constants::SQUARE_BBS[starting_square] & RANK_2_BITBOARD) != 0) //if promotion
                    {
                        STARTING_SQUARES[ply][move_count] = starting_square;
                        TARGET_SQUARES[ply][move_count] = target_square;
                        TAGS[ply][move_count] = TAG_B_CAPTURE_BISHOP_PROMOTION;
                        PIECES[ply][move_count] = BP;
                        move_count+=1;

                        STARTING_SQUARES[ply][move_count] = starting_square;
                        TARGET_SQUARES[ply][move_count] = target_square;
                        TAGS[ply][move_count] = TAG_B_CAPTURE_ROOK_PROMOTION;
                        PIECES[ply][move_count] = BP;
                        move_count+=1;

                        STARTING_SQUARES[ply][move_count] = starting_square;
                        TARGET_SQUARES[ply][move_count] = target_square;
                        TAGS[ply][move_count] = TAG_B_CAPTURE_QUEEN_PROMOTION;
                        PIECES[ply][move_count] = BP;
                        move_count+=1;

                        STARTING_SQUARES[ply][move_count] = starting_square;
                        TARGET_SQUARES[ply][move_count] = target_square;
                        TAGS[ply][move_count] = TAG_B_CAPTURE_KNIGHT_PROMOTION;
                        PIECES[ply][move_count] = BP;
                        move_count+=1;
                    }
                    else
                    {
                        STARTING_SQUARES[ply][move_count] = starting_square;
                        TARGET_SQUARES[ply][move_count] = target_square;
                        TAGS[ply][move_count] = TAG_CAPTURE;
                        PIECES[ply][move_count] = BP;
                        move_count+=1;
                    }
                }

                if ((constants::SQUARE_BBS[starting_square] & RANK_4_BITBOARD) != 0) //check rank for ep
                {
                    if (EP != NO_SQUARE)
                    {
                        if ((((constants::BLACK_PAWN_ATTACKS[starting_square] & constants::SQUARE_BBS[EP]) & check_bitboard) & temp_pin_bitboard) != 0)
                        {
                            if ((PIECE_ARRAY[BK] & RANK_4_BITBOARD) == 0) //if no king on rank 5
                            {
                                STARTING_SQUARES[ply][move_count] = starting_square;
                                TARGET_SQUARES[ply][move_count] = EP;
                                TAGS[ply][move_count] = TAG_BLACKEP;
                                PIECES[ply][move_count] = BP;
                                move_count+=1;
                            }
                            else if ((PIECE_ARRAY[WR] & RANK_4_BITBOARD) == 0 && (PIECE_ARRAY[WQ] & RANK_4_BITBOARD) == 0) // if no b rook or queen on rank 5
                            {
                                STARTING_SQUARES[ply][move_count] = starting_square;
                                TARGET_SQUARES[ply][move_count] = EP;
                                TAGS[ply][move_count] = TAG_BLACKEP;
                                PIECES[ply][move_count] = BP;
                                move_count+=1;
                            }
                            else //wk and br or bq on rank 5
                            {
                                let mut occupancy_without_ep_pawns: u64 = combined_occupancies
                                & !constants::SQUARE_BBS[starting_square ];
                            occupancy_without_ep_pawns &=
                                !constants::SQUARE_BBS[(EP - 8) ];

                            let rook_attacks_from_king = get_rook_attacks_fast(
                                black_king_position,
                                occupancy_without_ep_pawns,
                            );
                                if ((rook_attacks_from_king & PIECE_ARRAY[WR]) == 0)
                                {
                                    if ((rook_attacks_from_king & PIECE_ARRAY[WQ]) == 0)
                                    {
                                        STARTING_SQUARES[ply][move_count] = starting_square;
                                        TARGET_SQUARES[ply][move_count] = EP;
                                        TAGS[ply][move_count] = TAG_BLACKEP;
                                        PIECES[ply][move_count] = BP;
                                        move_count+=1;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            temp_bitboard = PIECE_ARRAY[BN];

            while (temp_bitboard != 0) {
                starting_square = bitscan_forward_separate(temp_bitboard); //looks for the startingSquare
                temp_bitboard &= temp_bitboard - 1; //removes the knight from that square to not infinitely loop

                temp_pin_bitboard = MAX_ULONG;
                if (pin_number != 0) {
                    for i in 0..pin_number { 
                        if (PIN_ARRAY_SQUARES[i] == starting_square) {
                            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[black_king_position][PIN_ARRAY_PIECES[i]];
                        }
                    }
                }

                temp_attack = ((constants::KNIGHT_ATTACKS[starting_square] & white_occupancies) & check_bitboard) & temp_pin_bitboard; //gets knight captures
                while (temp_attack != 0) {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    STARTING_SQUARES[ply][move_count] = starting_square;
                    TARGET_SQUARES[ply][move_count] = target_square;
                    TAGS[ply][move_count] = TAG_CAPTURE;
                    PIECES[ply][move_count] = BN;
                    move_count+=1;
                }

                temp_attack = ((constants::KNIGHT_ATTACKS[starting_square] & (!combined_occupancies)) & check_bitboard) & temp_pin_bitboard;

                while (temp_attack != 0) {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    STARTING_SQUARES[ply][move_count] = starting_square;
                    TARGET_SQUARES[ply][move_count] = target_square;
                    TAGS[ply][move_count] = TAG_NONE;
                    PIECES[ply][move_count] = BN;
                    move_count+=1;
                }
            }

            temp_bitboard = PIECE_ARRAY[BB];
            while (temp_bitboard != 0) {
                starting_square = bitscan_forward_separate(temp_bitboard);
                temp_bitboard &= temp_bitboard - 1;

                temp_pin_bitboard = MAX_ULONG;
                if (pin_number != 0) {
                    for i in 0..pin_number {
                        if (PIN_ARRAY_SQUARES[i] == starting_square) {
                            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[black_king_position][PIN_ARRAY_PIECES[i]];
                        }
                    }
                }

                let bishop_attacks =
                get_bishop_attacks_fast(starting_square, combined_occupancies);

                temp_attack = ((bishop_attacks & white_occupancies) & check_bitboard) & temp_pin_bitboard;
                while (temp_attack != 0) {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    STARTING_SQUARES[ply][move_count] = starting_square;
                    TARGET_SQUARES[ply][move_count] = target_square;
                    TAGS[ply][move_count] = TAG_CAPTURE;
                    PIECES[ply][move_count] = BB;
                    move_count+=1;
                }

                temp_attack = ((bishop_attacks & (!combined_occupancies)) & check_bitboard) & temp_pin_bitboard;
                while (temp_attack != 0) {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    STARTING_SQUARES[ply][move_count] = starting_square;
                    TARGET_SQUARES[ply][move_count] = target_square;
                    TAGS[ply][move_count] = TAG_NONE;
                    PIECES[ply][move_count] = BB;
                    move_count+=1;
                }
            }

            temp_bitboard = PIECE_ARRAY[BR];
            while (temp_bitboard != 0) {
                starting_square = bitscan_forward_separate(temp_bitboard);
                temp_bitboard &= temp_bitboard - 1;

                temp_pin_bitboard = MAX_ULONG;
                if (pin_number != 0) {
                    for i in 0..pin_number {
                        if (PIN_ARRAY_SQUARES[i] == starting_square)
                        {
                            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[black_king_position][PIN_ARRAY_PIECES[i]];
                        }
                    }
                }

                let rook_attacks = get_rook_attacks_fast(starting_square, combined_occupancies);

                temp_attack = ((rook_attacks & white_occupancies) & check_bitboard) & temp_pin_bitboard;
                while (temp_attack != 0) {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    STARTING_SQUARES[ply][move_count] = starting_square;
                    TARGET_SQUARES[ply][move_count] = target_square;
                    TAGS[ply][move_count] = TAG_CAPTURE;
                    PIECES[ply][move_count] = BR;
                    move_count+=1;
                }

                temp_attack = ((rook_attacks & (!combined_occupancies)) & check_bitboard) & temp_pin_bitboard;
                while (temp_attack != 0) {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    STARTING_SQUARES[ply][move_count] = starting_square;
                    TARGET_SQUARES[ply][move_count] = target_square;
                    TAGS[ply][move_count] = TAG_NONE;
                    PIECES[ply][move_count] = BR;
                    move_count+=1;
                }
            }

            temp_bitboard = PIECE_ARRAY[BQ];
            while (temp_bitboard != 0) {

                starting_square = bitscan_forward_separate(temp_bitboard);
                temp_bitboard &= temp_bitboard - 1;

                temp_pin_bitboard = MAX_ULONG;
                if (pin_number != 0) {

                    for i in 0..pin_number {
                    
                        if (PIN_ARRAY_SQUARES[i] == starting_square) {
                            temp_pin_bitboard = constants::INBETWEEN_BITBOARDS[black_king_position][PIN_ARRAY_PIECES[i]];
                        }
                    }
                }

                let mut queen_attacks =
                get_rook_attacks_fast(starting_square, combined_occupancies);
                queen_attacks |= get_bishop_attacks_fast(starting_square, combined_occupancies);

                temp_attack = ((queen_attacks & white_occupancies) & check_bitboard) & temp_pin_bitboard;

                while (temp_attack != 0) {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    STARTING_SQUARES[ply][move_count] = starting_square;
                    TARGET_SQUARES[ply][move_count] = target_square;
                    TAGS[ply][move_count] = TAG_CAPTURE;
                    PIECES[ply][move_count] = BQ;
                    move_count+=1;
                }

                temp_attack = ((queen_attacks & (!combined_occupancies)) & check_bitboard) & temp_pin_bitboard;
                while (temp_attack != 0) {
                    target_square = bitscan_forward_separate(temp_attack);
                    temp_attack &= temp_attack - 1;

                    STARTING_SQUARES[ply][move_count] = starting_square;
                    TARGET_SQUARES[ply][move_count] = target_square;
                    TAGS[ply][move_count] = TAG_NONE;
                    PIECES[ply][move_count] = BQ;
                    move_count+=1;
                }
            }

            temp_attack = constants::KING_ATTACKS[black_king_position] & white_occupancies; //gets knight captures
            while (temp_attack != 0)
            {
                target_square = bitscan_forward_separate(temp_attack);
                temp_attack &= temp_attack - 1;

                if ((PIECE_ARRAY[WP] & constants::BLACK_PAWN_ATTACKS[target_square]) != 0)
                {
                    continue;
                }
                if ((PIECE_ARRAY[WN] & constants::KNIGHT_ATTACKS[target_square]) != 0)
                {
                    continue;
                }
                if ((PIECE_ARRAY[WK] & constants::KING_ATTACKS[target_square]) != 0)
                {
                    continue;
                }
                let occupancy_without_black_king = combined_occupancies & (!PIECE_ARRAY[BK]);
                let bishop_attacks =
                    get_bishop_attacks_fast(target_square, occupancy_without_black_king);
                if ((PIECE_ARRAY[WB] & bishop_attacks) != 0)
                {
                    continue;
                }
                if ((PIECE_ARRAY[WQ] & bishop_attacks) != 0)
                {
                    continue;
                }
                let rook_attacks =
                get_rook_attacks_fast(target_square, occupancy_without_black_king);
                if ((PIECE_ARRAY[WR] & rook_attacks) != 0)
                {
                    continue;
                }
                if ((PIECE_ARRAY[WQ] & rook_attacks) != 0)
                {
                    continue;
                }

                STARTING_SQUARES[ply][move_count] = black_king_position;
                TARGET_SQUARES[ply][move_count] = target_square;
                TAGS[ply][move_count] = TAG_CAPTURE;
                PIECES[ply][move_count] = BK;
                move_count+=1;
            }

            temp_attack = constants::KING_ATTACKS[black_king_position] & EMPTY_OCCUPANCIES; //get knight moves to emtpy squares

            while (temp_attack != 0)
            {
                target_square = bitscan_forward_separate(temp_attack);
                temp_attack &= temp_attack - 1;

                if ((PIECE_ARRAY[WP] & constants::BLACK_PAWN_ATTACKS[target_square]) != 0)
                {
                    continue;
                }
                if ((PIECE_ARRAY[WN] & constants::KNIGHT_ATTACKS[target_square]) != 0)
                {
                    continue;
                }
                if ((PIECE_ARRAY[WK] & constants::KING_ATTACKS[target_square]) != 0)
                {
                    continue;
                }
                let occupancy_without_black_king = combined_occupancies & (!PIECE_ARRAY[BK]);
                let bishop_attacks =
                    get_bishop_attacks_fast(target_square, occupancy_without_black_king);
                if ((PIECE_ARRAY[WB] & bishop_attacks) != 0)
                {
                    continue;
                }
                if ((PIECE_ARRAY[WQ] & bishop_attacks) != 0)
                {
                    continue;
                }
                let rook_attacks =
                get_rook_attacks_fast(target_square, occupancy_without_black_king);
                if ((PIECE_ARRAY[WR] & rook_attacks) != 0)
                {
                    continue;
                }
                if ((PIECE_ARRAY[WQ] & rook_attacks) != 0)
                {
                    continue;
                }

                STARTING_SQUARES[ply][move_count] = black_king_position;
                TARGET_SQUARES[ply][move_count] = target_square;
                TAGS[ply][move_count] = TAG_NONE;
                PIECES[ply][move_count] = BK;
                move_count+=1;
            }
        }
        if (black_king_check_count == 0)
        {
            if (CASTLE_RIGHTS[BKS_CASTLE_RIGHTS] == true)
            {
                if (black_king_position == E8) //king on e1
                {
                    if ((BKS_EMPTY_BITBOARD & combined_occupancies) == 0) //f1 and g1 empty
                    {
                        if ((PIECE_ARRAY[BR] & constants::SQUARE_BBS[H8]) != 0) //rook on h1
                        {
                            if (Is_Square_Attacked_By_White_Global(F8, combined_occupancies) == false)
                            {
                                if (Is_Square_Attacked_By_White_Global(G8, combined_occupancies) == false)
                                {
                                    STARTING_SQUARES[ply][move_count] = E8;
                                    TARGET_SQUARES[ply][move_count] = G8;
                                    TAGS[ply][move_count] = TAG_BCASTLEKS;
                                    PIECES[ply][move_count] = BK;
                                    move_count+=1;
                                }
                            }
                        }
                    }
                }
            }
            if (CASTLE_RIGHTS[BQS_CASTLE_RIGHTS] == true)
            {
                if (black_king_position == E8) //king on e1
                {
                    if ((BQS_EMPTY_BITBOARD & combined_occupancies) == 0) //f1 and g1 empty
                    {
                        if ((PIECE_ARRAY[BR] & constants::SQUARE_BBS[A8]) != 0) //rook on h1
                        {
                            if (Is_Square_Attacked_By_White_Global(C8, combined_occupancies) == false)
                            {
                                if (Is_Square_Attacked_By_White_Global(D8, combined_occupancies) == false)
                                {
                                    STARTING_SQUARES[ply][move_count] = E8;
                                    TARGET_SQUARES[ply][move_count] = C8;
                                    TAGS[ply][move_count] = TAG_BCASTLEQS;
                                    PIECES[ply][move_count] = BK;
                                    move_count+=1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if (depth == 1)
    {
        return move_count;
    }

    let mut nodes: usize = 0;
    //let mut prior_nodes: u64;
    let copy_ep: usize = EP;
    let mut copy_castle: [bool; 4] = [true, true, true, true];
    copy_castle[0] = CASTLE_RIGHTS[0];
    copy_castle[1] = CASTLE_RIGHTS[1];
    copy_castle[2] = CASTLE_RIGHTS[2];
    copy_castle[3] = CASTLE_RIGHTS[3];

    for move_index in 0..move_count {

        let starting_square: usize = STARTING_SQUARES[ply][move_index];
        let target_square: usize = TARGET_SQUARES[ply][move_index];
        let piece: usize = PIECES[ply][move_index];
        let tag: usize = TAGS[ply][move_index];

        let mut capture_index: usize = 0;

        if (WHITE_TO_PLAY == true) {
            WHITE_TO_PLAY = false;
        } else {
            WHITE_TO_PLAY = true;
        }

        match tag {
            TAG_NONE => {
                //none
                PIECE_ARRAY[piece] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                EP = NO_SQUARE ;
            }
            TAG_CAPTURE => {
                //capture
                PIECE_ARRAY[piece] |= constants::SQUARE_BBS[target_square ];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                if piece <= WK {
                    for i in BP..BK + 1 {
                        if (PIECE_ARRAY[i] & constants::SQUARE_BBS[target_square ]) != 0
                        {
                            capture_index = i;
                            break;
                        }
                    }
                    PIECE_ARRAY[capture_index ] &=
                        !constants::SQUARE_BBS[target_square ]
                } else {
                    //is black

                    for i in WP..BP {
                        if (PIECE_ARRAY[i] & constants::SQUARE_BBS[target_square]) != 0 {
                            capture_index = i;
                            break;
                        }
                    }
                    PIECE_ARRAY[capture_index] &= !constants::SQUARE_BBS[target_square];
                }
                EP = NO_SQUARE ;
            }
            TAG_WHITEEP => {
                //white ep
                //move piece
                PIECE_ARRAY[WP] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[WP] &= !constants::SQUARE_BBS[starting_square];
                //remove
                PIECE_ARRAY[BP] &= !constants::SQUARE_BBS[target_square + 8];
                EP = NO_SQUARE ;
            }
            TAG_BLACKEP => {
                //black ep
                //move piece
                PIECE_ARRAY[BP] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[BP] &= !constants::SQUARE_BBS[starting_square];
                //remove white pawn square up
                PIECE_ARRAY[WP] &= !constants::SQUARE_BBS[target_square - 8];
                EP = NO_SQUARE ;
            }
            TAG_WCASTLEKS => {
                //WKS
                //white king
                PIECE_ARRAY[WK] |= constants::SQUARE_BBS[G1];
                PIECE_ARRAY[WK] &= !constants::SQUARE_BBS[E1];
                //white rook
                PIECE_ARRAY[WR] |= constants::SQUARE_BBS[F1];
                PIECE_ARRAY[WR] &= !constants::SQUARE_BBS[H1];
                //occupancies
                CASTLE_RIGHTS[WKS_CASTLE_RIGHTS] = false;
                CASTLE_RIGHTS[WQS_CASTLE_RIGHTS] = false;
                EP = NO_SQUARE ;
            }
            TAG_WCASTLEQS => {
                //WQS
                //white king
                PIECE_ARRAY[WK] |= constants::SQUARE_BBS[C1];
                PIECE_ARRAY[WK] &= !constants::SQUARE_BBS[E1];
                //white rook
                PIECE_ARRAY[WR] |= constants::SQUARE_BBS[D1];
                PIECE_ARRAY[WR] &= !constants::SQUARE_BBS[A1];

                CASTLE_RIGHTS[WKS_CASTLE_RIGHTS] = false;
                CASTLE_RIGHTS[WQS_CASTLE_RIGHTS] = false;
                EP = NO_SQUARE ;
            }
            TAG_BCASTLEKS => {
                //BKS
                //white king
                PIECE_ARRAY[BK] |= constants::SQUARE_BBS[G8];
                PIECE_ARRAY[BK] &= !constants::SQUARE_BBS[E8];
                //white rook
                PIECE_ARRAY[BR] |= constants::SQUARE_BBS[F8];
                PIECE_ARRAY[BR] &= !constants::SQUARE_BBS[H8];
                CASTLE_RIGHTS[BKS_CASTLE_RIGHTS] = false;
                CASTLE_RIGHTS[BQS_CASTLE_RIGHTS] = false;
                EP = NO_SQUARE ;
            }
            TAG_BCASTLEQS => {
                //BQS
                //white king
                PIECE_ARRAY[BK] |= constants::SQUARE_BBS[C8];
                PIECE_ARRAY[BK] &= !constants::SQUARE_BBS[E8];
                //white rook
                PIECE_ARRAY[BR] |= constants::SQUARE_BBS[D8];
                PIECE_ARRAY[BR] &= !constants::SQUARE_BBS[A8];
                CASTLE_RIGHTS[BKS_CASTLE_RIGHTS] = false;
                CASTLE_RIGHTS[BQS_CASTLE_RIGHTS] = false;
                EP = NO_SQUARE ;
            }
            TAG_B_KNIGHT_PROMOTION => {
                //BNPr
                PIECE_ARRAY[BN] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                EP = NO_SQUARE ;
            }
            TAG_B_BISHOP_PROMOTION => {
                //BBPr
                PIECE_ARRAY[BB] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                EP = NO_SQUARE ;
            }
            TAG_B_QUEEN_PROMOTION => {
                //BQPr
                PIECE_ARRAY[BQ] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                EP = NO_SQUARE ;
            }
            TAG_B_ROOK_PROMOTION => {
                //BRPr
                PIECE_ARRAY[BR] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                EP = NO_SQUARE ;
            }
            TAG_W_KNIGHT_PROMOTION => {
                //WNPr
                PIECE_ARRAY[WN] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                EP = NO_SQUARE ;
            }
            TAG_W_BISHOP_PROMOTION => {
                //WBPr
                PIECE_ARRAY[WB] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                EP = NO_SQUARE ;
            }
            TAG_W_QUEEN_PROMOTION => {
                //WQPr
                PIECE_ARRAY[WQ] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                EP = NO_SQUARE ;
            }
            TAG_W_ROOK_PROMOTION => {
                //WRPr
                PIECE_ARRAY[WR] |= constants::SQUARE_BBS[target_square ];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                EP = NO_SQUARE ;
            }
            TAG_B_CAPTURE_KNIGHT_PROMOTION => {
                //BNPrCAP
                PIECE_ARRAY[BN] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                EP = NO_SQUARE ;
                for i in WP..BP {
                    if (PIECE_ARRAY[i] & constants::SQUARE_BBS[target_square]) != 0 {
                        capture_index = i;
                        break;
                    }
                }
                PIECE_ARRAY[capture_index] &= !constants::SQUARE_BBS[target_square]
            }
            TAG_B_CAPTURE_BISHOP_PROMOTION => {
                //BBPrCAP
                PIECE_ARRAY[BB] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];

                EP = NO_SQUARE ;
                for i in WP..BP {
                    if (PIECE_ARRAY[i] & constants::SQUARE_BBS[target_square]) != 0 {
                        capture_index = i;
                        break;
                    }
                }
                PIECE_ARRAY[capture_index] &= !constants::SQUARE_BBS[target_square];
            }
            TAG_B_CAPTURE_QUEEN_PROMOTION => {
                //BQPrCAP
                PIECE_ARRAY[BQ] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                EP = NO_SQUARE ;
                for i in WP..BP {
                    if (PIECE_ARRAY[i] & constants::SQUARE_BBS[target_square]) != 0 {
                        capture_index = i;
                        break;
                    }
                }
                PIECE_ARRAY[capture_index] &= !constants::SQUARE_BBS[target_square];
            }
            TAG_B_CAPTURE_ROOK_PROMOTION => {
                //BRPrCAP
                PIECE_ARRAY[BR] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                EP = NO_SQUARE ;
                for i in WP..BP {
                    if (PIECE_ARRAY[i] & constants::SQUARE_BBS[target_square]) != 0 {
                        capture_index = i;
                        break;
                    }
                }
                PIECE_ARRAY[capture_index] &= !constants::SQUARE_BBS[target_square];
            }
            TAG_W_CAPTURE_KNIGHT_PROMOTION => {
                //WNPrCAP
                PIECE_ARRAY[WN] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                EP = NO_SQUARE ;
                for i in BP..BK + 1 {
                    if (PIECE_ARRAY[i] & constants::SQUARE_BBS[target_square]) != 0 {
                        capture_index = i;
                        break;
                    }
                }
                PIECE_ARRAY[capture_index] &= !constants::SQUARE_BBS[target_square]
            }
            TAG_W_CAPTURE_BISHOP_PROMOTION => {
                //WBPrCAP
                PIECE_ARRAY[WB] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                EP = NO_SQUARE ;
                for i in BP..BK + 1 {
                    if (PIECE_ARRAY[i] & constants::SQUARE_BBS[target_square]) != 0 {
                        capture_index = i;
                        break;
                    }
                }
                PIECE_ARRAY[capture_index] &= !constants::SQUARE_BBS[target_square]
            }
            TAG_W_CAPTURE_QUEEN_PROMOTION => {
                //WQPrCAP
                PIECE_ARRAY[WQ] |= constants::SQUARE_BBS[target_square ];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                EP = NO_SQUARE ;
                for i in BP..BK + 1 {
                    if (PIECE_ARRAY[i] & constants::SQUARE_BBS[target_square ]) != 0 {
                        capture_index = i;
                        break;
                    }
                }
                PIECE_ARRAY[capture_index] &= !constants::SQUARE_BBS[target_square];
            }
            TAG_W_CAPTURE_ROOK_PROMOTION => {
                //WRPrCAP
                PIECE_ARRAY[WR] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[starting_square];
                EP = NO_SQUARE ;
                for i in BP..BK + 1 {
                    if (PIECE_ARRAY[i] & constants::SQUARE_BBS[target_square]) != 0 {
                        capture_index = i;
                        break;
                    }
                }
                PIECE_ARRAY[capture_index] &= !constants::SQUARE_BBS[target_square];
            }
            TAG_DOUBLE_PAWN_WHITE => {
                //WDouble
                PIECE_ARRAY[WP] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[WP] &= !constants::SQUARE_BBS[starting_square];
                EP = (target_square + 8) ;
            }
            TAG_DOUBLE_PAWN_BLACK => {
                //BDouble
                PIECE_ARRAY[BP] |= constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[BP] &= !constants::SQUARE_BBS[starting_square];
                EP = (target_square - 8) ;
            }
            _ => {}
        }
        if (piece == WK)
        {
            CASTLE_RIGHTS[WKS_CASTLE_RIGHTS] = false;
            CASTLE_RIGHTS[WQS_CASTLE_RIGHTS] = false;
        }
        else if (piece == BK)
        {
            CASTLE_RIGHTS[BKS_CASTLE_RIGHTS] = false;
            CASTLE_RIGHTS[BQS_CASTLE_RIGHTS] = false;
        }
        else if (piece == WR)
        {
            if (CASTLE_RIGHTS[WKS_CASTLE_RIGHTS] == true)
            {
                if ((PIECE_ARRAY[WR] & constants::SQUARE_BBS[H1]) == 0)
                {
                    CASTLE_RIGHTS[WKS_CASTLE_RIGHTS] = false;
                }
            }
            if (CASTLE_RIGHTS[WQS_CASTLE_RIGHTS] == true)
            {
                if ((PIECE_ARRAY[WR] & constants::SQUARE_BBS[A1]) == 0)
                {
                    CASTLE_RIGHTS[WQS_CASTLE_RIGHTS] = false;
                }
            }
        }
        else if (piece == BR)
        {
            if (CASTLE_RIGHTS[BKS_CASTLE_RIGHTS] == true)
            {
                if ((PIECE_ARRAY[BR] & constants::SQUARE_BBS[H8]) == 0)
                {
                    CASTLE_RIGHTS[BKS_CASTLE_RIGHTS] = false;
                }
            }
            if (CASTLE_RIGHTS[BQS_CASTLE_RIGHTS] == true)
            {
                if ((PIECE_ARRAY[BR] & constants::SQUARE_BBS[A8]) == 0)
                {
                    CASTLE_RIGHTS[BQS_CASTLE_RIGHTS] = false;
                }
            }
        }

        nodes += perft_inline(depth - 1, ply + 1);

        WHITE_TO_PLAY = !WHITE_TO_PLAY;

        match tag {
            TAG_NONE => {
                //none
                PIECE_ARRAY[piece] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[target_square];
            }
            TAG_CAPTURE => {
                //capture
                PIECE_ARRAY[piece] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[piece] &= !constants::SQUARE_BBS[target_square];
                if piece <= WK {
                    PIECE_ARRAY[capture_index] |= constants::SQUARE_BBS[target_square];
                } else {
                    //is black

                    PIECE_ARRAY[capture_index] |= constants::SQUARE_BBS[target_square];
                }
            }
            TAG_WHITEEP => {
                //white ep
                PIECE_ARRAY[WP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[WP] &= !constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[BP] |= constants::SQUARE_BBS[target_square + 8];
            }
            TAG_BLACKEP => {
                //black ep
                PIECE_ARRAY[BP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[BP] &= !constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[WP] |= constants::SQUARE_BBS[target_square - 8];
            }
            TAG_WCASTLEKS => {
                //WKS
                //white king
                PIECE_ARRAY[WK] |= constants::SQUARE_BBS[E1];
                PIECE_ARRAY[WK] &= !constants::SQUARE_BBS[G1];
                //white rook
                PIECE_ARRAY[WR] |= constants::SQUARE_BBS[H1];
                PIECE_ARRAY[WR] &= !constants::SQUARE_BBS[F1];
            }
            TAG_WCASTLEQS => {
                //WQS
                //white king
                PIECE_ARRAY[WK] |= constants::SQUARE_BBS[E1];
                PIECE_ARRAY[WK] &= !constants::SQUARE_BBS[C1];
                //white rook
                PIECE_ARRAY[WR] |= constants::SQUARE_BBS[A1];
                PIECE_ARRAY[WR] &= !constants::SQUARE_BBS[D1];
            }
            TAG_BCASTLEKS => {
                //BKS
                //white king
                PIECE_ARRAY[BK] |= constants::SQUARE_BBS[E8];
                PIECE_ARRAY[BK] &= !constants::SQUARE_BBS[G8];
                //white rook
                PIECE_ARRAY[BR] |= constants::SQUARE_BBS[H8];
                PIECE_ARRAY[BR] &= !constants::SQUARE_BBS[F8];
            }
            TAG_BCASTLEQS => {
                //BQS
                //white king
                PIECE_ARRAY[BK] |= constants::SQUARE_BBS[E8];
                PIECE_ARRAY[BK] &= !constants::SQUARE_BBS[C8];
                //white rook
                PIECE_ARRAY[BR] |= constants::SQUARE_BBS[A8];
                PIECE_ARRAY[BR] &= !constants::SQUARE_BBS[D8];
            }
            TAG_B_KNIGHT_PROMOTION => {
                //BNPr
                PIECE_ARRAY[BP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[BN] &= !constants::SQUARE_BBS[target_square];
            }
            TAG_B_BISHOP_PROMOTION => {
                //BBPr
                PIECE_ARRAY[BP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[BB] &= !constants::SQUARE_BBS[target_square];
            }
            TAG_B_QUEEN_PROMOTION => {
                //BQPr
                PIECE_ARRAY[BP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[BQ] &= !constants::SQUARE_BBS[target_square];
            }
            TAG_B_ROOK_PROMOTION => {
                //BRPr
                PIECE_ARRAY[BP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[BR] &= !constants::SQUARE_BBS[target_square];
            }
            TAG_W_KNIGHT_PROMOTION => {
                //WNPr
                PIECE_ARRAY[WP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[WN] &= !constants::SQUARE_BBS[target_square];
            }
            TAG_W_BISHOP_PROMOTION => {
                //WBPr
                PIECE_ARRAY[WP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[WB] &= !constants::SQUARE_BBS[target_square];
            }
            TAG_W_QUEEN_PROMOTION => {
                //WQPr
                PIECE_ARRAY[WP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[WQ] &= !constants::SQUARE_BBS[target_square];
            }
            TAG_W_ROOK_PROMOTION => {
                //WRPr
                PIECE_ARRAY[WP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[WR] &= !constants::SQUARE_BBS[target_square];
            }
            TAG_B_CAPTURE_KNIGHT_PROMOTION => {
                //BNPrCAP
                PIECE_ARRAY[BP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[BN] &= !constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[capture_index] |= constants::SQUARE_BBS[target_square];
            }
            TAG_B_CAPTURE_BISHOP_PROMOTION => {
                //BBPrCAP
                PIECE_ARRAY[BP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[BB] &= !constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[capture_index] |= constants::SQUARE_BBS[target_square];
            }
            TAG_B_CAPTURE_QUEEN_PROMOTION => {
                //BQPrCAP
                PIECE_ARRAY[BP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[BQ] &= !constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[capture_index] |= constants::SQUARE_BBS[target_square];
            }
            TAG_B_CAPTURE_ROOK_PROMOTION => {
                //BRPrCAP
                PIECE_ARRAY[BP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[BR] &= !constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[capture_index] |= constants::SQUARE_BBS[target_square];
            }
            TAG_W_CAPTURE_KNIGHT_PROMOTION => {
                //WNPrCAP
                PIECE_ARRAY[WP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[WN] &= !constants::SQUARE_BBS[target_square ];
                PIECE_ARRAY[capture_index] |= constants::SQUARE_BBS[target_square ];
            }
            TAG_W_CAPTURE_BISHOP_PROMOTION => {
                //WBPrCAP
                PIECE_ARRAY[WP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[WB] &= !constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[capture_index] |= constants::SQUARE_BBS[target_square];
            }
            TAG_W_CAPTURE_QUEEN_PROMOTION => {
                //WQPrCAP
                PIECE_ARRAY[WP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[WQ] &= !constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[capture_index] |= constants::SQUARE_BBS[target_square];
            }
            TAG_W_CAPTURE_ROOK_PROMOTION => {
                //WRPrCAP
                PIECE_ARRAY[WP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[WR] &= !constants::SQUARE_BBS[target_square];
                PIECE_ARRAY[capture_index] |= constants::SQUARE_BBS[target_square];
            }
            TAG_DOUBLE_PAWN_WHITE => {
                //WDouble
                PIECE_ARRAY[WP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[WP] &= !constants::SQUARE_BBS[target_square];
            }
            TAG_DOUBLE_PAWN_BLACK => {
                //BDouble
                PIECE_ARRAY[BP] |= constants::SQUARE_BBS[starting_square];
                PIECE_ARRAY[BP] &= !constants::SQUARE_BBS[target_square];
            }
            _ => {}
        }

        CASTLE_RIGHTS[0] = copy_castle[0];
        CASTLE_RIGHTS[1] = copy_castle[1];
        CASTLE_RIGHTS[2] = copy_castle[2];
        CASTLE_RIGHTS[3] = copy_castle[3];
        EP = copy_ep;

        //if (epGlobal != NO_SQUARE)
        //{
        //    std::cout << "   ep: " << SQ_CHAR_X[epGlobal] << SQ_CHAR_Y[epGlobal] << '\n';
        //}

        //if (ply == 0)
        //{
           //PrMoveNoNL(startingSquare, targetSquare);
           //print!(": {}\n", .{nodes - priorNodes});
        //}
    }

    return nodes
}
}

fn run_perft(depth:i8) {

    let timestamp_start = Instant::now();

    let nodes = perft_inline(depth, 0);

    let elapsed = timestamp_start.elapsed();

    println!("Nodes: {}", nodes);
    println!("Elapsed time: {:?}", elapsed);
}

fn main() {

    set_starting_position();
    print_board();
    run_perft(6);
}
