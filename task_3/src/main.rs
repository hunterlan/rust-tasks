fn main() {
    //Task1

    // Start values
    let mut start_sum = 1000;
    let percent = 10.00;
    let years = 5;
    let mut calculating_result = start_sum as f64;

    let float_percent = 1.00 + (percent / 100.00);

    let mut i = 1;

    let _result = loop {
        calculating_result *= float_percent;

        if i > years {
            break calculating_result;
        }

        i += 1;
    };


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

    let divider = 36;
    let mut is_divider_perfect = false;

    let mut k = 1;
    let mut summary = 0;

    while k < divider {
        if divider % k == 0 {
            summary += k;
        }

        k+=1;
    }

    if summary == divider {
        is_divider_perfect = true;
    }

    //Task4

    let start_number = 1234231;

    let mut number_zero_count = 0;
    let mut number_one_count = 0;
    let mut number_two_count = 0;
    let mut number_three_count = 0;
    let mut number_four_count = 0;
    let mut number_five_count = 0;
    let mut number_six_count = 0;
    let mut number_seven_count = 0;
    let mut number_eight_count = 0;
    let mut number_nine_count = 0;

    let mut copy_start_number = start_number;

    while copy_start_number > 0 {
        let number = copy_start_number % 10;

        match number {
            0 => number_zero_count += 1,
            1 => number_one_count += 1,
            2 => number_two_count += 1,
            3 => number_three_count += 1,
            4 => number_four_count += 1,
            5 => number_five_count += 1,
            6 => number_six_count += 1,
            7 => number_seven_count += 1,
            8 => number_eight_count += 1,
            9 => number_nine_count += 1,
            _ => number_zero_count += 1,
        }

        copy_start_number = copy_start_number / 10;
    }

    //Task5
    let factorial_number = 75;
    let mut factorial_calculation = 10;
    let mut j = factorial_number;
    while j > 0 {
        factorial_calculation *= j;
        j -= 1;
    }
}
