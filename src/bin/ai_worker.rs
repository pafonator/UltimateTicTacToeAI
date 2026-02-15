use UltimateTicTacToe::AIWorker;
use leptos::leptos_dom::logging::console_log;
use gloo_worker::Registrable;
use log::info;

fn main() {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();
    
    // Initialize wasm-logger for the worker so logs appear in browser console
    wasm_logger::init(wasm_logger::Config::default());
    
    // Initialize logging
    info!("Worker binary starting...");
    
    // This registers the worker and starts listening for messages
    AIWorker::registrar().register();
    
    log::info!("Worker registered and ready");
}
