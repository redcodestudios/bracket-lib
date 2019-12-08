#![allow(unused_variables)]

// Benchmark field of view calculations,
// most of the code copied from ex04-fov.rs

extern crate rand;
use crate::rand::Rng;

#[macro_use]
extern crate criterion;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};

use rltk::{
    Algorithm2D,
    BaseMap,
    Point,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("all fov 8", |b| b.iter(|| {
        let s = State::new();
        let mut i = 0;
        for x in 0 .. W-1 {
            for y in 0 .. H-1 {
                i += 1;
                if i % 10 != 0 { continue; }
                let p = Point::new(x, y);
                let fov = rltk::field_of_view(p, 8, &s);
                black_box(fov);
            }
        }
    }));
    c.bench_function("all fov 20", |b| b.iter(|| {
        let s = State::new();
        let mut i = 0;
        for x in 0 .. W-1 {
            for y in 0 .. H-1 {
                i += 1;
                if i % 10 != 0 { continue; }
                let p = Point::new(x, y);
                let fov = rltk::field_of_view(p, 20, &s);
                black_box(fov);
            }
        }
    }));
}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

struct State {
    map: Vec<TileType>,
}

const H:i32 = 50;
const W:i32 = 80;

fn xy_idx(x: i32, y: i32) -> usize {
    ((y * W) + x) as usize
}

fn idx_xy(idx: usize) -> (i32, i32) {
    (idx as i32 % W, idx as i32 / W)
}

impl BaseMap for State {
    fn is_opaque(&self, idx: i32) -> bool {
        self.map[idx as usize] == TileType::Wall
    }
    fn get_available_exits(&self, _idx: i32) -> Vec<(i32, f32)> {
        Vec::new()
    }
    fn get_pathing_distance(&self, _idx1: i32, _idx2: i32) -> f32 {
        0.0
    }
}

impl Algorithm2D for State {
    fn point2d_to_index(&self, pt: Point) -> i32 {
        xy_idx(pt.x, pt.y) as i32
    }
    fn index_to_point2d(&self, idx: i32) -> Point {
        Point::new(idx % W, idx / W)
    }
    fn in_bounds(&self, pos:Point) -> bool {
        let idx = self.point2d_to_index(pos);
        !(idx < 0 || idx >= W-1 * H-1)
    }
}

impl State {
    pub fn new() -> Self {
        let mut state = State {
            map: vec![TileType::Floor; (W * H) as usize],
        };

        let mut rng = rand::thread_rng();
        for _ in 0..400 {
            let x = rng.gen_range(1, W-1);
            let y = rng.gen_range(1, H-1);
            let idx = xy_idx(x, y);
            state.map[idx] = TileType::Wall;
        }

        state
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
