// Модуль window — реализация виджета Window

use super::Widget;  // Импортируем трейт Widget

/// Окно — контейнер для других виджетов.
pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    /// Создает новое окно с заданным заголовком.
    pub fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    /// Добавляет виджет в окно.
    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    /// Вычисляет внутреннюю ширину окна (без отступов).
    fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets
                .iter()
                .map(|w| w.width())
                .max()
                .unwrap_or(0),
        )
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        // Внешняя ширина = внутренняя ширина + 4 (отступы слева и справа)
        self.inner_width() + 4
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let inner_width = self.inner_width();

        // Собираем содержимое всех виджетов
        let mut inner = String::new();
        for widget in &self.widgets {
            widget.draw_into(&mut inner);
        }

        // Верхняя граница
        writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();

        // Заголовок окна (центрированный)
        writeln!(buffer, "| {:^inner_width$} |", &self.title).unwrap();

        // Разделитель между заголовком и содержимым
        writeln!(buffer, "+={:=<inner_width$}=+", "").unwrap();

        // Содержимое окна (виджеты)
        for line in inner.lines() {
            writeln!(buffer, "| {:inner_width$} |", line).unwrap();
        }

        // Нижняя граница
        writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();
    }
}