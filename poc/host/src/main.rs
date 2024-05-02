use polkavm::{Config, Linker};

struct HostFunctions;

impl poc_executor::XcqExecutorContext for HostFunctions {
    fn register_host_functions<T>(&mut self, linker: &mut Linker<T>) {
        linker.func_wrap("foo", || -> u32 { 100 }).unwrap();
    }
}

fn main() {
    env_logger::init();

    let raw_blob = include_bytes!("../../../output/poc-guest.polkavm");

    let config = Config::from_env().unwrap();

    let mut executor: poc_executor::XcqExecutor<HostFunctions> = poc_executor::XcqExecutor::new(config, HostFunctions);
    let res = executor.execute(raw_blob).unwrap();
    println!("Result: {}", res);
}
