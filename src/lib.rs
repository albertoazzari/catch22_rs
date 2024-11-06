

mod catch22;
mod statistics;

pub const N_CATCH22: usize = 25;

pub fn compute(x: &[f64], n: usize) -> f64 {
    let r;
    match n {
        0 => r = catch22::dn_outlier_include_np_001_mdrmd(x, false),
        1 => r = catch22::dn_outlier_include_np_001_mdrmd(x, true),
        2 => r = catch22::dn_histogram_mode_n(x, 5),
        3 => r = catch22::dn_histogram_mode_n(x, 10),
        4 => r = catch22::co_embed2_dist_tau_d_expfit_meandiff(x),
        5 => r = catch22::co_f1ecac(x),
        6 => r = catch22::co_first_min_ac(x),
        7 => r = catch22::co_histogram_ami_even_tau_bins(x, 2, 5),
        8 => r = catch22::co_trev_1_num(x),
        9 => r = catch22::fc_local_simple_mean_tauresrat(x, 1),
        10 => r = catch22::fc_local_simple_mean_stderr(x, 3),
        11 => r = catch22::in_auto_mutual_info_stats_tau_gaussian_fmmi(x, 40.0),
        12 => r = catch22::md_hrv_classic_pnn(x, 40),
        13 => r = catch22::sb_binary_stats_diff_longstretch0(x),
        14 => r = catch22::sb_binary_stats_mean_longstretch1(x),
        15 => r = catch22::sb_motif_three_quantile_hh(x),
        16 => r = catch22::sc_fluct_anal_2_50_1_logi_prop_r1(x, 1, "rsrangefit"),
        17 => r = catch22::sc_fluct_anal_2_50_1_logi_prop_r1(x, 2, "dfa"),
        18 => r = catch22::sp_summaries_welch_rect(x, "area_5_1"),
        19 => r = catch22::sp_summaries_welch_rect(x, "centroid"),
        20 => r = catch22::sb_transition_matrix_3ac_sumdiagcov(x),
        21 => r = catch22::pd_periodicity_wang_th0_01(x),
        22 => r = statistics::mean(x),
        23 => r = statistics::std_dev(x),
        24 => r = statistics::slope(x),
        _ => panic!("Invalid feature index"),
    }
    if r.is_finite() {
        r
    } else {
        0.0
    }
}