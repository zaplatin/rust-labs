// Главный модуль приложения

mod widgets;  // Объявляем модуль widgets

use widgets::{Button, Label, Widget, Window};  // Импортируем необходимые типы

fn main() {
    let mut window = Window::new("Демонстрация графического интерфейса Rust 1.23");
    window.add_widget(Box::new(Label::new(
        "Это маленькая демонстрация графического интерфейса.",
    )));
    window.add_widget(Box::new(Button::new("Нажми меня!")));
    window.draw();
}