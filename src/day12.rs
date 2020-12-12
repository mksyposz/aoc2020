enum Action {
    North(i64),
    South(i64),
    West(i64),
    East(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

struct Ship {
    pos: (i64, i64),
    direction: i64
}

impl Ship {
    const DIR: [(i64, i64); 4] = [(1, 0), (0, -1), (-1, 0), (0,1)];
    const EAST: usize = 0;
    const SOUTH: usize = 1;
    const WEST: usize = 2;
    const NORTH: usize = 3;

    fn new() -> Ship {
        Ship {
            pos: (0,0),
            direction: Ship::EAST as i64
        }
    }

    fn exec(&mut self, action: &Action) {
        match action {
            Action::North(v) => self.pos.1 += v*Ship::DIR[Ship::NORTH].1,
            Action::South(v) => self.pos.1 += v*Ship::DIR[Ship::SOUTH].1,
            Action::West(v) => self.pos.0 += v*Ship::DIR[Ship::WEST].0,
            Action::East(v) => self.pos.0 += v*Ship::DIR[Ship::EAST].0,
            Action::Left(v) => self.direction = ((self.direction - v)+4)%4,
            Action::Right(v) => self.direction = ((self.direction + v)+4)%4,
            Action::Forward(v) => {
                self.pos.0 += v*Ship::DIR[self.direction as usize].0;
                self.pos.1 += v*Ship::DIR[self.direction as usize].1;
            },
        };
    }

    fn manhattan_dist(&self) -> i64 {
        (self.pos.0).abs() + (self.pos.1).abs()
    }
}

struct Waypoint<'a> {
    pos: (i64, i64),
    ship: &'a mut Ship,
}

impl<'a> Waypoint<'a> {
    fn new(ship: &'a mut Ship) -> Waypoint {
        Waypoint {
            pos: (ship.pos.0 + 10, ship.pos.1+1 ),
            ship,
        }
    }
    fn exec(&mut self, action: &Action) {
        match action {
            Action::North(v) => self.pos.1 += v*Ship::DIR[Ship::NORTH].1,
            Action::South(v) => self.pos.1 += v*Ship::DIR[Ship::SOUTH].1,
            Action::West(v) => self.pos.0 += v*Ship::DIR[Ship::WEST].0,
            Action::East(v) => self.pos.0 += v*Ship::DIR[Ship::EAST].0,
            Action::Left(v) => {
                let mut counter = v.clone();
                while counter > 0 {
                    self.pos = (-self.pos.1, self.pos.0);
                    counter = counter -1;
                }
            },
            Action::Right(v) => {
                let mut counter = v.clone();
                while counter > 0 {
                    self.pos = (self.pos.1, -self.pos.0);
                    counter = counter -1;
                }
            },
            Action::Forward(v) => {
                if self.pos.0 > 0 {
                    self.ship.exec(&Action::East(v*self.pos.0));
                } else if self.pos.0 < 0 {
                    self.ship.exec(&Action::West((-v)*self.pos.0));
                }
                if self.pos.1 > 0 {
                    self.ship.exec(&Action::North(v*self.pos.1));
                } else if self.pos.1 < 0 {
                    self.ship.exec(&Action::South((-v)*self.pos.1));
                }
            },
        }
    }
}

fn parse_line(s: &String) -> Action {
    let v = s[1..].parse::<i64>().unwrap();
    match &s[..1] {
        "N" => Action::North(v),
        "S" => Action::South(v),
        "E" => Action::East(v),
        "W" => Action::West(v),
        "L" => Action::Left(v/90),
        "R" => Action::Right(v/90),
        "F" => Action::Forward(v),
        _ => unreachable!(),
    }
}

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let actions: Vec<Action> = input.lines()
                                    .map(|x| x.unwrap())
                                    .map(|x| parse_line(&x))
                                    .collect();
    let ans1 = part_one(&actions);
    let ans2 = part_two(&actions);
    (ans1, ans2)
}

fn part_one(actions: &Vec<Action>) -> String {
    let mut ship = Ship::new();
    actions.iter().for_each(|a| ship.exec(a));
    ship.manhattan_dist().to_string()
}

fn part_two(actions: &Vec<Action>) -> String {
    let mut ship = Ship::new();
    let mut waypoint = Waypoint::new(&mut ship);
    actions.iter().for_each(|a| waypoint.exec(a));
    ship.manhattan_dist().to_string()
}
