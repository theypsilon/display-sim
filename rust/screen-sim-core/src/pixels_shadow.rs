pub const TEXTURE_SIZE: usize = 510;

pub const SHADOWS_LEN: isize = 24;

pub fn get_shadows() -> [Box<Fn(usize, usize) -> f64>; SHADOWS_LEN as usize] {
    [
        Box::new(|_i, _j| 255.0),
        Box::new(|i, j| calc_with_log(i, 0) * calc_with_log(j, 0) * 1.0 * 255.0),
        Box::new(|i, j| calc_with_log(i, 1) * calc_with_log(j, 1) * 1.5 * 255.0),
        Box::new(|i, j| calc_with_log(i, 2) * calc_with_log(j, 2) * 3.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 0) * 0.9 + calc_with_log(j, 0) * 0.1) * 1.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 1) * 0.9 + calc_with_log(j, 1) * 0.1) * 1.5 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 2) * 0.9 + calc_with_log(j, 2) * 0.1) * 3.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 3) * 0.9 + calc_with_log(j, 3) * 0.1) * 6.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 0) * 0.8 + calc_with_log(j, 0) * 0.2) * 1.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 1) * 0.8 + calc_with_log(j, 1) * 0.2) * 1.5 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 2) * 0.8 + calc_with_log(j, 2) * 0.2) * 3.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 3) * 0.8 + calc_with_log(j, 3) * 0.2) * 6.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 0) * 0.5 + calc_with_log(j, 0) * 0.5) * 1.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 1) * 0.5 + calc_with_log(j, 1) * 0.5) * 1.5 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 2) * 0.5 + calc_with_log(j, 2) * 0.5) * 3.0 * 255.0),
        Box::new(|i, j| (calc_with_log(i, 3) * 0.5 + calc_with_log(j, 3) * 0.5) * 6.0 * 255.0),
        Box::new(|i, _j| calc_with_log(i, 0) * 1.0 * 255.0),
        Box::new(|i, _j| calc_with_log(i, 1) * 1.5 * 255.0),
        Box::new(|i, _j| calc_with_log(i, 2) * 3.0 * 255.0),
        Box::new(|i, _j| calc_with_log(i, 3) * 6.0 * 255.0),
        Box::new(|i, _j| calc_with_log(i, 4) * 9.0 * 255.0),
        Box::new(|i, j| calc_diamond(i, 0) * calc_diamond(j, 0) * 1.0 * 255.0),
        Box::new(|i, _j| calc_diamond(i, 0) * 1.0 * 255.0),
        Box::new(|i, _j| calc_diamond(i, 1) * 1.5 * 255.0),
    ]
}

fn calc_with_log(number: usize, count: usize) -> f64 {
    let result = log(TEXTURE_SIZE - number);
    pow(result, count)
}
fn log(number: usize) -> f64 {
    f64::log(number as f64, (TEXTURE_SIZE / 2) as f64)
}
fn calc_diamond(number: usize, count: usize) -> f64 {
    let result = 1.0 - ((number - TEXTURE_SIZE / 2) as f64 / (TEXTURE_SIZE as f64 / 2.0));
    pow(result, count)
}
fn pow(mut number: f64, count: usize) -> f64 {
    for _i in 0..count {
        number *= number;
    }
    number
}
