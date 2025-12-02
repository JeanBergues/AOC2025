use std::fs::read_to_string;

fn invalid_ids_in_range(start: i64, stop: i64) -> i64 {
    let mut total = 0;
    for i in start..=stop {
        let i_str = i.to_string();
        let len_str = i_str.chars().count();

        // Loop over all possible sequence lengths, checking whether at least one repeats
        let mut invalid = false;
        for divisor in 2..=len_str {
            if len_str % divisor != 0 { continue }; // If you cannot split the number evenly, skip

            // Split the entire number into evenly sized chunks
            let i_chunks = i_str.as_bytes()
                .chunks(len_str / divisor)
                .map(str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .expect("One of the elements is not valid UTF-8");

            // Only if all chunks are equal, the ID is invalid
            if i_chunks.iter().all(|chunk| chunk == &i_chunks[0]) {
                invalid = true;
            }
        }
        if invalid { total += i };
    };
    total
}

// TODO: ipv alle getallen in range checken, alle mogelijke invalid ID's constructen en kijken of die in range liggen.

fn main() {
    let f = read_to_string("src/input.txt").unwrap();
    let invalid_id_sum: i64 = f.split(",")
        .map(|range| {
            let mut split_range = range.split("-");
            invalid_ids_in_range(split_range.next().unwrap().parse().expect("Is not a number"),
                                 split_range.next().unwrap().parse().expect("Is not a number"))
        })
        .sum();
    println!("Solution: {}", invalid_id_sum);
}
