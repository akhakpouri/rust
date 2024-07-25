mod student;

fn main() {
    let mut x = 10;
    let word = "ten";
    println!("x is {}, the word is {}", x, word);
    x = 15;
    println!("now, x is {}, the word is {}", x, word);

    let num = 5;
    let num = num + 5;
    let num = num * 2;
    println!("shadow num is {}", num);

    let is_bigger = 1 > 4;
    println!("is 1 > 4? {}", is_bigger);

    let character_1: char = 'S';
    let character_2: char = 'f';
    let smiley_face = '😃';
    let string_1 = "miley ";
    let string_2: &str = "ace";

    println!(
        "{} is a {}{}{}{}.",
        smiley_face, character_1, string_1, character_2, string_2
    );

    let e = ('e', 5i32, true);
    println!(
        "Id '{}' tghe {}the letter of the alphabet? {}",
        e.0, e.1, e.2
    );

    let ali = student::Student {
        name: String::from("Ali Khakpuri"),
        remote: true,
        level: 2,
    };

    let jon = student::Student {
        name: String::from("Jon Doe"),
        remote: false,
        level: 1,
    };

    let grade_a = Grades('A', 'A', 'B', 'A', 3.75);
    let grade_b = Grades('B', 'B', 'A', 'A', 3.25);

    println!(
        "{}, level {} is remote {}. His grades are: {}, {}, {}, {}. His gpa is: {}",
        ali.name, ali.level, ali.remote, grade_a.0, grade_a.1, grade_a.2, grade_a.3, grade_a.4
    );

    println!(
        "{}, level {} is remote {}. His grades are: {}, {}, {}, {}. His gpa is: {}",
        jon.name, jon.level, jon.remote, grade_b.0, grade_b.1, grade_b.2, grade_b.3, grade_b.4
    );
}

struct Grades(char, char, char, char, f32);
