use std::fs;
use std::env;
use simple_matrix::Matrix;

fn input_string() -> String {
  let args: Vec<String> = env::args().collect();
  let filename = match args.get(1) {
    Some(a) if a == "test" => "test_data.txt",
    _ => "data.txt"
  };
  fs::read_to_string(filename).unwrap()
}

#[derive(Clone)]
struct Cell {
    number: u32,
    marked: bool,
}

fn parse_input(input: String) -> (Vec<u32>, Vec<(Matrix<Cell>, bool)>)
{
    let mut lines = input.lines();
    let calls = lines.next().unwrap().split(',').map(|n| n.parse::<u32>().unwrap());

    let board_lines: Vec<&str> = lines.collect();
    let boards: Vec<(Matrix<Cell>, bool)> = board_lines.chunks(6).map(|six_lines| {
        let rows: Vec<Vec<u32>> = six_lines[1..].iter()
                                                .map(|line| line.split_whitespace()
                                                                .map(|n| n.parse::<u32>().unwrap())
                                                                .collect()).collect();
        (Matrix::from_iter(5, 5, rows.iter().flatten().map(|n| Cell{
            number: *n,
            marked: false
        })), false)
    }).collect();

    (calls.collect(), boards)
}

fn part1(calls: &Vec<u32>, boards: &Vec<(Matrix<Cell>, bool)>)
{
    let (winning_board, last_call) = play_bingo(calls, boards, false).unwrap();
    let mut sum_unmarked = 0;
    winning_board.apply(|cell| if !cell.marked { sum_unmarked += cell.number });
    println!("Score = {}", sum_unmarked * last_call);
}
fn part2(calls: &Vec<u32>, boards: &Vec<(Matrix<Cell>, bool)>)
{
    let (losing_board, last_call) = play_bingo(calls, boards, true).unwrap();
    let mut sum_unmarked = 0;
    losing_board.apply(|cell| if !cell.marked { sum_unmarked += cell.number });
    println!("Score = {}", sum_unmarked * last_call);
}

fn play_bingo(calls: &Vec<u32>, boards: &Vec<(Matrix<Cell>, bool)>, to_lose: bool) -> Option<(Matrix<Cell>, u32)> {
    let mut boards = boards.clone();
    for i in 0..calls.len() {
        let call = calls[i];
        let len = boards.len();
        for board in boards.iter_mut() {
            board.0.apply_mut(|cell| if cell.number == call {cell.marked = true});

            if i > 5 && board_wins(&board.0) {
                if to_lose && len > 1 {
                    board.1 = true;
                }
                else {
                    return Some((board.0.clone(), call))
                }
            }
        }
        boards.retain(|b| !b.1);
    }
    None
}

fn board_wins(board: &Matrix<Cell>) -> bool {
    (0..board.rows()).any(|rown| board.get_row(rown).unwrap().all(|cell| cell.marked)) ||
        (0..board.cols()).any(|coln| board.get_col(coln).unwrap().all(|cell| cell.marked))
}

fn main() {
    let input_str = input_string();
    let (calls, boards) = parse_input(input_str);

    part1(&calls, &boards);
    part2(&calls, &boards);
}
