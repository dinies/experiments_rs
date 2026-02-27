// Given a string s representing a valid expression, implement a basic
// calculator to evaluate it, and return the result of the evaluation.
// Note: You are not allowed to use any built-in function which
// evaluates strings as mathematical expressions, such as eval().
//
// Constraints:
// 1 <= s.length <= 3 * 105
// s consists of digits, '+', '-', '(', ')', and ' '.
// s represents a valid expression.
// '+' is not used as a unary operation (i.e., "+1" and "+(2 + 3)" is invalid).
// '-' could be used as a unary operation (i.e., "-1" and "-(2 + 3)" is valid).
// There will be no two consecutive operators in the input.
// Every number and running calculation will fit in a signed 32-bit integer.

struct BasicCalculator {}

impl BasicCalculator {
    pub fn calculate(s: String) -> i32 {
        let mut is_sum: bool = true;
        let upper_bound = 3.0 * 10e6;
        let mut stack = Vec::<(bool, i32)>::with_capacity(upper_bound as usize);
        stack.push((is_sum, 0));
        let mut elems = s.chars().peekable();
        let mut curr_char = elems.next();
        let mut curr_number: Vec<char> = Vec::new();

        let build_num = |chars: &mut Vec<char>, is_positive: &mut bool| -> i32 {
            let mut num_string: String = chars.into_iter().map(|x| x.to_string()).collect();
            if !*is_positive {
                num_string.insert(0, '-');
                *is_positive = true;
            }
            let num = num_string.parse::<i32>().unwrap();
            chars.clear();
            return num;
        };

        while curr_char != None {
            let c = curr_char.unwrap();
            match c {
                ' ' => {}
                '0'..='9' => {
                    curr_number.push(c);
                    match elems.peek() {
                        Some('0'..='9') => {}
                        _ => {
                            let n = build_num(&mut curr_number, &mut is_sum);
                            if is_sum {
                                stack.last_mut().unwrap().1 += n
                            } else {
                                stack.last_mut().unwrap().1 -= n
                            }
                        }
                    }
                }
                '(' => {
                    stack.push((is_sum, 0));
                    is_sum = true;
                }
                ')' => {
                    let (is_sum_op, val) = stack.pop().unwrap();
                    if is_sum_op {
                        stack.last_mut().unwrap().1 += val
                    } else {
                        stack.last_mut().unwrap().1 -= val
                    }
                }
                '-' => {
                    is_sum = false;
                }
                '+' => {
                    is_sum = true;
                }
                _ => {
                    panic!("Not possible, char {} not recognised", c)
                }
            }
            curr_char = elems.next();
        }

        return stack.pop().unwrap().1;
    }
    pub fn test() {
        let valid_expressions: Vec<(String, i32)> = vec![
            (String::from("1 + 1"), 2),
            (String::from(" 2-1 + 2 "), 3),
            (String::from("(1+(4+5+2)-3)+(6+8)"), 23),
            (String::from("-1"), -1),
            (String::from("-(1)"), -1),
            (String::from("17 + 2"), 19),
            (String::from("(7 + 2)"), 9),
            (String::from(" (  7  +  2 )  "), 9),
            (String::from("2147483647"), 2147483647),
            (String::from("-2147483648"), -2147483648),
        ];

        for (expr, res) in valid_expressions {
            assert_eq!(BasicCalculator::calculate(expr), res);
        }
    }
}

fn main() {
    BasicCalculator::test();
}
