pub fn run_training_examples() {
    // Chapter 2: Data Types
    let age: i32 = 21;
    let height: f64 = 6.3;
    let is_student: bool = true;
    let grade: char = 'A';

    println!("Age: {}", age);
    println!("Height: {}", height);
    println!("Student: {}", is_student);
    println!("Grade: {}", grade);

    // Chapter 3: Variables
    let mut score = 80;
    println!("Original score: {}", score);

    score = 95;
    println!("New score: {}", score);

    // Chapter 4: Functions
    let result = add_numbers(10, 5);
    println!("10 + 5 = {}", result);

    // Chapter 5: Control Flow
    if result > 10 {
        println!("The result is greater than 10.");
    } else {
        println!("The result is 10 or less.");
    }

    for number in 1..=5 {
        println!("Number: {}", number);
    }
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}