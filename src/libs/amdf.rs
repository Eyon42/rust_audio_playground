// average magnitude difference function
use num;

pub fn amdf(sample: Vec<f64>) -> u32 {
    let l = sample.size();
    let k = l / 10;
    let mut g = Vec::new();
    for s in 0..(l - k - 1) {
        g.push(num::abs(sample(s) - sample(s + k)))
    }

    // Search local minimums
    let mins = Vec::new();
    for s in g.slice(2) {}
}
