use std::collections::HashMap;

pub fn convert_to_replaceable(txt: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut idx = 0;
    for line in txt.lines() {
        idx += 1;
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.chars().filter(|c| *c == ' ').count() != 1 {
            panic!("Invalid amount of spaces at line {}", idx);
        }
        let (original, replacement) = line.split_once(' ').unwrap();
        if map.contains_key(original) {
            panic!("Duplicate key {} at line {}", original, idx);
        }
        map.insert(original.to_string(), replacement.to_string());
    }
    return map;
}

pub fn get_common() -> HashMap<String, String> {
    let common_txt = include_str!("common.txt");
    return convert_to_replaceable(common_txt);
}
pub fn get_uncommon() -> HashMap<String, String> {
    let uncommon_txt = include_str!("uncommon.txt");
    return convert_to_replaceable(uncommon_txt);
}
pub fn get_alias() -> Vec<Vec<String>> {
    let alias_txt = include_str!("alias.txt");
    let mut alias = Vec::new();
    for line in alias_txt.lines() {
        if line.is_empty() || line.starts_with('#') || line.contains(" ") {
            continue;
        }
        let mut aliased = Vec::new();
        for c in line.chars() {
            aliased.push(c.into());
        }
        alias.push(aliased);
    }
    return alias;
}
pub fn optimize_without_watching_case(
    string: &str,
    map: &HashMap<String, String>,
) -> String {
    let mut s = string.to_string();
    for (original, replacement) in map.iter() {
        let temp = s.to_lowercase().replace(original, replacement);
        if temp != s.to_lowercase() {
            s = temp;
        }
    }
    return s;
}

pub fn optimize_loosely_without_watching_case(
    string: &str,
    map: &HashMap<String, String>,
) -> String {
    let mut s = string.to_string();
    let aliases = get_alias();
    let mut found;
    for (original, replacement) in map.iter() {
        found = false;
        for alias in aliases.iter() {
            if alias.contains(original) {
                for a in alias.iter() {
                    let temp = s.replace(a, replacement);
                    if temp != s {
                        s = temp;
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }
        if found {
            continue;
        }
        let temp = s.replace(original, replacement);
        if temp != s.to_lowercase() {
            s = temp;
        }
    }
    return s;
}

pub fn optimize_loosely(string: &str, map: &HashMap<String, String>) -> String {
    let mut s = string.to_string();
    let aliases = get_alias();
    let mut found;
    for (original, replacement) in map.iter() {
        found = false;
        for alias in aliases.iter() {
            if alias.contains(original) {
                for a in alias.iter() {
                    let temp = s.replace(a, replacement);
                    if temp != s {
                        s = temp;
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }
        if found {
            continue;
        }
        s = s.replace(original, replacement);
    }
    return s;
}

pub fn optimize(string: &str, map: &HashMap<String, String>) -> String {
    let mut s = string.to_string();
    for (original, replacement) in map.iter() {
        s = s.replace(original, replacement);
    }
    return s;
}
