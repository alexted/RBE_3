// unit (единичная/пустая) структура
struct Unit;

// Кортежная структура / именованный кортеж
struct Pair(i32, f32);

// Структура с двумя именованными полями
struct Point {
    x: f32,
    y: f32,
}

// Структуры могут быть использованы в качестве полей другой структуры
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle { top_left: p1, bottom_right: p2 } = rectangle;
    ((p1.x - p2.x) * (p1.y - p2.y)).abs()
}

fn square(point: Point, value:f32) -> Rectangle {
    let Point { x: a, y: b } = point;
    Rectangle {
        top_left: point,
        bottom_right: Point { x: a + value, y: b + value}
    }
}


pub fn play_with_structs() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person {name, age };

    // println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };

    // println!("координаты точки: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };

    // println!("вторая точка: ({}, {})", bottom_right.x, bottom_right.y);

    // Деструктурируем структуру при помощи `let`
    let Point { x: left_edge, y: top_edge} = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    // println!("pair содержит {:?} и {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    // println!("pair содержит {:?} и {:?}", integer, decimal);

    let my_rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: Point { x: 4.5, y: 7.4 },
    };

    let r1 = rect_area(my_rectangle);
    println!("Площадь my_rectangle равна: {:.2}", r1);

    let my_rectangle = square(point, 6.0);
    println!(
        "Создан новый rectangle с точками: ({}, {}) ({}, {})",
        my_rectangle.top_left.x, my_rectangle.top_left.y,
        my_rectangle.bottom_right.x, my_rectangle.bottom_right.y
    );
}