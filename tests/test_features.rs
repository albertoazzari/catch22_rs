use catch22::compute;

#[test]
fn test_catch22() {
    let time_series = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let n_features = 22;

    let features = (0..n_features).map(|i| compute(&time_series, i)).collect::<Vec<_>>();
    println!("Catch22 features: {:?}", features);
}