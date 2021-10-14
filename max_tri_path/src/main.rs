use std::io;

fn main() -> io::Result<()> {
    let mut num_tris_str = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut num_tris_str)?;
    num_tris_str = num_tris_str.trim().to_string();
    let num_tris = num_tris_str.parse::<u16>().unwrap();

    for _ in 0..num_tris {
        if let Ok(curr_tri) = max_tri_path::construct_tri() {
            println!("{}", max_tri_path::condense_tri(curr_tri));
        }
    }

    Ok(())
}
