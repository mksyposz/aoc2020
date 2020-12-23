pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let input = input.lines().next().unwrap().unwrap();
    let initial_order = input.chars().map(|x| x.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();
    let now = std::time::Instant::now();
    let ans1 = part_one(&initial_order);
    println!("{}", now.elapsed().as_millis());
    let now = std::time::Instant::now();
    let ans2 = part_two(&initial_order);
    println!("{}", now.elapsed().as_millis());
    (ans1, ans2)
}

fn part_one(io: &Vec<usize>) -> String {
    let mut dll = CrabDLinkedList::from_vec(io, 9);
    play_crab_cups(&mut dll, 100);
    dll.after_one()
}

fn part_two(io: &Vec<usize>) -> String {
    let mut dll = CrabDLinkedList::from_vec(io, 1000000);
    play_crab_cups(&mut dll, 10000000);
    dll.mult_after_one()
}


fn play_crab_cups(cups: &mut CrabDLinkedList, rounds: usize) {
    for _ in 0..rounds {
        cups.make_three_move();
        cups.go_next_leader();
    }
}

const ARR_SIZE: usize = 1000001;
#[derive(Debug)]
struct CrabDLinkedList { //double linked list
    arr: [usize; ARR_SIZE],
    size: usize,
    current: usize,
}

impl CrabDLinkedList {
    fn from_vec(list: &Vec<usize>, size: usize) -> Self {
        let mut dll = Self{arr: [0; ARR_SIZE], size, current: list[0] };
        let mut prev = list[0];
        for v in list.iter().skip(1) {
            dll.arr[prev] = *v;
            prev = (*v) as usize;
        }
        for i in 10..=size {
            dll.arr[prev] = i;
            prev = i
        }
        dll.arr[prev] = list[0];
        dll
    }

    fn go_next_leader(&mut self) {
        self.current = self.arr[self.current];
    }

    fn make_three_move(&mut self) {
        let leader = self.current;
        let f = self.arr[leader as usize];
        let s = self.arr[f as usize];
        let t = self.arr[s as usize];
        let next = self.arr[t as usize];
        let dest = CrabDLinkedList::get_insert_pos(self.size, leader, (f,s,t));
        let dest_next = self.arr[dest as usize];
        self.arr[leader as usize] = next;
        self.arr[dest as usize] = f;
        self.arr[t as usize] = dest_next;
    }

    fn get_insert_pos(size: usize, leader: usize, (f,s,t): (usize, usize, usize)) -> usize {
        let mut c = leader;
        loop {
            c = c - 1;
            if c == 0 {c = size;}
            if c != f && c != s && c != t {
                return c;
            }
        }
    }

    fn after_one(&self) -> String {
        let mut result = String::new();
        let mut pos = 1;
        loop {
            if self.arr[pos] == 1 {break;}
            if self.arr[pos] != 0 {
                result += &self.arr[pos].to_string();
            }
            pos = self.arr[pos] as usize;
        }
        return result;
    }

    fn mult_after_one(&self) -> String {
        let f = self.arr[1] as u64;
        let s = self.arr[f as usize] as u64;
        return (f*s).to_string();
    }
}
