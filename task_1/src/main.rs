fn main() {
    // Task 1
    let a = 2;
    let a4 = (a * a) * (a * a);
    let a6 = (a * a) * (a * a) * (a * a);
    let _a8 = a4 * a4;
    let _a10 = a4 * a6;

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

    let _square_distance = (x_result * x_result) + (y_result * y_result) + (z_result * z_result);

    //Task 3
    let mut _number = 1234;
    let number_four = _number % 10;
    _number /= 10;
    let number_three = _number % 10;
    _number /= 10;
    let number_two = _number % 10;
    _number /= 10;
    let number_one = _number % 10;

    let _sum = number_four + number_three + number_two + number_one;

    //Task 4
    //It's important to specify numbers, because it will throw an error with unspecified ones
    let radius = 10.00;
    const PI: f64 = std::f64::consts::PI;
    let _l = (2.00 * PI) * radius;
    let _s = PI * (radius * radius);
    let _v = (4.00/3.00) * PI * (radius * radius * radius);

    //Task 5
    _number = 123;
    let three = _number % 10;
    _number /= 10;
    let two = _number % 10;
    _number /= 10;
    let one = _number % 10;
    _number = (three * 100) + (two * 10) + one;

    //It's necessary to see final results of each task before losing them.
    let pause_one = 1;
    let pause_two = 2;
    let _pause_result = pause_one + pause_two;
}
