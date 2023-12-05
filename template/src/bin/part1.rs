use {{crate_name}}::part1::process;

fn main() {
    let file = include_str!("../../input1.txt");
    let timing = std::time::Instant::now();
    println!("{}", process(file));
    println!("Elapsed: {}ms", timing.elapsed().as_millis());
}