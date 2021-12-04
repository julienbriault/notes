use regex::Regex;
use std::fs;

fn main() {
    // part 1
    println!("Hello, world!");
    let data = read_input("input.txt");
    //println!("{:?}", data);
    //let test_data = test_input();
    //println!("test_data:\n{:?}", test_data);
    let mut bingo_game = parse_data(&data);
    bingo_game.show_game();
    bingo_game.play_game();
    println!("test_data:\n{:?}", bingo_game.input);
}

pub struct BingoGame {
    input: Vec<i32>,
    vec_of_bingo_cards: Vec<BingoCard>,
}

impl BingoGame {
    pub fn new(input: Vec<i32>, vec_of_bingo_cards: Vec<BingoCard>) -> BingoGame {
        Self {
            input: input,
            vec_of_bingo_cards: vec_of_bingo_cards,
        }
    }

    fn show_game(&self) {
        println!("input of the game: {:?}", self.input);
        println!("number of bingocards: {:?}", self.vec_of_bingo_cards.len());
        for card in &self.vec_of_bingo_cards {
            println!("{:?}", card);
        }
    }

    fn play_game(&mut self) -> i32 {
        for round in &self.input {
            println!("playing round with the following value: {}", round);
            for card in &mut self.vec_of_bingo_cards {
                card.play_round(*round);
                println!("\tupdated card {}", round);
            }
            for card in &mut self.vec_of_bingo_cards {
                if card.bingo() == true {
                    println!("{:?}", card);
                    let mut sum: i32 = 0;
                    for vec in card.vec_of_vec.iter() {
                        for number in vec {
                            if *number != -1 {
                                sum += number
                            }
                        }
                    }
                    println!("RESULT {}", sum * round);
                    return sum * round;
                }
            }
        }
        0
    }
}

#[derive(Debug)]
pub struct BingoCard {
    vec_of_vec: Vec<Vec<i32>>,
}
impl BingoCard {
    pub fn new(vec: Vec<i32>) -> BingoCard {
        //println!("{}", vec.len());
        let mut vec_of_vec: Vec<Vec<i32>> = vec![Vec::new(); 5];
        for (idx, row) in vec.chunks(5).enumerate() {
            //println!("row- {:?} -idx{}", row, idx);
            let mut vec_to_add: Vec<i32> = vec![];
            for item in row {
                vec_to_add.push(*item);
            }
            //println!("vec_to_add- {:?}", vec_to_add);
            vec_of_vec[idx].append(&mut vec_to_add);
        }
        //println!("{:?}", vec_of_vec);
        Self {
            vec_of_vec: vec_of_vec,
        }
    }
    // verify if there is any card that has a bingo:
    pub fn bingo(&self) -> bool {
        let mut result: bool = false;
        let bingo: Vec<i32> = vec![-1, -1, -1, -1, -1];
        // check if the rows produce bingo
        for row in &self.vec_of_vec {
            if *row == bingo {
                result = true;
            }
        }

        // check if the columns produce bingo
        let count = self.vec_of_vec.len() - 1;
        for idx in 0..count {
            let column: Vec<i32> = self
                .vec_of_vec
                .iter() // iterate over rows
                .map(|x| x[idx]) // get the icolumn-th element from each row
                .collect(); // collect into new vector
            if column == bingo {
                result = true
            }
            //println!("{}-nt column: {:?}", count, column);
        }

        result
    }

    pub fn play_round(&mut self, number: i32) {
        for row in self.vec_of_vec.iter_mut() {
            for value in row.iter_mut() {
                if *value == number {
                    *value = -1;
                }
            }
        }
    }
}

fn parse_data(data: &str) -> BingoGame {
    let re_identify_line = Regex::new(r"[0-9]").unwrap();
    //let re_card_line = Regex::new(r"(\d{2}).(\d{2}).(\d{2}).(\d{2}).(\d{2})").unwrap();
    // create input required for bingocard:
    let mut bingo_card_input: Vec<i32> = Vec::new();
    let mut result: Vec<i32> = Vec::new();
    for line in data.lines() {
        //println!("{}", line);
        if line.contains(",") {
            for item in line.split(",") {
                println!("{}", item);
                result.push(item.parse::<i32>().unwrap());
            }
            //let result_addition: Vec<char> = line.chars().filter(|x| *x != ',').collect();
            //for item in result_addition {
            //    result.push(item.to_digit(10).unwrap() as i32);
            // }
        } else if re_identify_line.is_match(line) {
            //println!("line: {}", line);
            let parts = line.split_whitespace().map(|s| s.parse::<i32>());
            for c in parts {
                bingo_card_input.push(c.unwrap());
            }
        }
    }
    //println!("{:?}", bingo_card_input);
    // create bingocards:
    //let count = bingo_card_input.len() / 5 / 5;
    //println!("count {}", count);
    // fill bingocards:
    let mut vec_of_bingo_cards: Vec<BingoCard> = Vec::new();
    for card_input in bingo_card_input.chunks(25) {
        let mut input_vec = vec![];
        for item in card_input {
            input_vec.push(*item);
        }
        //println!("{:?}", input_vec);
        let new_card = BingoCard::new(input_vec);
        vec_of_bingo_cards.push(new_card);
    }
    let bingo_game = BingoGame::new(result, vec_of_bingo_cards);
    bingo_game
}

fn read_input(file: &str) -> String {
    let data = fs::read_to_string(file).expect("Unable to read file");
    data
}

fn test_input() -> String {
    let data = String::from(
        "
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7",
    );
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bingo_card() {
        let card = BingoCard::new(vec![
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ]);
        assert_eq!(card.vec_of_vec.len(), 5);
    }
    #[test]
    fn bingo_win_1() {
        let card_1 = BingoCard::new(vec![
            -1, -1, -1, -1, -1, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12,
            3, 7,
        ]);

        assert_eq!(card_1.bingo(), true);
    }

    #[test]
    fn bingo_win_2() {
        let card_2 = BingoCard::new(vec![
            -1, -1, -1, -1, 10, -1, 16, 15, 9, 19, -1, 8, 23, 26, 20, -1, 11, 13, 6, 5, -1, 0, 12,
            3, 7,
        ]);

        assert_eq!(card_2.bingo(), true);
    }
    #[test]
    fn bingo_los_1() {
        let card_3 = BingoCard::new(vec![
            1, 1, 1, 1, 11, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7,
        ]);

        assert_eq!(card_3.bingo(), false);
    }

    #[test]
    fn play_rounds() {
        let mut card_1 = BingoCard::new(vec![
            43, 1, 1, 1, 11, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7,
        ]);

        card_1.play_round(43);
        card_1.play_round(16);
        assert_eq!(card_1.vec_of_vec[0][0], -1);
        assert_eq!(card_1.vec_of_vec[1][1], -1);
    }

    #[test]
    fn play_rounds_and_bingo_1() {
        let mut card_1 = BingoCard::new(vec![
            1, 2, 3, 4, 5, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7,
        ]);
        assert_eq!(card_1.bingo(), false);
        card_1.play_round(1);
        card_1.play_round(2);
        card_1.play_round(3);
        card_1.play_round(4);
        card_1.play_round(5);
        assert_eq!(card_1.bingo(), true);
    }

    #[test]
    fn test_game_creation() {
        let test_data = test_input();
        let mut bingo_game = parse_data(&test_data);
        println!("test_data:\n{:?}", test_data);
        println!("{:?}", bingo_game.input);
        assert_eq!(bingo_game.input[9], 14)
    }

    #[test]
    fn test_game_creation_and_bingo() {
        let test_data = test_input();
        let mut bingo_game = parse_data(&test_data);
        bingo_game.play_game()
    }
}
