use std::collections::HashSet;
use std::mem;

use crate::tetromino::Tetromino;
use crate::tetromino::Pos;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Tetris {
    width: i32,
    height: i32,
    current_tetromino: Tetromino,
    fixed_tetrominos: Vec<Tetromino>,
    lost: bool,
}

impl Tetris {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width as i32,
            height: height as i32,
            current_tetromino: &Tetromino::new_random() + Pos((width as i32) / 2, 0),
            fixed_tetrominos: vec![],
            lost: false,
        }
    }

    pub fn iter_positions(&self) -> impl Iterator<Item = Pos> {
        let width = self.width;
        let height = self.height;

        (0..height).flat_map(move |y| (0..width).map(move |x| Pos(x, y)))
    }

    pub fn get(&self, pos: Pos) -> Option<&'static str> {
        if self.current_tetromino.has_position(pos) {
            Some(self.current_tetromino.get_name())
        } else {
            self
                .fixed_tetrominos
                .iter()
                .find(|tetromino| tetromino.has_position(pos))
                .map(|tetromino| tetromino.get_name())
        }
    }

    pub fn move_tetromino(&mut self, direction: Direction) {
        if self.lost {
            return;
        }
        
        let translated_current_tetromino = &self.current_tetromino + match direction {
            Direction::Left => Pos(-1, 0),
            Direction::Right => Pos(1, 0),
        };
        if !self.is_colliding(&translated_current_tetromino) && !self.is_out_of_bounds(&translated_current_tetromino) {
            self.current_tetromino = translated_current_tetromino
        }
    }

    pub fn rotate_tetromino(&mut self) {
        if self.lost {
            return;
        }
        
        let rotated_current_tetromino = self.current_tetromino.rotated();

        if !self.is_colliding(&rotated_current_tetromino) && !self.is_out_of_bounds(&rotated_current_tetromino) {
            self.current_tetromino = rotated_current_tetromino
        }
    }

    pub fn is_out_of_bounds(&self, tetromino: &Tetromino) -> bool {
        !tetromino.get_positions().all(|pos| {
            0 <= pos.0 && pos.0 < self.width && 0 <= pos.1 && pos.1 < self.height
        })
    }

    pub fn is_colliding(&self, tetromino: &Tetromino) -> bool  {
        self
            .fixed_tetrominos
            .iter()
            .any(|fixed_tetromino| fixed_tetromino.collides_with(tetromino))
    }

    pub fn is_line_full(&self, y: i32) -> bool {
        self
            .fixed_tetrominos
            .iter()
            .flat_map(|tetromino| tetromino.get_positions())
            .filter(|pos| pos.1 == y)
            .collect::<HashSet<_>>()
            .len() as i32
            == self.width
    }

    fn remove_line(&mut self, y: i32) {
        for tetromino in self.fixed_tetrominos.iter_mut() {
            tetromino.remove_line(y)
        }
    }

    fn remove_full_lines(&mut self) {
        for y in 0..self.height {
            if self.is_line_full(y) {
                self.remove_line(y)
            }
        }
    }

    pub fn tick(&mut self) {
        if self.lost {
            return;
        }

        let translated_current_tetromino = &self.current_tetromino + Pos(0, 1);

        if self.is_out_of_bounds(&translated_current_tetromino) || self.is_colliding(&translated_current_tetromino) {
            // Freeze current tetromino to this position
            let new_fixed_tetromino = mem::replace(
                &mut self.current_tetromino, 
                &Tetromino::new_random() + Pos(self.width / 2, 0)
            );

            self.fixed_tetrominos.push(new_fixed_tetromino);
            self.remove_full_lines();

            if self.is_colliding(&self.current_tetromino) {
                self.lost = true;
            }
        } else {
            self.current_tetromino = translated_current_tetromino;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tetris;

    #[test]
    fn test() {
        let mut tetris = Tetris::new(10, 30);
        tetris.tick();
        tetris.tick();
        tetris.tick();
        println!("{:#?}", tetris);
    }
}