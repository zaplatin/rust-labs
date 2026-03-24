use std::cmp::Ordering; // подключаем библиотеку std в RUST

// Обобщенная функция min, работающая с любыми типами, реализующими трейт Ord
fn min<T: Ord>(a: T, b: T) -> T {
    // Используем метод cmp из трейта Ord для сравнения
    match a.cmp(&b) {   // Метод cmp принимает ссылку на второй аргумент, чтобы не забирать владение и не копировать значение. а на первый аргумент итак берется ссылка
        Ordering::Less | Ordering::Equal => a, // если a меньше или равен b, возвращаем a. Ordering — это перечисление (enum) с тремя вариантами
        Ordering::Greater => b,                 // если a больше b, возвращаем b
    }
}

fn main() {
    assert_eq!(min(0, 10), 0);
    println!("min(0, 10) = {}", min(0, 10));

    assert_eq!(min(500, 123), 123);
    println!("min(500, 123) = {}", min(500, 123));

    assert_eq!(min('a', 'z'), 'a');
    println!("min('a', 'z') = {}", min('a', 'z'));

    assert_eq!(min('7', '1'), '1');
    println!("min('7', '1') = {}", min('7', '1'));

    assert_eq!(min("hello", "goodbye"), "goodbye");
    println!("Тест 5 пройден");

    assert_eq!(min("bat", "armadillo"), "armadillo");
    println!("Тест 6 пройден");
}