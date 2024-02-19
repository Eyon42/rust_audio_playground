// average magnitude difference function
use num;

pub fn amdf(sample: Vec<f64>) -> usize {
    /*
    Returns the period of one of the wave's harmonics in number of samples
    */
    let sample_len = sample.len();
    let mut g: Vec<f64> = Vec::new();
    for k in 0..(sample_len - 1) {
        // G(k)
        let mut diff_acc = 0.0;
        for s in 0..(sample_len - k - 1) {
            diff_acc += num::abs(sample[s] - sample[s + k]);
        }
        g.push((1.0 / (sample_len - k) as f64) * diff_acc);
    }

    // Search local minimums
    let mut mins: Vec<usize> = Vec::new();
    for s in 1..(sample_len - 2) {
        if (g[s - 1] > g[s]) && (g[s + 1] > g[s]) {
            mins.push(s)
        }
        if mins.len() == 2 {
            break;
        }
    }

    return (mins[1] - mins[0]);
}
