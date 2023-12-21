// pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
//     let mut max = 0;
//     let mut points = points;
//     points.sort_by(|a, b| a[0].cmp(&b[0]));

//     for i in 0..points.len() - 1 {
//         max = std::cmp::max(max, points[i + 1][0] - points[i][0])
//     }
//     max
// }

pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_unstable_by_key(|arr| arr[0]);
    points.windows(2).map(|w| w[1][0] - w[0][0]).max().unwrap()
}

