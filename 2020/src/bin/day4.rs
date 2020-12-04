use aoc2020::aoc_input::get_input;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CheckResult {
    Invalid,
    Present,
    PresentAndValid,
}

struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn check_digit_field(&self, name: &str, digits: usize, min: usize, max: usize) -> CheckResult {
        let value = match self.fields.get(name) {
            None => return CheckResult::Invalid,
            Some(value) => value,
        };

        if value.len() != digits {
            return CheckResult::Present;
        }

        let value = match value.parse::<usize>() {
            Err(_) => return CheckResult::Present,
            Ok(value) => value,
        };

        if value < min || value > max {
            CheckResult::Present
        } else {
            CheckResult::PresentAndValid
        }
    }

    fn check_birth_year(&self) -> CheckResult {
        self.check_digit_field("byr", 4, 1920, 2002)
    }

    fn check_issue_year(&self) -> CheckResult {
        self.check_digit_field("iyr", 4, 2010, 2020)
    }

    fn check_expiration_year(&self) -> CheckResult {
        self.check_digit_field("eyr", 4, 2020, 2030)
    }

    fn check_height(&self) -> CheckResult {
        let value = match self.fields.get("hgt") {
            None => return CheckResult::Invalid,
            Some(value) => value,
        };

        let last_two_idx = match value.char_indices().rev().nth(1) {
            None => return CheckResult::Present,
            Some((i, _)) => i,
        };

        let unit = &value[last_two_idx..];
        let num = match &value[..last_two_idx].parse::<usize>() {
            Err(_) => return CheckResult::Present,
            Ok(num) => *num,
        };

        let (min, max): (usize, usize) = match unit {
            "cm" => (150, 193),
            "in" => (59, 76),
            _ => return CheckResult::Present,
        };
        if num < min || num > max {
            CheckResult::Present
        } else {
            CheckResult::PresentAndValid
        }
    }

    fn check_hair_color(&self) -> CheckResult {
        let value = match self.fields.get("hcl") {
            None => return CheckResult::Invalid,
            Some(value) => value,
        };

        if value.len() != 7 {
            return CheckResult::Present;
        }

        let mut chars = value.chars();
        if chars.next() != Some('#') {
            return CheckResult::Present;
        }

        if chars.any(|c| !c.is_ascii_hexdigit() || c.is_ascii_uppercase()) {
            CheckResult::Present
        } else {
            CheckResult::PresentAndValid
        }
    }

    fn check_eye_color(&self) -> CheckResult {
        let value = match self.fields.get("ecl") {
            None => return CheckResult::Invalid,
            Some(value) => value,
        };

        match value.as_ref() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => CheckResult::PresentAndValid,
            _ => CheckResult::Present,
        }
    }

    fn check_passport_id(&self) -> CheckResult {
        self.check_digit_field("pid", 9, 0, 999999999)
    }

    fn check(&self) -> CheckResult {
        let results = [
            self.check_birth_year(),
            self.check_issue_year(),
            self.check_expiration_year(),
            self.check_height(),
            self.check_hair_color(),
            self.check_eye_color(),
            self.check_passport_id(),
        ];
        // dbg!(results);

        if results.iter().any(|r| *r == CheckResult::Invalid) {
            return CheckResult::Invalid;
        }

        if results.iter().any(|r| *r != CheckResult::PresentAndValid) {
            return CheckResult::Present;
        }

        return CheckResult::PresentAndValid;
    }
}

fn parse_passport(lines: &[&str]) -> Passport {
    let mut fields = HashMap::<String, String>::new();
    for line in lines {
        for field in line.split(' ') {
            let mut kv = field.split(':');
            let key = kv.next().expect("Malformed field key");
            let value = kv.next().expect("Malformed field value");
            if !kv.next().is_none() {
                panic!("Malformed field structure");
            }

            fields.insert(key.to_owned(), value.to_owned());
        }
    }
    Passport { fields }
}

fn main() {
    let input = get_input(4);
    let lines: Vec<_> = input.lines().collect();

    let passports: Vec<_> = lines
        .split(|line| line.is_empty())
        .map(parse_passport)
        .collect();

    let mut all_present_count = 0usize;
    let mut all_valid_count = 0usize;

    for passport in &passports {
        let result = passport.check();
        all_present_count += (result != CheckResult::Invalid) as usize;
        all_valid_count += (result == CheckResult::PresentAndValid) as usize;
    }

    dbg!(all_present_count);
    dbg!(all_valid_count);
}
