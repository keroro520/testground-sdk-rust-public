use crate::runtime::runenv::RunEnv;
use crate::runtime::runparams::RunParams;
use rand::{thread_rng, Rng};
use tempdir::TempDir;

pub fn random_test_run_env() -> (RunEnv, TempDir) {
    let mut rng = thread_rng();
    let output_dir = TempDir::new("testground-tests").expect("create tempdir");

    // let rp = RunParams {
    //     test_plan: format!("testplan-{}", 5),
    //     test_case: format!("testcase-{}", 1),
    //     test_run: format!("testrun-{}", 1),
    //     test_repo: Default::default(),
    //     test_commit: Default::default(),
    //     test_branch: Default::default(),
    //     test_tag: Default::default(),
    //     test_outputs_path: output_dir.path().to_string_lossy().to_string(),
    //     test_instance_count: 1,
    //     test_instance_role: Default::default(),
    //     test_instance_params: Default::default(),
    //     test_group_id: format!("testgroup-{}", 1),
    //     test_group_instance_count: 1,
    //     test_sidecar: false,
    // };
    let rp = RunParams {
        test_plan: format!("testplan-{}", rng.gen::<u64>()),
        test_case: format!("testcase-{}", rng.gen::<u64>()),
        test_run: format!("testrun-{}", rng.gen::<u64>()),
        test_repo: Default::default(),
        test_commit: Default::default(),
        test_branch: Default::default(),
        test_tag: Default::default(),
        test_outputs_path: output_dir.path().to_string_lossy().to_string(),
        test_instance_count: rng.gen_range(1u64, 999u64),
        test_instance_role: Default::default(),
        test_instance_params: Default::default(),
        test_group_id: format!("testgroup-{}", rng.gen::<u64>()),
        test_group_instance_count: rng.gen_range(1u64, 999u64),
        test_sidecar: false,
    };
    let runenv = RunEnv::new(rp);

    (runenv, output_dir)
}
