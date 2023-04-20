fn main() {
    let list_1 = vec![1, 230, 232, 20, 5, 6, 100];
    println!("Largest num in list_1 -- {:}", largest_num(&list_1));

    let list_2 = vec![40, 1200, 50000, 12121344, 24524, 425, 0];
    println!("Largest num in list_2 -- {:}", largest_num(&list_2));

    // Float list.
    let list_3 = vec![12.12, 124.12, 124325.32, 23423_f64];
    println!("Largest num in list_3 -- {:}", largest_num(&list_3));

    // Char list
    let list_4 = vec!['a', 'b', 'd', 'a', '1', '5', '%'];
    println!("Largest num in list_4 -- {:}", largest_num(&list_4));
}

// Find largest num in list
fn largest_num<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}
