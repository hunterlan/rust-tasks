fn main() {
    // Task 1
    let x = 3;
    let m = 2;

    let mut result = 0;

    if x > m {
        result = (x * x) - (m * m);
    } else {
        result = 5 * (x * x) - x + (3 * m);
    }

    // Task 2
    let letter_mark = 'A';
    let mut number_mark = 2;

    if letter_mark == 'A' {
        number_mark = 5;
    } else if letter_mark == 'B' || letter_mark == 'C' {
        number_mark = 4;
    } else {
        number_mark = 3;
    }

    // Task 5
    let a = 90;
    let b = 90;
    let c = 90;
    let d = 90;

    let mut rectangle_exists = 0;

    if a == b && a == c && a == d {
        if a == 90 {
            rectangle_exists = 1;
        }
    }
}
