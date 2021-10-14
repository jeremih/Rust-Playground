use std::io;

fn main() -> io::Result<()> {
    let mut num_lines_str = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut num_lines_str)?;
    num_lines_str = num_lines_str.trim().to_string();
    let num_lines = num_lines_str.parse::<u8>().unwrap();

    let mut i = 0u8;
    while i < num_lines {
        // TODO: Test trimming functionality
        let mut curr_num_str = String::new();
        stdin.read_line(&mut curr_num_str)?;
        let curr_num_slice = curr_num_str.trim();

        println!("{}", scalar_elaborator::deconstruct(curr_num_slice));

        i += 1;
    }

    Ok(())
}
