pub fn run(exponent: &u16) -> () {
    if *exponent > 10u16.pow(4) || *exponent < 1 {
        panic!("Exp ain't right, my pal")
    }
    let target_exp = *exponent;
    let mut iter_exp = 1u16;
    // We want a data structure that can hold at least 2^1000
    // No base rust data type can hold above 2^128, so we will have
    // to use a vector that manually represents decimal digits instead
    let mut result_vec = vec![2u8];

    // Each loop counts as a single doubling, meaning our result vector will
    // contain 2^iter_exp
    while iter_exp < target_exp {
        let result_length = result_vec.len();
        // We will need to carry the 1 over to the next digit if any digit
        // goes above 10. Standard long addition
        let mut carry_over = 0;
        for n in 0..result_length {
            result_vec[n] += result_vec[n] + carry_over;
            carry_over = 0;
            if result_vec[n] >= 10u8 {
                result_vec[n] -= 10u8;
                carry_over = 1;
            }
        }
        if carry_over > 0 {
            result_vec.push(1u8);
        }

        iter_exp += 1u16;
    }

    // Sum up all the digits in our now-finished result vector
    let mut final_sum: u64 = 0;
    for i in result_vec.iter() {
        final_sum += u64::from(*i);
    }
    println!("{}", final_sum);
}
