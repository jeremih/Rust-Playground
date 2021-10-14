use std::io;

fn main() -> io::Result<()> {
    let mut num_lines_str = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut num_lines_str)?;
    num_lines_str = num_lines_str.trim().to_string();
    let num_lines = num_lines_str.parse::<u16>().unwrap();

    longest_collatz::find_longest_collatz(num_lines);    

    Ok(())
}
