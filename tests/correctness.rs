use catch22::compute;
use core::panic;
use csv::ReaderBuilder;
use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

pub fn read_csv(
    path: impl AsRef<Path>,
    delimiter: u8,
    header: bool,
) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let reader = BufReader::new(File::open(path)?);
    let mut reader = ReaderBuilder::new()
        .has_headers(header)
        .delimiter(delimiter)
        .from_reader(reader);

    let mut samples = Vec::new();

    for result in reader.deserialize() {
        let record: Vec<f64> = result.unwrap();
        // Replace NaNs with 0s
        // let record = record.to_vec()
        // .iter()
        // .map(|v| if v.is_nan() { 0.0 } else { *v })
        // .collect::<Vec<f64>>();
        samples.push(record);
    }
    Ok(samples)
}

mod native_catch22 {
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/catch22.rs"));
}

fn compute_native(x: &[f64], n: usize) -> f64 {
    let f = unsafe {
        match n {
            0 => native_catch22::DN_OutlierInclude::DN_OutlierInclude_n_001_mdrmd(
                x.as_ptr(),
                x.len() as i32,
            ),
            1 => native_catch22::DN_OutlierInclude::DN_OutlierInclude_p_001_mdrmd(
                x.as_ptr(),
                x.len() as i32,
            ),
            2 => native_catch22::DN_HistogramMode_5::DN_HistogramMode_5(x.as_ptr(), x.len() as i32),
            3 => {
                native_catch22::DN_HistogramMode_10::DN_HistogramMode_10(x.as_ptr(), x.len() as i32)
            }
            4 => native_catch22::CO_AutoCorr::CO_Embed2_Dist_tau_d_expfit_meandiff(
                x.as_ptr(),
                x.len() as i32,
            ),
            5 => native_catch22::CO_AutoCorr::CO_f1ecac(x.as_ptr(), x.len() as i32),
            6 => native_catch22::CO_AutoCorr::CO_FirstMin_ac(x.as_ptr(), x.len() as i32) as f64,
            7 => native_catch22::CO_AutoCorr::CO_HistogramAMI_even_2_5(x.as_ptr(), x.len() as i32),
            8 => native_catch22::CO_AutoCorr::CO_trev_1_num(x.as_ptr(), x.len() as i32),
            9 => native_catch22::FC_LocalSimple::FC_LocalSimple_mean_tauresrat(
                x.as_ptr(),
                x.len() as i32,
                1,
            ),
            10 => native_catch22::FC_LocalSimple::FC_LocalSimple_mean_stderr(
                x.as_ptr(),
                x.len() as i32,
                3,
            ),
            11 => native_catch22::IN_AutoMutualInfoStats::IN_AutoMutualInfoStats_40_gaussian_fmmi(
                x.as_ptr(),
                x.len() as i32,
            ),
            12 => native_catch22::MD_hrv::MD_hrv_classic_pnn40(x.as_ptr(), x.len() as i32),
            13 => native_catch22::SB_BinaryStats::SB_BinaryStats_diff_longstretch0(
                x.as_ptr(),
                x.len() as i32,
            ),
            14 => native_catch22::SB_BinaryStats::SB_BinaryStats_mean_longstretch1(
                x.as_ptr(),
                x.len() as i32,
            ),
            15 => {
                native_catch22::SB_MotifThree::SB_MotifThree_quantile_hh(x.as_ptr(), x.len() as i32)
            }
            16 => native_catch22::SC_FluctAnal::SC_FluctAnal_2_rsrangefit_50_1_logi_prop_r1(
                x.as_ptr(),
                x.len() as i32,
            ),
            17 => native_catch22::SC_FluctAnal::SC_FluctAnal_2_dfa_50_1_2_logi_prop_r1(
                x.as_ptr(),
                x.len() as i32,
            ),
            18 => native_catch22::SP_Summaries::SP_Summaries_welch_rect_area_5_1(
                x.as_ptr(),
                x.len() as i32,
            ),
            19 => native_catch22::SP_Summaries::SP_Summaries_welch_rect_centroid(
                x.as_ptr(),
                x.len() as i32,
            ),
            20 => native_catch22::SB_TransitionMatrix::SB_TransitionMatrix_3ac_sumdiagcov(
                x.as_ptr(),
                x.len() as i32,
            ),
            21 => native_catch22::PD_PeriodicityWang::PD_PeriodicityWang_th0_01(
                x.as_ptr(),
                x.len() as i32,
            ) as f64,
            22 => native_catch22::DN_Mean::DN_Mean(x.as_ptr(), x.len() as i32),
            23 => native_catch22::DN_Spread_Std::DN_Spread_Std(x.as_ptr(), x.len() as i32),
            _ => panic!("Invalid function number {}", n),
        }
    };
    f
}

#[test]
pub fn test_catch22() {
    use std::fs;

    let mut time_native = 0.0;
    let mut time_rs = 0.0;

    let paths = fs::read_dir("../DATA/ucr").unwrap();
    let mut datasets = Vec::new();
    for entry in paths {
        // Unwrap the entry or handle the error, if any.
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            datasets.push(entry);
        }
    }
    datasets.sort_by_key(|dir| dir.file_name().to_string_lossy().to_string());
    for path in &datasets[1..] {
        let mut feature_count = HashMap::new();

        println!("Processing {}", path.file_name().to_string_lossy());
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

        let start_time = std::time::Instant::now();
        let res_native = ds
            .iter()
            .map(|x| {
                (0..24)
                    .collect::<Vec<usize>>()
                    .iter()
                    .map(|n| compute_native(x, *n))
                    .collect::<Vec<f64>>()
            })
            .collect::<Vec<Vec<f64>>>();
        let elapsed_native = start_time.elapsed().as_secs_f64();
        time_native += elapsed_native;
        let start_time = std::time::Instant::now();
        let res_rs = ds
            .iter()
            .map(|x| {
                (0..24)
                    .collect::<Vec<usize>>()
                    .iter()
                    .map(|n| compute(x, *n))
                    .collect::<Vec<f64>>()
            })
            .collect::<Vec<Vec<f64>>>();
        let elapsed_rs = start_time.elapsed().as_secs_f64();
        time_rs += elapsed_rs;
        for i in 0..res_rs.len() {
            for j in 0..res_rs[i].len() {
                if (res_rs[i][j] - res_native[i][j]).abs() > 1e-8 {
                    feature_count.entry(j).and_modify(|e| *e += 1).or_insert(1);
                }
            }
        }
        println!(
            "\tResults Discrepancies {:?}",
            feature_count
        );
        println!(
            "\tNative: {:?}, Rust: {:?}",
            elapsed_native, elapsed_rs
        );
    }
    println!(
        "Total time: Native: {:?}, Rust: {:?}, Ratio: {:?}",
        time_native, time_rs, time_native / time_rs
    );
}
