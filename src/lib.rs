pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod catch22;
mod statistics;

#[macro_export]
macro_rules! assert_eq_with_tol {
    ($left:expr, $right:expr, $tol:expr $(,)?) => {{
        let (left_val, right_val, tol_val) = ($left, $right, $tol);
        if (left_val - right_val).abs() > tol_val {
            panic!(
                "assertion failed: `(left ≈ right)`\n  left: `{:?}`,\n right: `{:?}`,\n  tolerance: `{:?}`",
                left_val, right_val, tol_val
            );
        }
    }};
}

#[cfg(test)]
mod tests {
    use statistics::zscore;

    use super::*;

    #[test]
    pub fn test_catch22() { 
        let a = (0..10000).map(|x| (x * x) as f64).collect::<Vec<f64>>();
        let a = zscore(&a);
        let mut tot_time = 0.0;
        let verbose = false;
        let repetitions = 1000;

        for _ in 0..repetitions {

        let start_time = std::time::Instant::now();
        let result = catch22::DN_OutlierInclude_np_001_mdrmd(&a, false);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, DN_OutlierInclude_n_001_mdrmd, {}", result, elapsed.as_secs_f64())};

        let start_time = std::time::Instant::now();
        let result = catch22::DN_OutlierInclude_np_001_mdrmd(&a, true);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, DN_OutlierInclude_p_001_mdrmd, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::DN_HistogramMode_n(&a, 5);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, DN_HistogramMode_5, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::DN_HistogramMode_n(&a, 10);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, DN_HistogramMode_10, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::CO_Embed2_Dist_tau_d_expfit_meandiff(&a);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, CO_Embed2_Dist_tau_d_expfit_meandiff, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::CO_f1ecac(&a);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, CO_f1ecac, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::CO_FirstMin_ac(&a);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, CO_FirstMin_ac, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::CO_HistogramAMI_even_tau_bins(&a, 2, 5);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, CO_HistogramAMI_even_tau_bins, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::CO_trev_1_num(&a);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, CO_trev_1_num, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::FC_LocalSimple_mean_tauresrat(&a, 1);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, FC_LocalSimple_mean_tauresrat, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::FC_LocalSimple_mean_stderr(&a, 3);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, FC_LocalSimple_mean_stderr, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::IN_AutoMutualInfoStats_tau_gaussian_fmmi(&a, 40.0);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, IN_AutoMutualInfoStats_tau_gaussian_fmmi, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::MD_hrv_classic_pnn(&a, 40);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, MD_hrv_classic_pnn, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::SB_BinaryStats_diff_longstretch0(&a);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, SB_BinaryStats_diff_longstretch0, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::SB_BinaryStats_mean_longstretch1(&a);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, SB_BinaryStats_mean_longstretch1, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::SB_MotifThree_quantile_hh(&a);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, SB_MotifThree_quantile_hh, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::SC_FluctAnal_2_50_1_logi_prop_r1(&a, 1, "rsrangefit");
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, SC_FluctAnal_2_50_1_logi_prop_r1_rsrangefit, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::SC_FluctAnal_2_50_1_logi_prop_r1(&a, 2, "dfa");
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, SC_FluctAnal_2_50_1_logi_prop_r1_dfa, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::SP_Summaries_welch_rect(&a, "area_5_1");
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, SP_Summaries_welch_rect_area_5_1, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::SP_Summaries_welch_rect(&a, "centroid");
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, SP_Summaries_welch_rect_centroid, {}", result, elapsed.as_secs_f64());}

        let start_time = std::time::Instant::now();
        let result = catch22::SB_TransitionMatrix_3ac_sumdiagcov(&a);
        let elapsed = start_time.elapsed();
        tot_time += elapsed.as_secs_f64();
        if verbose {println!("{}, SB_TransitionMatrix_3ac_sumdiagcov, {}", result, elapsed.as_secs_f64());}

        }

        println!("Total time elapsed is: {} ms", (tot_time/repetitions as f64) * 1e3);

    }

    #[test]
    pub fn test_dn_n() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::DN_OutlierInclude_np_001_mdrmd(&a, false);
        assert_eq_with_tol!(result, 0.54450000000000, 1e-8);
    }

    #[test]
    pub fn test_dn_p() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::DN_OutlierInclude_np_001_mdrmd(&a, true);
        assert_eq_with_tol!(result, 0.55700000000000, 1e-8);
    }

    #[test]
    pub fn test_dn_hist_5() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::DN_HistogramMode_n(&a, 5);
        assert_eq_with_tol!(result, 0.01015257759552, 1e-8);
    }

    #[test]
    pub fn test_dn_hist_10() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::DN_HistogramMode_n(&a, 10);
        assert_eq_with_tol!(result, -0.23321852564886, 1e-8);
    }

    #[test]
    pub fn test_dist_tau() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::CO_Embed2_Dist_tau_d_expfit_meandiff(&a);
        assert_eq_with_tol!(result, 0.24469637413960, 1e-8);
    }

    #[test]
    pub fn test_f1ecac() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::CO_f1ecac(&a);
        assert_eq_with_tol!(result, 1.17888764335491, 1e-8);
    }

    #[test]
    pub fn test_firstmin() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::CO_FirstMin_ac(&a);
        assert_eq_with_tol!(result, 3.00000000000000, 1e-8);
    }

    #[test]
    pub fn test_histami() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let tau = 2;
        let n_bins = 5;
        let result = catch22::CO_HistogramAMI_even_tau_bins(&a, tau, n_bins);
        assert_eq_with_tol!(result, 0.09880550181314, 1e-8);
    }

    #[test]
    pub fn test_trev() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::CO_trev_1_num(&a);
        assert_eq_with_tol!(result, 0.00848392756030, 1e-8);
    }

    #[test]
    pub fn test_lsmean() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::FC_LocalSimple_mean_tauresrat(&a, 1);
        assert_eq_with_tol!(result, 1.00000000000000, 1e-8);
    }

    #[test]
    pub fn test_lsdev() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::FC_LocalSimple_mean_stderr(&a, 3);
        assert_eq_with_tol!(result, 1.43484806370263, 1e-8);
    }

    #[test]
    pub fn test_amis() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::IN_AutoMutualInfoStats_tau_gaussian_fmmi(&a, 40.0);
        assert_eq_with_tol!(result, 1.00000000000000, 1e-8);
    }

    #[test]
    pub fn test_hrv() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::MD_hrv_classic_pnn(&a, 40);
        assert_eq_with_tol!(result, 0.93993993993994, 1e-8);
    }

    #[test]
    pub fn test_diffstretch() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::SB_BinaryStats_diff_longstretch0(&a);
        assert_eq_with_tol!(result, 5.00000000000000, 1e-8);
    }

    #[test]
    pub fn test_meanstretch() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::SB_BinaryStats_mean_longstretch1(&a);
        assert_eq_with_tol!(result, 5.00000000000000, 1e-8);
    }

    #[test]
    pub fn test_hh() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::SB_MotifThree_quantile_hh(&a);
        assert_eq_with_tol!(result, 2.07406052453604, 1e-8);
    }

    #[test]
    pub fn test_rsrangefit() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::SC_FluctAnal_2_50_1_logi_prop_r1(&a, 1, "rsrangefit");
        assert_eq_with_tol!(result, 0.12500000000000, 1e-8);
    }

    #[test]
    pub fn test_dfa() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::SC_FluctAnal_2_50_1_logi_prop_r1(&a, 2, "dfa");
        assert_eq_with_tol!(result, 0.85416666666667, 1e-8);
    }

    #[test]
    pub fn test_area(){
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let result = catch22::SP_Summaries_welch_rect(&a, "area_5_1");
        assert_eq_with_tol!(result, 0.00057662161135, 1e-8);
    }

    #[test]
    pub fn test_centroid(){
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let result = catch22::SP_Summaries_welch_rect(&a, "centroid");
        assert_eq_with_tol!(result, 1.00015547370150, 1e-8);
    }

    #[test]
    pub fn test_sumd() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::SB_TransitionMatrix_3ac_sumdiagcov(&a);
        assert_eq_with_tol!(result, 0.00849661915682, 1e-8);
    }

    #[test]
    pub fn test_wang() {
        let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
        let a = zscore(&a);
        let result = catch22::PD_PeriodicityWang_th0_01(&a);
        assert_eq_with_tol!(result, 5.00000000000000, 1e-8);
    }
}
