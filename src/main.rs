fn main() {
    let name1 = String::from("Blessed");
    let name2 = String::from("Chiedza");
    let mut my_vec = Vec::new();
    my_vec.push(name1);
    my_vec.push(name2);
    println!("{:?}", my_vec);

    let my_vec = vec![8, 10, 10];
    println!("{:?}", my_vec);

    let vec_of_ten = vec![1,2,3,4,5,6,7,8,9,10];

    let three_to_five = &vec_of_ten[2..5];
    println!("{:?}", three_to_five);

    let start_at_two = &vec_of_ten[1..];
    println!("{:?}", start_at_two);

    let my_vec: Vec<u8> = [1,2,3].into();
    println!("{:?}", my_vec);

    let my_vec2: Vec<_> = [9,0,10].into();
    println!("{:?}", my_vec2);
}
