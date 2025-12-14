mod agents;
mod lsp;


#[tokio::main]
async fn main() {
    
    println!("Coding Agent - Background Service");
    println!("TODO: Implement background service for function autocompletion");

    lsp::lsp_main().await;
    
    // Future: Background service that:
    // 1. Monitors mouse position / editor cursor
    // 2. Detects keyboard shortcut
    // 3. Uses tree-sitter to parse code and find function
    // 4. Sends function signature to LLM
    // 5. Completes function body only

}
