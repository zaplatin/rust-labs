// Модуль button — реализация виджета Button

use super::{Label, Widget};  // Импортируем Label и Widget

/// Кнопка — содержит текст (Label) и может быть нажата.
pub struct Button {
    label: Label,
}

impl Button {
    /// Создает новую кнопку с заданным текстом.
    pub fn new(label: &str) -> Button {
        Button {
            label: Label::new(label),
        }
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        // Ширина кнопки = ширина текста + отступы (4 слева + 4 справа)
        self.label.width() + 8
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = self.width();
        let mut label_buffer = String::new();
        self.label.draw_into(&mut label_buffer);

        // Верхняя граница
        writeln!(buffer, "+{:-<width$}+", "").unwrap();

        // Текст кнопки (центрированный)
        for line in label_buffer.lines() {
            writeln!(buffer, "|{:^width$}|", &line).unwrap();
        }

        // Нижняя граница
        writeln!(buffer, "+{:-<width$}+", "").unwrap();
    }
}