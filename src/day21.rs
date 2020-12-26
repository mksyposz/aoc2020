use std::collections::HashMap;
use std::collections::HashSet;

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let mut all_products = HashMap::new();
    let mut allergens = Allergens{hm:HashMap::new()};
    for line in input.lines() {
        let line = line.unwrap();
        let l = line.split('(').collect::<Vec<&str>>();
        let products =
            l[0][..l[0].len()-1].split_ascii_whitespace().collect::<Vec<&str>>();
        products.iter().for_each(|s| {let c = all_products.entry(s.to_string()).or_insert(0); *c += 1 });
        let s = l[1][9..l[1].len()-1].split(", ").collect::<Vec<&str>>();
        s.iter().for_each(|a| allergens.insert(a, &products));
    }
    let ans1 = part_one(&all_products, &allergens);
    let ans2 = part_two(&mut allergens);
    (ans1, ans2)
}

fn part_one(ap: &HashMap::<String,u32>, allergens: &Allergens) -> String {
    ap.iter().filter(|&(p,_)| allergens.hm.values().all(|v| v.iter().all(|s| p != s)))
             .map(|(_,x)| x).sum::<u32>().to_string()
}

fn part_two(allergens: &mut Allergens) -> String {
    let mut translated = HashSet::new();
    loop {
        if allergens.hm.values().all(|v| v.len() == 1) {
            break;
        }
        for value in allergens.hm.values_mut() {
            if value.len() != 1 {
                *value = value.iter()
                            .filter(|&v| !translated.contains(v))
                            .map(|v| v.to_string())
                            .collect();
            }
            if value.len() == 1 {
                translated.insert(value[0].clone());
            }
        }
    }
    let mut fun_vec = allergens.hm.iter().map(|(k,v)| (k.clone(),v[0].clone())).collect::<Vec<(String, String)>>();
    fun_vec.sort();
    let result = fun_vec.iter().map(|(_,p)| p.clone()).collect::<Vec<String>>().join(",");
    result
}

#[derive(Debug)]
struct Allergens {
    hm: HashMap<String, Vec<String>>,
}

impl Allergens {
    fn insert(&mut self, name: &str, products: &Vec<&str>) {
        match self.hm.get_mut(&name.to_string()) {
            Some(v) => {
                *v = products.iter().filter(|&p| v.iter().any(|s| s == p)).map(|p| p.to_string()).collect();
            },
            None => {
                self.hm.insert(name.to_string(), products.iter().map(|p| p.to_string()).collect());
            },
        }
    }
}
