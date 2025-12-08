#[derive(Clone, Debug)]
struct Box {
    x: u64,
    y: u64,
    z: u64,
}

impl From<&str> for Box {
    fn from(value: &str) -> Self {
        let mut coords = value.split(',').map(|coord| coord.parse().unwrap());

        Box {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap(),
        }
    }
}

impl Box {
    fn distance(&self, other: &Box) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

#[allow(unused)]
pub fn part1(input: &str) -> usize {
    let now = std::time::Instant::now();
    let boxes: Vec<Box> = input.lines().map(Box::from).collect();

    let mut pairs: Vec<(usize, usize)> = Vec::with_capacity(500_000);

    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            pairs.push((i, j));
        }
    }

    pairs.sort_unstable_by_key(|(i, j)| boxes[*i].distance(&boxes[*j]));

    let mut circuits: Vec<Vec<usize>> = Vec::with_capacity(500);

    for (i, j) in pairs.iter().take(1000) {
        let i_circuit = circuits.iter().position(|circuit| circuit.contains(i));
        let j_circuit = circuits.iter().position(|circuit| circuit.contains(j));

        match (i_circuit, j_circuit) {
            (None, None) => circuits.push(vec![*i, *j]),
            (None, Some(j_circuit_idx)) => circuits[j_circuit_idx].push(*i),
            (Some(i_circuit_idx), None) => circuits[i_circuit_idx].push(*j),
            (Some(i_circuit_idx), Some(j_circuit_idx)) => {
                if i_circuit_idx == j_circuit_idx {
                    continue;
                }

                let j_circuit: Vec<_> = circuits[j_circuit_idx].drain(..).collect();
                circuits[i_circuit_idx].extend(j_circuit);
            }
        }
    }

    circuits.sort_unstable_by_key(Vec::len);
    circuits.reverse();

    circuits.iter().take(3).map(Vec::len).product()
}

#[allow(unused)]
pub fn part2(input: &str) -> u64 {
    let boxes: Vec<Box> = input.lines().map(Box::from).collect();

    let mut pairs: Vec<(usize, usize)> = Vec::with_capacity(boxes.len() * (boxes.len() - 1) / 2);

    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            pairs.push((i, j));
        }
    }

    pairs.sort_unstable_by_key(|(i, j)| boxes[*i].distance(&boxes[*j]));

    let mut circuits: Vec<Vec<usize>> = Vec::new();
    let mut res = 0;

    for (i, j) in &pairs {
        let i_circuit = circuits.iter().position(|circuit| circuit.contains(i));
        let j_circuit = circuits.iter().position(|circuit| circuit.contains(j));

        match (i_circuit, j_circuit) {
            (None, None) => circuits.push(vec![*i, *j]),
            (None, Some(j_circuit_idx)) => circuits[j_circuit_idx].push(*i),
            (Some(i_circuit_idx), None) => circuits[i_circuit_idx].push(*j),
            (Some(i_circuit_idx), Some(j_circuit_idx)) => {
                if i_circuit_idx == j_circuit_idx {
                    continue;
                }

                let j_circuit: Vec<_> = circuits[j_circuit_idx].drain(..).collect();
                circuits[i_circuit_idx].extend(j_circuit);
            }
        }

        if circuits.iter().map(Vec::len).max() == Some(1000) {
            res = boxes[*i].x * boxes[*j].x;
        }
    }

    res
}
