// Turns a number up to 10^12 into a word, that word being
// the word that one would say if speaking that number out loud.
pub fn deconstruct(num_slice: &str) -> String {
    // Handle zero edge case. Zero is only elaborated if it's the entire number; we'll do that here
    if num_slice == "0" {
        return String::from("Zero");
    }
    let len = num_slice.len();
    // We're going to iterate through num_slice and parse each three-digit sequence.
    // remaining_len will tell us how much is left in the string so we know when to stop
    let mut remaining_len = len;
    // Array to keep track of the parsed result of each 3-digit segment
    let mut phrase_vec: Vec<String> = Vec::new();
    while remaining_len >= 3 {
        // Parse each distinct three letter phrase and add to vec
        phrase_vec.push(parse_three_digits(num_slice, remaining_len));
        remaining_len -= 3;
    }
    // The final few digits will get parsed according to how many remain
    let highest_digits: Option<String> = match remaining_len {
        2 => Some(String::from(parse_two_digits(&num_slice[0..2]))),
        1 => Some(String::from(parse_one_digit(&num_slice[0..1]))),
        0 => None,
        _ => panic!("More than three digits remaining!? That shouldn't be possible...")
    };
    // If there were 1 or 2 digits left, add their phrase to the vec
    if let Some(digits) = highest_digits {
        phrase_vec.push(digits);
    }
    elaborate(phrase_vec)
}

// Parses three digits in a string slice into their verbal form
// @param num_slice: the current string we're operating on. Represents a number
// @param remaining_len: We're traversing this string from the ones digit on up,
//   so this tells us how far we have to go until the greatest digit
pub fn parse_three_digits(num_slice: &str, remaining_len: usize) -> String {
    if remaining_len < 3 {
        panic!("Can't parse three digits on a number string with a length of heart!");
    }
    // Get only the hundreds digit. Same as getting the ones digit
    let mut hundreds = parse_one_digit(&num_slice[(remaining_len -  3) .. (remaining_len - 2)]);
    // Add the word "Hundred" to the hundreds digit, but only if the hundreds digit isn't zero
    // which would here be represented by an empty string
    // Leave a space at the end if it isn't zero
    hundreds = match hundreds.len() {
        0 => hundreds,
        _ => format!("{} Hundred ", hundreds)
    };
    // The tens digit is more complex, so we'll pass it on to its own function
    let tens_and_ones = parse_two_digits(&num_slice[(remaining_len - 2) .. (remaining_len)]);
    // Concatenate it all together. Leave no trailing spaces
    format!("{}{}", hundreds, tens_and_ones)
    
}

// If you have a number where either the tens digit is three through nine
// or it's in the teens and the ones digit is three through nine,
// the prefix will be the same in either case.
// This function will return the proper prefix
pub fn tens_or_teens_three_thru_nine(digit: &str) -> String {
    match digit {
        "3" => String::from("Thir"),
        "5" => String::from("Fif"),
        "8" => String::from("Eigh"),
        "0" | "1" | "2" => panic!("Three through nine only! Zero through two should never end up here!"),
        _ => parse_one_digit(digit)
    }
}

// Parses two digits in a string slice into their verbal form
pub fn parse_two_digits(num_slice: &str) -> String {
    let tens = &num_slice[0..1];
    let ones = &num_slice[1..2];
    // The most complicated value for the tens digit
    if tens == "1" {
        match ones {
            "0" => String::from("Ten"),
            "1" => String::from("Eleven"),
            "2" => String::from("Twelve"),
            _ => format!("{}teen", tens_or_teens_three_thru_nine(ones))
        }
    } else {
        // We can process the ones digit normally now
        let ones_parsed = parse_one_digit(ones);
        let result = match tens {
            "0" => ones_parsed,
            "2" => format!("Twenty {}", ones_parsed),
            "4" => format!("Forty {}", ones_parsed),
            _ => format!("{}ty {}", tens_or_teens_three_thru_nine(tens), ones_parsed)
        };
        // If the ones digit was a zero, then we trim off the trailing space that the lack of a ones digit word
        // representation leaves us with
        String::from(result.trim())
    }
}

// Parses a digit in a string slice into their verbal form
pub fn parse_one_digit(digit: &str) -> String {
    String::from(match digit {
        "0" => "",
        "1" => "One",
        "2" => "Two",
        "3" => "Three",
        "4" => "Four",
        "5" => "Five",
        "6" => "Six",
        "7" => "Seven",
        "8" => "Eight",
        "9" => "Nine",
        _ => panic!("Not a numerical string!")
    })
}

// Converts a vector full of individual phrases, each representing the verbal form of a number up to three digits,
// into a single string which represents the verbal form of all of those three-or-less digit numerical strings
// if they were concatenated together.
// Phrases are added to vec in order from least digit cluster to greatest
// e.g. 123456 would be represented as ["Four Hundred Fifty Six", "One Hundred Twenty Three"]
// phrase_vec must include at least one phrase
pub fn elaborate(phrase_vec: Vec<String>) -> String {
    // Store the vector length for loop comparisons later on
    let len = phrase_vec.len();
    let mut iter: usize = 0;
    // Add first three digits, which have no magnitude signifier
    let mut elaborated_vec = vec!();
    while iter < len {
        // Represents the word used to describe which digit cluster we're at
        let magnitude = match iter {
            0 => "",
            1 => " Thousand",
            2 => " Million",
            3 => " Billion",
            4 => " Trillion",
            _ => panic!("This function only supports input values under One Quadrillion!")
        };
        if phrase_vec[iter] != "" {
            elaborated_vec.push(format!("{}{}", phrase_vec[iter], magnitude));
        }
        iter += 1;
    }
    elaborated_vec.reverse();
    let elaborated = elaborated_vec.join(" ");
    elaborated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_one_digit() {
        let zero = "0";
        let one = "1";
        let two = "2";
        let three = "3";

        assert_eq!(parse_one_digit(zero), "");
        assert_eq!(parse_one_digit(one), "One");
        assert_eq!(parse_one_digit(two), "Two");
        assert_eq!(parse_one_digit(three), "Three");
    }

    #[test]
    fn test_parse_two_digits() {
        let standard = "42";
        let starts_with_zero = "08";
        let all_zero = "00";
        let ten = "10";
        let teen = "19";
        let teen_ends_with_shared_suffix = "13";
        let ends_with_zero_starts_with_shared_prefix = "50";

        // TODO: This function returns a String. Will assert_eq! list the output of that function
        // and a string literal defined here as equal if they contain identical text?
        assert_eq!(parse_two_digits(standard), "Forty Two");
        assert_eq!(parse_two_digits(starts_with_zero), "Eight");
        assert_eq!(parse_two_digits(all_zero), "");
        assert_eq!(parse_two_digits(ten), "Ten");
        assert_eq!(parse_two_digits(teen), "Nineteen");
        assert_eq!(parse_two_digits(teen_ends_with_shared_suffix), "Thirteen");
        assert_eq!(parse_two_digits(ends_with_zero_starts_with_shared_prefix), "Fifty");
    }

    #[test]
    fn test_parse_three_digits() {
        let test_data = "11291748021";

        assert_eq!(parse_three_digits(test_data, 3), "One Hundred Twelve");
        assert_eq!(parse_three_digits(test_data, 4), "One Hundred Twenty Nine");
        assert_eq!(parse_three_digits(test_data, 5), "Two Hundred Ninety One");
        assert_eq!(parse_three_digits(test_data, 6), "Nine Hundred Seventeen");
        assert_eq!(parse_three_digits(test_data, 7), "One Hundred Seventy Four");
        assert_eq!(parse_three_digits(test_data, 8), "Seven Hundred Forty Eight");
        assert_eq!(parse_three_digits(test_data, 9), "Four Hundred Eighty");
        assert_eq!(parse_three_digits(test_data, 10), "Eight Hundred Two");
        assert_eq!(parse_three_digits(test_data, 11), "Twenty One");
    }

    #[test]
    fn test_elaborate_small() {
        let phrase_one = String::from("Seven Hundred Twenty Four");
        let phrase_two = String::from("Three");

        assert_eq!(elaborate(vec!(phrase_two, phrase_one)), "Seven Hundred Twenty Four Thousand Three");
    }

    #[test]
    fn test_elaborate_large() {
        let phrase_one = String::from("Seven Hundred Twenty Four");
        let phrase_two = String::from("Three");
        let phrase_three = String::from("Forty Five");
        let phrase_four = String::from("One Hundred Ninety");

        assert_eq!(
            elaborate(vec!(phrase_four, phrase_two, phrase_three, phrase_one)),
            "Seven Hundred Twenty Four Billion Forty Five Million Three Thousand One Hundred Ninety"
        );
    }

    #[test]
    fn test_deconstruct() {
        let num_one = "724003";
        let num_two = "3724";
        let billion = "1000000000";

        assert_eq!(deconstruct(num_one), "Seven Hundred Twenty Four Thousand Three");
        assert_eq!(deconstruct(num_two), "Three Thousand Seven Hundred Twenty Four");
        assert_eq!(deconstruct(billion), "One Billion");
    }
}