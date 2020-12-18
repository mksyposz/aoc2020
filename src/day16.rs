const TICKETFIELDS: i32 = 0;
const MYTICKET_NAME: i32 = 1;
const MYTICKET_VALUES: i32 = 2;
const NEARBYTICKETS_NAME: i32 = 3;
const NEARBYTICKETS_VALUES: i32 = 4;

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let mut input_part = TICKETFIELDS;
    let mut tf = TicketFields{ranges:HashMap::new()};
    let mut my_ticket: Vec<i64> = Vec::new();
    let mut nearby_tickets: Vec<Vec<i64>> = Vec::new();
    for line in input.lines() {
        let line = line.unwrap();
        match input_part {
            TICKETFIELDS => {
                if line.is_empty() {
                    input_part = MYTICKET_NAME;
                } else {
                    tf.add_from_line(&line);
                }
            },
            MYTICKET_NAME => {
                input_part = MYTICKET_VALUES;
            },
            MYTICKET_VALUES => {
                if line.is_empty() {
                    input_part = NEARBYTICKETS_NAME;
                } else {
                    my_ticket = line_values(&line);
                }
            },
            NEARBYTICKETS_NAME => {
                input_part = NEARBYTICKETS_VALUES;
            },
            NEARBYTICKETS_VALUES => {
                nearby_tickets.push(line_values(&line));
            },
            _ => unreachable!(),
        }
    }
    let ans1 = part_one(&tf, &mut nearby_tickets);
    let nearby_tickets = nearby_tickets.into_iter()
                                       .filter(|x| x.iter().all(|&x| !tf.completely_invalid(x)))
                                       .collect::<Vec<Vec<i64>>>();
    let ans2 = part_two(&tf, &my_ticket, &nearby_tickets);
    (ans1, ans2)
}

fn line_values(s: &String) -> Vec<i64> {
    s.split(",")
     .map(|x| x.parse::<i64>().unwrap())
     .collect()
}

fn part_one(tf: &TicketFields, nts: &mut Vec<Vec<i64>>) -> String {
    nts.iter()
       .map(|x| x.iter().filter(|&&x| tf.completely_invalid(x)).sum::<i64>())
       .sum::<i64>().to_string()
}

fn part_two(tf: &TicketFields, mt: &Vec<i64>, nts: &Vec<Vec<i64>>) -> String {
    let fc = tf.ranges.len();
    let mut g: HashMap<String, Vec<usize>> = HashMap::new();
    for (k, v) in tf.ranges.iter() {
        let possible_fields = (0..fc).filter(|&i| v.iter().any(|&(s,b)| s <= mt[i] && mt[i] <= b) &&
                                             nts.iter().all(|x| v.iter().any(|&(s,b)| s <= x[i] && x[i] <= b)))
                                     .collect::<Vec<usize>>();
        g.insert(k.clone(), possible_fields);
    }
    let mut used: Vec<usize> = Vec::new();
    loop {
        let used_count = used.len();
        used = g.values().filter(|x| x.len() == 1).map(|x| x[0]).collect::<Vec<usize>>();
        if used.len() == used_count {
            break;
        }
        g.iter_mut()
         .for_each(|(_,val)| if val.len() != 1 {
             *val = val
                 .into_iter()
                 .filter(|x| used.iter().all(|y| x != &y))
                 .map(|x| *x as usize)
                 .collect::<Vec<usize>>()
         });
    }
    g.iter()
     .filter(|&(k, _)| k.starts_with("departure"))
     .map(|(_, v)| mt[v[0]])
     .product::<i64>()
        .to_string()
}

use std::collections::HashMap;
struct TicketFields {
    ranges: HashMap<String, Vec<(i64, i64)>>,
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn process_str_range(s: &String) -> (i64, i64){
    let r = s.split("-").collect::<Vec<&str>>();
    let r1 = r[0].parse::<i64>().unwrap();
    let r2 = r[1].parse::<i64>().unwrap();
    (r1,r2)
}

impl TicketFields {
    fn add_from_line(&mut self, line: &String) {
        let s = line.split(":").collect::<Vec<&str>>();
        let field_name = s[0].to_string();
        let ranges = s[1].split("or").collect::<Vec<&str>>();
        let ranges = ranges.iter()
                           .map(|x| process_str_range(&remove_whitespace(x)))
                           .collect::<Vec<(i64, i64)>>();
        self.ranges.insert(field_name, ranges);
    }

    fn completely_invalid(&self, value: i64) -> bool {
        !self.ranges
             .values()
             .any(|x| x.iter().any(|&(s,b)| s <= value && value <= b ))
    }
}
