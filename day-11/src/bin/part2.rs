use day_11::part2::process;

fn main() {
    let file = include_str!("../../input2.txt");
    let timing = std::time::Instant::now();
    println!("{}", process(file, 1000000));
    println!("Elapsed: {}ms", timing.elapsed().as_millis());
}
