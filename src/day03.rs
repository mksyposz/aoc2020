struct Slope {
    down: usize,
    right: usize,
}

impl Slope {
    fn count_trees(&self, hill: &Vec<String>) -> usize {
        let mut result = 0;
        let mut x = 0usize;
        let mut y = 0usize;
        let height = hill.len();
        let width = hill[0].len();
        while y < height {
            result = result +
            if hill[y].chars().nth(x).unwrap() == '#' {
                1
            } else {
                0
            };
            y += self.down;
            x = (x + self.right)%width;
        }
        result
    }
}

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let mut hill : Vec<String> = Vec::new();
    for line in input.lines() {
        let line = line.unwrap();
        hill.push(line);
    }
    let ans1 = part_one(&hill);
    let ans2 = part_two(&hill);
    (ans1, ans2)
}

fn part_one(input: &Vec<String>) -> String {
    let slope1 = Slope{down: 1, right: 3};
    return slope1.count_trees(&input).to_string();
}

fn part_two(input: &Vec<String>) -> String {
    let slope1 = Slope{down: 1, right: 1};
    let slope2 = Slope{down: 1, right: 3};
    let slope3 = Slope{down: 1, right: 5};
    let slope4 = Slope{down: 1, right: 7};
    let slope5 = Slope{down: 2, right: 1};
    return (slope1.count_trees(&input) *
            slope2.count_trees(&input) *
            slope3.count_trees(&input) *
            slope4.count_trees(&input) *
            slope5.count_trees(&input)).to_string()
}
