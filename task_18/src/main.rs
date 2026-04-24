/// Проверяет, проходит ли номер кредитной карты алгоритм Луна.
/// Поддерживает пробелы, игнорирует нецифровые символы.
fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut double = false;

    // Проходим по символам справа налево
    for c in cc_number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            if double {
                let double_digit = digit * 2;
                // Если результат больше 9, вычитаем 9 (эквивалентно сумме цифр)
                sum += if double_digit > 9 { double_digit - 9 } else { double_digit };
            } else {
                sum += digit;
            }
            // Переключаем флаг для следующей цифры
            double = !double;
        }
        // Нецифровые символы (пробелы, дефисы, буквы) игнорируем
    }

    // Сумма должна быть кратна 10
    sum % 10 == 0
}

fn main() {
    let test_cards = [
        "4263 9826 4026 9299", // Валидный
        "4539 3195 0343 6467", // Валидный
        "7992 7398 713",       // Валидный
        "4223 9826 4026 9299", // Невалидный
        "4539 3195 0343 6476", // Невалидный
        "8273 1232 7352 0569", // Невалидный
    ];

    println!("Проверка номеров карт алгоритмом Луна:\n");
    for card in test_cards {
        let result = if luhn(card) { "✅ Валиден" } else { "❌ Невалиден" };
        println!("{} : {}", card, result);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Исходные тесты
    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }

    // Дополнительные тесты
    #[test]
    fn test_empty_string() {
        assert!(!luhn(""));
    }

    #[test]
    fn test_single_digit_zero() {
        assert!(!luhn("0"));
    }

    #[test]
    fn test_only_spaces() {
        assert!(!luhn("   "));
    }

    #[test]
    fn test_non_space_separators() {
        // Дефисы не считаются пробелами, они игнорируются, но номер невалидный
        assert!(!luhn("4539-3195-0343-6467"));
    }

    #[test]
    fn test_letters_mixed() {
        assert!(!luhn("abc4539319503436467"));
    }

    #[test]
    fn test_simple_valid_without_spaces() {
        assert!(luhn("79927398713")); // Известный валидный пример
    }
}