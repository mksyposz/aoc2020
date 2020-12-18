pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let expressions: Vec<String> = input.lines().map(|s| s.unwrap()).collect();
    let ans1 = part_one(&expressions);
    let ans2 = part_two(&expressions);
    (ans1, ans2)
}

fn part_one(exps: &Vec<String>) -> String {
    exps.iter()
        .map(|s| Expression::from_str(s.as_str()).simple_eval())
        .sum::<i64>()
        .to_string()
}

fn part_two(exps: &Vec<String>) -> String {
    exps.iter()
        .map(|s| Expression::from_str(s.as_str()).advanced_eval())
        .sum::<i64>()
        .to_string()
}

struct Expression<'a> {
    value: Option<i64>,
    s: &'a str,
}

impl <'a>Expression<'a> {

    fn from_str(s: &'a str) -> Self {
        Self {
            value: None,
            s,
        }
    }

    fn eval_with_policy(&mut self, policy: fn(&str) -> Result<(char,usize),()>) -> i64 {
        if self.value == None {
            match policy(self.s) {
                Ok((c, i)) => {
                    let mut left = Expression::from_str(&self.s[..(i-1)]);
                    let mut right = Expression::from_str(&self.s[(i+2)..]);
                    match c {
                        '*' => {
                            self.value = Some(left.eval_with_policy(policy) * right.eval_with_policy(policy));
                        },
                        '+' => {
                            self.value = Some(left.eval_with_policy(policy) + right.eval_with_policy(policy));
                        },
                        _ => unreachable!(),
                    }
                },
                Err(_) => {
                    match self.s.parse::<i64>() {
                        Ok(v) => self.value = Some(v),
                        Err(_) => {
                            let len = self.s.len();
                            self.value = Some(Expression::from_str(&self.s[1..(len-1)]).eval_with_policy(policy));
                        },
                    }
                },
            }
        }
        return self.value.unwrap();
    }

    fn simple_left_right(s: &str) -> Result<(char, usize),()> {
        let mut open_paren = 0;
        let mut res: Result<(char, usize), ()> = Err(());
        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => open_paren+=1,
                ')' => open_paren-=1,
                '+' => if open_paren == 0 {res = Ok(('+', i))},
                '*' => if open_paren == 0 {res = Ok(('*', i))},
                _ => {},
            }
        }
        res
    }

    fn simple_eval(&mut self) -> i64 {
        self.eval_with_policy(Expression::simple_left_right)
    }

    fn advanced_plus_mult(s: &str) -> Result<(char, usize),()> {
        let mut open_paren = 0;
        let mut res: Result<(char, usize), ()> = Err(());
        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => open_paren+=1,
                ')' => open_paren-=1,
                '+' => if open_paren == 0 {
                    if let Ok((c,_)) = res {
                        if c == '+' {
                            res = Ok(('+', i))
                        }
                    } else {
                        res = Ok(('+', i));
                    }
                },
                '*' => if open_paren == 0 {res = Ok(('*', i))},
                _ => {},
            }
        }
        res
    }

    fn advanced_eval(&mut self) -> i64 {
        self.eval_with_policy(Expression::advanced_plus_mult)
    }
}

#[test]
fn test1() {
    let s = "2 * 3 + (4 * 5)";
    assert_eq!(Expression::from_str(s).simple_eval(), 26);
}

#[test]
fn test2() {
    let s = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    assert_eq!(Expression::from_str(s).simple_eval(), 437);
}

#[test]
fn test3() {
    let s = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    assert_eq!(Expression::from_str(s).simple_eval(), 12240);
}

#[test]
fn test4() {
    let s = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(Expression::from_str(s).simple_eval(), 13632);
}

#[test]
fn test5() {
    let s = "(9 + 4)";
    assert_eq!(Expression::from_str(s).simple_eval(), 13);
}

#[test]
fn test6() {
    let s = "1 + (2 * 3) + (4 * (5 + 6))";
    assert_eq!(Expression::from_str(s).advanced_eval(), 51);
}

#[test]
fn test7() {
    let s = "2 * 3 + (4 * 5)";
    assert_eq!(Expression::from_str(s).advanced_eval(), 46);
}

#[test]
fn test8() {
    let s = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    assert_eq!(Expression::from_str(s).advanced_eval(), 1445);
}

#[test]
fn test9() {
    let s = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    assert_eq!(Expression::from_str(s).advanced_eval(), 669060);
}

#[test]
fn test10() {
    let s = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(Expression::from_str(s).advanced_eval(), 23340);
}
