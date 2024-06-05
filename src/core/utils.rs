pub fn mean(data: &[f64]) -> f64 {
    let length = data.len() as f64;
    let sum: f64 = data.iter().sum();
    
    sum / length
}

