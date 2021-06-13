use std::collections::HashMap;
use std::collections::HashSet;
use std::io;


fn main() {

    let mut departments: HashMap<String, HashSet<String>> = HashMap::new();
    let mut input = String::new();
    let mut command: &str;

    loop {
        input.clear();
        println!("Enter command:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if let Some(space_index) = input.find(char::is_whitespace) {
            command = &input[0..space_index];
        } else {
            command = &input
        }

        match command {
            "Quit" => break,
            "List" => {
                for (dpt, persons) in &departments {
                    let mut sorted = persons
                        .into_iter()
                        .collect::<Vec<_>>();
                    sorted.sort();
                    println!("Department {}: {:?}", dpt, sorted)
                }
            },
            "Add" => {
                let rest = &mut input[command.len()+' '.len_utf8()..].split_whitespace();
                if let Some(person) = rest.next() {
                    if let Some(to) = rest.next() {
                        if to != "to" {
                            println!("Wrong command Add: expect `to` after {}: {}", person, input)
                        } else if let Some(dpt) = rest.next() {
                            departments
                                .entry(dpt.to_string())
                                .or_insert_with(HashSet::new)
                                .insert(person.to_string());
                        } else {
                            println!("No department after `to` in Add command: {}", input)
                        }
                    } else {
                        println!("No `to` after {} in Add command: {}", person, input)
                    }
                } else {
                    println!("No person in Add command: {}", input)
                }
                //
            },
            _ => println!("Unknown command: {}", command)
        }

    }

}
