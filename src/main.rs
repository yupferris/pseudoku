use std::fmt;

const N: usize = 9;
const NUM_ENTRIES: usize = N * N;

#[derive(Clone)]
struct Board {
    cells: [Option<u8>; NUM_ENTRIES],
}

impl Board {
    fn is_solved(&self) -> bool {
        self.is_full() && self.satisfies_constraints()
    }

    // TODO: Better name?
    fn is_full(&self) -> bool {
        self.cells.iter().all(|x| x.is_some())
    }

    fn satisfies_constraints(&self) -> bool {
        self.satisfies_row_constraints() && self.satisfies_column_constraints() && self.satisfies_box_constraints()
    }

    fn satisfies_row_constraints(&self) -> bool {
        for y in 0..N {
            let mut digit_map = [false; 9];
            for x in 0..N {
                let cell_x = x;
                let cell_y = y;
                let cell_index = cell_y * N + cell_x;
                if let Some(digit) = self.cells[cell_index] {
                    let digit_index = (digit - 1) as usize;
                    if digit_map[digit_index] {
                        return false;
                    }
                    digit_map[digit_index] = true;
                }
            }
        }
        return true;
    }

    fn satisfies_column_constraints(&self) -> bool {
        for x in 0..N {
            let mut digit_map = [false; 9];
            for y in 0..N {
                let cell_x = x;
                let cell_y = y;
                let cell_index = cell_y * N + cell_x;
                if let Some(digit) = self.cells[cell_index] {
                    let digit_index = (digit - 1) as usize;
                    if digit_map[digit_index] {
                        return false;
                    }
                    digit_map[digit_index] = true;
                }
            }
        }
        return true;
    }

    fn satisfies_box_constraints(&self) -> bool {
        let box_dim = 3;
        for box_y in 0..box_dim {
            for box_x in 0..box_dim {
                let mut digit_map = [false; 9];
                for y in 0..box_dim {
                    for x in 0..box_dim {
                        let cell_x = box_x * box_dim + x;
                        let cell_y = box_y * box_dim + y;
                        let cell_index = cell_y * N + cell_x;
                        if let Some(digit) = self.cells[cell_index] {
                            let digit_index = (digit - 1) as usize;
                            if digit_map[digit_index] {
                                return false;
                            }
                            digit_map[digit_index] = true;
                        }
                    }
                }
            }
        }

        return true;
    }

    fn solve(&self) -> (Option<Board>, u32) {
        fn go(board: Board, partial_solutions_checked: &mut u32) -> Option<Board> {
            *partial_solutions_checked += 1;

            //println!("Checking board: {:?}", board);

            if !board.satisfies_constraints() {
                return None;
            }

            if board.is_full() {
                return Some(board);
            }
    
            if let Some(free_index) = board.cells.iter().position(|x| x.is_none()) {
                for candidate_digit in 1..=9 {
                    let mut candidate_board = board.clone();
                    candidate_board.cells[free_index] = Some(candidate_digit);
                    let candidate_solution = go(candidate_board, partial_solutions_checked);
                    if candidate_solution.is_some() {
                        return candidate_solution;
                    }
                }
            }
    
            return None;
        }
        let mut partial_solutions_checked = 0;
        (go(self.clone(), &mut partial_solutions_checked), partial_solutions_checked)
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f)?;
        for y in 0..N {
            if (y % 3) == 0 {
                writeln!(f, "+---+---+---+")?;
            }
            for x in 0..N {
                if (x % 3) == 0 {
                    write!(f, "|")?;
                }
                let cell_x = x;
                let cell_y = y;
                let cell_index = cell_y * N + cell_x;
                if let Some(digit) = self.cells[cell_index] {
                    write!(f, "{}", digit)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "|")?;
        }
        write!(f, "+---+---+---+")?;

        Ok(())
    }
}

fn main() {
    let board = Board {
        cells: [
            None, None, Some(5), /* | */ Some(3), None, None, /* | */ None, None, None,
            Some(8), None, None, /* | */ None, None, None, /* | */ None, Some(2), None,
            None, Some(7), None, /* | */ None, Some(1), None, /* | */ Some(5), None, None,
            // --------------------------------------------------------------- //
            Some(4), None, None, /* | */ None, None, Some(5), /* | */ Some(3), None, None,
            None, Some(1), None, /* | */ None, Some(7), None, /* | */ None, None, Some(6),
            None, None, Some(3), /* | */ Some(2), None, None, /* | */ None, Some(8), None,
            // --------------------------------------------------------------- //
            None, Some(6), None, /* | */ Some(5), None, None, /* | */ None, None, Some(9),
            None, None, Some(4), /* | */ None, None, None, /* | */ None, Some(3), None,
            None, None, None, /* | */ None, None, Some(9), /* | */ Some(7), None, None,
            /*None, None, None, /* | */ None, None, None, /* | */ None, None, None,
            None, None, None, /* | */ None, None, None, /* | */ None, None, None,
            None, None, None, /* | */ None, None, None, /* | */ None, None, None,
            // --------------------------------------------------------------- //
            None, None, None, /* | */ None, None, None, /* | */ None, None, None,
            None, None, None, /* | */ None, None, None, /* | */ None, None, None,
            None, None, None, /* | */ None, None, None, /* | */ None, None, None,
            // --------------------------------------------------------------- //
            None, None, None, /* | */ None, None, None, /* | */ None, None, None,
            None, None, None, /* | */ None, None, None, /* | */ None, None, None,
            None, None, None, /* | */ None, None, None, /* | */ None, None, None,*/
        ],
    };
    println!("Before solving: {:?}", board);

    let (solved, partial_solutions_checked) = board.solve();
    if let Some(solved) = solved {
        assert!(solved.is_solved());
        println!("Solved after checking {} partial solutions: {:?}", partial_solutions_checked, solved);
    } else {
        println!("Board is determined as not solvable after checking {} partial solutions", partial_solutions_checked);
    }
}
