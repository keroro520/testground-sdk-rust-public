use crate::runtime::runenv::RunEnv;
use crate::sync::client::Client as SyncClient;
use log::warn;

pub const STATE_NETWORK_INITIALIZED: &str = "network-initialized";

pub struct Client {
    runenv: RunEnv,
    sync: SyncClient,
}

impl Client {
    // Returns a new network client. Use this client to request network
    // changes, such as setting latencies, jitter, packet loss, connectedness, etc.
    pub fn new(runenv: RunEnv, sync: SyncClient) -> Self {
        Self {runenv, sync }
    }

    // // Waits for the sidecar to initialize the network, if
    // // the sidecar is enabled. If not, it returns immediately.
    // pub fn wait_network_initialize(&self) -> Result<(), String> {
    //     self.sync.barrier(STATE_NETWORK_INITIALIZED, self.runenv.test_instance_count)
    // }
    //
    // // Calls wait_network_initialized, and panics if it
    // // errors. It is suitable to use with runner.Invoke/InvokeMap, as long as
    // // this method is called from the main goroutine of the test plan.
    // pub fn must_wait_network_initialized(&self) {
    //     self.wait_network_initialize().expect("must_wait_network_initialized")
    // }
    //
    // // ConfigureNetwork asks the sidecar to configure the network, and returns
    // // either when the sidecar signals back to us, or when the context expires.
    // pub fn configure_network(&self, config: &Config) -> Result<(), String> {
    //     if !self.runenv.test_sidecar {
    //         warn!("ignoring network change request; running in a sidecar-less environment");
    //         return Ok(());
    //     }
    //
    //     let hostname = gethostname::gethostname().to_string_lossy().to_string();
    //     // topic := sync.NewTopic("network:"+hostname, &Config{})
    //
    //     let target = config.callback_target;
    //     if target == 0 {
    //         // Fall back to instance count on zero value
    //         target = self.runenv.test_instance_count
    //     }
    //
    //     self.sync.publish_and_wait(topic, config, config.callback_state, target)
    // }
    //
    // pub fn must_configure_network(&self, config: &Config) {
    //     self.configure_network(config)
    //         .expect("must_configure_network")
    }
}
