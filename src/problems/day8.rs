#[derive(Clone, Debug)]
struct Position {
    x: u64,
    y: u64,
    z: u64,
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let mut coords = value.split(',').map(|coord| coord.parse().unwrap());

        Position {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap(),
        }
    }
}

impl Position {
    fn distance(&self, other: &Position) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

#[allow(unused)]
pub fn part1(input: &str) -> usize {
    let boxes: Vec<Position> = input.lines().map(Position::from).collect();

    // (start_idx, end_idx, distance)
    let mut pairs: Vec<(usize, usize, u64)> = get_closest_pairs::<1000>(&boxes);

    let mut circuits = Dsu::new(1000);

    for (i, j, _) in pairs {
        circuits.add_pair(i, j);
    }

    circuits.nodes.sort_by_key(|node| node.size);

    circuits
        .nodes
        .iter()
        .rev()
        .take(3)
        .map(|circuit| circuit.size)
        .product()
}

#[allow(unused)]
pub fn part2(input: &str) -> u64 {
    let now = std::time::Instant::now();
    let boxes: Vec<Position> = input.lines().map(Position::from).collect();

    // (start_idx, end_idx, distance)
    let mut pairs: Vec<(usize, usize, u64)> = get_closest_pairs::<5_000>(&boxes);

    let mut circuits = Dsu::new(1000);

    for (i, j, _) in pairs {
        if circuits.add_pair(i, j) == 1000 {
            return boxes[i].x * boxes[j].x;
        }
    }

    unreachable!()
}

struct Node {
    parent: usize,
    size: usize,
}

struct Dsu {
    nodes: Vec<Node>,
}

impl Dsu {
    fn new(len: usize) -> Self {
        Self {
            nodes: (0..len).map(|parent| Node { parent, size: 1 }).collect(),
        }
    }

    fn parent(&mut self, mut x: usize) -> usize {
        loop {
            // get parent
            let parent = self.nodes[x].parent;
            // reached top most parent
            if parent == x {
                break parent;
            }
            // update parent
            self.nodes[x].parent = self.nodes[parent].parent;
            // update x
            x = parent;
        }
    }

    fn add_pair(&mut self, u: usize, v: usize) -> usize {
        let (mut pu, mut pv) = (self.parent(u), self.parent(v));

        // same parent => same subset
        if pu == pv {
            return self.nodes[pu].size;
        }

        // make sure pu is bigger than pv
        if self.nodes[pu].size < self.nodes[pv].size {
            std::mem::swap(&mut pu, &mut pv);
        }

        self.nodes[pv].parent = pu;
        self.nodes[pu].size += self.nodes[pv].size;
        self.nodes[pu].size
    }
}

fn get_closest_pairs<const N: usize>(boxes: &[Position]) -> Vec<(usize, usize, u64)> {
    // (start_idx, end_idx, distance)
    let mut pairs: Vec<(usize, usize, u64)> = Vec::with_capacity(N);

    let mut iter = (0..boxes.len()).flat_map(|i| (i + 1..boxes.len()).map(move |j| (i, j)));

    for _ in 0..N {
        let (i, j) = iter.next().unwrap();
        let d = boxes[i].distance(&boxes[j]);
        pairs.push((i, j, d));
    }

    pairs.sort_unstable_by_key(|(_, _, d)| *d);

    for (i, j) in iter {
        let d = boxes[i].distance(&boxes[j]);

        if d > pairs.last().unwrap().2 {
            continue;
        }

        pairs.pop();
        let insert_idx = pairs.partition_point(|(_, _, dist)| dist < &d);
        pairs.insert(insert_idx, (i, j, d));
    }

    pairs
}
