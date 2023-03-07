fn encode_number_under_hundred(n: u64) -> Option<String> {

    if n == 0 {
        return None;
    }

    let units_say = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let ten_to_nineteen = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let tens_say = ["", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    let tens = n / 10;
    let units = n % 10;

    if tens == 0 {
        return Some(units_say[units as usize].to_string());
    }

    if tens == 1 {
        return Some(ten_to_nineteen[units as usize].to_string());
    }

    if units == 0 {
        return Some(tens_say[tens as usize].to_string());
    }

    Some(tens_say[tens as usize].to_string() + "-" + units_say[units as usize])

}

fn encode_number_under_thousand(n: u64) -> Option<String> {
    let units_say = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    if n >= 1000 {
        return None;
    }
    if n == 0 {
        return None;
    }

    let hundreds = n / 100;
    let n_without_hundreds = n % 100;

    let mut res = String::new();
    if hundreds > 0 {

        res = units_say[hundreds as usize].to_string() + " hundred ";
    }
    if let Some(r) = encode_number_under_hundred(n_without_hundreds) {
        res = res + &r;
    }

    Some(res)
}

pub fn encode(n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }
    let mut res: String = String::new();
    let n_upper_quintillion = n / 1_000_000_000_000_000_000;
    let n_upper_quadrillion_lower_quintillion = (n / 1_000_000_000_000_000) % 1_000;
    let n_upper_trillion_lower_quadrillion = (n / 1_000_000_000_000) % 1_000;
    let n_upper_billion_lower_trillion = (n / 1_000_000_000) % 1_000;
    let n_upper_million_lower_billion = (n / 1_000_000) % 1_000;
    let n_upper_thousand_lower_million = (n / 1_000) % 1_000;
    let n_lower_thousand = n % 1_000;

    if let Some(r) = encode_number_under_thousand(n_upper_quintillion) {
        res = res + &r + " quintillion ";
    }

    if let Some(r) = encode_number_under_thousand(n_upper_quadrillion_lower_quintillion) {
        res = res + &r + " quadrillion ";
    }

    if let Some(r) = encode_number_under_thousand(n_upper_trillion_lower_quadrillion) {
        res = res + &r + " trillion ";
    }

    if let Some(r) = encode_number_under_thousand(n_upper_billion_lower_trillion) {
        res = res + &r + " billion ";
    }
    
    if let Some(r) = encode_number_under_thousand(n_upper_million_lower_billion) {
        res = res + &r + " million ";
    }
    
    if let Some(r) = encode_number_under_thousand(n_upper_thousand_lower_million) {
        res = res + &r + " thousand ";
    }

    if let Some(r) = encode_number_under_thousand(n_lower_thousand) {
        res = res + &r;
    }

    if res.ends_with(" ") {
        res.pop();
    }

    res
}
