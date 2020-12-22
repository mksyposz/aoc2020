const PLAYER1: u32 = 0;
const PLAYER1_CARDS: u32 = 1;
const PLAYER2: u32 = 2;
const PLAYER2_CARDS: u32 = 3;
const END: u32 = 4;

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let mut input_part = PLAYER1;
    let mut d1 = Deck::new();
    let mut d2 = Deck::new();
    for line in input.lines() {
        let line = line.unwrap();
        match input_part {
            PLAYER1 => input_part = PLAYER1_CARDS,
            PLAYER1_CARDS => {
                if line.is_empty() {input_part = PLAYER2}
                else {d1.deck.push(line.parse::<u64>().unwrap())}
            },
            PLAYER2 => input_part = PLAYER2_CARDS,
            PLAYER2_CARDS => {
                if line.is_empty() {input_part = END}
                else {d2.deck.push(line.parse::<u64>().unwrap())}
            },
            _ => unreachable!(),
        }
    }
    let ans1 = part_one(d1.clone(), d2.clone());
    let ans2 = part_two(d1.clone(), d2.clone());
    (ans1, ans2)
}

fn part_one(d1: Deck, d2: Deck) -> String {
    play_combat(d1,d2).1.count_score().to_string()
}

fn part_two(d1: Deck, d2: Deck) -> String {
    play_recursive_combat(d1,d2).1.count_score().to_string()
}

fn play_combat(mut d1: Deck, mut d2: Deck) -> (u32,Deck) {
    while !d1.is_empty() && !d2.is_empty() {
        let t1 = d1.top_card();
        let t2 = d2.top_card();
        if t1 > t2 {
            d1.deck.push(t1);
            d1.deck.push(t2);
        } else if t1 < t2 {
            d2.deck.push(t2);
            d2.deck.push(t1);
        } else {
            unreachable!()
        }
    }
    if d1.is_empty() {(2,d2)} else {(1,d1)}
}

use std::collections::hash_map::DefaultHasher;
fn hash_cards(d1: &Deck, d2: &Deck) -> u64 {
    use std::hash::Hash;
    use std::hash::Hasher;
    let mut hasher = DefaultHasher::new();
    d1.deck.hash(&mut hasher);
    d2.deck.hash(&mut hasher);
    hasher.finish()
}

fn play_recursive_combat(mut d1: Deck, mut d2: Deck) -> (u32, Deck) {
    use std::collections::HashSet;
    let mut history = HashSet::new();
    while !d1.is_empty() && !d2.is_empty() {
        let state = hash_cards(&d1, &d2);
        if !history.insert(state) {
            return (1, d1);
        }
        let c1 = d1.top_card();
        let c2 = d2.top_card();
        let winner =
            if c1 <= (d1.len() as u64) && c2 <= (d2.len() as u64) {
                play_recursive_combat(d1.make_copy(c1),d2.make_copy(c2)).0
            } else {0};
        if  (winner == 0 && c1 > c2) || winner == 1 {
            d1.deck.push(c1);
            d1.deck.push(c2);
        } else if (winner == 0 && c2 > c1 ) || winner == 2 {
            d2.deck.push(c2);
            d2.deck.push(c1);
        } else {
            unreachable!()
        }
    }
    if d1.is_empty() {(2, d2)} else {(1, d1)}
}

#[derive(Clone)]
struct Deck {
    deck: collections::Queue<u64>
}

impl Deck {
    fn new() -> Self {
        Self {
            deck: collections::Queue::new(),
        }
    }
    fn count_score(&self) -> u64 {
        self.deck.iter().fold((0,self.deck.len() as u64), |acc, x|
        (acc.0 + acc.1*x, acc.1-1)).0
    }

    fn is_empty(&self) -> bool {
        self.deck.len() == 0
    }

    fn top_card(&mut self) -> u64 {
        self.deck.pop().unwrap()
    }

    fn len(&self) -> usize {
        self.deck.len()
    }

    fn make_copy(&self, len: u64) -> Self {
        Self {
            deck: self.deck.iter()
                           .take(len as usize)
                           .map(|x| *x)
                           .collect::<collections::Queue<u64>>(),
        }
    }
}

mod collections {
    //vecdeque is faster than linkedlist 317ms vs 476ms
    use std::collections::VecDeque;
    use std::collections::vec_deque::Iter;
    #[derive(Clone, Hash)]
    pub struct Queue<T> {
        queue: VecDeque<T>,
    }

    impl<T> Queue<T> {
        pub fn new() -> Self {
            Self {
                queue: VecDeque::<T>::new()
            }
        }

        pub fn push(&mut self, el: T) {
            self.queue.push_back(el);
        }

        pub fn pop(&mut self) -> Option<T> {
            self.queue.pop_front()
        }

        pub fn len(&self) -> usize {
            self.queue.len()
        }

        pub fn iter(&self) -> Iter<'_,T> {
            self.queue.iter()
        }
    }

    impl<T> std::iter::FromIterator<T> for Queue<T> {
        fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
            let mut q = Self::new();
            for i in iter {
                q.push(i);
            }
            q
        }
    }
}
