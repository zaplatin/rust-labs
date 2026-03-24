#[derive(Debug)]
/// Событие лифта на которое должен реагировать контроллер.
enum Event  // Событие лифта, на которое должен реагировать контроллер. Event — это перечисление
{
    // Событие: лифт прибыл на этаж
    Arrived(i32),           // содержит номер этажа
    
    // Событие: двери открылись
    DoorOpened,
    
    // Событие: двери закрылись
    DoorClosed,
    
    // Событие: нажата кнопка вызова на этаже
    LobbyCall(i32, Direction),  // содержит этаж и направление
    
    // Событие: нажата кнопка этажа внутри кабины
    CarFloorPressed(i32),   // содержит номер этажа
}

/// A direction of travel.
#[derive(Debug)]

enum Direction  // Перечисление Direction — направление движения
{
    Up,
    Down,
}

// Дальше идут функции, которые создают события. Каждая функция соответствует реальному действию с лифтом.

fn car_arrived(floor: i32) -> Event // Кабина приехала на заданный этаж.
{
    Event::Arrived(floor)  // Возвращаем событие "прибыл на этаж"
}

/// Двери кабины открыты.
fn car_door_opened() -> Event
{
    Event::DoorOpened  // Возвращаем событие "двери открылись"
}

/// Двери кабины закрыты.
fn car_door_closed() -> Event
{
    Event::DoorClosed  // Возвращаем событие "двери закрылись"
}

/// Кнопка вызова лифта нажата на заданном этаже.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event
{
    Event::LobbyCall(floor, dir)  // Возвращаем событие "вызов с этажа"
}

/// Кнопка этажа нажата в кабине лифта.
fn car_floor_button_pressed(floor: i32) -> Event
{
    Event::CarFloorPressed(floor)  // Возвращаем событие "нажата кнопка этажа"
}

fn main()
{
    println!(
        "Пассажир на первом этаже нажал кнопку вызова: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("Лифт приехал на первый этаж: {:?}", car_arrived(0));
    println!("Дверь лифта открылась: {:?}", car_door_opened());
    println!(
        "Пассажир нажал кнопку третьего этажа: {:?}",
        car_floor_button_pressed(3)
    );
    println!("Двери лифта закрылись: {:?}", car_door_closed());
    println!("Лифт прибыл на третий этаж: {:?}", car_arrived(3));
}