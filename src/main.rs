use std::result;

fn main() {
    let string_one = String::from("Hello");
    let string_two = String::from(" world");
    concatenate_strings(string_one, string_two);
}

fn concatenate_strings(str1: String, str2: String) -> String{
    let mut result = str1;
    result.push_str(&str2);
    println!("Result: {}", result);
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate_strings() {
        let string_one = String::from("Hello");
        let string_two = String::from(" world");
        let result = concatenate_strings(string_one, string_two);
        assert_eq!(result, String::from("Hello world"));
    }
}



