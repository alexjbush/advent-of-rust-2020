use regex::Regex;

use crate::Day;

pub struct Day4 {}

impl<'a> Day<'a> for Day4 {
    fn get_tasks(&self) -> Vec<(usize, &dyn Fn() -> String)> {
        vec![(1, &|| task(false)), (2, &|| task(true))]
    }

    fn get_day_number(&self) -> usize {
        4
    }
}

fn task(strict: bool) -> String {
    let passport = get_passport();
    get_input()
        .iter()
        .filter(|d| d.is_passport(&passport, strict))
        .count()
        .to_string()
}

fn get_input() -> Vec<Document> {
    input_to_documents(INPUT)
}

const INPUT: &str = include_str!("input.txt");

fn input_to_documents(input: &str) -> Vec<Document> {
    input
        .split("\n\n")
        .map(|d| Document {
            fields: d
                .to_string()
                .lines()
                .map(|l| l.split_ascii_whitespace().map(|w| w.to_string()))
                .flatten()
                .collect(),
        })
        .collect()
}

struct Document {
    fields: Vec<String>,
}

struct DocumentType {
    required_fields: Vec<DocumentField>,
}

impl<'a> DocumentField {
    fn is_valid(&self, doc: &str, strict: bool) -> bool {
        if !doc.contains(&format!("{}:", self.id)) {
            return false;
        } else {
            return !strict || self.valid_by_rule(doc);
        }
    }

    fn valid_by_rule(&self, doc: &str) -> bool {
        self.pattern.is_match(doc)
    }
}

struct DocumentField {
    id: String,
    pattern: Regex,
}

fn get_passport() -> DocumentType {
    let birth_year: DocumentField = DocumentField {
        id: "byr".to_string(),
        pattern: Regex::new(r"^byr:(19[2-9][0-9]|200[0-2])$").unwrap(),
    };
    let issue_year: DocumentField = DocumentField {
        id: "iyr".to_string(),
        pattern: Regex::new(r"^iyr:(201[0-9]|2020)$").unwrap(),
    };
    let expiration_date: DocumentField = DocumentField {
        id: "eyr".to_string(),
        pattern: Regex::new(r"^eyr:(202[0-9]|2030)$").unwrap(),
    };
    let height: DocumentField = DocumentField {
        id: "hgt".to_string(),
        pattern: Regex::new(r"^hgt:(1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)$").unwrap(),
    };
    let hair_colour: DocumentField = DocumentField {
        id: "hcl".to_string(),
        pattern: Regex::new(r"^hcl:#[0-9a-f]{6}$").unwrap(),
    };
    let eye_colour: DocumentField = DocumentField {
        id: "ecl".to_string(),
        pattern: Regex::new(r"^ecl:(amb|blu|brn|gry|grn|hzl|oth)$").unwrap(),
    };
    let passport_id: DocumentField = DocumentField {
        id: "pid".to_string(),
        pattern: Regex::new(r"^pid:\d{9}$").unwrap(),
    };
    DocumentType {
        required_fields: vec![
            birth_year,
            issue_year,
            expiration_date,
            height,
            hair_colour,
            eye_colour,
            passport_id,
        ],
    }
}

impl DocumentType {
    fn document_is_valid_type(&self, doc: &Document, strict: bool) -> bool {
        self.required_fields
            .iter()
            .all(|t| doc.fields.iter().any(|f| t.is_valid(f, strict)))
    }
}

impl Document {
    fn is_passport(&self, doc_type: &DocumentType, strict: bool) -> bool {
        doc_type.document_is_valid_type(self, strict)
    }
}

#[cfg(test)]
mod tests {
    use crate::day4::{get_passport, input_to_documents, Document};

    const TESTINPUT1: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const TESTINPUT2: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const TESTINPUT3: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn test_case_1() {
        let input: Vec<Document> = input_to_documents(TESTINPUT1);
        let count = input
            .iter()
            .filter(|d| d.is_passport(&get_passport(), false))
            .count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_case_2() {
        let input: Vec<Document> = input_to_documents(TESTINPUT2);
        let count = input
            .iter()
            .filter(|d| d.is_passport(&get_passport(), true))
            .count();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_case_3() {
        let input: Vec<Document> = input_to_documents(TESTINPUT3);
        let count = input
            .iter()
            .filter(|d| d.is_passport(&get_passport(), true))
            .count();
        assert_eq!(count, 4);
    }
}
