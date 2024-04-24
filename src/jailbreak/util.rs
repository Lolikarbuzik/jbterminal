pub fn shorten_number(number: f64) -> String {
    let suffixes = vec!["", "k", "m", "b", "t"]; // Add more suffixes as needed
    let tier = (number.abs().log10() / 3.0).floor() as usize;

    if tier == 0 {
        return number.to_string();
    }

    let suffix = suffixes[tier];
    let scale = 10_f64.powi((tier * 3) as i32);

    let scaled_number = number / scale;

    // Check if the scaled number has no decimal part
    if scaled_number.fract() == 0.0 {
        return scaled_number.to_string() + suffix;
    } else {
        // Format the number with up to 1 decimal places
        let formatted_number = format!("{:.1}", scaled_number);
        return formatted_number + suffix;
    }
}

pub fn string_count(str: &String, search: &String) -> i32 {
    let str = str.to_lowercase();
    if search.len() > str.len() {
        return -1;
    }
    let mut count = 0;
    let mut nstr = String::new();

    for i in 0..search.len() {
        if str.starts_with(&nstr) {
            count += 2;
        } else if str.contains(&nstr) {
            count += 1;
        }
        nstr.push(search.chars().nth(i).unwrap());
    }

    return count;
}
