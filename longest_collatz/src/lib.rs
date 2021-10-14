use std::collections::HashMap;
use std::io;
use std::convert::TryInto;

pub fn find_longest_collatz(num_lines: u16) {
    let mut i = 0u16;
    // This will be used to contain the collatz number for any given input, mapping inputs to collatz nums
    let mut collatz_map: HashMap<u64, u64> = HashMap::new();
    collatz_map.insert(1, 0);
    // This will contain any integer n for which its collatz num is higher than all the collatz nums for
    // all ints i where 0 < i < n
    let mut max_collatz_vec: Vec<u64> = vec!(1);
    // This will represent the greatest collatz starter we've tested thus far
    let mut max_tested: u64 = 1;

    while i < num_lines {
        // Read a number from stdin, trim it, and parse it
        let mut curr_num_str = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut curr_num_str).unwrap();
        let curr_num_slice = curr_num_str.trim();
        let n = curr_num_slice.parse::<u64>().unwrap();

        // Find all collatz numbers from collatz sequences starting with any integers between 1 and n
        collatz_nums(n, &mut collatz_map, &mut max_collatz_vec, &mut max_tested);
        // Print out the collatz sequence starter < n which has the greatest collatz number
        let result = greatest_val_lea_n_easy(&n, &max_collatz_vec);
        println!("{}", result);

        i += 1;
    }
} 

pub fn collatz_nums(n: u64, coll_map: &mut HashMap<u64, u64>, max_coll_vec: &mut Vec<u64>, max_tested: &mut u64) {

    // If we've already tested up to this number, no need to do anything
    if n <= *max_tested {
        return;
    }

    // Iterate and fill up the collatz map with starters all the way up to n
    // Start by setting an index up to the current tested value plus one
    let mut iter_index: u64 = *max_tested + 1;
    while iter_index <= n {
        // Make sure this isn't already in our map
        if coll_map.contains_key(&iter_index) {
            iter_index += 1;
            continue;
        }
        // Our operating variable as we traverse the collatz sequence
        let mut coll_iter = iter_index;
        // Represents the collatz sequence starting with the current value
        let mut collatz_seq = vec!(coll_iter);
        let mut col_num: u64 = 0;
        
        while coll_iter != 1 {
            let map_query = coll_map.get(&coll_iter);
            // Unlike the earlier map contains_key call, this time, we've got extra info to add to our map
            if let Some(collatz_num) = map_query {
                // Stop here, we can extrapolate the collatz nums in our sequence from this value
                col_num = *collatz_num;
                break;
            } else {
                if coll_iter % 2 == 0 {
                    coll_iter /= 2;
                } else {
                    coll_iter *= 3;
                    coll_iter += 1;
                }
                collatz_seq.push(coll_iter);
            }
        }
        // We've found a new, wholly unique collatz sequence. Update coll_map and max_coll_vec accordingly
        add_coll_arr_to_map(&collatz_seq, coll_map, max_coll_vec, col_num);
        iter_index += 1;
    }
    // Keep track of how far we've tested
    *max_tested = n;
}

// Adds each number in coll_vec and its corresponding collatz length to the coll_map
// Checks max_coll_vec 
pub fn add_coll_arr_to_map(
    coll_vec: &Vec<u64>,
    coll_map: &mut HashMap<u64, u64>,
    max_coll_vec: &mut Vec<u64>,
    base_num: u64) 
{
    // Update coll_map
    // Get the length of coll_vec for reverse order iteration
    let mut index = coll_vec.len() - 1;
    let len: u64 = coll_vec.len().try_into().unwrap();
    while index != 0 {
        let coll_input: u64 = coll_vec[index - 1];
        // Not sure how to get this with the type conversion all in one line. Choosing to use u64 was
        // a mistake in retrospect; I should've just used usize everywhere
        let u64_index: u64 = index.try_into().unwrap();
        // We can get the collatz num for any value in the vector by seeing how far it is from the end of the vec
        // and adding that to the base collatz num argument
        let coll_num: u64 = len + base_num - u64_index;
        coll_map.insert(coll_input, coll_num);
        index -= 1;
    }
    // Update max_coll_vec
    // Will only be testing the first value in coll_vec since it will have the highest coll num in the vec
    // and presumably be potentially small enough to be the new max
    let (coll_input, coll_num) = (coll_vec[0], *coll_map.get(&coll_vec[0]).unwrap());
    // Compare the current coll_num to whatever the previous max was and append coll_input if we've got a new max
    // The new coll_input is guaranteed to be greater than any other val in max_coll_vec since this algorithm
    // goes through coll_inputs in ascending order, so there's no need to "bubble up" through max_coll_vec
    // and replace any existing values
    // We can be assured that the unwrap() call here won't panic, since any value in max_coll_vec must be
    // a key in coll_map
    let oldMaxKey = max_coll_vec[max_coll_vec.len() - 1];
    let oldMaxValue = *coll_map.get(&oldMaxKey).unwrap();
    if *coll_map.get(&max_coll_vec[max_coll_vec.len() - 1]).unwrap() <= coll_num {
        max_coll_vec.push(coll_input);
    }
}

// Finds the greatest value in a vector that is less than or equal to an integer n
pub fn greatest_val_lea_n_easy(n: &u64, values: &Vec<u64>) -> u64 {
    // If the number is in there, skip all the hard work
    if values.contains(n) {
        return *n;
    }
    let mut iter = values.len();
    // Iterate through the vec in reverse
    while iter != 0 {
        // Find the greatest value less than or equal to n
        if values[iter - 1] <= *n {
            return values[iter - 1];
        }
        iter -= 1;
    }
    // If we can't find it in here, return 0
    return 0;
}

// Finds the nearest input in the max collatz vec by constantly splitting the vec in two and comparing the value
// at the midpoint with the input value
pub fn greatest_val_lea_n(n: &u64, values: &Vec<u64>) -> u64 {
    // If the number is in there, skip all the hard work
    if values.contains(n) {
        return *n;
    }
    let len = values.len();
    let mut midpoint = len / 2;
    let mut next_step_offset = midpoint - midpoint / 2;
    if midpoint == 1 {
        next_step_offset = 0;
    }
    // Keep going until the length of sub-vectors is too small to split any further
    while next_step_offset > 0 {
        // Veer towards a greater or lesser index depending on how the value at our current index compares to n
        match values[midpoint] < *n {
            true => {
                midpoint += next_step_offset;
                if midpoint >= len {
                    // Input is greater than the max value, return the max
                    return values[len - 1];
                }
            },
            false => {
                midpoint -= next_step_offset;
            }
        }
        if next_step_offset == 1 {
            next_step_offset = 0;
        } else {
            next_step_offset = next_step_offset - next_step_offset / 2;
        }
    }
    // Down to the last three elements
    // Return the greatest one which is still less than n
    if values[midpoint] < *n {
        // If we're at the end of the array, or the next elem is greater than n, return the curr value
        if midpoint == len - 1 || values[midpoint + 1] > *n {
            return values[midpoint];
        } else {
            return values[midpoint + 1];
        }
    } else {
        if midpoint == 0 {
            // Default to 1 as base minimum
            return 1;
        } else {
            // That only leaves the lower value as our answer
            return values[midpoint - 1];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_find_longest_collatz_three_inputs() {
        let test_num_lines = 3;

        find_longest_collatz(test_num_lines);
    }

    // #[test]
    fn test_collatz_nums() {
        let test_n = 5;
        let mut test_coll_map: HashMap<u64, u64> = HashMap::new();
        test_coll_map.insert(1, 1);
        test_coll_map.insert(2, 2);
        let mut test_max_vec: Vec<u64> = vec!(1, 2);
        let mut test_max_tested: u64 = 2;

        collatz_nums(test_n, &mut test_coll_map, &mut test_max_vec, &mut test_max_tested);
        assert_eq!(test_coll_map[&3], 8);
    }

    // #[test]
    fn test_add_coll_arr_to_map_small() {
        let input_coll_vec: Vec<u64> = vec!(7, 5, 3, 1);
        let mut test_coll_map: HashMap<u64, u64> = HashMap::new();
        test_coll_map.insert(5, 2);
        let mut test_max_vec: Vec<u64> = vec!(5);
        let input_base_num = 2;

        add_coll_arr_to_map(&input_coll_vec, &mut test_coll_map, &mut test_max_vec, input_base_num);
        assert_eq!(test_coll_map[&7], 5);
        assert_eq!(test_max_vec[1], 7);
    }

    // #[test]
    fn test_greatest_val_lea_n_find_middle_elem_of_five() {
        let even_ints = vec!(2, 4, 6, 8, 10);
        let n = 5;
        let result = greatest_val_lea_n(&n, &even_ints);
        assert_eq!(result, 4u64);
    }

    // #[test]
    fn test_greatest_val_lea_n_find_rightmost_elem_of_five() {
        let even_ints = vec!(2, 4, 6, 8, 10);
        let n = 11;
        let result = greatest_val_lea_n(&n, &even_ints);
        assert_eq!(result, 10u64);
    }

    // #[test]
    fn test_greatest_val_lea_n_find_leftmost_elem_of_six() {
        let even_ints = vec!(2, 4, 6, 8, 10, 12);
        let n = 3;
        let result = greatest_val_lea_n(&n, &even_ints);
        assert_eq!(result, 2u64);
    }

    // #[test]
    fn test_greatest_val_lea_n_find_index_one_elem_of_six() {
        let even_ints = vec!(2, 4, 6, 8, 10, 12);
        let n = 5;
        let result = greatest_val_lea_n(&n, &even_ints);
        assert_eq!(result, 4u64);
    }

    // #[test]
    fn test_greatest_val_lea_n_find_from_vec_of_fourteen() {
        let even_ints = vec!(2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28);
        assert_eq!(greatest_val_lea_n(&17, &even_ints), 16);
    }
}