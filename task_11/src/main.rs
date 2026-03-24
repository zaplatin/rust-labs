pub trait Logger {
    /// Помещает в лог сообщения заданного уровня.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

// Определяем структуру Filter с обобщенным типом F для замыкания
struct Filter<F> {
    inner: StderrLogger,
    predicate: F,
}

// Определяет методы самой структуры	Для работы с Filter как с типом
impl<F> Filter<F>           // 1. Объявляем обобщенный тип F
where                        // 2. Начинаем блок ограничений/ where — ключевое слово для перечисления требований к типам
    F: Fn(u8, &str) -> bool, // 3. Ограничение: F должно быть вызываемым. F: Fn(u8, &str) -> bool — говорит: "Тип F должен реализовывать трейт Fn"
{
    fn new(inner: StderrLogger, predicate: F) -> Self { // 4. Метод new. Ассоциированная функция. Принимает: inner (логгер) и predicate (замыкание)
        Filter { inner, predicate } // Возвращает Self (т.е. Filter<F>)
    }
}
// Реализует внешний трейт	Чтобы Filter соответствовал интерфейсу Logger
impl<F> Logger for Filter<F>
where
    F: Fn(u8, &str) -> bool,
{
    fn log(&self, verbosity: u8, message: &str) {
        // Вызываем замыкание для проверки, нужно ли логировать сообщение
        if (self.predicate)(verbosity, message) {   // (self.predicate) — получаем доступ к полю predicate. Скобки обязательны, чтобы отделить вызов от синтаксиса метода. (verbosity, message) — передаем параметры в замыкание
            self.inner.log(verbosity, message);     // (передает) логирование внутреннему логгеру после успешной фильтрации.
        }
        // Если замыкание вернуло false - игнорируем сообщение
    }
}

fn main() {
    // Создаем фильтр, который пропускает только сообщения, содержащие "yikes"
    let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
    /*
    StderrLogger,	Первый аргумент — базовый логгер (вывод в stderr)
    |_verbosity, msg| msg.contains("yikes")	Второй аргумент — замыкание-фильтр
    _verbosity — уровень логирования (не используется, поэтому с подчеркиванием)
    msg — само сообщение
    msg.contains("yikes") — проверяет, содержит ли сообщение подстроку "yikes"
    Возвращает bool: true если содержит, false если нет
    Тип замыкания выводится автоматически: Fn(u8, &str) -> bool
     */

    
    logger.log(5, "FYI");                    // Не содержит "yikes" → не выведется
    logger.log(1, "yikes, something went wrong"); // Содержит "yikes" → выведется
    logger.log(2, "uhoh");                   // Не содержит "yikes" → не выведется
}