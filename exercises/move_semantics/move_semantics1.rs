// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);
    vec1.push(50);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    println!("The vec sum is {}", add_vec(vec1));
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn add_vec(vec: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in vec {
        sum += i;
    }

    sum
}
