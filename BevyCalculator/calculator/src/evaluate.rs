use std::collections::VecDeque;
use crate::clicked_buttons::ClickedButtons;

pub fn evaluate_sequence(sequence: &str) -> String {
    let mut numbers: VecDeque<f64> = VecDeque::new();
    let mut operators: VecDeque<char> = VecDeque::new();
    let mut current_number = String::new();

    fn apply_operator(numbers: &mut VecDeque<f64>, operator: char) {
        let b = numbers.pop_back().unwrap();
        let a = numbers.pop_back().unwrap();
        let result = match operator {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => {
                if b == 0.0 {
                    println!("Invalid input: Division by zero");
                    return; // Exit the function if division by zero
                    }
                a / b
            }
            _ =>      {println!("Invalid input: Invalid operator");
            return;} // Exit the function if invalid operator

        };
        numbers.push_back(result);
    }

    let mut prev_char: Option<char> = None;

    for c in sequence.chars() {
        if c.is_digit(10) || c == '.' {
            current_number.push(c);
        } else if c == '-' && (prev_char.is_none() || "+-*/(".contains(prev_char.unwrap())) {
            // Handle negation as a standalone number
            if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<f64>() {
                    numbers.push_back(num);
                    current_number.clear();
                }
            }
            current_number.push(c); // Add the '-' to current number
        } else if "+-*/()".contains(c) {
            if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<f64>() {
                    numbers.push_back(num);
                    current_number.clear();
                } else {
                    return "Invalid input".to_string();
                }
            }

            if c == '(' {
                operators.push_back(c);
            } else if c == ')' {
                while let Some(&top_operator) = operators.back() {
                    if top_operator == '(' {
                        operators.pop_back();
                        break;
                    }
                    apply_operator(&mut numbers, operators.pop_back().unwrap());
                }
            } else {
                while let Some(&top_operator) = operators.back() {
                    if precedence(top_operator) >= precedence(c) {
                        apply_operator(&mut numbers, operators.pop_back().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push_back(c);
            }
        }
        prev_char = Some(c);
    }

    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<f64>() {
            numbers.push_back(num);
        } else {
            return "Invalid input".to_string();
        }
    }


    while let Some(operator) = operators.pop_back() {
        apply_operator(&mut numbers, operator);
    }

    // Round the final result
    let final_result = numbers.pop_back().map_or("Invalid input".to_string(), |result| {
        format!("{:.4}", result) // Round to 2 decimal places
    });

    final_result}


pub fn toggle_last_number_sign(clicked_buttons: &mut ClickedButtons) {
    if let Some(last) = clicked_buttons.buttons.last_mut() {
        if let Ok(mut number) = last.parse::<f64>() {
            number = -number;
            *last = number.to_string(); // Update the last button to reflect the new sign
        } else if last == "+" {
            *last = "-".to_string(); // Toggle between "+" and "-" if the last entry is a sign
        } else if last == "-" {
            *last = "+".to_string();
        }
    }
}

pub fn precedence(operator: char) -> usize {
    match operator {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}
