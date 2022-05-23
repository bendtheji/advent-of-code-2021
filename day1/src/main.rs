use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt");

    println!(
        "{}",
        contents
            .unwrap()
            .lines()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u16>>()
            .windows(2)
            .flat_map(<[u16; 2]>::try_from)
            .filter(|[a, b]| a < b)
            .count()
    );
}
