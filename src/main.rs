fn main() {
    println!("Enter a binary number");
    let mut input = String::new();
    read_input_from_stdin(&mut input);
    let mut result: usize = 0;
    process_input(&input, &mut result);
    println!("The result is {}", result);
}

fn process_input(input: &String, result: &mut usize) {
    for (index, char) in input.trim().chars().rev().enumerate() {
        match char {
            '0' => (),
            '1' => {
                let amount_to_add = 2usize.pow(index as u32);
                *result += amount_to_add;
            }
            _ => panic!("Invalid binary"),
        }
    }
}

fn read_input_from_stdin(input: &mut String) {
    std::io::stdin()
        .read_line(input)
        .expect("Error reading input");
}
