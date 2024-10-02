mod catch22;
mod statistics;
mod utils;

pub const N_CATCH22 : usize = 24;

pub fn compute(x: &[f64], n: usize) -> f64 {
    match n {
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
        17 => catch22::sp_summaries_welch_rect(x, "area_5_1"),
        18 => catch22::sp_summaries_welch_rect(x, "centroid"),
        19 => catch22::sb_transition_matrix_3ac_sumdiagcov(x),
        20 => catch22::pd_periodicity_wang_th0_01(x),
        21 => statistics::mean(x),
        22 => statistics::std_dev(x),
        23 => statistics::slope(x),
        _ => panic!("Invalid feature index"),
    }
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

// #[test]
// pub fn test_catch22() {

//     use std::fs;
//     use crate::utils::read_csv;

//     let paths = fs::read_dir("/media/DATA/albertoazzari/UCRArchive_2018").unwrap();

//     let mut datasets = Vec::new();
//     for entry in paths {
//         // Unwrap the entry or handle the error, if any.
//         let entry = entry.unwrap();
//         if entry.file_type().unwrap().is_dir() {
//             datasets.push(entry);
//         }
//     }
//     datasets.sort_by_key(|dir| dir.file_name().to_string_lossy().to_string());
//     for path in &datasets {
//         println!("\tProcessing {}", path.file_name().to_string_lossy());
//         let train_path = path
//             .path()
//             .join(format!("{}_TRAIN.tsv", path.file_name().to_string_lossy()));
//         let test_path = path
//             .path()
//             .join(format!("{}_TEST.tsv", path.file_name().to_string_lossy()));

//         let ds_train = read_csv(train_path, b'\t', false).unwrap();
//         let ds_test = read_csv(test_path, b'\t', false).unwrap();

//         let mut ds = ds_train.clone();
//         ds.extend(ds_test.clone());

//         let res_native = ds
//             .iter()
//             .map(|x| {
//                 (0..24)
//                     .collect::<Vec<usize>>()
//                     .iter()
//                     .map(|n| compute_catch_single_native(x, *n))
//                     .collect::<Vec<f64>>()
//             })
//             .collect::<Vec<Vec<f64>>>();

//         let res_rs = ds
//             .iter()
//             .map(|x| {
//                 (0..24)
//                     .collect::<Vec<usize>>()
//                     .iter()
//                     .map(|n| compute_catch_single(x, *n))
//                     .collect::<Vec<f64>>()
//             })
//             .collect::<Vec<Vec<f64>>>();

//         for i in 0..res_rs.len() {
//             for j in 0..res_rs[i].len() {
//                 if (res_rs[i][j] - res_native[i][j]).abs() > 1e-8 {
//                     compute_catch_single(&ds[i], j);
//                     compute_catch_single_native(&ds[i], j);
//                     panic!(
//                         "i = {}, j = {}, rs = {}, native = {}",
//                         i, j, res_rs[i][j], res_native[i][j]
//                     );
//                 }
//                 //assert_eq_with_tol!(res_rs[i][j], res_native[i][j], 1e-8);
//             }
//         }
//     }
// }
