use rand::Rng;
use std::collections::HashMap;


fn main() {

    let len = 50;
    let mut v = Vec::with_capacity(len);

    for _ in 1..=v.capacity() {
        v.push(rand::thread_rng().gen_range(1..=100));
    }

    println!("{:?}", v);
    println!("Average: {}", average(&v).unwrap());
    println!("Median: {}", median(&v).unwrap());
    println!("Mode: {:?}", mode(&v));
}

fn average(v: &[i32]) -> Option<f64> {
    match v.len() {
        0 => None,
        _ => Some(v.iter().sum::<i32>() as f64 / v.len() as f64)
    }
}

fn median (v: &[i32]) -> Option<i32> {
    if v.is_empty() {
        return None
    }

    let mut sorted = vec![0; v.len()];
    sorted.copy_from_slice(v);
    sorted.sort_unstable();
    println!("Sorted: {:?}", sorted);
    Some(sorted[v.len()/2])
}

fn mode (v: &[i32]) -> Vec<i32> {
    let mut current_max_times = 0;
    let mut frequency = HashMap::new();

    // count frequencies for unique numbers
    for i in v {
        let count = frequency.entry(*i).or_insert(0);
        *count += 1;
        if *count > current_max_times {
            current_max_times = *count;
        }
    };

    // collect numbers with maximum frequency
    let res = frequency
        .iter()
        .filter(|(_, &f)| f == current_max_times)
        .map(|(&i, _)| i)
        .collect::<Vec<_>>();

    println!("Frequency: {:?}", frequency);
    //println!("Filtered: {:?}", res);

    res
}
