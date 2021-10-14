use std::io;

fn main() -> io::Result<()> {

    let mut rep_count_str = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut rep_count_str)?;
    // Subtract 2 from the length to account for the newline and carriage return chars
    let mut len = rep_count_str.len() - 2;
    let rep_count = rep_count_str[0..len]
        .parse::<u32>()
        .unwrap();

    let mut i = 0;
    while i < rep_count {
        let mut curr_exp_str = String::new();
        stdin.read_line(&mut curr_exp_str)?;
        len = curr_exp_str.len() - 2;
        let curr_exp = curr_exp_str[0..len]
            .parse::<u16>()
            .unwrap();
        powers_of_two::run(&curr_exp);
        i += 1;
    }
    Ok(())
}
