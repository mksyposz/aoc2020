struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
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
                "byr" => {
                    pass.byr = Some(value.to_string());
                },
                "iyr" => {
                    pass.iyr = Some(value.to_string());
                },
                "eyr" => {
                    pass.eyr = Some(value.to_string());
                },
                "hgt" => {
                    pass.hgt = Some(value.to_string());
                },
                "hcl" => {
                    pass.hcl = Some(value.to_string());
                },
                "ecl" => {
                    pass.ecl = Some(value.to_string());
                },
                "pid" => {
                    pass.pid = Some(value.to_string());
                },
                "cid" => {
                    pass.cid = Some(value.to_string());
                }
                _ => {continue;}
            }
        }
        pass
    }

    fn is_byr_valid(&self) -> bool {
        let byr = self.byr.as_ref().unwrap();
        let byr = byr.parse::<i32>().unwrap_or(-1);
        if byr < 1920 || byr > 2002 {
            return false;
        }
        return true;
    }

    fn is_iyr_valid(&self) -> bool {
        let iyr = self.iyr.as_ref().unwrap();
        let iyr = iyr.parse::<i32>().unwrap_or(-1);
        if iyr < 2010 || iyr > 2020 {
            return false;
        }
        return true;
    }

    fn is_eyr_valid(&self) -> bool {
        let eyr = self.eyr.as_ref().unwrap();
        let eyr = eyr.parse::<i32>().unwrap_or(-1);
        if eyr < 2020 || eyr > 2030 {
            return false;
        }
        return true;
    }

    fn is_hgt_valid(&self) -> bool {
        let hgt = self.hgt.as_ref().unwrap();
        let hgt: String = hgt.chars().rev().collect();
        let measure_unit: String = hgt[..2].to_string().chars().rev().collect();
        let measurment: String = hgt[2..].to_string().chars().rev().collect();
        let measurment = measurment.parse::<i32>().unwrap_or(-1);
        if measure_unit == "cm" && (measurment < 150 || measurment > 193) {
            return false;
        }
        else if measure_unit == "in" && (measurment < 59 || measurment > 76) {
            return false;
        } else if measure_unit != "in" && measure_unit != "cm" {
            return false;
        }
        return true;
    }

    fn is_hcl_valid(&self) -> bool {
        let hcl = self.hcl.as_ref().unwrap();
        if hcl.len() != 7 {
            return false;
        }
        if &hcl[..1] != "#" {
            return false;
        }
        for c in hcl[1..].chars() {
            match c {
                'a'..='f' => {
                    continue;
                },
                '0'..='9' => {
                    continue;
                },
                _ => {
                    return false;
                }
            }
        }
        return true;
    }

    fn is_ecl_valid(&self) -> bool {
        let ecl = self.ecl.as_ref().unwrap();
        match &ecl[..] {
            "amb" | "blu" | "brn" | "gry" |
            "grn" | "hzl" | "oth" => {
                return true;
            },
            _ => {
                return false;
            }

        }
    }

    fn is_pid_valid(&self) -> bool {
        let pid = self.pid.as_ref().unwrap();
        if pid.len() != 9 {
            return false;
        } else {
            for p in pid.chars() {
                match p {
                    '0'..='9' => {
                        continue;
                    },
                    _ => {
                        return false;
                    }
                }
            }
        }
        return true;
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
