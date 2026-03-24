use std::collections::HashMap;
use std::hash::Hash;

/// Counter определяет количество значений типа T в коллекции.
/// T должен поддерживать сравнение (Eq) и хеширование (Hash)
struct Counter<T> {
    values: HashMap<T, u64>,    // Это объявление поля в структуре Counter. Оно говорит: "У каждого счетчика будет поле с именем values, которое представляет собой HashMap"
}

impl<T: Eq + Hash> Counter<T> { // реализую методы для Counter<T>, но только если тип T удовлетворяет определенным требованиям".
    /// Создаем новый счетчик Counter.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// Считает количество появлений заданного значения.
    fn count(&mut self, value: T) { // Увеличивает счетчик для переданного значения на 1. Если значение встречается впервые — добавляет его со счетчиком 1.
        if self.values.contains_key(&value) {
            *self.values.get_mut(&value).unwrap() += 1;
        } else {
            self.values.insert(value, 1);
        }
    }

    /// Возвращает количество появлений заданного значения.
    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

fn main() {
    // Счетчик для чисел (u32)
    let mut ctr = Counter::new();   // Создает новый пустой счетчик и делает его изменяемым. Вызов ассоциированной функции new у типа Counter
    ctr.count(13);  // Добавляет число 13 в счетчик (увеличивает счетчик для значения 13 на 1).
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("Значение {} видели {} раз", i, ctr.times_seen(i));
    }

    // Счетчик для строк (&str)
    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("Получили {} яблок", strctr.times_seen("apple"));
}