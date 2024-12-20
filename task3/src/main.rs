fn main() {
    //Task1

    // Start values
    let mut start_sum = 1000;
    let percent = 10.00;
    let years = 5;
    let mut result = start_sum as f64;

    let float_percent = 1.00 + (percent / 100.00);

    let mut i = 1;
    while i <= years {
        result *= float_percent;
        i += 1;
    }
    //Task2
    let required_sum = 34191;
    let mut count_one_banknote = 0;
    let mut count_two_banknote = 0;
    let mut count_five_banknote = 0;
    let mut count_ten_banknote = 0;

    let mut i = required_sum;

    while i > 0 {
        if i % 10 == 0 {
            i -= 10;
            count_ten_banknote+=1;
        } else if i % 5 == 0 {
            i -= 5;
            count_five_banknote+=1;
        } else if i % 2 == 0 || i % 3 == 0 || i % 7 == 0 || i % 9 == 0 {
            i -= 2;
            count_two_banknote += 1;
        } else {
            i -= 1;
            count_one_banknote += 1;
        }
    }

    let result_count_banknotes = count_one_banknote + count_two_banknote
        + count_five_banknote + count_ten_banknote;
    //Task3

    //Task4

    //Task5
    let factorial_number = 75;
    let mut factorial_calculation = 10;
    let mut j = factorial_number;
    while j > 0 {
        factorial_calculation *= j;
        j -= 1;
    }
}
