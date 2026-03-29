// Модуль widgets — точка входа для всех виджетов

mod button;   // Объявляем подмодуль button
mod label;    // Объявляем подмодуль label
mod window;   // Объявляем подмодуль window

// Публично экспортируем типы из подмодулей
pub use button::Button;
pub use label::Label;
pub use window::Window;

// Трейт Widget должен быть доступен всем виджетам
// и пользователям библиотеки
pub trait Widget {
    /// Ширина виджета.
    fn width(&self) -> usize;

    /// Прорисовка виджета в буфер.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Прорисовка виджета.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}