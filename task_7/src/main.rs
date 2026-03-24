pub trait Logger {
    /// логирует сообщение указанного уровня.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

/// Логировать сообщения только заданного уровня.
struct VerbosityFilter {
    max_verbosity: u8,
    inner: StderrLogger,
}

// Реализуем типаж `Logger` для `VerbosityFilter`
impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: &str) {
        // Проверяем, что уровень сообщения не превышает максимальный
        if verbosity <= self.max_verbosity {
            // Если уровень подходит, передаём логирование внутреннему логгеру
            self.inner.log(verbosity, message);
        }
        // Если уровень превышает max_verbosity — ничего не делаем (игнорируем)
    }
}

fn main() {
    let logger = VerbosityFilter { max_verbosity: 3, inner: StderrLogger };
    logger.log(5, "Какое то");    // Это сообщение НЕ будет выведено (5 > 3)
    logger.log(2, "Сообщение");    // Это сообщение будет выведено (2 ≤ 3)
}