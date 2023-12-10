use std::ops::Range;

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

    pub fn get(&self, row: i32, col: i32) -> Option<char> {
        let (w, h) = (
            self.width.try_into().unwrap(),
            self.height().try_into().unwrap(),
        );
        (row >= 0 && col >= 0 && row < h && col < w)
            .then(|| self.lines[row as usize][col as usize] as char)
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

    pub fn iter_row(&self, row: usize) -> impl '_ + Iterator<Item = (usize, char)> {
        self.lines[row].iter().map(|&c| c as char).enumerate()
    }
}

pub fn grid_neighbours(tile: (usize, usize)) -> impl Iterator<Item = (i32, i32)> {
    [-1, 0, 1].into_iter().flat_map(move |i| {
        [-1, 0, 1]
            .into_iter()
            .map(move |j| (tile.0 as i32 + i, tile.1 as i32 + j))
    })
}
