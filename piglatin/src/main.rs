
const VOWELS: [char; 10] = ['a','e','i','o','u','A','E','I','O','U'];

fn main() {
    let s = String::from("fxcvbxc");
    println!("{} converted: {}", s, piglatin(&s));
}

fn piglatin(s: &String) -> String {
    if s.is_empty() {
        return String::new()
    }
    if s.starts_with(&VOWELS[..]) {

        String::from(s) + "-hay"
    } else {
        let mut chars= s.chars();
        let first = chars.next().unwrap();
        let (_, rest) = s.split_at(first.len_utf8());
        let mut result = rest.to_string();
        result.push('-');
        result.push(first);
        result.push_str("ay");

        result
    }
}