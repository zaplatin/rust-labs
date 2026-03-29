/// Вычислите разность между значениями values на расстоянии offset друг от друга,
/// переходя по модулю в начало коллекции.
///
/// Элемент n результата это разность values[(n+offset)%len] - values[n].
fn offset_differences(offset: usize, values: Vec<i32>) -> Vec<i32> {
    let len = values.len();
    
    // Если вектор пустой, сразу возвращаем пустой результат
    if len == 0 {
        return vec![];
    }
    
    // Создаем итератор по индексам 0..len
    (0..len)
        // Для каждого индекса вычисляем разность
        .map(|i| {
            // Индекс следующего элемента с учетом циклического перехода
            let next_index = (i + offset) % len;
            // Разность: следующий элемент минус текущий
            values[next_index] - values[i]
        })
        // Собираем все результаты в вектор
        .collect()
}

#[test]
fn test_offset_one() {
    assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
    assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
    assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
    assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
    assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
    assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_degenerate_cases() {
    assert_eq!(offset_differences(1, vec![0]), vec![0]);
    assert_eq!(offset_differences(1, vec![1]), vec![0]);
    let empty: Vec<i32> = vec![];
    assert_eq!(offset_differences(1, empty), vec![]);
}

fn main() {
    // Пример для демонстрации
    let values = vec![1, 3, 5, 7];
    println!("Исходный вектор: {:?}", values);
    println!("Разность с offset=1: {:?}", offset_differences(1, values.clone()));
    println!("Разность с offset=2: {:?}", offset_differences(2, values.clone()));
    println!("Разность с offset=3: {:?}", offset_differences(3, values));
}