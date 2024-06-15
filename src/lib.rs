pub mod model;

use std::cell::RefCell;

use model::Minesweeper;
use wasm_bindgen::prelude::*;

// This is like a global variable, but it's only accessible from the current thread.
// JS is single-threaded, so this is fine.
thread_local! {
    static MS: RefCell<Minesweeper> = RefCell::new(Minesweeper::new(10, 10, 15));
}

#[wasm_bindgen(js_name = "getGameState")]
pub fn get_game_state() -> String {
    MS.with_borrow(|ms| ms.to_string())
}

#[wasm_bindgen(js_name = "openCell")]
pub fn open_cell(x: usize, y: usize) {
    MS.with_borrow_mut(|ms| {
        ms.open((x as u16, y as u16));
    });
}

#[wasm_bindgen(js_name = "toggleFlag")]
pub fn toggle_flag(x: usize, y: usize) {
    MS.with_borrow_mut(|ms| {
        ms.toggle_flag((x as u16, y as u16));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_game_state() {
        let state = get_game_state();
        let lines = state.lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), 10);
        assert_eq!(lines[0].chars().count(), 20);
    }

    #[test]
    fn test_open_cell() {
        MS.with_borrow_mut(|ms| {
            for x in 0..10 {
                for y in 0..10 {
                    if !ms.mines.contains(&(x, y)) {
                        ms.open((x, y));
                    }
                }
            }
            assert!(!ms.game_over, "Game should not be over");
            assert_eq!(ms.open_positions.len(), 85, "85 cells should be open");
        });
    }

    #[test]
    fn test_toggle_flag() {
        MS.with_borrow_mut(|ms| {
            for x in 0..10 {
                for y in 0..10 {
                    if ms.mines.contains(&(x, y)) {
                        ms.toggle_flag((x, y));
                    }
                }
            }
            assert_eq!(ms.flagged_positions.len(), 15, "15 cells should be flagged");
        });
    }
}
