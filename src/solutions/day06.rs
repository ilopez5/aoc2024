use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

type Record = HashSet<Coord>;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
struct DataPad {
    start: Coord,
    map: Record,
    empty: Record,
    obstacles: Record,
    walked: HashMap<Coord, Record>,
}

impl DataPad {
    fn new() -> Self {
        let mut walked = HashMap::new();
        walked.insert(Coord::up(), HashSet::new());
        walked.insert(Coord::right(), HashSet::new());
        walked.insert(Coord::down(), HashSet::new());
        walked.insert(Coord::left(), HashSet::new());
        Self {
            start: Coord::new(0, 0),
            map: HashSet::new(),
            empty: HashSet::new(),
            obstacles: HashSet::new(),
            walked,
        }
    }

    fn has_walked(&self, pos: Coord) -> bool {
        self.walked.values().any(|dir| dir.contains(&pos))
    }

    fn is_loop(&self, pos: Coord, direction: Coord) -> bool {
        self.walked.get(&direction).unwrap().contains(&pos)
    }
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn step(&self, inc: Self) -> Self {
        Self::new(self.x + inc.x, self.y + inc.y)
    }

    fn turn(&self) -> Self {
        if *self == Self::up() {
            Self::right()
        } else if *self == Self::right() {
            Self::down()
        } else if *self == Self::down() {
            Self::left()
        } else if *self == Self::left() {
            Self::up()
        } else {
            panic!("Coord is not a cardinal direction");
        }
    }

    fn up() -> Self {
        Self::new(-1, 0)
    }

    fn right() -> Self {
        Self::new(0, 1)
    }

    fn down() -> Self {
        Self::new(1, 0)
    }

    fn left() -> Self {
        Self::new(0, -1)
    }
}

fn main() {
    let input: &str = include_str!("inputs/day06.txt");
    let lines: Vec<&str> = input.lines().collect();
    let pad: DataPad = parse(&lines);

    let mut current_pad: DataPad = pad.clone();
    run(&mut current_pad, false);
    let mut walked: HashSet<&Coord> = current_pad.walked.values().flatten().collect();

    println!("Part 1: {}", walked.len());

    walked.remove(&pad.start); // Remove start position
    let new_obstructions: usize = walked
        .iter()
        .map(|candidate| {
            let mut faux_pauxd: DataPad = pad.clone();
            faux_pauxd.obstacles.insert(**candidate);
            if run(&mut faux_pauxd, true) {
                1
            } else {
                0
            }
        })
        .sum();

    println!("Part 2: {}", new_obstructions);
}

fn parse(lines: &Vec<&str>) -> DataPad {
    let mut pad = DataPad::new();
    lines.iter().enumerate().for_each(|(x, line)| {
        line.char_indices().for_each(|(y, char)| {
            let c = Coord::new(x as isize, y as isize);
            match char {
                '.' => pad.empty.insert(c),
                '#' => pad.obstacles.insert(c),
                '^' => {
                    pad.start = c;
                    pad.walked.get_mut(&Coord::up()).unwrap().insert(c)
                }
                _ => false,
            };
            pad.map.insert(c);
        });
    });
    pad
}

fn run(pad: &mut DataPad, stop_on_loop: bool) -> bool {
    let mut direction: Coord = Coord::up();
    let mut guard: Coord = pad.start.clone();
    while pad.map.contains(&guard) {
        let next: Coord = guard.step(direction);
        if !pad.map.contains(&next) {
            break;
        } else if pad.obstacles.contains(&next) {
            direction = direction.turn();
        } else if pad.empty.contains(&next) {
            pad.empty.remove(&next);
            pad.walked.get_mut(&direction).unwrap().insert(next);
            guard = next;
        } else if pad.has_walked(next) {
            if stop_on_loop && pad.is_loop(next, direction) {
                return true;
            }
            guard = next;
        }
    }
    return false;
}
