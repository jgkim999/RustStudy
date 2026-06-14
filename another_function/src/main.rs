fn main() {
    println!("Hello, world!");

    another_function(4, 'h');
}

fn another_function(value: i32, unit_label: char) {
    println!("The value of x is: {value}");
    println!("The unit label is {unit_label}");
    println!("The measurement is: {value}{unit_label}");
}
