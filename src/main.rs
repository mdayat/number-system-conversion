fn main() {
    // 1. Binary to Octal
    let mut binary_num1 = 101001;
    let result1 = binary_to_octal(&mut binary_num1);
    println!("Hasil Convert Binary ke Octal adalah: {}", result1);

    // 2. Binary to Decimal
    let mut binary_num2 = 1101;
    let result2 = binary_to_decimal(&mut binary_num2);
    println!("Hasil Convert Binary ke Decimal adalah: {}", result2);

    // 3. Binary to Hexa
    let mut binary_num3 = 101010;
    let result3 = binary_to_decimal(&mut binary_num3);
    println!("Hasil Convert Binary ke Hexadecimal adalah: {:X}", result3);

    // 4. Decimal to Binary
    let mut decimal_num1 = 13;
    let result4 = decimal_to_binary(&mut decimal_num1);
    println!("Hasil Convert Decimal ke Binary adalah: {}", result4);

    // 5. Decimal to Octal
    let mut decimal_num2 = 78;
    let result5 = decimal_to_octal(&mut decimal_num2);
    println!("Hasil Convert Decimal ke Octal adalah: {}", result5);

    // 6. Decimal to Hexa
    let decimal_num3 = 42;
    println!(
        "Hasil Convert Decimal ke Hexadecimal adalah: {:X}",
        decimal_num3
    );
}

fn binary_to_octal(binary_num: &mut i32) -> i32 {
    let mut decimal_num = binary_to_decimal(binary_num);
    return decimal_to_octal(&mut decimal_num);
}

fn binary_to_decimal(binary_num: &mut i32) -> i32 {
    let mut decimal_num = 0;
    let mut binary_pow = 0;

    while *binary_num != 0 {
        decimal_num += (*binary_num % 10) * i32::pow(2, binary_pow);
        *binary_num /= 10;
        binary_pow += 1;
    }

    return decimal_num;
}

fn decimal_to_binary(decimal_num: &mut i32) -> i32 {
    let mut binary_num = 0;
    let mut decimal_pow = 0;

    while *decimal_num != 0 {
        binary_num += (*decimal_num % 2) * i32::pow(10, decimal_pow);
        *decimal_num /= 2;
        decimal_pow += 1;
    }

    return binary_num;
}

fn decimal_to_octal(decimal_num: &mut i32) -> i32 {
    let mut octal_num = 0;
    let mut decimal_pow = 0;

    while *decimal_num != 0 {
        octal_num += (*decimal_num % 8) * i32::pow(10, decimal_pow);
        *decimal_num /= 8;
        decimal_pow += 1;
    }

    return octal_num;
}
