#[derive(Debug)]
enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Clone, Debug)]
struct Dependency {
    name: String,
    version_expression: String,
}

/// Описывает пакет программного обеспечения.
#[derive(Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    language: Option<Language>,
}

impl Package {
    /// Возвращает этот пакет как зависимость необходимую для
    /// компиляции другого пакета.
    fn as_dependency(&self) -> Dependency {
        // 1. Создаем зависимость из текущего пакета
        Dependency {
            name: self.name.clone(),                    // копируем имя
            version_expression: self.version.clone(),   // копируем версию
        }
    }
}

/// Компилятор пакета Package. Использует метод build() для создания пакета Package
struct PackageBuilder(Package);  // кортежная структура, содержащая Package

impl PackageBuilder {
    fn new(name: impl Into<String>) -> Self {
        // 2. Создаем новый билдер с пустым пакетом
        PackageBuilder(Package {
            name: name.into(),              // преобразуем в String
            version: String::new(),          // пустая версия
            authors: Vec::new(),             // пустой список авторов
            dependencies: Vec::new(),        // пустой список зависимостей
            language: None,                  // язык не указан
        })
    }

    /// Задает версию пакета.
    fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    /// Задает автора пакета.
    fn authors(mut self, authors: Vec<String>) -> Self {
        // 3. Устанавливаем список авторов
        self.0.authors = authors;
        self
    }

    /// Добавляет зависимость.
    fn dependency(mut self, dependency: Dependency) -> Self {
        // 4. Добавляем зависимость в вектор
        self.0.dependencies.push(dependency);
        self
    }

    /// Задает язык. Если не указан язык, используется значение по умолчанию None.
    fn language(mut self, language: Language) -> Self {
        // 5. Устанавливаем язык (оборачиваем в Some)
        self.0.language = Some(language);
        self
    }

    fn build(self) -> Package {
        // Возвращаем готовый пакет
        self.0
    }
}

fn main() {
    let base64 = PackageBuilder::new("base64").version("0.13").build();
    println!("base64: {base64:?}");
    
    let log = PackageBuilder::new("log")
        .version("0.4")
        .language(Language::Rust)
        .build();
    println!("log: {log:?}");
    
    let serde = PackageBuilder::new("serde")
        .authors(vec!["djmitche".into()])
        .version(String::from("4.0"))
        .dependency(base64.as_dependency())
        .dependency(log.as_dependency())
        .build();
    println!("serde: {serde:?}");
}