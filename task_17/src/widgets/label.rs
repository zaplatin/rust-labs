// Модуль label — реализация виджета Label

use super::Widget;  // Импортируем трейт Widget из родительского модуля

/// Простой текстовый виджет.
pub struct Label {
    label: String,
}

impl Label {
    /// Создает новый Label с заданным текстом.
    pub fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

impl Widget for Label {
    fn width(&self) -> usize {
        // Ширина Label — максимальная длина строки
        self.label
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0)
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        // Просто выводим текст
        writeln!(buffer, "{}", &self.label).unwrap();
    }
}