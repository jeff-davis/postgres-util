use std::process::Command;
use std::collections::HashMap;
use regex::Regex;

// extract numerical major and minor version from version string
fn parse_version(s: &str) -> (u32, u32) {
    let re = Regex::new(r"^PostgreSQL (\d+)\.(\d+).*$").unwrap();
    let caps = re.captures(s).unwrap();

    (caps[1].parse::<u32>().unwrap(), caps[2].parse::<u32>().unwrap())
}

// extract key and value from the form "KEY = VALUE"
fn parse_line(s: &str) -> (&str, &str) {
    let v: Vec<&str> = s.splitn(2, " = ").collect();

    (v[0], v[1])
}

// parse "K = V" lines into HashMap
fn parse_output(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in s.lines() {
        let (k, v) = parse_line(line);
        map.insert(k.to_string(), v.to_string());
    }
    map
}

// execute pg_config and return stdout as String
fn pg_config() -> String {
    let output = Command::new("pg_config")
        .output()
        .expect("failed to run pg_config");
    assert!(output.status.success());

    String::from_utf8(output.stdout).unwrap()
}

pub fn postgres() -> HashMap<String, String> {
    let stdout = pg_config();
    let mut map = parse_output(&stdout);

    let (major, minor) = parse_version(&map["VERSION"]);
    map.insert("VERSION_MAJOR".to_string(), major.to_string());
    map.insert("VERSION_MINOR".to_string(), minor.to_string());
    map
}

#[test]
fn test1() {
    dbg!(postgres());
}
