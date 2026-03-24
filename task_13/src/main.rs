/// Узел двоичного дерева.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// Возможно пустое поддерево.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

/// Контейнер сохраняющий множество значений, с помощью двоичного дерева.
///
/// Если одно значение добавляется несколько раз, сохраняется только одно.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self { root: Subtree::new() }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }
}

// Реализуйте методы new, insert, len, и has для `Subtree`.
impl<T: Ord> Subtree<T> {
    /// Создает новое пустое поддерево
    fn new() -> Self {
        Subtree(None)
    }

    /// Вставляет значение в поддерево
    fn insert(&mut self, value: T) {
        match &mut self.0 {
            // Если поддерево не пустое
            Some(node) => {
                // Сравниваем вставляемое значение со значением в узле
                match value.cmp(&node.value) {
                    std::cmp::Ordering::Less => {
                        // Меньше → вставляем в левое поддерево
                        node.left.insert(value);
                    }
                    std::cmp::Ordering::Greater => {
                        // Больше → вставляем в правое поддерево
                        node.right.insert(value);
                    }
                    std::cmp::Ordering::Equal => {
                        // Равно → ничего не делаем (множество хранит уникальные значения)
                    }
                }
            }
            // Если поддерево пустое
            None => {
                // Создаем новый узел с пустыми поддеревьями
                self.0 = Some(Box::new(Node {
                    value,
                    left: Subtree::new(),
                    right: Subtree::new(),
                }));
            }
        }
    }

    /// Проверяет, содержится ли значение в поддереве
    fn has(&self, value: &T) -> bool {
        match &self.0 {
            Some(node) => {
                match value.cmp(&node.value) {
                    std::cmp::Ordering::Less => node.left.has(value),   // ищем в левом
                    std::cmp::Ordering::Greater => node.right.has(value), // ищем в правом
                    std::cmp::Ordering::Equal => true, // нашли!
                }
            }
            None => false, // пустое поддерево → значение не найдено
        }
    }

    /// Возвращает количество элементов в поддереве
    fn len(&self) -> usize {
        match &self.0 {
            Some(node) => {
                // Длина = 1 (текущий узел) + длина левого + длина правого
                1 + node.left.len() + node.right.len()
            }
            None => 0, // пустое поддерево
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> =
                (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}