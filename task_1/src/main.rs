fn main() {
    // Task 1
    let a = 2;
    let a4 = (a * a) * (a * a);
    let a6 = (a * a) * (a * a) * (a * a);
    let a8 = a4 * a4;
    let a10 = a4 * a6;

    //Task 2

    let x_1 = 1;
    let y_1 = 2;
    let z_1 = 3;

    let x_2 = 4;
    let y_2 = 5;
    let z_2 = 6;

    let x_result = x_2 - x_1;
    let y_result = y_2 - y_1;
    let z_result = z_2 - z_1;

    let square_distance = (x_result * x_result) + (y_result * y_result) + (z_result * z_result);

    //Task 3
    let mut number = 1234;
    let number_four = number % 10;
    number /= 10;
    let number_three = number % 10;
    number /= 10;
    let number_two = number % 10;
    number /= 10;
    let number_one = number % 10;

    let sum = number_four + number_three + number_two + number_one;

    //Task 4
    //It's important tp specify numbers, because it will throw an error with unspecified ones
    let radius = 10.00;
    const PI: f64 = std::f64::consts::PI;
    let l = (2.00 * PI) * radius;
    let s = PI * (radius * radius);
    let v = (4.00/3.00) * PI * (radius * radius * radius);

    //Task 5
    number = 123;
    let three = number % 10;
    number /= 10;
    let two = number % 10;
    number /= 10;
    let one = number % 10;
    number = (three * 100) + (two * 10) + one;

    //It's necessary to see final results of each task before losing them.
    let pause_one = 1;
    let pause_two = 2;
    let pause_result = pause_one + pause_two;
}
