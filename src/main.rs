pub struct Puzzle {
  cells: utils::Cells,
}

fn main() {
  // let mut puzzle = Puzzle {
  //   cells: [
  //     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 9, 0, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 4,
  //     0, 3, 0, 8, 0, 0, 7, 0, 0, 8, 0, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  //     3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 2, 0, 7,
  //   ],
  // };

  let puzzle = Puzzle {
    cells: generator::generate(),
  };

  utils::print_grid(&puzzle.cells);

  let solutions = fast_backtracking_solver::solve(puzzle.cells, true, true);

  println!("# of Solutions: {}", solutions.len());

  utils::print_grid(&solutions[0]);
}

mod generator {
  use rand::seq::SliceRandom;
  use rand::Rng;
  use std::collections;

  use crate::fast_backtracking_solver;
  use crate::utils;

  pub fn generate() -> utils::Cells {
    let mut grid = generate_valid_grid();
    utils::print_grid(&grid);

    remove_cells(&mut grid, Some(35));

    grid
  }

  fn generate_valid_grid() -> utils::Cells {
    let mut rng = rand::thread_rng();

    let mut grid = [0; 81];
    let mut indices: collections::HashSet<usize> = (0..81).collect();

    while indices.len() > 0 {
      // NOTE - Pick a random cell.

      let index = indices
        .iter()
        .nth(rng.gen_range(0, indices.len()))
        .unwrap()
        .clone();

      // NOTE - Pick a random candidate value.

      let candidates = utils::get_candidates(&grid, index);
      grid[index] = candidates.choose(&mut rng).unwrap().clone();

      // NOTE - Verify that we can still solve the grid.

      let solutions = fast_backtracking_solver::solve(grid, true, false);

      if solutions.len() == 0 {
        grid[index] = 0;
      } else {
        indices.remove(&index);
      }
    }

    grid
  }

  fn remove_cells(grid: &mut utils::Cells, desired_clue_threshold: Option<u8>) {
    let mut rng = rand::thread_rng();

    let mut indices: Vec<usize> = (0..81).collect();
    indices.shuffle(&mut rng);

    let mut counter = 0;

    for i in indices.into_iter() {
      let old_value = grid[i];

      grid[i] = 0;

      if fast_backtracking_solver::solve(*grid, false, true).len() > 1 {
        // NOTE - No longer have a unique solution, so need to revert.
        grid[i] = old_value
      } else {
        counter += 1;

        if desired_clue_threshold.is_some() && counter >= (81 - desired_clue_threshold.unwrap()) {
          break;
        }
      }
    }
  }
}

mod naive_backtracking_solver {
  use crate::utils;

  struct Step {
    index: usize,
    value: u8,
  }

  pub fn solve(
    mut grid: utils::Cells,
    check_solvable: bool,
    check_unique: bool,
  ) -> Vec<utils::Cells> {
    let mut solutions: Vec<utils::Cells> = Vec::new();

    let mut index = 0;
    let mut initial_value = 1;

    let mut steps: Vec<Step> = Vec::new();

    loop {
      if index == 81 {
        let mut solution: utils::Cells = [0; 81];
        solution.copy_from_slice(&grid);

        solutions.push(solution);

        // NOTE - If we found a solution, we have proved solvability, and can
        // stop looking.

        if check_solvable {
          break;
        }

        // NOTE - If we found multiple solutions, we have disproved uniqueness, and can
        // stop looking.

        if check_unique && solutions.len() > 1 {
          break;
        }

        // NOTE - Continue on!

        match steps.pop() {
          Some(step) => {
            grid[step.index] = 0;

            index = step.index;
            initial_value = step.value + 1;
          }

          None => break,
        }
      }

      // NOTE - Skip clues.

      if grid[index] != 0 {
        index += 1;
        initial_value = 1;

        continue;
      }

      // NOTE - Determine the next possible value, if any, for the current cell,
      // starting from the initial value. If no value is possible, then
      // backtrack!

      match generate_step(&grid, index, initial_value) {
        Some(step) => {
          grid[index] = step.value;

          index += 1;
          initial_value = 1;

          steps.push(step);
        }

        None => match steps.pop() {
          Some(step) => {
            grid[step.index] = 0;

            index = step.index;
            initial_value = step.value + 1;
          }

          None => break,
        },
      }
    }

    solutions
  }

  fn generate_step(grid: &utils::Cells, index: usize, initial_value: u8) -> Option<Step> {
    let mut value = initial_value;

    return loop {
      if value > 9 {
        break None;
      }

      if utils::validate_candidate(grid, index, value) {
        let step = Step { index, value };
        break Some(step);
      }

      value += 1;
    };
  }
}

mod fast_backtracking_solver {
  use crate::utils;

  struct Step {
    index: usize,
    candidates: Vec<u8>,
  }

  pub fn solve(
    mut grid: utils::Cells,
    check_solvable: bool,
    check_unique: bool,
  ) -> Vec<utils::Cells> {
    let mut solutions: Vec<utils::Cells> = Vec::new();

    let mut steps: Vec<Step> = Vec::new();

    loop {
      match generate_step(&grid) {
        Some(mut step) => match step.candidates.pop() {
          Some(candidate) => {
            // NOTE - Try next candidate for this step.

            grid[step.index] = candidate;
            steps.push(step);
          }

          None => {
            // NOTE - No candidates left to try for this step, so back we go!

            if !try_backtrack(&mut grid, &mut steps) {
              break;
            }
          }
        },

        None => {
          // NOTE - Unable to generate a new step, which means we found a
          // solution! Add it to the list and use the parameters to determine
          // if we can stop.

          let mut solution: utils::Cells = [0; 81];

          solution.copy_from_slice(&grid);
          solutions.push(solution);

          // NOTE - If we found a solution, we have proved solvability, and can
          // stop looking.

          if check_solvable {
            break;
          }
          // NOTE - If we found multiple solutions, we have disproved uniqueness, and can
          // stop looking.

          if check_unique && solutions.len() > 1 {
            break;
          }

          // NOTE - Continue on!

          if !try_backtrack(&mut grid, &mut steps) {
            break;
          }
        }
      }
    }

    solutions
  }

  fn generate_step(grid: &utils::Cells) -> Option<Step> {
    let first_empty_cell = grid.iter().position(|&x| x == 0);

    if first_empty_cell == None {
      return None;
    }

    let mut best_cell_index: usize = first_empty_cell.unwrap();
    let mut best_cell_candidates: Vec<u8> = utils::get_candidates(&grid, best_cell_index);

    for i in (best_cell_index + 1)..81 {
      if grid[i] != 0 {
        continue;
      }

      let candidates = utils::get_candidates(&grid, i);

      if candidates.len() < best_cell_candidates.len() {
        best_cell_index = i;
        best_cell_candidates = candidates;
      }
    }

    Some(Step {
      index: best_cell_index,
      candidates: best_cell_candidates,
    })
  }

  fn try_backtrack(grid: &mut utils::Cells, steps: &mut Vec<Step>) -> bool {
    loop {
      match steps.pop() {
        Some(mut step) => match step.candidates.pop() {
          Some(candidate) => {
            grid[step.index] = candidate;
            steps.push(step);

            break true;
          }

          None => grid[step.index] = 0,
        },

        None => break false,
      }
    }
  }
}

mod utils {
  pub type Cells = [u8; 81];

  pub fn print_grid(cells: &Cells) {
    let mut result: String = String::new();

    result.push_str("╔═══╤═══╤═══╦═══╤═══╤═══╦═══╤═══╤═══╗\n");

    for i in 0..9 {
      result.push_str("║");

      for j in 0..9 {
        let cell = cells[i * 9 + j];

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

    println!("{}", result)
  }

  pub fn get_candidates(grid: &Cells, index: usize) -> Vec<u8> {
    let mut candidates: Vec<u8> = Vec::new();

    for i in 1..10 {
      if validate_candidate(grid, index, i) {
        candidates.push(i);
      }
    }

    candidates
  }

  pub fn validate_candidate(grid: &Cells, index: usize, value: u8) -> bool {
    // NOTE - Check column.
    let column_index = index % 9;

    for i in 0..9 {
      if grid[9 * i + column_index] == value {
        return false;
      }
    }

    // NOTE - Check row.

    let row_index = index / 9;

    for j in 0..9 {
      if grid[9 * row_index + j] == value {
        return false;
      }
    }

    // NOTE - Check box.

    let band_index = row_index / 3;
    let stack_index = column_index / 3;

    for k in 0..9 {
      let index = 3 * (9 * band_index + stack_index) + (9 * (k / 3) + (k % 3));

      if grid[index] == value {
        return false;
      }
    }

    true
  }
}
