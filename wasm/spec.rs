use wasmedge_sdk::{
    config::ConfigBuilder,
    error::Result,
    params,
    Module,
    Store,
    VmBuilder,
};

fn main() -> Result<()> {
    // Configure WasmEdge instance
    let config = ConfigBuilder::new()
        .with_bulk_memory_operations(true)
        .build()?;

    // Load WASM module
    let module = Module::from_file(Some(&config), "hash_hunter.wasm")?;
    
    // Create VM instance
    let mut vm = VmBuilder::new()
        .with_config(config)
        .build()?;
    
    // Execute address generation
    vm.run_func("generate_addresses", params!())?;
    
    Ok(())
}