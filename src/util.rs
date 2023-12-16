use std::ops::Range;

use itertools::Itertools;

pub struct LineGrid<'a> {
    lines: Vec<&'a [u8]>,
    width: usize,
    height: usize,
}

impl<'a> LineGrid<'a> {
    pub fn new(string: &'a str) -> Self {
        let lines = string.trim().lines().map(str::as_bytes).collect::<Vec<_>>();
        let width = lines[0].len();
        let height = lines.len();
        LineGrid {
            lines,
            width,
            height,
        }
    }

    pub fn in_bounds(&self, row: i32, col: i32) -> bool {
        let (w, h) = (self.width() as i32, self.height() as i32);
        row >= 0 && col >= 0 && row < h && col < w
    }

    pub fn get(&self, row: usize, col: usize) -> Option<char> {
        (row < self.height() && col < self.width()).then(|| self.lines[row][col] as char)
    }

    pub fn try_get(&self, row: i32, col: i32) -> Option<char> {
        let (r, c) = (row as usize, col as usize);
        self.in_bounds(row, col).then(|| self.lines[r][c] as char)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn section(&self, row: usize, range: Range<usize>) -> &'a str {
        std::str::from_utf8(&self.lines[row][range]).unwrap()
    }

    pub fn iter(&self) -> impl '_ + Iterator<Item = (usize, usize, char)> {
        self.lines.iter().enumerate().flat_map(|(i, row)| {
            row.iter()
                .copied()
                .enumerate()
                .map(move |(j, c)| (i, j, c as char))
        })
    }

    pub fn iter_row(&self, row: usize) -> impl '_ + Iterator<Item = char> {
        self.lines[row].iter().map(|&c| c as char)
    }

    pub fn iter_col(&self, col: usize) -> impl '_ + Iterator<Item = char> {
        (0..self.height()).map(move |r| self.lines[r][col] as char)
    }
}

impl<'a> ToString for LineGrid<'a> {
    fn to_string(&self) -> String {
        self.lines
            .iter()
            .map(|l| std::str::from_utf8(l).unwrap())
            .join("\n")
    }
}

pub fn grid_neighbours(tile: (usize, usize)) -> impl Iterator<Item = (i32, i32)> {
    [-1, 0, 1].into_iter().flat_map(move |i| {
        [-1, 0, 1]
            .into_iter()
            .map(move |j| (tile.0 as i32 + i, tile.1 as i32 + j))
    })
}
