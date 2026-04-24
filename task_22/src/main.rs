use reqwest::blocking::Client;
use reqwest::Url;
use scraper::{Html, Selector};
use thiserror::Error;
use std::collections::HashSet;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// --- Обработка ошибок ---
#[derive(Error, Debug)]
enum Error {
    #[error("ошибка запроса: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("неправильный http ответ: {0}")]
    BadResponse(String),
}

// --- Команда для воркера ---
#[derive(Debug)]
struct CrawlCommand {
    url: Url,
    extract_links: bool, // всегда true для внутренних ссылок
}

// --- Функция посещения страницы (без изменений) ---
fn visit_page(client: &Client, command: &CrawlCommand) -> Result<Vec<Url>, Error> {
    println!("Проверяем {:#}", command.url);
    let response = client.get(command.url.clone()).send()?;
    if !response.status().is_success() {
        return Err(Error::BadResponse(response.status().to_string()));
    }

    let mut link_urls = Vec::new();
    if !command.extract_links {
        return Ok(link_urls);
    }

    let base_url = response.url().to_owned();
    let body_text = response.text()?;
    let document = Html::parse_document(&body_text);

    let selector = Selector::parse("a").unwrap();
    let href_values = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"));
    for href in href_values {
        match base_url.join(href) {
            Ok(link_url) => {
                link_urls.push(link_url);
            }
            Err(err) => {
                println!("Ссылку {base_url:#}: невозможно разобрать {href:?}: {err}");
            }
        }
    }
    Ok(link_urls)
}

// --- Главная функция ---
fn main() {
    // Настройка HTTP-клиента
    let client = Client::builder()
        .user_agent("link-checker/1.0")
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Не удалось создать HTTP-клиент");

    // let start_url = Url::parse("https://ru.wikipedia.org").unwrap();
    // let start_url = Url::parse("https://ya.ru").unwrap();
    let start_url = Url::parse("https://lenta.ru").unwrap();
    let target_domain = start_url.domain().unwrap().to_string();

    // Канал для отправки команд воркерам (multiple producers, single consumer)
    let (cmd_tx, cmd_rx) = mpsc::channel::<CrawlCommand>();
    // Разделяемый приёмник команд (воркеры будут конкурировать через Mutex)
    let shared_cmd_rx = Arc::new(Mutex::new(cmd_rx));

    // Канал для возврата найденных ссылок от воркеров главному потоку
    let (result_tx, result_rx) = mpsc::channel::<Url>();

    // Общие данные: посещённые URL и счётчик обработанных страниц
    let visited = Arc::new(Mutex::new(HashSet::new()));
    let processed_count = Arc::new(Mutex::new(0));
    const MAX_LINKS: usize = 100;

    // Добавляем стартовый URL как посещённый и отправляем первую команду
    {
        let mut vis = visited.lock().unwrap();
        vis.insert(start_url.to_string());
    }
    cmd_tx
        .send(CrawlCommand {
            url: start_url.clone(),
            extract_links: true,
        })
        .unwrap();

    // Запускаем пул воркеров
    let num_workers = 4;
    let mut worker_handles = Vec::new();

    for id in 0..num_workers {
        let worker_client = client.clone();
        let worker_cmd_rx = Arc::clone(&shared_cmd_rx);
        let worker_result_tx = result_tx.clone();
        let worker_processed_count = Arc::clone(&processed_count);
        let worker_target_domain = target_domain.clone();

        let handle = thread::spawn(move || {
            loop {
                // Получаем следующую команду
                let cmd = {
                    let rx = worker_cmd_rx.lock().unwrap();
                    match rx.recv() {
                        Ok(cmd) => cmd,
                        Err(_) => break, // Канал закрыт – выходим
                    }
                };

                // Проверяем, не превышен ли лимит
                {
                    let count = worker_processed_count.lock().unwrap();
                    if *count >= MAX_LINKS {
                        continue; // Пропускаем обработку, ждём новых команд (их не будет)
                    }
                }

                // Обрабатываем страницу
                match visit_page(&worker_client, &cmd) {
                    Ok(links) => {
                        // Увеличиваем счётчик обработанных
                        {
                            let mut count = worker_processed_count.lock().unwrap();
                            *count += 1;
                            println!("Обработано {}: {}", *count, cmd.url);
                        }

                        // Отправляем найденные внутренние ссылки в канал результатов
                        for link in links {
                            if let Some(domain) = link.domain() {
                                if domain == worker_target_domain {
                                    let _ = worker_result_tx.send(link);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Ошибка при обработке {}: {}", cmd.url, e);
                    }
                }
            }
            println!("Воркер {} завершил работу", id);
        });
        worker_handles.push(handle);
    }

    // Закрываем result_tx в главном потоке, чтобы result_rx мог завершиться
    drop(result_tx);

    // Главный поток читает найденные ссылки и пополняет очередь команд
    for link in result_rx {
        let link_str = link.to_string();
        let mut vis = visited.lock().unwrap();
        if !vis.contains(&link_str) {
            let count = processed_count.lock().unwrap();
            if *count >= MAX_LINKS {
                break; // Достигнут лимит – прекращаем добавление новых команд
            }
            vis.insert(link_str);
            // Отправляем новую команду на обработку
            if cmd_tx
                .send(CrawlCommand {
                    url: link,
                    extract_links: true,
                })
                .is_err()
            {
                break; // Канал закрыт – выходим
            }
        }
    }

    // Закрываем канал команд, чтобы воркеры завершились
    drop(cmd_tx);

    // Ожидаем завершения всех воркеров
    for handle in worker_handles {
        handle.join().unwrap();
    }

    println!(
        "Проверка завершена. Обработано ссылок: {}",
        *processed_count.lock().unwrap()
    );
}