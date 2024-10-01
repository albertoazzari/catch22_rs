mod catch22;
mod statistics;
mod utils;
use std::{error::Error, fs};

use ctrlc;
use pyo3::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use utils::read_csv;

// #[pymodule]
// #[pyo3(name = "catch22")]
// fn py_module(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
//     let _ = ctrlc::set_handler(move || {
//         println!("\nraise KeyboardInterrupt (Ctrl+C pressed)");
//         std::process::exit(1);
//     });
//     m.add_function(wrap_pyfunction!(compute_catch, m)?)?;
//     Ok(())
// }

// #[pyfunction]
// #[pyo3(signature = (x, n))]
// pub fn compute_catch(
//     x: Vec<Vec<f64>>,
//     n: Vec<usize>
// ) -> PyResult<Vec<Vec<f64>>> {
//     // let mut res = vec![vec![0.0; n.len()]; x.len()];
//     // for i in 0..x.len() {
//     //     for j in 0..n.len() {
//     //         res[i][j] = compute_catch_single(&x[i], n[j]);
//     //     }
//     // }
//     // Ok(res)
//     let res = x.par_iter().map(|x| {
//         n.iter().map(|n| compute_catch_single(x, *n)).collect::<Vec<f64>>()
//     }).collect::<Vec<Vec<f64>>>();
//     Ok(res)
// }

fn compute_catch_single(x: &[f64], n: usize) -> f64 {
    let f = match n {
        0 => catch22::dn_outlier_include_np_001_mdrmd(x, false),
        1 => catch22::dn_outlier_include_np_001_mdrmd(x, true),
        2 => catch22::dn_histogram_mode_n(x, 5),
        3 => catch22::dn_histogram_mode_n(x, 10),
        4 => catch22::co_embed2_dist_tau_d_expfit_meandiff(x),
        5 => catch22::co_f1ecac(x),
        6 => catch22::co_first_min_ac(x),
        7 => catch22::co_histogram_ami_even_tau_bins(x, 2, 5),
        8 => catch22::co_trev_1_num(x),
        9 => catch22::fc_local_simple_mean_tauresrat(x, 1),
        10 => catch22::fc_local_simple_mean_stderr(x, 3),
        11 => catch22::in_auto_mutual_info_stats_tau_gaussian_fmmi(x, 40.0),
        12 => catch22::md_hrv_classic_pnn(x, 40),
        13 => catch22::sb_binary_stats_diff_longstretch0(x),
        14 => catch22::sb_binary_stats_mean_longstretch1(x),
        15 => catch22::sb_motif_three_quantile_hh(x),
        16 => catch22::sc_fluct_anal_2_50_1_logi_prop_r1(x, 1, "rsrangefit"),
        17 => catch22::sc_fluct_anal_2_50_1_logi_prop_r1(x, 2, "dfa"),
        18 => catch22::sp_summaries_welch_rect(x, "area_5_1"),
        19 => catch22::sp_summaries_welch_rect(x, "centroid"),
        20 => catch22::sb_transition_matrix_3ac_sumdiagcov(x),
        21 => catch22::pd_periodicity_wang_th0_01(x),
        22 => statistics::mean(x),
        23 => statistics::std_dev(x),
        // 24 => statistics::slope(x),
        _ => panic!("Invalid function number {}", n),
    };
    return f;
}

fn compute_catch_single_native(x: &[f64], n: usize) -> f64 {
    unimplemented!();
    // let f = unsafe {
    //     match n {
    //         0 => native_catch22::DN_OutlierInclude::DN_OutlierInclude_n_001_mdrmd(
    //             x.as_ptr(),
    //             x.len() as i32,
    //         ),
    //         1 => native_catch22::DN_OutlierInclude::DN_OutlierInclude_p_001_mdrmd(
    //             x.as_ptr(),
    //             x.len() as i32,
    //         ),
    //         2 => native_catch22::DN_HistogramMode_5::DN_HistogramMode_5(x.as_ptr(), x.len() as i32),
    //         3 => {
    //             native_catch22::DN_HistogramMode_10::DN_HistogramMode_10(x.as_ptr(), x.len() as i32)
    //         }
    //         4 => native_catch22::CO_AutoCorr::CO_Embed2_Dist_tau_d_expfit_meandiff(
    //             x.as_ptr(),
    //             x.len() as i32,
    //         ),
    //         5 => native_catch22::CO_AutoCorr::CO_f1ecac(x.as_ptr(), x.len() as i32),
    //         6 => native_catch22::CO_AutoCorr::CO_FirstMin_ac(x.as_ptr(), x.len() as i32) as f64,
    //         7 => native_catch22::CO_AutoCorr::CO_HistogramAMI_even_2_5(x.as_ptr(), x.len() as i32),
    //         8 => native_catch22::CO_AutoCorr::CO_trev_1_num(x.as_ptr(), x.len() as i32),
    //         9 => native_catch22::FC_LocalSimple::FC_LocalSimple_mean_tauresrat(
    //             x.as_ptr(),
    //             x.len() as i32,
    //             1,
    //         ),
    //         10 => native_catch22::FC_LocalSimple::FC_LocalSimple_mean_stderr(
    //             x.as_ptr(),
    //             x.len() as i32,
    //             3,
    //         ),
    //         11 => native_catch22::IN_AutoMutualInfoStats::IN_AutoMutualInfoStats_40_gaussian_fmmi(
    //             x.as_ptr(),
    //             x.len() as i32,
    //         ),
    //         12 => native_catch22::MD_hrv::MD_hrv_classic_pnn40(x.as_ptr(), x.len() as i32),
    //         13 => native_catch22::SB_BinaryStats::SB_BinaryStats_diff_longstretch0(
    //             x.as_ptr(),
    //             x.len() as i32,
    //         ),
    //         14 => native_catch22::SB_BinaryStats::SB_BinaryStats_mean_longstretch1(
    //             x.as_ptr(),
    //             x.len() as i32,
    //         ),
    //         15 => {
    //             native_catch22::SB_MotifThree::SB_MotifThree_quantile_hh(x.as_ptr(), x.len() as i32)
    //         }
    //         16 => native_catch22::SC_FluctAnal::SC_FluctAnal_2_rsrangefit_50_1_logi_prop_r1(
    //             x.as_ptr(),
    //             x.len() as i32,
    //         ),
    //         17 => native_catch22::SC_FluctAnal::SC_FluctAnal_2_dfa_50_1_2_logi_prop_r1(
    //             x.as_ptr(),
    //             x.len() as i32,
    //         ),
    //         18 => native_catch22::SP_Summaries::SP_Summaries_welch_rect_area_5_1(
    //             x.as_ptr(),
    //             x.len() as i32,
    //         ),
    //         19 => native_catch22::SP_Summaries::SP_Summaries_welch_rect_centroid(
    //             x.as_ptr(),
    //             x.len() as i32,
    //         ),
    //         20 => native_catch22::SB_TransitionMatrix::SB_TransitionMatrix_3ac_sumdiagcov(
    //             x.as_ptr(),
    //             x.len() as i32,
    //         ),
    //         21 => native_catch22::PD_PeriodicityWang::PD_PeriodicityWang_th0_01(
    //             x.as_ptr(),
    //             x.len() as i32,
    //         ) as f64,
    //         22 => native_catch22::DN_Mean::DN_Mean(x.as_ptr(), x.len() as i32),
    //         23 => native_catch22::DN_Spread_Std::DN_Spread_Std(x.as_ptr(), x.len() as i32),
    //         // 24 => native_catch22::DN_Slope::DN_Slope(x.as_ptr(), x.len() as i32),
    //         _ => panic!("Invalid function number {}", n),
    //     }
    // };
    // f
}

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

// mod native_catch22 {
//     #![allow(warnings)]
//     include!(concat!(env!("OUT_DIR"), "/catch22.rs"));
// }

#[test]
pub fn test_catch22() {
    let paths = fs::read_dir("/media/DATA/albertoazzari/UCRArchive_2018").unwrap();

    let mut datasets = Vec::new();
    for entry in paths {
        // Unwrap the entry or handle the error, if any.
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            datasets.push(entry);
        }
    }
    datasets.sort_by_key(|dir| dir.file_name().to_string_lossy().to_string());
    for path in &datasets {
        println!("\tProcessing {}", path.file_name().to_string_lossy());
        let train_path = path
            .path()
            .join(format!("{}_TRAIN.tsv", path.file_name().to_string_lossy()));
        let test_path = path
            .path()
            .join(format!("{}_TEST.tsv", path.file_name().to_string_lossy()));

        let ds_train = read_csv(train_path, b'\t', false).unwrap();
        let ds_test = read_csv(test_path, b'\t', false).unwrap();

        let mut ds = ds_train.clone();
        ds.extend(ds_test.clone());


        let res_native = ds
            .par_iter()
            .map(|x| {
                (0..24)
                    .collect::<Vec<usize>>()
                    .iter()
                    .map(|n| compute_catch_single_native(x, *n))
                    .collect::<Vec<f64>>()
            })
            .collect::<Vec<Vec<f64>>>();

        let res_rs = ds
            .par_iter()
            .map(|x| {
                (0..24)
                    .collect::<Vec<usize>>()
                    .iter()
                    .map(|n| compute_catch_single(x, *n))
                    .collect::<Vec<f64>>()
            })
            .collect::<Vec<Vec<f64>>>();

        for i in 0..res_rs.len() {
            for j in 0..res_rs[i].len() {
                if (res_rs[i][j] - res_native[i][j]).abs() > 1e-8 {
                    compute_catch_single(&ds[i], j);
                    compute_catch_single_native(&ds[i], j);
                    panic!(
                        "i = {}, j = {}, rs = {}, native = {}",
                        i, j, res_rs[i][j], res_native[i][j]
                    );
                }
                //assert_eq_with_tol!(res_rs[i][j], res_native[i][j], 1e-8);
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use statistics::zscore;

//     use super::*;

//     #[test]
//     pub fn test_catch22() {
//         let a = (0..1000).map(|x| (x * x) as f64).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let mut tot_time = 0.0;
//         let verbose = true;
//         let repetitions = 1;

//         for _ in 0..repetitions {

//         let start_time = std::time::Instant::now();
//         let result = catch22::dn_outlier_include_np_001_mdrmd(&a, false);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, DN_OutlierInclude_n_001_mdrmd, {}", result, elapsed.as_secs_f64())};

//         let start_time = std::time::Instant::now();
//         let result = catch22::dn_outlier_include_np_001_mdrmd(&a, true);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, DN_OutlierInclude_p_001_mdrmd, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::dn_histogram_mode_n(&a, 5);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, DN_HistogramMode_5, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::dn_histogram_mode_n(&a, 10);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, DN_HistogramMode_10, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::co_embed2_dist_tau_d_expfit_meandiff(&a);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, CO_Embed2_Dist_tau_d_expfit_meandiff, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::co_f1ecac(&a);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, CO_f1ecac, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::co_first_min_ac(&a);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, CO_FirstMin_ac, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::co_histogram_ami_even_tau_bins(&a, 2, 5);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, CO_HistogramAMI_even_tau_bins, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::co_trev_1_num(&a);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, CO_trev_1_num, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::fc_local_simple_mean_tauresrat(&a, 1);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, FC_LocalSimple_mean_tauresrat, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::fc_local_simple_mean_stderr(&a, 3);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, FC_LocalSimple_mean_stderr, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::in_auto_mutual_info_stats_tau_gaussian_fmmi(&a, 40.0);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, IN_AutoMutualInfoStats_tau_gaussian_fmmi, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::md_hrv_classic_pnn(&a, 40);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, MD_hrv_classic_pnn, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::sb_binary_stats_diff_longstretch0(&a);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, SB_BinaryStats_diff_longstretch0, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::sb_binary_stats_mean_longstretch1(&a);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, SB_BinaryStats_mean_longstretch1, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::sb_motif_three_quantile_hh(&a);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, SB_MotifThree_quantile_hh, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::sc_fluct_anal_2_50_1_logi_prop_r1(&a, 1, "rsrangefit");
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, SC_FluctAnal_2_50_1_logi_prop_r1_rsrangefit, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::sc_fluct_anal_2_50_1_logi_prop_r1(&a, 2, "dfa");
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, SC_FluctAnal_2_50_1_logi_prop_r1_dfa, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::sp_summaries_welch_rect(&a, "area_5_1");
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, SP_Summaries_welch_rect_area_5_1, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::sp_summaries_welch_rect(&a, "centroid");
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, SP_Summaries_welch_rect_centroid, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::sb_transition_matrix_3ac_sumdiagcov(&a);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, SB_TransitionMatrix_3ac_sumdiagcov, {}", result, elapsed.as_secs_f64());}

//         let start_time = std::time::Instant::now();
//         let result = catch22::pd_periodicity_wang_th0_01(&a);
//         let elapsed = start_time.elapsed();
//         tot_time += elapsed.as_secs_f64();
//         if verbose {println!("{}, PD_PeriodicityWang_th0_01, {}", result, elapsed.as_secs_f64());}

//         }
//         println!("Total time elapsed is: {} ms", (tot_time/repetitions as f64) * 1e3);

//     }

//     #[test]
//     pub fn test_dn_outlier_include_n_001_mdrmd() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::dn_outlier_include_np_001_mdrmd(&a, false);
//         assert_eq_with_tol!(result, 0.54450000000000, 1e-8);
//     }

//     #[test]
//     pub fn test_dn_outlier_include_p_001_mdrmd() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::dn_outlier_include_np_001_mdrmd(&a, true);
//         assert_eq_with_tol!(result, 0.55700000000000, 1e-8);
//     }

//     #[test]
//     pub fn test_dn_histogram_mode_5() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::dn_histogram_mode_n(&a, 5);
//         assert_eq_with_tol!(result, 0.01015257759552, 1e-8);
//     }

//     #[test]
//     pub fn test_dn_histogram_mode_10() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::dn_histogram_mode_n(&a, 10);
//         assert_eq_with_tol!(result, -0.23321852564886, 1e-8);
//     }

//     #[test]
//     pub fn test_co_embed2_dist_tau_d_expfit_meandiff() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::co_embed2_dist_tau_d_expfit_meandiff(&a);
//         assert_eq_with_tol!(result, 0.24469637413960, 1e-8);
//     }

//     #[test]
//     pub fn test_co_f1ecac() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::co_f1ecac(&a);
//         assert_eq_with_tol!(result, 1.17888764335491, 1e-8);
//     }

//     #[test]
//     pub fn test_co_first_min_ac() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::co_first_min_ac(&a);
//         assert_eq_with_tol!(result, 3.00000000000000, 1e-8);
//     }

//     #[test]
//     pub fn test_co_histogram_ami_even_2_5() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let tau = 2;
//         let n_bins = 5;
//         let result = catch22::co_histogram_ami_even_tau_bins(&a, tau, n_bins);
//         assert_eq_with_tol!(result, 0.09880550181314, 1e-8);
//     }

//     #[test]
//     pub fn test_co_trev_1_num() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::co_trev_1_num(&a);
//         assert_eq_with_tol!(result, 0.00848392756030, 1e-8);
//     }

//     #[test]
//     pub fn test_fc_local_simple_mean1_tauresrat() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::fc_local_simple_mean_tauresrat(&a, 1);
//         assert_eq_with_tol!(result, 1.00000000000000, 1e-8);
//     }

//     #[test]
//     pub fn test_fc_local_simple_mean3_stderr() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::fc_local_simple_mean_stderr(&a, 3);
//         assert_eq_with_tol!(result, 1.43484806370263, 1e-8);
//     }

//     #[test]
//     pub fn test_in_auto_mutual_info_stats_40_gaussian_fmmi() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::in_auto_mutual_info_stats_tau_gaussian_fmmi(&a, 40.0);
//         assert_eq_with_tol!(result, 1.00000000000000, 1e-8);
//     }

//     #[test]
//     pub fn test_md_hrv_classic_pnn40() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::md_hrv_classic_pnn(&a, 40);
//         assert_eq_with_tol!(result, 0.93993993993994, 1e-8);
//     }

//     #[test]
//     pub fn test_sb_binary_stats_diff_longstretch0() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::sb_binary_stats_diff_longstretch0(&a);
//         assert_eq_with_tol!(result, 5.00000000000000, 1e-8);
//     }

//     #[test]
//     pub fn test_sb_binary_stats_mean_longstretch1() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::sb_binary_stats_mean_longstretch1(&a);
//         assert_eq_with_tol!(result, 5.00000000000000, 1e-8);
//     }

//     #[test]
//     pub fn test_sb_motif_three_quantile_hh() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::sb_motif_three_quantile_hh(&a);
//         assert_eq_with_tol!(result, 2.07406052453604, 1e-8);
//     }

//     #[test]
//     pub fn test_sc_fluct_anal_2_rsrangefit_50_1_logi_prop_r1() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::sc_fluct_anal_2_50_1_logi_prop_r1(&a, 1, "rsrangefit");
//         assert_eq_with_tol!(result, 0.12500000000000, 1e-8);
//     }

//     #[test]
//     pub fn test_sc_fluct_anal_2_dfa_50_1_2_logi_prop_r1() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::sc_fluct_anal_2_50_1_logi_prop_r1(&a, 2, "dfa");
//         assert_eq_with_tol!(result, 0.85416666666667, 1e-8);
//     }

//     #[test]
//     pub fn test_sp_summaries_welch_rect_area_5_1(){
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::sp_summaries_welch_rect(&a, "area_5_1");
//         assert_eq_with_tol!(result, 0.00057662161135, 1e-8);
//     }

//     #[test]
//     pub fn test_sp_summaries_welch_rect_centroid(){
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::sp_summaries_welch_rect(&a, "centroid");
//         assert_eq_with_tol!(result, 1.00015547370150, 1e-8);
//     }

//     #[test]
//     pub fn test_sb_transition_matrix_3ac_sumdiagcov() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::sb_transition_matrix_3ac_sumdiagcov(&a);
//         assert_eq_with_tol!(result, 0.00849661915682, 1e-8);
//     }

//     #[test]
//     pub fn test_pd_periodicity_wang_th0_01() {
//         let a = (0..1000).map(|x| x as f64 * (x as f64).cos()).collect::<Vec<f64>>();
//         let a = zscore(&a);
//         let result = catch22::pd_periodicity_wang_th0_01(&a);
//         assert_eq_with_tol!(result, 5.00000000000000, 1e-8);
//     }

//     #[test]
//     pub fn mean() {
//         let a = (0..1000).map(|x| x as f64).collect::<Vec<f64>>();
//         let result = a.iter().sum::<f64>() / a.len() as f64;
//         assert_eq_with_tol!(result, 499.50000000000000, 1e-8);
//     }
// }
