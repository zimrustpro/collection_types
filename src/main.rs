fn main() {
    let _array1 = ["One", "Two"];
    let _array2 = ["One", "Two", "Five"];
    let my_array = ["a"; 5];
    println!("{:?}", my_array);
    println!("{:?}", b"Hello there");
    let my_numbers = [0, 10, -20];
    println!("{}", my_numbers[1]);

    let array_of_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("Array: {:?}", array_of_ten);

    let two_of_five = &array_of_ten[2..5];
    println!("2..5 = {:?}", two_of_five);

    let start_at_one = &array_of_ten[1..];
    println!("1.. = {:?}", start_at_one);

    let end_at_five = &array_of_ten[..=5];
    println!("..=5 = {:?}", end_at_five);

    let everything = &array_of_ten[..];
    println!("everything = {:?}", everything);
}
