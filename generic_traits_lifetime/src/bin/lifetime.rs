fn main() {
    let test_vec_1 = vec![1, 20, 100];
    let test_vec_2 = vec![1, 20, 200];
    println!("Result - {:?}", test_lifetime_1(&test_vec_1));
    println!("Result - {:?}", test_lifetime_2(&test_vec_1, &test_vec_2));
}

// Lifetime are required when '&' is used with input or output.
// Compiler mostly handles lifetime by self.
// Only with some edge cases we need to handle it explicity.

// Lifetime are way to correlate multiple ref parameter & return value of a funtion.

// Eg 1

fn test_lifetime_1(val: &Vec<i32>) -> &Vec<i32> {
    val
}

// Compiler converts this to-

#[allow(dead_code)]
fn test_lifetime_1_convert<'a>(val: &'a Vec<i32>) -> &'a Vec<i32> {
    val
}

// Eg 2
// Explicity lifetime needed here.

fn test_lifetime_2<'a>(val: &'a Vec<i32>, val2: &'a Vec<i32>) -> &'a Vec<i32> {
    if val.len() > val2.len() {
        val
    } else {
        val2
    }
}

// Compiler converts this to-
// Compiler doesn't know whether to return value 'a or 'b. Hence, causing error.
// fn test_lifetime_2_convert<'a, 'b>(val: &'a Vec<i32>, val2: &'b Vec<i32>) ->  &Vec<i32> {
//     if val.len() > val2.len() {
//         val
//     } else {
//         val2
//     }
// }
