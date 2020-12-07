use std::collections::HashMap;
use std::collections::HashSet;

fn node_process(s: &str) -> String {
    let s: Vec<&str> = s.split_whitespace().collect();
    return s[0].to_string() + &s[1].to_string();
}

fn son_process(s: &str) -> String {
    let s: Vec<&str> = s.split_whitespace().collect();
    return s[1].to_string() + &s[2].to_string();
}

fn son_process_with_counter(s: &str) -> (i32, String) {
    let s: Vec<&str> = s.split_whitespace().collect();
    return (s[0].parse().unwrap_or(0), s[1].to_string() + &s[2].to_string())
}

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let requirements: Vec<String> = input.lines().map(|x| x.unwrap()).collect();
    let ans1 = part_one(&requirements);
    let ans2 = part_two(&requirements);
    (ans1, ans2)
}

fn part_one(req: &Vec<String>) -> String {
    let mut g: HashMap<String, String> = HashMap::new();
    for r in req.iter() {
        let edges: Vec<&str> = r.split("contain").collect();
        let main_node = node_process(edges[0]);
        let sons: Vec<&str> = edges[1].split(",").collect();
        for s in sons.iter() {
            let son = son_process(s);
            let current_edges = g.get_mut(&son);
            if let Some(e) = current_edges {
                e.push_str(":");
                e.push_str(&main_node);
            } else {
                g.insert(son, main_node.clone());
            }
        }
    }
    let mut s: HashSet<String> = HashSet::new();
    dfs("shinygold".to_string(), &g, &mut s);
    s.len().to_string()
}

struct Edge {
    counter: u64,
    node: String
}

fn part_two(req: &Vec<String>) -> String {
    let mut g: HashMap<String, Vec<Edge>> = HashMap::new();
    for r in req.iter() {
        let edges: Vec<&str> = r.split("contain").collect();
        let main_node = node_process(edges[0]);
        let sons: Vec<&str> = edges[1].split(",").collect();
        for s in sons.iter() {
            let son = son_process_with_counter(s);
            let edge = Edge{counter: son.0 as u64, node: son.1};
            let current_edges = g.get_mut(&main_node);
            if let Some(ce) = current_edges {
                ce.push(edge);
            } else {
                g.insert(main_node.clone(), vec![edge]);
            }
        }
    }
    (dfs2("shinygold".to_string(), &g)-1).to_string()
}

fn dfs(node: String, g: &HashMap<String, String>, vis: &mut HashSet<String>) {
    vis.insert(node.clone());
    let sons = g.get(&node);
    if let Some(v) = sons {
        for s in v.split(":") {
            dfs(s.to_string(), g, vis);
        }
    }
}

fn dfs2(node: String, g: &HashMap<String, Vec<Edge>>) -> u64 {
    let mut res = 1;
    if let Some(edges) = g.get(&node) {
        for e in edges.iter() {
            res += e.counter * dfs2(e.node.clone(), g);
        }
    }
    res
}
