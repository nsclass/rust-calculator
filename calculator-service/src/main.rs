use calculator_engine::calculate_str;

fn main() {
    let input = "1 + 2";
    let result = calculate_str(input, true);
    match result {
        Ok((val, _trace_details)) => println!("{} is {}", input, val),
        Err(err) => println!("{}, error: {:?}", input, err),
    }
}
