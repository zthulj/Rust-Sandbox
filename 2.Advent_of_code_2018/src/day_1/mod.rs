
use std::collections::HashSet;

pub fn execute(_input: String) -> (i32, i32) {
    let changes = {
        let lines = _input.lines();
        let mapped_lines = lines.map(
            |e| {
                match e.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => 0
                }
            }).collect::<Vec<i32>>();
        mapped_lines
    };

    (
        calculate_total(&changes),
        calculate_frequency_repeat(&changes)
    )
}

fn calculate_total(_changes: &Vec<i32>) -> i32 {
    _changes.iter().sum()
}

fn calculate_frequency_repeat(_changes: &Vec<i32>) -> i32 {
    let mut result = 0;
    let mut finded_frequencies= HashSet::new();
    finded_frequencies.insert(0);

    for change in _changes.iter().cycle() {
        result += change;
        if finded_frequencies.contains(&result) {
            break;
        }
        finded_frequencies.insert(result);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_sum() {
        assert_eq!(calculate_total(&vec!(1, 1, 1)), 3);
        assert_eq!(calculate_total(&vec!(1, 1, -2)), 0);
        assert_eq!(calculate_total(&vec!(-1, -2, -3)), -6);
    }

    #[test]
    fn test_frequencies() {
        assert_eq!(calculate_frequency_repeat(&vec!(1, -1)), 0);
        assert_eq!(calculate_frequency_repeat(&vec!(3, 3, 4, -2, -4)), 10);
        assert_eq!(calculate_frequency_repeat(&vec!(-6, 3, 8, 5, -6)), 5);
        assert_eq!(calculate_frequency_repeat(&vec!(7, 7, -2, -7, -4)), 14);
    }
}
