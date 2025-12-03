#[derive(Clone, Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => panic!("No direction matching: {value}"),
        }
    }
}

#[derive(Clone, Debug)]
struct Move {
    direction: Direction,
    amount: i32,
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let (direction, amount) = value.split_at(1);
        let Some(direction) = direction.chars().next() else {
            panic!("can't turn empty string into Move");
        };
        let direction = Direction::from(direction);
        let Ok(amount) = amount.parse() else {
            panic!("failed to parse amount from {amount}");
        };

        Self { direction, amount }
    }
}

#[derive(Debug)]
struct Dial {
    pointed_number: i32,
}

impl Dial {
    fn new(pointed_number: i32) -> Self {
        Self { pointed_number }
    }

    // returns the number of times the dial points at 0 during the move
    fn apply_move(&mut self, m: &Move) -> usize {
        let prev = self.pointed_number;
        match m.direction {
            Direction::Left => self.pointed_number -= m.amount,
            Direction::Right => self.pointed_number += m.amount,
        }

        let mut zero_passes = (self.pointed_number / 100).abs();

        if self.pointed_number == 0 {
            zero_passes += 1;
        }

        if self.pointed_number < 0 && prev != 0 {
            zero_passes += 1;
        }

        self.pointed_number = self.pointed_number.rem_euclid(100);

        zero_passes as usize
    }
}

#[allow(unused)]
pub fn part1(input: &str) -> usize {
    let moves = input.lines().map(Move::from);

    let mut dial = Dial::new(50);
    let mut count = 0;

    for m in moves {
        dial.apply_move(&m);
        if dial.pointed_number == 0 {
            count += 1;
        }
    }

    count
}

#[allow(unused)]
pub fn part2(input: &str) -> usize {
    let moves = input.lines().map(Move::from);

    let mut dial = Dial::new(50);

    let mut count = 0;

    for m in moves {
        count += dial.apply_move(&m);
    }

    count
}
