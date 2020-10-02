use magpie::othello_board::OthelloBoard;
use magpie::stone::Stone;

fn main() {
    let board = OthelloBoard::new();
    println!("{}", display(&board, Some(Stone::Black)));
}

fn display(board: &OthelloBoard, stone: Option<Stone>) -> String {
    let legal_moves = stone.map(|stone| board.legal_moves_for(stone)).unwrap_or(0);
    let char_at = |rank: usize, file: usize| {
        let pos = coord_to_bitboard(rank, file);
        board
            .stone_at(pos)
            .map(|stone| match stone {
                Stone::White => "X",
                Stone::Black => "O",
            })
            .or_else(|| Some(if legal_moves & pos > 0 { "*" } else { " " }))
            .unwrap_or(" ")
    };

    let horizontal = format!("  +{}", "---+".repeat(8));
    let top_row = "    A   B   C   D   E   F   G   H\n";

    let display = (0..8)
        .map(|rank| {
            format!(
                "{}{} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                format!("{}\n", horizontal),
                rank + 1,
                char_at(rank, 0),
                char_at(rank, 1),
                char_at(rank, 2),
                char_at(rank, 3),
                char_at(rank, 4),
                char_at(rank, 5),
                char_at(rank, 6),
                char_at(rank, 7)
            )
        })
        .fold("".to_owned(), |acc, val| acc + &val);

    let display = &[top_row, &display, &horizontal].concat();
    display.to_string()
}

fn coord_to_bitboard(rank: usize, file: usize) -> u64 {
    RANKS[rank] & FILES[file]
}

const FILE_A: u64 = 0x80_80_80_80_80_80_80_80;
const FILE_B: u64 = 0x40_40_40_40_40_40_40_40;
const FILE_C: u64 = 0x20_20_20_20_20_20_20_20;
const FILE_D: u64 = 0x10_10_10_10_10_10_10_10;
const FILE_E: u64 = 0x08_08_08_08_08_08_08_08;
const FILE_F: u64 = 0x04_04_04_04_04_04_04_04;
const FILE_G: u64 = 0x02_02_02_02_02_02_02_02;
const FILE_H: u64 = 0x01_01_01_01_01_01_01_01;
const FILES: [u64; 8] = [
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H,
];

const RANK_1: u64 = 0xff_00_00_00_00_00_00_00;
const RANK_2: u64 = 0x00_ff_00_00_00_00_00_00;
const RANK_3: u64 = 0x00_00_ff_00_00_00_00_00;
const RANK_4: u64 = 0x00_00_00_ff_00_00_00_00;
const RANK_5: u64 = 0x00_00_00_00_ff_00_00_00;
const RANK_6: u64 = 0x00_00_00_00_00_ff_00_00;
const RANK_7: u64 = 0x00_00_00_00_00_00_ff_00;
const RANK_8: u64 = 0x00_00_00_00_00_00_00_ff;
const RANKS: [u64; 8] = [
    RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
];
