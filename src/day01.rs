pub fn run(input: &str) {
    let mut dial_position = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let absolute_offset = line[1..].parse::<i32>().unwrap();

        dial_position += if line.starts_with('L') {
            -absolute_offset
        } else {
            absolute_offset
        };

        dial_position = dial_position.rem_euclid(100);

        if dial_position == 0 {
            zero_count += 1;
        }
    }

    println!("Part 1:");
    println!("Dial hit zero {zero_count} times");
}
