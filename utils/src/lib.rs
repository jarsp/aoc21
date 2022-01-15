use std::{str::FromStr, error::Error};
use core::fmt;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl FromStr for Coord {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<usize> =
            s.split(',')
             .map(|x| x.parse::<usize>())
             .try_collect()?;

        if v.len() != 2 {
            Err("Wrong number of entries for Coord".into())
        } else {
            Ok(Coord{x: v[0], y: v[1]})
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Grid<T> {
        let height = data.len();
        let width = if height == 0 {
            0
        } else {
            data[0].len()
        };

        Grid {data, height, width}
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y).and_then(|r| r.get(x))
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.data.get_mut(y).and_then(|r| r.get_mut (x))
    }

    pub fn index(&self, x: usize, y: usize) -> &T {
        &self.data[y][x]
    }

    pub fn index_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[y][x]
    }

    pub fn iter_neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let (w, h) = (self.width, self.height);
        (0..3).cartesian_product(0..3)
                .filter_map(move |(dx, dy)| {
                    let nx = (x + dx).wrapping_sub(1);
                    let ny = (y + dy).wrapping_sub(1);
                    if nx < w && ny < h && (x != nx || y != ny) {
                        Some((nx, ny))
                    } else {
                        None
                    }
                })
    }

    pub fn iter_coords(&self)
        -> impl Iterator<Item = impl Iterator<Item = (&T, (usize, usize))>>
    {
        let coords =
            (0..self.height)
                .map(|y| (0..self.width).map(move |x| (x, y)));
        self.data
            .iter()
            .zip(coords)
            .map(|(v, cs)| v.iter().zip(cs))
    }

    pub fn iter_mut_coords(&mut self)
        -> impl Iterator<Item = impl Iterator<Item = (&mut T, (usize, usize))>>
    {
        let coords =
            (0..self.height)
                .map(|y| (0..self.width).map(move |x| (x, y)));
        self.data
            .iter_mut()
            .zip(coords)
            .map(|(v, cs)| v.iter_mut().zip(cs))
    }
}

impl<T: fmt::Display> Grid<T> {
    pub fn show(&self) {
        for row in &self.data {
            for t in row {
                print!("{}", t);
            }
            println!();
        }
    }
}

impl<T, V: IntoIterator<Item = T>> FromIterator<V> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self  {
        let data =
            iter.into_iter()
                .map(|it| it.into_iter().collect_vec())
                .collect_vec();
        Grid::new(data)
    }
}

impl<T: FromStr> Grid<T> {
    pub fn parse_grid<I>(iter: I, sep: &str)
        -> Result<Grid<T>, &'static str>
    where
        I: Iterator<Item = String>
    {
        // TODO: Kind of dumb?
        let data: Vec<Vec<T>> =
            iter.map(|s| {
                    if sep.is_empty() {
                        s.chars()
                            .map(|c| T::from_str(&c.to_string()))
                            .try_collect()
                            .map_err(|_| "parse error")
                    } else {
                        s.split(sep)
                            .map(&str::parse::<T>)
                            .try_collect()
                            .map_err(|_| "parse error")
                    }
                })
                .try_collect()?;

        Ok(Grid::new(data))
    }
}