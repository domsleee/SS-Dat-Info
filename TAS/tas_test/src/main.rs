use tas_shared::TasSharedMemoryClient;

fn main() {
    println!("Supreme Snowboarding TAS Test Runner");
    println!("=====================================");

    match TasSharedMemoryClient::open() {
        Ok(shared) => {
            let state = shared.state();
            println!("Connected to TAS_Helper.dll (version {})", state.version);
            println!("Hook status:");
            println!("  Cave 2  (Supreme::Cycle): {}", if state.cave2_hooked == 1 { "OK" } else { "NOT HOOKED" });
            println!("  Cave 1C (handler gate):   {}", if state.cave1c_hooked == 1 { "OK" } else { "NOT HOOKED" });
            println!("  Cave 1D (BB3B10):         {}", if state.cave1d_hooked == 1 { "OK" } else { "NOT HOOKED" });
            println!("  Cave 5  (fixed tick):     {}", if state.cave5_hooked == 1 { "OK" } else { "NOT HOOKED" });
            println!("Frame count: {}", state.frame_count);
            println!("Mode: {}", state.mode);
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            eprintln!("Make sure TAS_Helper.dll is loaded in Supreme.exe");
            std::process::exit(1);
        }
    }
}
