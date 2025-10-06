use codspeed::codspeed::WARMUP_RUNS;
use codspeed::walltime_results::WalltimeBenchmark;

const NAME: &str = "warmup";
const URI: &str = "warmup://integration";

fn stats_json(benchmark: &WalltimeBenchmark) -> serde_json::Value {
    serde_json::to_value(benchmark)
        .expect("serialize benchmark")
        .get("stats")
        .cloned()
        .expect("stats value")
}

#[test]
fn warmup_rounds_trimmed_integration() {
    let iters_per_round = vec![1u128; WARMUP_RUNS as usize + 2];
    let mut times_per_round = vec![200u128; WARMUP_RUNS as usize];
    times_per_round.extend([100u128, 100u128]);

    let benchmark = WalltimeBenchmark::from_runtime_data(
        NAME.to_string(),
        URI.to_string(),
        iters_per_round,
        times_per_round,
        None,
    );

    let stats_value = stats_json(&benchmark);
    let stats = stats_value.as_object().expect("stats object");
    assert_eq!(stats["warmup_iters"].as_u64().unwrap(), WARMUP_RUNS as u64);
    assert_eq!(stats["rounds"].as_u64().unwrap(), 2);
    assert!((stats["mean_ns"].as_f64().unwrap() - 100.0).abs() < f64::EPSILON);

    let expected_total = 200f64 / 1_000_000_000.0;
    assert!((stats["total_time"].as_f64().unwrap() - expected_total).abs() < f64::EPSILON);
}

#[test]
fn warmup_rounds_preserved_when_insufficient_samples() {
    let iters_per_round = vec![1u128; WARMUP_RUNS as usize + 1];
    let mut times_per_round = vec![200u128; WARMUP_RUNS as usize];
    times_per_round.push(100u128);
    let expected_total = times_per_round.iter().sum::<u128>() as f64 / 1_000_000_000.0;
    let expected_mean = (WARMUP_RUNS as f64 * 200.0 + 100.0) / (WARMUP_RUNS as f64 + 1.0);

    let benchmark = WalltimeBenchmark::from_runtime_data(
        NAME.to_string(),
        URI.to_string(),
        iters_per_round,
        times_per_round,
        None,
    );

    let stats_value = stats_json(&benchmark);
    let stats = stats_value.as_object().expect("stats object");
    assert_eq!(stats["warmup_iters"].as_u64().unwrap(), 0);
    assert_eq!(stats["rounds"].as_u64().unwrap(), WARMUP_RUNS as u64 + 1);

    assert!((stats["total_time"].as_f64().unwrap() - expected_total).abs() < f64::EPSILON);
    assert!((stats["mean_ns"].as_f64().unwrap() - expected_mean).abs() < f64::EPSILON);
}
