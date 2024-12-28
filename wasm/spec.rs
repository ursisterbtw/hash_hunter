use wasmedge_sdk::{
    config::ConfigBuilder,
    error::Result,
    params,
    Module,
    Store,
    VmBuilder,
};

fn main() -> Result<()> {
    // configure WasmEdge instance    let config = ConfigBuilder::new()
        .with_bulk_memory_operations(true)
        .build()?;

    // load WASM module    let module = Module::from_file(Some(&config), "hash_hunter.wasm")?;
    
    // create VM instance    let mut vm = VmBuilder::new()
        .with_config(config)
        .build()?;
    
    // execute address generation    vm.run_func("generate_addresses", params!())?;
    
    Ok(())
}