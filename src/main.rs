mod leetcode;
mod tests;

fn main() {
    let test = leetcode::remove_element(&mut vec![3, 2, 2, 3], 3);

    println!("{:?}", test);
}
