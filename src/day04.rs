struct Passport {
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn from(info: &String) -> Passport {
        let mut pass = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };
        for field in info.split_whitespace() {
            let key = &field[..3];
            let value = &field[4..];
            match key {
                "byr" => pass.byr = value.parse::<i32>().ok(),
                "iyr" => pass.iyr = value.parse::<i32>().ok(),
                "eyr" => pass.eyr = value.parse::<i32>().ok(),
                "hgt" => pass.hgt = Some(value.to_string()),
                "hcl" => pass.hcl = Some(value.to_string()),
                "ecl" => pass.ecl = Some(value.to_string()),
                "pid" => pass.pid = Some(value.to_string()),
                "cid" => pass.cid = Some(value.to_string()),
                _ => {continue;}
            }
        }
        pass
    }

    fn is_byr_valid(&self) -> bool {
        let byr = self.byr.unwrap();
        1920 <= byr && byr <= 2002
    }

    fn is_iyr_valid(&self) -> bool {
        let iyr = self.iyr.unwrap();
        2010 <= iyr && iyr <= 2020
    }

    fn is_eyr_valid(&self) -> bool {
        let eyr = self.eyr.unwrap();
        2020 <= eyr && eyr <= 2030
    }

    fn is_hgt_valid(&self) -> bool {
        let hgt = self.hgt.as_ref().unwrap();
        let mut id = 0;
        while hgt.chars().nth(id).unwrap_or(' ').is_digit(10) {id += 1}
        let num = hgt[..id].parse::<i32>().unwrap_or(0);
        (hgt.len() == 5 && hgt.contains("cm") && 150 <= num && num <= 193) ||
            (hgt.len() == 4 && hgt.contains("in") && 59 <= num && num <= 76)
    }

    fn is_hcl_valid(&self) -> bool {
        let hcl = self.hcl.as_ref().unwrap();
        &hcl[..1] == "#" && hcl[1..].chars().all(|c| c.is_digit(16))
    }

    fn is_ecl_valid(&self) -> bool {
        let ecl = self.ecl.as_ref().unwrap();
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str())
    }

    fn is_pid_valid(&self) -> bool {
        let pid = self.pid.as_ref().unwrap();
        pid.len() == 9 && pid.chars().all(|c| c.is_digit(10))
    }

    fn is_valid(&self) -> bool {
        (self.byr != None) &&
            (self.iyr != None) &&
            (self.eyr != None) &&
            (self.hgt != None) &&
            (self.hcl != None) &&
            (self.ecl != None) &&
            (self.pid != None)
    }

    fn has_valid_fields(&self) -> bool {
        self.is_valid() &&
            self.is_byr_valid() &&
            self.is_iyr_valid() &&
            self.is_eyr_valid() &&
            self.is_hgt_valid() &&
            self.is_hcl_valid() &&
            self.is_ecl_valid() &&
            self.is_pid_valid()
    }
}

pub fn run<R>(input: R) -> (String, String)
where
    R: std::io::BufRead,
{
    let mut pass_info = String::new();
    let mut passports: Vec<Passport> = Vec::new();
    for line in input.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            let pass = Passport::from(&pass_info);
            passports.push(pass);
            pass_info = String::new();
        } else {
            if pass_info.is_empty() {
                pass_info += &line;
            } else {
                pass_info = pass_info + " " + &line;
            }
        }
    }
    let ans1 = part_one(&passports);
    let ans2 = part_two(&passports);
    (ans1, ans2)
}

fn part_one(input: &Vec<Passport>) -> String {
    let mut res = 0;
    for p in input.iter() {
        res += if p.is_valid() {
            1
        } else {0};
    }
    res.to_string()
}

fn part_two(input: &Vec<Passport>) -> String {
    let mut res = 0;
    for p in input.iter() {
        res += if p.has_valid_fields() {
            1
        } else {0};
    }
    res.to_string()
}
