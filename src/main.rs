fn main() {
    let number_list = vec![1, 3, 45, 7, 8, 4, 63, 6];
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         larget = number;
    //     }
    // }
    // println!("The largest number is {}", largest);
    // assert_eq!(larget, 63);
    largest(&number_list);
}
fn largest(vec: &[i32]) -> &i32 {
    let mut large = &vec[0];
    for num in vec {
        if num > large {
            large = num
        }
    }
    large
}
//Generic types
fn largest_i32(vec: &[i32]) -> &i32 {
    let mut large = &vec[0];
    for num in vec {
        if num > large {
            large = num
        }
    }
    large
}
fn largest_char(vec: &[char]) -> &char {
    let mut large = &vec[0];
    for num in vec {
        if num > large {
            large = num
        }
    }
    large
}
