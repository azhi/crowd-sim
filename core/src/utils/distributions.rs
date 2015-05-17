extern crate rand;

use self::rand::Rng;

use configuration::DistributionValue;

pub fn generate(distribution_info: &DistributionValue) -> f64 {
    match *distribution_info {
        DistributionValue::UniformDistributionValue{from: from, to: to} => generate_uniform(from, to),
        DistributionValue::NormalDistributionValue{mean: mean, std_deviation: std_deviation} => generate_normal(mean, std_deviation),
        DistributionValue::TimeInfiniteDistributionValue{avg_rate: avg_rate, rate_deviation: rate_deviation} => avg_rate * rate_deviation,
    }
}

pub fn generate_uniform(from: f64, to: f64) -> f64 {
    let rand = rand::random::<f64>();
    from + rand * (to - from)
}

pub fn generate_normal(mean: f64, std_deviation: f64) -> f64 {
    let group_n = 6;
    let mut sum = 0.0f64;
    let mut rng = rand::thread_rng();

    for i in (0 .. group_n) {
        sum += rng.gen::<f64>();
    }

    mean + std_deviation * f64::sqrt(12_f64 / group_n as f64) * (sum - group_n as f64 / 2.0_f64)
}
