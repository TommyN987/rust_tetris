use std::{collections::HashSet, ops::Add};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pos(pub i32, pub i32);

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Clone)]
pub struct Tetromino {
    name: &'static str,
    positions: HashSet<Pos>,
    anchor: Pos,
}

macro_rules! impl_tetromino_constructor {
    ($( $new:ident $name:literal: [ $( $pos:expr ),* ] @ $anchor:expr; )*) => {
        $(
            pub fn $new() -> Self {
                Self {
                    name: $name,
                    positions: [$( $pos),* ].into_iter().collect(),
                    anchor: $anchor,
                }
            }
        )*
    };
}

impl Tetromino {
    impl_tetromino_constructor! {
        new_i "🟥": [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)] @ Pos(1, 0);
        new_o "🟨": [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(1, 1)] @ Pos(0, 0);
        new_t "🟨": [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(1, 1)] @ Pos(0, 0);
        new_j "🟪": [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(-1, 2)] @ Pos(0, 1);
        new_l "🟧": [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(1, 2)] @ Pos(0, 1);
        new_s "🟩": [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(-1, 1)] @ Pos(0, 0);
        new_z "🟥": [Pos(0, 0), Pos(-1, 0), Pos(0, 1), Pos(1, 1)] @ Pos(0, 0);
    }

    pub fn new_random() -> Self {
        let random = (rand::random::<f64>() * 7.0).floor() as u8;

        match random {
            0 => Self::new_i(),
            1 => Self::new_o(),
            2 => Self::new_t(),
            3 => Self::new_j(),
            4 => Self::new_l(),
            5 => Self::new_s(),
            6 => Self::new_z(),
            _ => unreachable!(),
        }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_positions(&self) -> impl Iterator<Item = Pos> + '_ {
        self.positions.iter().copied()
    }

    pub fn has_position(&self, pos: Pos) -> bool {
        self.positions.contains(&pos)
    }

    pub fn rotated(&self) -> Self {
        let Pos(a, b) = self.anchor;

        Self { 
            name: self.name,
            positions: self
                .get_positions()
                .map(|Pos(x, y)| Pos(-y + b + a, x - a + b))
                .collect(), 
            anchor: self.anchor, 
        }
    }

    pub fn remove_line(&mut self, y: i32) {
        self.positions = self
            .positions
            .iter()
            .copied()
            .filter(|pos| pos.1 != y)
            .map(|pos| {
                if pos.1 >= y {
                    pos
                } else {
                    Pos(pos.0, pos.1 + 1)
                }
            })
            .collect()
    }

    pub fn collides_with(&self, other: &Tetromino) -> bool {
        self.positions.intersection(&other.positions).count() > 0
    }
}

impl Add<Pos> for &Tetromino {
    type Output = Tetromino;

    fn add(self, rhs: Pos) -> Self::Output {
        Tetromino {
            name: self.name,
            positions: self.positions.iter().map(|&pos| pos + rhs).collect(),
            anchor: self.anchor + rhs,
        }
    }
}