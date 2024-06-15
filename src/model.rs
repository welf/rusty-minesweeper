use rand::Rng;
use std::{
    collections::HashSet,
    fmt::{Display, Formatter, Write},
};

const CELL: char = '🟨';
const FLAG: &str = "🇷🇺";
const MINE: char = '💣';
const EXPLOSION: char = '💥';

pub type Position = (u16, u16);

#[derive(Debug, PartialEq)]
enum OpeningResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
pub struct Minesweeper {
    width: u16,
    height: u16,
    pub open_positions: HashSet<Position>,
    pub mines: HashSet<Position>,
    pub flagged_positions: HashSet<Position>,
    pub game_over: bool,
}

impl Minesweeper {
    pub fn new(width: u16, height: u16, mines_count: u16) -> Self {
        // Check if the parameters are valid
        assert!(
            width > 0 && height > 0 && mines_count > 0 && mines_count < width * height,
            "Invalid parameters"
        );

        // Convert mines_count to usize to convert it to usize only once
        let mines_count = mines_count as usize;

        Self {
            width,
            height,
            open_positions: HashSet::with_capacity(width as usize * height as usize - mines_count),
            flagged_positions: HashSet::new(),
            game_over: false,
            mines: {
                let mut mines = HashSet::with_capacity(mines_count);
                while mines.len() < mines_count {
                    let x = rand::thread_rng().gen_range(0..width);
                    let y = rand::thread_rng().gen_range(0..height);
                    mines.insert((x, y));
                }
                mines
            },
        }
    }

    pub fn open(&mut self, pos: Position) -> &mut Self {
        if let Some(result) = self.open_position(pos) {
            match result {
                OpeningResult::Mine => {
                    self.open_positions.insert(pos);
                    self.game_over = true;
                    self
                }
                OpeningResult::NoMine(mines_around) => {
                    // If the position doesn't have mines around, open the positions around it
                    match mines_around {
                        0 => {
                            self.open_positions.insert(pos);
                            // Recursively open the positions around the current one except the flagged ones and the already open ones
                            self.neighbours(pos).iter().for_each(|position| {
                                if self.can_be_opened(position) {
                                    self.open(*position);
                                }
                            });
                            self
                        }
                        _ => {
                            self.open_positions.insert(pos);
                            self
                        }
                    }
                }
            }
        } else {
            // If the position is already open or flagged, do nothing
            self
        }
    }

    pub fn mines_around(&self, pos: Position) -> u8 {
        // Safely iterate over the 3x3 grid around the position and count the mines
        self.neighbours(pos)
            .iter()
            .filter(|&position| self.mines.contains(position))
            .count() as u8
    }

    fn open_position(&mut self, position: Position) -> Option<OpeningResult> {
        // Check if the position is already open or flagged. If so, return None
        if !self.can_be_opened(&position) {
            return None;
        }

        // Insert the position in the open fields
        self.open_positions.insert(position);

        // Check if the position contains a mine
        if self.mines.contains(&position) {
            return Some(OpeningResult::Mine);
        }

        // Count the mines around the position
        let mines_around = self.mines_around(position);

        Some(OpeningResult::NoMine(mines_around))
    }

    fn neighbours(&self, (x, y): Position) -> HashSet<Position> {
        // Safely iterate over the 3x3 grid around the position and get neighbours' positions
        (x.saturating_sub(1)..=x.saturating_add(1))
            .flat_map(move |i| (y.saturating_sub(1)..=y.saturating_add(1)).map(move |j| (i, j)))
            .filter(move |&(i, j)| (i, j) != (x, y) && i < self.width && j < self.height)
            .collect() // Collect the positions in a HashSet to avoid duplicates
    }

    pub fn toggle_flag(&mut self, position: Position) {
        if !self.game_over {
            if self.flagged_positions.contains(&position) {
                self.flagged_positions.remove(&position);
            } else {
                self.flagged_positions.insert(position);
            }
        }
    }

    fn can_be_opened(&self, position: &Position) -> bool {
        !self.open_positions.contains(position)
            && !self.flagged_positions.contains(position)
            && !self.game_over
    }
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Iterate over the rows
        for y in 0..self.height {
            // Iterate over the columns
            for x in 0..self.width {
                let position = (x, y);

                if !self.game_over {
                    // Check if the position is open
                    if self.open_positions.contains(&position) {
                        // If the position doesn't contain a mine, add the number of mines around it
                        let mines_around = self.mines_around(position);
                        // We can't have more than 8 mines around a position
                        f.write_fmt(format_args!("{} ", mines_around))?;
                    } else if self.flagged_positions.contains(&position) {
                        // If the position is flagged, add a flag to the board
                        f.write_str(&format!("{} ", FLAG))?;
                    } else {
                        f.write_str(&format!("{} ", CELL))?;
                    }
                } else {
                    // If the game is over, show the mines
                    if self.mines.contains(&position) {
                        if self.open_positions.contains(&position) {
                            f.write_str(&format!("{} ", EXPLOSION))?;
                        } else {
                            f.write_str(&format!("{} ", MINE))?;
                        }
                    } else {
                        // If the position doesn't contain a mine, show the number of mines around it
                        let mines_around = self.mines_around(position);
                        f.write_fmt(format_args!("{} ", mines_around))?;
                    }
                }
            }
            // Add a newline character to the board to separate the rows
            // We don't add a newline character after the last row
            if (0..self.height).contains(&y) {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn insert_mines_around_neighbours(minesweeper: &mut Minesweeper, cell: Position) {
        // Get cell's neighbours
        let cell_neighbours = minesweeper.neighbours(cell);

        // Insert mines around cell's neighbours
        cell_neighbours.iter().for_each(|pos| {
            minesweeper.neighbours(*pos).iter().for_each(|position| {
                if !cell_neighbours.contains(position) && *position != cell {
                    minesweeper.mines.insert(*position);
                }
            })
        });
    }

    #[test]
    #[should_panic = "Invalid parameters"]
    fn test_new_invalid_parameters_1() {
        // ================================================
        // This test will panic because the width is 0
        Minesweeper::new(0, 10, 10);
    }

    #[test]
    #[should_panic = "Invalid parameters"]
    fn test_new_invalid_parameters_2() {
        // ================================================
        // This test will panic because the height is 0
        Minesweeper::new(10, 0, 10);
    }

    #[test]
    #[should_panic = "Invalid parameters"]
    fn test_new_invalid_parameters_3() {
        // ================================================
        // This test will panic because the mines_count is 0
        Minesweeper::new(10, 10, 0);
    }

    #[test]
    #[should_panic = "Invalid parameters"]
    fn test_new_invalid_parameters_4() {
        // ================================================
        // This test will panic because the mines_count greater than width * height - 1
        Minesweeper::new(10, 10, 100);
    }

    #[test]
    fn test_new() {
        // ================================================
        // Test the creation of a Minesweeper instance
        let minesweeper = Minesweeper::new(10, 10, 10);
        assert_eq!(minesweeper.width, 10);
        assert_eq!(minesweeper.height, 10);
        assert_eq!(minesweeper.mines.len(), 10);
    }

    #[test]
    fn test_can_be_opened() {
        let mut minesweeper = Minesweeper::new(10, 10, 10);

        // ================================================
        // Test the case when the position is not open and not flagged
        let position = (0, 0);
        assert!(minesweeper.can_be_opened(&position));

        // ================================================
        // Test the case when the position is open
        minesweeper.open_positions.insert(position);
        assert!(!minesweeper.can_be_opened(&position));

        // ================================================
        // Test the case when the position is flagged
        minesweeper.open_positions.remove(&position);
        minesweeper.flagged_positions.insert(position);
        assert!(!minesweeper.can_be_opened(&position));
    }

    #[test]
    fn test_open_position() {
        let cell = (0, 0);

        // ================================================
        // Test the case when the cell has a mine
        let mut minesweeper = Minesweeper::new(10, 10, 10);

        // Insert mine in the cell
        minesweeper.mines.insert(cell);
        assert_eq!(
            minesweeper.open_position(cell),
            Some(OpeningResult::Mine),
            "Mine in the cell"
        );

        // ================================================
        // Test the case when the cell and all its neighbors don't have mines
        // Test the case when we try to open the already opened cell
        let mut minesweeper = Minesweeper::new(10, 10, 10);

        // Remove mines from the cell and around it
        minesweeper.mines.remove(&cell);
        minesweeper.mines.remove(&(1, 0));
        minesweeper.mines.remove(&(0, 1));
        minesweeper.mines.remove(&(1, 1));
        assert_eq!(
            minesweeper.open_position(cell),
            Some(OpeningResult::NoMine(0)),
            "No mines around the cell"
        );
        // Try to open the cell again
        assert_eq!(minesweeper.open_position(cell), None, "Cell already open");

        // ================================================
        // Test the case when the cell has 1 mine around it
        let mut minesweeper = Minesweeper::new(10, 10, 10);

        // Remove mines from the cell from its 2 neighbours
        minesweeper.mines.remove(&cell);
        minesweeper.mines.remove(&(1, 0));
        minesweeper.mines.remove(&(1, 1));
        // Insert 1 mine around the cell
        minesweeper.mines.insert((0, 1));
        assert_eq!(
            minesweeper.open_position(cell),
            Some(OpeningResult::NoMine(1)),
            "1 mine around the cell"
        );

        // ================================================
        // Test the case when the cell has 2 mines around it
        let mut minesweeper = Minesweeper::new(10, 10, 10);

        // Remove mines from the cell from one of its neighbour
        minesweeper.mines.remove(&cell);
        minesweeper.mines.remove(&(1, 1));
        // Insert 2 mines around the cell
        minesweeper.mines.insert((0, 1));
        minesweeper.mines.insert((1, 0));
        assert_eq!(
            minesweeper.open_position(cell),
            Some(OpeningResult::NoMine(2)),
            "2 mines around the cell"
        );

        // ================================================
        // Test the case when the cell has 3 mines around it
        let mut minesweeper = Minesweeper::new(10, 10, 10);

        // Remove mines from the cell
        minesweeper.mines.remove(&cell);
        // Insert 3 mines around the cell
        minesweeper.mines.insert((0, 1));
        minesweeper.mines.insert((1, 0));
        minesweeper.mines.insert((1, 1));
        assert_eq!(
            minesweeper.open_position(cell),
            Some(OpeningResult::NoMine(3)),
            "3 mines around the cell"
        );
    }

    #[test]
    fn test_neighbors() {
        let minesweeper = Minesweeper::new(10, 10, 10);

        // ================================================
        // Test the case when the cell is in the upper left corner
        let cell = (0, 0);
        let neighbors = minesweeper.neighbours(cell);
        assert_eq!(neighbors.len(), 3);
        assert!(neighbors.contains(&(0, 1)));
        assert!(neighbors.contains(&(1, 0)));
        assert!(neighbors.contains(&(1, 1)));

        // ================================================
        // Test the case when the cell is in the lower right corner
        let cell = (9, 9);
        let neighbors = minesweeper.neighbours(cell);
        assert_eq!(neighbors.len(), 3);
        assert!(neighbors.contains(&(8, 8)));
        assert!(neighbors.contains(&(8, 9)));
        assert!(neighbors.contains(&(9, 8)));

        // ================================================
        // Test the case when the cell is near the border
        let cell = (0, 5);
        let neighbors = minesweeper.neighbours(cell);
        assert_eq!(neighbors.len(), 5);
        assert!(neighbors.contains(&(0, 4)));
        assert!(neighbors.contains(&(0, 6)));
        assert!(neighbors.contains(&(1, 4)));
        assert!(neighbors.contains(&(1, 5)));
        assert!(neighbors.contains(&(1, 6)));

        // ================================================
        // Test the case when the cell is in the middle
        let cell = (5, 5);
        let neighbors = minesweeper.neighbours(cell);
        assert_eq!(neighbors.len(), 8);
        assert!(neighbors.contains(&(4, 4)));
        assert!(neighbors.contains(&(4, 5)));
        assert!(neighbors.contains(&(4, 6)));
        assert!(neighbors.contains(&(5, 4)));
        assert!(neighbors.contains(&(5, 6)));
        assert!(neighbors.contains(&(6, 4)));
        assert!(neighbors.contains(&(6, 5)));
        assert!(neighbors.contains(&(6, 6)));
    }

    #[test]
    fn test_open() {
        // ================================================
        // Test the case when the cell has a mine
        let mut minesweeper = Minesweeper::new(10, 10, 10);
        let cell = (0, 0);
        // Insert mine in the cell
        minesweeper.mines.insert(cell);
        minesweeper.open(cell);
        assert!(minesweeper.game_over, "Mine in the cell, game over");
        assert_eq!(minesweeper.open_positions.len(), 1, "1 open position");

        // ================================================
        // Test the case when the cell and all its neighbors don't have mines
        // but all neighbors have mines around them. Test it with the cell (0, 0)
        // The cell has 3 neighbors
        let mut minesweeper = Minesweeper::new(10, 10, 10);
        let cell = (0, 0);
        // Remove all mines
        minesweeper.mines.drain();
        assert!(minesweeper.mines.is_empty(), "No mines");
        assert_eq!(minesweeper.open_positions.len(), 0, "No open positions");

        // Insert mines around cell's neighbours
        insert_mines_around_neighbours(&mut minesweeper, cell);
        assert_eq!(
            minesweeper.mines.len(),
            5,
            "5 mines around the cell neighbours"
        );

        // Open the cell
        minesweeper.open(cell);
        assert!(!minesweeper.game_over, "No mine in the cell, game not over");
        assert_eq!(
            minesweeper.open_positions.len(),
            4,
            "The cell and its neighbours (3) are opened"
        );

        // ================================================
        // Test the case when the cell and all its neighbors don't have mines
        // but all neighbors have mines around them. Test it with the cell (9, 0)
        // The cell has 3 neighbours
        let mut minesweeper = Minesweeper::new(10, 10, 10);
        let cell = (9, 0);
        // Remove all mines
        minesweeper.mines.drain();
        assert!(minesweeper.mines.is_empty(), "No mines");
        assert_eq!(minesweeper.open_positions.len(), 0, "No open positions");

        // Insert mines around cell's neighbours
        insert_mines_around_neighbours(&mut minesweeper, cell);
        assert_eq!(
            minesweeper.mines.len(),
            5,
            "5 mines around the cell neighbours"
        );

        // Open the cell
        minesweeper.open(cell);
        assert!(!minesweeper.game_over, "No mine in the cell, game not over");
        assert_eq!(
            minesweeper.open_positions.len(),
            4,
            "The cell and its neighbours (3) are opened"
        );

        // ================================================
        // Test the case when the cell and all its neighbors don't have mines
        // but all neighbors have mines around them. Test it with the cell (5, 5)
        // The cell has 8 neighbours
        let mut minesweeper = Minesweeper::new(10, 10, 10);
        let cell = (5, 5);
        // Remove all mines
        minesweeper.mines.drain();
        assert!(minesweeper.mines.is_empty(), "No mines");
        assert_eq!(minesweeper.open_positions.len(), 0, "No open positions");

        // Insert mines around cell's neighbours
        insert_mines_around_neighbours(&mut minesweeper, cell);
        assert_eq!(
            minesweeper.mines.len(),
            16,
            "16 mines around the cell neighbours"
        );

        // Open the cell
        minesweeper.open(cell);
        assert!(!minesweeper.game_over, "No mine in the cell, game not over");
        assert_eq!(
            minesweeper.open_positions.len(),
            9,
            "The cell and its neighbours (8) are opened"
        );

        // ================================================
        // Test the case when the cell and all its neighbors don't have mines
        // but all neighbors have mines around them. Test it with the cell (0, 5)
        // The cell has 5 neighbours
        let mut minesweeper = Minesweeper::new(10, 10, 10);
        let cell = (0, 5);
        // Remove all mines
        minesweeper.mines.drain();
        assert!(minesweeper.mines.is_empty(), "No mines");
        assert_eq!(minesweeper.open_positions.len(), 0, "No open positions");

        // Insert mines around cell's neighbours
        insert_mines_around_neighbours(&mut minesweeper, cell);
        assert_eq!(
            minesweeper.mines.len(),
            9,
            "9 mines around the cell neighbours"
        );

        // Open the cell
        minesweeper.open(cell);
        assert!(!minesweeper.game_over, "No mine in the cell, game not over");
        assert_eq!(
            minesweeper.open_positions.len(),
            6,
            "The cell and its neighbours (5) are opened"
        );

        // ================================================
        // Test the case when the cell has a mine next to it
        let mut minesweeper = Minesweeper::new(10, 10, 10);
        let cell = (5, 5);
        // Insert 1 mine around the cell
        minesweeper.mines.insert((5, 6));
        minesweeper.open(cell);
        assert_eq!(
            minesweeper.open_positions.len(),
            1,
            "1 mine around the cell. only 1 cell is opened"
        );

        // ================================================
        // Test the case when the cell is flagged
        let mut minesweeper = Minesweeper::new(10, 10, 10);
        let cell = (5, 5);
        // No cell is flagged
        assert_eq!(minesweeper.flagged_positions.len(), 0, "No cell is flagged");
        // No cell is opened
        assert_eq!(minesweeper.open_positions.len(), 0, "No cell is opened");
        // Insert insert flag into the cell
        minesweeper.toggle_flag(cell);
        assert_eq!(minesweeper.flagged_positions.len(), 1, "1 cell is flagged");
        assert!(
            minesweeper.flagged_positions.contains(&cell),
            "Tested cell is flagged"
        );
        // Try to open the cell
        minesweeper.open(cell);
        assert_eq!(minesweeper.open_positions.len(), 0, "No cell is opened");

        // ================================================
        // Test the case when the cell is already opened
        let mut minesweeper = Minesweeper::new(10, 10, 10);
        let cell = (5, 5);
        // Remove all mines
        minesweeper.mines.drain();
        // No cell is opened
        assert_eq!(minesweeper.open_positions.len(), 0, "No cell is opened");
        // Insert 1 mine around the cell to prevent opening other cells
        minesweeper.mines.insert((5, 6));
        // Open the cell
        minesweeper.open(cell);
        assert_eq!(minesweeper.open_positions.len(), 1, "1 cell is opened");
        assert!(
            minesweeper.open_positions.contains(&cell),
            "Tested cell is opened"
        );
        // Try to open the cell again
        minesweeper.open(cell);
        assert_eq!(minesweeper.open_positions.len(), 1, "1 cell is opened");
    }

    #[test]
    fn test_toggle_flag() {
        let mut minesweeper = Minesweeper::new(10, 10, 10);
        let cell = (5, 5);
        assert_eq!(
            minesweeper.flagged_positions.len(),
            0,
            "No position flagged"
        );
        minesweeper.toggle_flag(cell);
        assert_eq!(minesweeper.flagged_positions.len(), 1, "1 position flagged");
        assert!(
            minesweeper.flagged_positions.contains(&cell),
            "Tested cell flagged"
        );
        minesweeper.toggle_flag(cell);
        assert_eq!(
            minesweeper.flagged_positions.len(),
            0,
            "1 flagged position removed"
        );
    }

    #[test]
    fn test_to_string() {
        // // ================================================
        // // Test a new game
        let minesweeper = Minesweeper::new(10, 10, 10);
        let minesweeper_str = minesweeper.to_string();
        let string_lines: Vec<&str> = minesweeper_str.lines().collect();
        assert_eq!(string_lines.len(), 10, "The board has 10 lines");
        assert_eq!(
            string_lines[0].chars().count(),
            20,
            "Each line has 20 characters"
        );
        let expected_line =
            format!("{CELL} {CELL} {CELL} {CELL} {CELL} {CELL} {CELL} {CELL} {CELL} {CELL} ");
        string_lines.into_iter().for_each(|line| {
            assert_eq!(line, expected_line.as_str(), "Line has 10 unopened cells");
        });

        // // ================================================
        // // Test mines around the cell
        let mut minesweeper = Minesweeper::new(10, 10, 10);
        let cell = (5, 5);
        // Remove all mines
        minesweeper.mines.drain();
        // Insert mines next to the cell
        minesweeper.mines.insert((5, 4));
        // Open the cell
        minesweeper.open(cell);
        // Convert the game to string
        let minesweeper_str = minesweeper.to_string();
        // Split the string into lines
        let string_lines: Vec<&str> = minesweeper_str.lines().collect();
        let expected_line =
            format!("{CELL} {CELL} {CELL} {CELL} {CELL} 1 {CELL} {CELL} {CELL} {CELL} ");
        assert_eq!(
            string_lines[5],
            expected_line.as_str(),
            "Cell (5, 5) is opened and has 1 mine next to it"
        );

        // // ================================================
        // // Test the case when the cell is flagged
        let mut minesweeper = Minesweeper::new(10, 10, 10);
        let cell = (5, 5);
        // Insert flag into the cell
        minesweeper.toggle_flag(cell);
        // Convert the game to string
        let minesweeper_str = minesweeper.to_string();
        // Split the string into lines
        let string_lines: Vec<&str> = minesweeper_str.lines().collect();
        let expected_line =
            format!("{CELL} {CELL} {CELL} {CELL} {CELL} {FLAG} {CELL} {CELL} {CELL} {CELL} ");
        assert_eq!(
            string_lines[5],
            expected_line.as_str(),
            "Cell (5, 5) is flagged"
        );

        // // ================================================
        // Test the case when the game is over and the cell has an explosion
        // Test mines around the cell's neighbors
        let mut minesweeper = Minesweeper::new(10, 10, 10);
        let cell = (0, 0);
        // Remove all mines
        minesweeper.mines.drain();
        // Add a mine to the cell
        minesweeper.mines.insert(cell);
        // Add mines around the cell's neighbors
        insert_mines_around_neighbours(&mut minesweeper, cell);
        // Open the cell
        minesweeper.open(cell);
        // Convert the game to string
        let minesweeper_str = minesweeper.to_string();
        // Split the string into lines
        let string_lines: Vec<&str> = minesweeper_str.lines().collect();
        let expected_line = format!("{EXPLOSION} 3 {MINE} 2 0 0 0 0 0 0 ");
        assert_eq!(
            string_lines[0], expected_line.as_str(),
            "Cell (0, 0) is opened and has an explosion in it, and cells (5,3) and (5,7) have a mine"
        );
        let expected_line = format!("{MINE} {MINE} {MINE} 2 0 0 0 0 0 0 ");
        assert_eq!(
            string_lines[2],
            expected_line.as_str(),
            "Line 2 has 3 mines in columns 0, 1, 2"
        );
        assert_eq!(
            string_lines[0].chars().next(),
            Some(EXPLOSION),
            "Cell (0, 0) has an explosion"
        );
    }
}
