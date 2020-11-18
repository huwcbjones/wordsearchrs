use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "wordsearch", about = "Wordsearcher")]
struct Args {
    #[structopt(parse(from_os_str))]
    grid: PathBuf,

    #[structopt(multiple = true)]
    words: Vec<String>,
}

struct Grid {
    size: usize,
    rows: Vec<u8>,
    columns: Vec<u8>,
}

/// Calculate grid size
fn calculate_grid_size(length: usize) -> Result<usize, String> {
    let size: usize = (length as f64).sqrt().floor() as usize;
    if size * size != length {
        return Err(String::from("Invalid grid length"));
    }
    return Ok(size);
}

/// Check whether word is overlapping a line
fn is_overlapping_line(position: usize, word: &String, grid_size: usize) -> bool {
    position % grid_size + word.len() > grid_size
}

fn delta1(pattern: &[u8]) -> [usize; 256] {
    let mut delta1 = [0; 256];
    for i in 0..256 {
        delta1[i] = pattern.len();
    }
    for i in 0..pattern.len() as isize - 2 {
        delta1[pattern[i as usize] as usize] = pattern.len() - 1 - i as usize;
    }
    return delta1;
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let n = haystack.len();
    let m = needle.len();
    let skip = delta1(needle)[needle[m - 1] as usize];
    let mut i = 0;
    while i <= n - m {
        if haystack[i + m - 1] == needle[m - 1] {  // (boyer - moore)
            // potential match
            if haystack[i..i + m - 1] == needle[..m - 1] {
                return Some(i);
            }
            if i + m < haystack.len() && !needle.contains(&haystack[i + m]) {
                i += m + 1  // sunday
            } else {
                i += skip  // horspool
            }
        } else {
            // skip
            if i + m < haystack.len() && !needle.contains(&haystack[i + m]) {
                i += m + 1; // sunday
            } else {
                i += 1;
            }
        }
    }
    None
}

/// Check whether word is present
fn is_present(grid: &Grid, word: &String) -> bool {
    if word.len() > grid.size {
        return false;
    }
    let pattern = word.as_bytes();

    let horizontal_position = find(&grid.rows, pattern);
    if let Some(position) = horizontal_position {
        if !is_overlapping_line(position, &word, grid.size) {
            return true;
        }
    }
    let vertical_position = find(&grid.columns, pattern);
    if let Some(position) = vertical_position {
        if !is_overlapping_line(position, &word, grid.size) {
            return true;
        }
    }
    // for y in 0..grid.size {
    //     for x in 0..grid.size {
    //         let horizontal_position = y * grid.size + x;
    //         let vertical_position = y + x * grid.size;
    //         if pattern_length <= grid.size - x
    //             && find(&grid.rows, pattern) {
    //             return true;
    //         }
    //         if pattern_length <= grid.size - y
    //             && grid.columns[vertical_position..vertical_position + pattern_length].eq(pattern) {
    //             return true;
    //         }
    //     }
    // }
    return false;
}

fn rows_to_cols(rows: &Vec<u8>, size: usize) -> Vec<u8> {
    let mut cols: Vec<u8> = vec![0; size * size];
    for y in 0..size {
        for x in 0..size {
            let src = x * size + y;
            let dst = x + y * size;
            cols[dst] = rows[src];
        }
    }
    cols
}

fn string_to_grid(grid: Vec<u8>) -> Result<Grid, String> {
    let size = calculate_grid_size(grid.len())?;
    let columns = rows_to_cols(&grid, size);
    Ok(Grid { size, rows: grid, columns })
}

fn main() -> Result<(), String> {
    let args = Args::from_args();

    // Load grid
    let mut file_handle = BufReader::new(File::open(&args.grid).expect("Failed to open grid"));
    let mut rows = Vec::new();
    file_handle.read_to_end(&mut rows).expect("Failed to read grid");
    let grid = string_to_grid(rows)?;

    // Check words
    for word in args.words {
        if is_present(&grid, &word) {
            println!("Found word '{}'", word);
        }
    }
    Ok(())
}

#[test]
fn test_rows_to_cols() {
    let rows = "abcd".into();
    assert_eq!("acbd".as_bytes(), rows_to_cols(&rows, 2));
}

#[test]
fn test_string_longer_than_size() {
    let grid = string_to_grid("abcd".into()).unwrap();
    assert!(!is_present(&grid, &String::from("asdf")));
}

#[test]
fn test_string_present_row() {
    let grid = string_to_grid("abcd".into()).unwrap();
    assert!(is_present(&grid, &String::from("ab")));
    assert!(is_present(&grid, &String::from("cd")));
}

#[test]
fn test_string_overlap_row() {
    let grid = string_to_grid("abcd".into()).unwrap();
    assert!(!is_present(&grid, &String::from("bc")));
}

#[test]
fn test_string_present_column() {
    let grid = string_to_grid("abcd".into()).unwrap();
    assert!(is_present(&grid, &String::from("ac")));
    assert!(is_present(&grid, &String::from("bd")));
}

#[test]
fn test_string_overlap_column() {
    let grid = string_to_grid("abcd".into()).unwrap();
    assert!(!is_present(&grid, &String::from("bc")));
}
