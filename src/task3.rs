#[allow(dead_code)]
pub fn pling_plang_plong(number: u32) -> String {
    let mut _answer = String::from("");
    let mut counter = 0;

    if number % 3 == 0 {
        _answer += "Pling";

        counter += 1;
    }
    if number % 5 == 0 {
        if counter > 0 {
            _answer += "-";
        }
        _answer += "Plang";
        counter += 1;
    }
    if number % 7 == 0 {
        if counter > 0 {
            _answer += "-";
        }
        _answer += "Plong";
        counter += 1;
    }

    if counter == 0 {
        return number.to_string();
    }
    _answer
}
