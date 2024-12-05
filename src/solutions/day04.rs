use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn xmas(&self, inc: Self) -> [Self; 3] {
        std::array::from_fn(|i| {
            Self::new(
                self.x + (inc.x * ((i as isize) + 1)),
                self.y + (inc.y * ((i as isize) + 1)),
            )
        })
    }

    fn mas(&self, inc: Self) -> [Self; 2] {
        [
            Self::new(self.x + inc.x, self.y + inc.y),
            Self::new(self.x - inc.x, self.y - inc.y),
        ]
    }
}

fn main() {
    let input: &str = include_str!("inputs/day04.txt");
    let lines: Vec<&str> = input.lines().collect();

    let mut x_set: HashSet<Coord> = HashSet::new();
    let mut m_set: HashSet<Coord> = HashSet::new();
    let mut a_set: HashSet<Coord> = HashSet::new();
    let mut s_set: HashSet<Coord> = HashSet::new();

    lines.iter().enumerate().for_each(|(x, line)| {
        line.char_indices().for_each(|(y, char)| {
            match char {
                'X' => x_set.insert(Coord::new(x as isize, y as isize)),
                'M' => m_set.insert(Coord::new(x as isize, y as isize)),
                'A' => a_set.insert(Coord::new(x as isize, y as isize)),
                'S' => s_set.insert(Coord::new(x as isize, y as isize)),
                _ => false,
            };
        });
    });

    let part1 = part1(x_set, &m_set, &a_set, &s_set);
    println!("Part 1: {}", part1);

    let part2 = part2(m_set, a_set, s_set);
    println!("Part 2: {}", part2);
}

fn part1(
    x_set: HashSet<Coord>,
    m_set: &HashSet<Coord>,
    a_set: &HashSet<Coord>,
    s_set: &HashSet<Coord>,
) -> usize {
    let mut directions: Vec<Coord> = Vec::new();
    for x in -1..=1 {
        for y in -1..=1 {
            if !(x == 0 && y == 0) {
                directions.push(Coord::new(x, y));
            }
        }
    }

    let mut count = 0;
    for x in x_set {
        for direction in &directions {
            let xmas = x.xmas(*direction);
            if m_set.contains(&xmas[0]) && a_set.contains(&xmas[1]) && s_set.contains(&xmas[2]) {
                count += 1;
            }
        }
    }
    count
}

fn part2(m_set: HashSet<Coord>, a_set: HashSet<Coord>, s_set: HashSet<Coord>) -> usize {
    let directions: Vec<Coord> = vec![
        Coord::new(-1, -1),
        Coord::new(-1, 1),
        Coord::new(1, -1),
        Coord::new(1, 1),
    ];

    let mut count = 0;
    for a in a_set {
        let mut mas_x_count = 0;
        for direction in &directions {
            let mas = a.mas(*direction);
            if m_set.contains(&mas[0]) && s_set.contains(&mas[1]) {
                mas_x_count += 1;
            }
            if mas_x_count == 2 {
                count += 1;
                break;
            }
        }
    }
    count
}
