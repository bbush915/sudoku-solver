use std::fmt;

struct Grid {
    cells: [u8; 81],
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = String::new();

        result.push_str("╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗\n");

        for i in 0..9 {
            result.push_str("║");

            for j in 0..9 {
                let cell = self.cells[i * 9 + j];

                result.push_str(&format!(
                    " {} ",
                    match cell {
                        0 => String::from(" "),
                        _ => cell.to_string(),
                    }
                ));

                if j < 8 {
                    if (j + 1) % 3 == 0 {
                        result.push_str("║");
                    } else {
                        result.push_str("│");
                    }
                }
            }

            result.push_str("║\n");

            if i < 8 {
                if (i + 1) % 3 == 0 {
                    result.push_str("╠═══╪═══╪═══╬═══╪═══╪═══╬═══╪═══╪═══╣\n");
                } else {
                    result.push_str("╟───┼───┼───╫───┼───┼───╫───┼───┼───╢\n");
                }
            }
        }

        result.push_str("╚═══╧═══╧═══╩═══╧═══╧═══╩═══╧═══╧═══╝");

        write!(f, "{}", result)
    }
}

struct Candidate {
    index: usize,
    value: u8,
}

fn main() {
    let mut puzzle = Grid {
        cells: [
            5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0,
            0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0,
            0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
        ],
    };

    println!("{}", puzzle);

    solve(&mut puzzle);

    println!("{}", puzzle);
}

fn solve(grid: &mut Grid) {
    let mut index = 0;
    let mut initial_value = 1;

    let mut candidates = Vec::new();

    while index < 81 {
        if grid.cells[index] != 0 {
            index += 1;
            initial_value = 1;

            continue;
        }

        match generate_candidate(&grid, index, initial_value) {
            Some(candidate) => {
                grid.cells[index] = candidate.value;

                index += 1;
                initial_value = 1;

                candidates.push(candidate);
            }

            None => match candidates.pop() {
                Some(candidate) => {
                    grid.cells[candidate.index] = 0;

                    index = candidate.index;
                    initial_value = candidate.value + 1;
                }

                None => panic!("Puzzle has no solution."),
            },
        }
    }
}

fn generate_candidate(grid: &Grid, index: usize, initial_value: u8) -> Option<Candidate> {
    let mut value = initial_value;

    return loop {
        if value > 9 {
            break None;
        }

        if validate_candidate(grid, index, value) {
            let candidate = Candidate { index, value };
            break Some(candidate);
        }

        value += 1;
    };
}

fn validate_candidate(grid: &Grid, index: usize, value: u8) -> bool {
    let column_index = index % 9;

    for i in 0..9 {
        if grid.cells[9 * i + column_index] == value {
            return false;
        }
    }

    let row_index = index / 9;

    for j in 0..9 {
        if grid.cells[9 * row_index + j] == value {
            return false;
        }
    }

    let band_index = row_index / 3;
    let stack_index = column_index / 3;

    for k in 0..9 {
        let index = 3 * (9 * band_index + stack_index) + (9 * (k / 3) + (k % 3));

        if grid.cells[index] == value {
            return false;
        }
    }

    true
}
