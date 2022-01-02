use calculator_engine::calculate_str;

fn main() {
    let input = "1 + 2";
    let result = calculate_str(input);
    match result {
        Ok(val) => println!("{} is {}", input, val),
        Err(err) => println!("{}, error: {:?}", input, err),
    }
}
