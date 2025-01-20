mod node;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = node::Node::new();
    node.serve().await?;
    Ok(())
}
