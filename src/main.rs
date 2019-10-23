fn main() {
    let iterations = 59;
    let answer = say_it_n_times(vec![1], iterations);
    println!("{} iterations digit count: {}", iterations, answer.len());
}

fn say(_look: &Vec<u8>) -> Vec<u8> {
    
    let mut _say: Vec<u8> = vec![];
    let mut _digit_count = 1u8;
    let mut _current_digit = _look[0];

    for _index in 1.._look.len() {
        if _current_digit == _look[_index] {
            _digit_count += 1;
        } else {
            _say.push(_digit_count);
            _say.push(_current_digit);
            _digit_count = 1;
            _current_digit = _look[_index];
        }
    }

    _say.push(_digit_count);
    _say.push(_current_digit);
    
    return _say;
}

fn say_it_n_times(_look: Vec<u8>, _ntimes: usize) -> Vec<u8> {
    
    let mut _answer: Vec<u8> = _look.clone();

    for x in 0.._ntimes {
        _answer = say(&_answer);
    }
    return _answer;
}

#[test]
fn _1_is_11() {
    let start = &vec![1];
    assert_eq!(say(start), vec![1,1]);
}

#[test]
fn _11_is_21() {
    let start = &vec![1,1];
    assert_eq!(say(start), vec![2,1]);
}

#[test]
fn _21_is_1211() {
    let start = &vec![2,1];
    assert_eq!(say(start), vec![1,2,1,1]);
}

#[test]
fn _111223144511777_is_3122131124152137() {
    let start = &vec![1,1,1,2,2,3,1,4,4,5,1,1,7,7,7];
    assert_eq!(say(start), vec![3,1,2,2,1,3,1,1,2,4,1,5,2,1,3,7]);
}

#[test]
fn _1_with_2_iterations() {
    let start = vec![1];
    assert_eq!(say_it_n_times(start,2), vec![2,1]);
}

#[test]
fn _1_with_40_iterations() {
    let start = vec![1];
    assert_eq!(say_it_n_times(start,40).len(), 82350);
}