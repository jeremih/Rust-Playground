use std::io;

/*
 * Constructs a tri from standard input
 * Expects the first inputted line to be the length of the tri
 * Each following line should be a row of the tri, starting from the top 
 */
pub fn construct_tri() -> io::Result<Vec<Vec<u32>>> {
    let stdin = io::stdin();
    let mut tri: Vec<Vec<u32>> = Vec::new();
    let mut num_tri_rows_str = String::new();
    stdin.read_line(&mut num_tri_rows_str)?;
    num_tri_rows_str = num_tri_rows_str.trim().to_string();
    let num_tri_rows = num_tri_rows_str.parse::<u16>().unwrap();

    for _ in 0..num_tri_rows {
        let mut tri_row_str = String::new();
        stdin.read_line(&mut tri_row_str)?;
        tri_row_str = tri_row_str.trim().to_string();

        let mut tri_row_vec: Vec<u32> = Vec::new();

        for num_str in tri_row_str.split_whitespace() {
            tri_row_vec.push(num_str.parse::<u32>().unwrap());
        }

        tri.push(tri_row_vec);
    }

    Ok(tri)
}

/*
 * Takes in a triangle of nums, structured row by row, where each row has one more num than the last,
 * finds the path from bottom to top where summing all nums in said path results in the greatest value possible
 * for that triangle, and returns said sum of said path.
 */
pub fn condense_tri(mut tri: Vec<Vec<u32>>) -> u32 {
    let mut remaining_len = tri.len();
    // If we only have one row remaining, we're at the top of the triangle and can thus stop processing
    // Until then, we work our way up the triangle
    while remaining_len > 1 {
        // Note that the amount of elems on this row of the triangle is equal to remaining_len
        // Iterate through all but the last elem on this row
        let row = remaining_len - 1;
        for i in 0..row {
            // Compare each elem to the next elem in the row
            if let true = tri[row][i] > tri[row][i + 1] {
                // First elem is greater; add it to elem above
                tri[row - 1][i] += tri[row][i];
            } else {
                // Second elem is greater; add it to elem above
                tri[row - 1][i] += tri[row][i + 1];                
            }
        }
        remaining_len -= 1;
    }
    return tri[0][0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_tri() {
        let results = construct_tri();
        // This line is here so we can inspect results in the debugger
        let x = 0;
        assert_eq!(0, x);
    }

    // #[test]
    fn test_condense_tri() {
        let row_one = vec!(3);
        let row_two = vec!(7, 4);
        let row_three = vec!(2, 4, 6);
        let row_four = vec!(8, 5, 9, 3);
        let tri = vec!(row_one, row_two, row_three, row_four);

        assert_eq!(23, condense_tri(tri));
    }
}