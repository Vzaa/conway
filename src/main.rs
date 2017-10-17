extern crate rand;

use std::{fmt, iter, thread, time};

use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Dead,
    Alive,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match *self {
            Cell::Dead => ' ',
            Cell::Alive => '█',
        };
        write!(f, "{}", ch)
    }
}

struct Grid {
    width: usize,
    height: usize,
    buf: Box<[Cell]>,
}

impl Grid {
    /// width, height and percentage
    pub fn new(w: usize, h: usize, p: f32) -> Self {
        let mut rng = rand::thread_rng();
        let buf_size = h * w;
        let live_cnt = ((buf_size as f32) * (p / 100.0)) as usize;
        let dead_cnt = buf_size - live_cnt;

        let mut buf = iter::repeat(Cell::Alive)
            .take(live_cnt)
            .chain(iter::repeat(Cell::Dead).take(dead_cnt))
            .collect::<Vec<Cell>>()
            .into_boxed_slice();

        rng.shuffle(&mut buf);

        Self {
            buf,
            width: w,
            height: h,
        }
    }

    pub fn display(&self) {
        (0..self.width).for_each(|_| print!("-"));
        println!("");

        self.buf.chunks(self.width).for_each(|l| {
            l.iter().for_each(|c| print!("{}", c));
            println!("");
        });

        (0..self.width).for_each(|_| print!("-"));
        println!("");
    }

    fn get_neigh(&self, idx: usize) -> [Cell; 8] {
        let l_edge = idx % self.width == 0;
        let r_edge = idx % self.width == self.width - 1;
        let t_edge = idx / self.width == 0;
        let b_edge = idx / self.width == self.height - 1;

        let l = if !l_edge {
            idx - 1
        } else {
            idx + self.width - 1
        };

        let r = if !r_edge {
            idx + 1
        } else {
            idx + 1 - self.width
        };

        let u = if !t_edge {
            idx - self.width
        } else {
            idx + (self.width * (self.height - 1))
        };

        let d = if !b_edge {
            idx + self.width
        } else {
            idx % self.width
        };

        let ru = if !t_edge {
            r - self.width
        } else {
            r + (self.width * (self.height - 1))
        };

        let rd = if !b_edge {
            r + self.width
        } else {
            r % self.width
        };

        let lu = if !t_edge {
            l - self.width
        } else {
            l + (self.width * (self.height - 1))
        };

        let ld = if !b_edge {
            l + self.width
        } else {
            l % self.width
        };

        //println!("{} {:?}", idx,  [l, r, u, d, ru, rd, lu, ld]);
        [
            self.buf[l],
            self.buf[r],
            self.buf[u],
            self.buf[d],
            self.buf[ru],
            self.buf[rd],
            self.buf[lu],
            self.buf[ld],
        ]
    }

    fn step(&mut self) {
        let counts: Vec<_> = (0..self.buf.len())
            .map(|idx| {
                self.get_neigh(idx)
                    .iter()
                    .filter(|&c| *c == Cell::Alive)
                    .count()
            })
            .collect();

        for (c, cnt) in self.buf.iter_mut().zip(counts) {
            *c = match (*c, cnt) {
                (Cell::Alive, cnt) if cnt < 2 => Cell::Dead,
                (Cell::Alive, cnt) if cnt < 4 => Cell::Alive,
                (Cell::Alive, cnt) if cnt >= 4 => Cell::Dead,
                (Cell::Dead, cnt) if cnt == 3 => Cell::Alive,
                _ => *c,
            }
        }
    }
}

fn main() {
    let mut g = Grid::new(50, 15, 15.0);

    g.display();
    for _ in 0..30 {
        thread::sleep(time::Duration::from_millis(200));
        g.step();
        g.display();
    }
}
