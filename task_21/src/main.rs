use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>, // умный указатель с разделяемым владением
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>, // много отправителей - один получатель. канал передачи сообщенйи между потоками
    // пять философов, каждый в сыоём потоке, передают сообщения в главный поток main
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
        thread::sleep(Duration::from_millis(1));
    }

    fn eat(&self) {
        // Взять вилки ...
        if self.name == "Пифагор" {
            let _right = self.right_fork.lock().unwrap();
            let _left = self.left_fork.lock().unwrap();
            println!("{} ест ...", &self.name);
            thread::sleep(Duration::from_millis(10));
        } else {
            let _left = self.left_fork.lock().unwrap();
            let _right = self.right_fork.lock().unwrap();
            println!("{} ест ...", &self.name);
            thread::sleep(Duration::from_millis(10));
        }
        // нам не нужно самостоятельно высвобождать блокировки мьютексов, ибо это делается автоматически при выходе из области видимости
    }
}

static PHILOSOPHERS: &[&str] =
    &["Сократ", "Гипатия", "Платон", "Aристотель", "Пифагор"];

fn main() {
    // создаём канал
    let (tx, rx) = mpsc::channel();
    // Создать вилки
    let forks = (0..5).map(|_| Arc::new(Mutex::new(Fork))).collect::<Vec<_>>();
    
    let philo = PHILOSOPHERS.iter().enumerate().map(|(i, name)| 
        Philosopher { 
            name: name.to_string(), 
            left_fork: Arc::clone(&forks[i]), // можно так написать:  forks[i].clone()
            right_fork: Arc::clone(&forks[(i+1) % 5]), 
            thoughts: tx.clone()}).collect::<Vec<_>>();
            
    
    // Дать им поесть и подумать 100 раз
    let handles = philo.into_iter() // передаёт владение into_iter
    .map(|philo| { thread::spawn(move || { // создаём новый поток spawn, move захватывает philo
    for _ in 0..100 {
            philo.think();
            philo.eat();
            }})}
        ).collect::<Vec<_>>();
            
    // Вывести их мысли
    drop(tx);
    
    for dym in rx{
        println!("{}", dym);
    }
    
    for potok in handles{
        potok.join().unwrap();
    }
}