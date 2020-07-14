use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const ENV_TEST_PLAN: &str = "TEST_PLAN";
pub const ENV_TEST_CASE: &str = "TEST_CASE";
pub const ENV_TEST_RUN: &str = "TEST_RUN";
pub const ENV_TEST_REPO: &str = "TEST_REPO";
pub const ENV_TEST_COMMIT: &str = "TEST_COMMIT";
pub const ENV_TEST_BRANCH: &str = "TEST_BRANCH";
pub const ENV_TEST_TAG: &str = "TEST_TAG";
pub const ENV_TEST_OUTPUTS_PATH: &str = "TEST_OUTPUTS_PATH";
pub const ENV_TEST_INSTANCE_COUNT: &str = "TEST_INSTANCE_COUNT";
pub const ENV_TEST_INSTANCE_ROLE: &str = "TEST_INSTANCE_ROLE";
pub const ENV_TEST_INSTANCE_PARAMS: &str = "TEST_INSTANCE_PARAMS";
pub const ENV_TEST_GROUP_ID: &str = "TEST_GROUP_ID";
pub const ENV_TEST_GROUP_INSTANCE_COUNT: &str = "TEST_GROUP_INSTANCE_COUNT";
pub const ENV_TEST_SIDECAR: &str = "TEST_SIDECAR";

// RunParams encapsulates the runtime parameters for this test.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunParams {
    #[serde(rename = "plan")]
    pub test_plan: String,
    #[serde(rename = "case")]
    pub test_case: String,
    #[serde(rename = "run")]
    pub test_run: String,

    #[serde(rename = "repo", default)]
    pub test_repo: String,
    #[serde(rename = "commit", default)]
    pub test_commit: String,
    #[serde(rename = "branch", default)]
    pub test_branch: String,
    #[serde(rename = "tag", default)]
    pub test_tag: String,

    #[serde(rename = "outputs_path", default)]
    pub test_outputs_path: String,

    #[serde(rename = "instances")]
    pub test_instance_count: u64,
    #[serde(rename = "role", default)]
    pub test_instance_role: String,
    #[serde(rename = "params", default)]
    pub test_instance_params: HashMap<String, String>,

    #[serde(rename = "group", default)]
    pub test_group_id: String,
    #[serde(rename = "group_instances", default)]
    pub test_group_instance_count: u64,

    #[serde(rename = "test_sidecar", default)]
    pub test_sidecar: bool,
    // TODO test_subnet/test_start_time
    //
    // The subnet on which this test is running.
    //
    // The test instance can use this to pick an IP address and/or determine
    // the "data" network interface.
    //
    // This will be 127.1.0.0/16 when using the local exec runner.
    // TestSubnet    *IPNet    `json:"network,omitempty"`
    // TestStartTime time.Time `json:"start_time,omitempty"`
}

impl RunParams {
    pub fn new(kvs: &HashMap<String, String>) -> Result<Self, String> {
        let get_or_ok = |key| {
            kvs.get(key)
                .cloned()
                .ok_or_else(|| format!("envvar \"{}\" not exist", key))
        };
        Ok(RunParams {
            test_plan: get_or_ok(ENV_TEST_PLAN)?,
            test_case: get_or_ok(ENV_TEST_CASE)?,
            test_run: get_or_ok(ENV_TEST_RUN)?,
            test_repo: get_or_ok(ENV_TEST_REPO).unwrap_or_default(),
            test_commit: get_or_ok(ENV_TEST_COMMIT).unwrap_or_default(),
            test_branch: get_or_ok(ENV_TEST_BRANCH).unwrap_or_default(),
            test_tag: get_or_ok(ENV_TEST_TAG).unwrap_or_default(),
            test_outputs_path: get_or_ok(ENV_TEST_OUTPUTS_PATH).unwrap_or_default(),
            test_instance_count: get_or_ok(ENV_TEST_INSTANCE_COUNT)?
                .parse::<u64>()
                .map_err(|err| err.to_string())?,
            test_instance_role: get_or_ok(ENV_TEST_INSTANCE_ROLE).unwrap_or_default(),
            test_instance_params: get_or_ok(ENV_TEST_INSTANCE_PARAMS)
                .map(|raw| unpack_params(&raw))
                .unwrap_or_default(),
            test_group_id: get_or_ok(ENV_TEST_GROUP_ID).unwrap_or_default(),
            test_group_instance_count: get_or_ok(ENV_TEST_GROUP_INSTANCE_COUNT)
                .and_then(|raw| raw.parse::<u64>().map_err(|err| err.to_string()))
                .unwrap_or_default(),
            test_sidecar: get_or_ok(ENV_TEST_SIDECAR)
                .and_then(|raw| raw.parse::<bool>().map_err(|err| err.to_string()))
                .unwrap_or_default(),
        })
    }

    // is_param_set checks if a certain parameter is set.
    pub fn is_param_set(&self, name: &String) -> bool {
        self.test_instance_params.contains_key(name)
    }

    pub fn string_param(&self, name: &String) -> Result<String, String> {
        self.test_instance_params
            .get(name)
            .cloned()
            .ok_or_else(|| format!("{} was not set", name))
    }

    pub fn int_param(&self, name: &String) -> Result<u64, String> {
        let v = self
            .test_instance_params
            .get(name)
            .cloned()
            .ok_or_else(|| format!("{} was not set", name))?;
        v.parse::<u64>()
            .map_err(|err| format!("{}={} value is not a valid u64: {:?}", name, v, err))
    }

    pub fn float_param(&self, name: &String) -> Result<f64, String> {
        let v = self
            .test_instance_params
            .get(name)
            .cloned()
            .ok_or_else(|| format!("{} was not set", name))?;
        v.parse::<f64>()
            .map_err(|err| format!("{}={} value is not a valid f64: {:?}", name, v, err))
    }

    pub fn bool_param(&self, name: &String) -> Result<bool, String> {
        let v = self
            .test_instance_params
            .get(name)
            .cloned()
            .ok_or_else(|| format!("{} was not set", name))?;
        v.parse::<bool>()
            .map_err(|err| format!("{}={} value is not a valid bool: {:?}", name, v, err))
    }
}

fn unpack_params(packed: &String) -> HashMap<String, String> {
    let spltparams: Vec<String> = packed.split("|").map(|s| s.to_string()).collect();
    let mut params: HashMap<String, String> = HashMap::with_capacity(spltparams.len());
    for s in spltparams {
        let v: Vec<String> = s.split("=").map(|s| s.to_string()).collect();
        if v.len() != 2 {
            continue;
        }
        params.insert(v[0].clone(), v[1].clone());
    }
    params
}
