use {{crate_name}}::part2::process;

fn main() {
    let file = include_str!("../../input2.txt");
    println!("{}", process(file));
}