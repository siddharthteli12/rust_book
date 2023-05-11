#![allow(dead_code)]
#[derive(Debug)]
struct Human<T> {
    age: T,
    height: T,
    name: String,
}

impl<T> Human<T> {
    fn new(age: T, height: T, name: String) -> Self {
        Self { age, height, name }
    }
}

pub fn closure_sort_fnmut() {
    let mut human_list = vec![
        Human::new(21, 172, "Siddharth".to_string()),
        Human::new(22, 165, "Tanuj".to_string()),
        Human::new(24, 168, "Akhil".to_string()),
    ];
    let mut sort_key_num = 0;
    // Sort by key takes FnMut closure only.
    // Because its updating human list.

    // Sorted by age.
    human_list.sort_by_key(|list| {
        sort_key_num = sort_key_num + 1;
        list.age
    });
    println!(
        "Human list sorted by age - {:?} & counter - {:}",
        human_list, sort_key_num
    );

    sort_key_num = 0;
    // Sorted by height.
    human_list.sort_by_key(|list| {
        sort_key_num = sort_key_num + 1;
        list.height
    });
    println!(
        "Human list sorted by height - {:?} & counter - {:}",
        human_list, sort_key_num
    );

    // Issue code.
    // If this closure moves envirnment value its will cause error.
    // Because sort by key doesn't take FnOnce trait.

    let _string_value = String::from("Siddharth");
    // Sorted by age.

    // Below code will cause error.
    // Bacause takes_ownership will move _string_value.
    // human_list.sort_by_key(|list| {
    //     takes_ownership(_string_value);
    //     list.age
    // });
}

fn takes_ownership(_string_value: String) {
    // Do something useful.
}
