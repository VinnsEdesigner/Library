use wasmtime::{Engine as WasmtimeEngine, Config};

pub struct WasmEngine {
    pub engine: WasmtimeEngine,
    pub state: String,
}

impl WasmEngine {
    pub fn init() -> Self {
        let mut config = Config::new();
        config.wasm_multi_memory(true);
        let engine = WasmtimeEngine::new(&config).expect("Failed to initialize WASM engine");
        Self { 
            engine,
            state: "initialized".to_string() 
        }
    }
}
