pub fn binary_to_byte(input: &[bool]) -> u8 {
    input
        .iter()
        .rev() // Reverse the iterator to make the next step easier
        .enumerate() // make the data a tuple of (index, data)
        .map(|(i, el)| match el {
            true => 2_u8.pow(u32::try_from(i).unwrap()), // add a number to the power of i
            false => 0,
        })
        .sum::<u8>() // sum everything up - welcome back to decimal!
}

// There probably is a native method, but to practice, I decided to make my own.
pub fn byte_to_binary(number: u8) -> Vec<bool> {
    let mut result: Vec<bool> = vec![];
    let mut cache = number;
    for i in (0..8).rev() {
        // Use powers of 2 (binary); check backwards: If greater than 64, subtract, set to true,
        // and so on. Cache essentially serves as the "remainder" of the numbers subtracted.
        let n: u8 = 2_u8.pow(i);

        if cache >= n {
            cache -= n;
            result.push(true);
        } else {
            result.push(false);
        };
    }

    result
}
