use std::fs;

use std::cmp::min;
///
/// save ppm
///
///
fn svppm(name: &String, w: i32, h: i32, arr: &Vec<i32>) -> bool {
    let mut contents = format!("P3\n{} {}\n255\n", w, h);
    for (i, j) in arr.iter().enumerate() {
        if i % 3 == 0 {
            contents.push_str(&(format!("{} {} {}\n", j, arr[i + 1], arr[i + 2])));
        }
    }
    if let Ok(_) = fs::write(name, contents) {
        return true;
    }
    false
}

fn main() {
    let mut arr: Vec<i32> = vec![];
    let (width, height) = (256, 256);
    let (mut r, mut g) = (0i32, 0i32);
    for _ in 0..width {
        r += 1;
        for _ in 0..height {
            g += 1;
            arr.push(r);
            arr.push(g);
            arr.push(min(r, g));
        }
    }
    let name = String::from("test.ppm");
    svppm(&name, width, height, &arr);
}
