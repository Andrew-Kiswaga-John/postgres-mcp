use anyhow::Result;
use rmcp::{ServiceExt, model::CallToolRequestParam, object, transport::TokioChildProcess};
use tokio::process::Command;
use std::io::{self, Write};

async fn display_menu() -> Result<String> {
    println!("\nAvailable commands:");
    println!("1. Register connection");
    println!("2. Create table");
    println!("3. Insert data");
    println!("4. Query data");
    println!("5. List tables");
    println!("6. Describe table");
    println!("7. Update data");
    println!("8. Delete data");
    println!("9. Create index");
    println!("10. Drop index");
    println!("11. Drop table");
    println!("12. Unregister connection");
    println!("13. Exit");
    
    print!("\nEnter your choice (1-13): ");
    io::stdout().flush()?;
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    Ok(choice.trim().to_string())
}

async fn get_input(prompt: &str) -> Result<String> {
    print!("{}: ", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .with_writer(std::io::stderr)
        .with_ansi(true)
        .init();

    // Start server
    let mut cmd = Command::new("postgres-mcp");
    let service = ().serve(TokioChildProcess::new(&mut cmd)?).await?;

    // Initialize
    let server_info = service.peer_info();
    tracing::info!("Connected to server: {server_info:#?}");

    // let tools = service.list_all_tools().await?;
    // tracing::info!("Available tools: {tools:#?}");

    let mut conn_id = String::new();

    loop {
        let choice = display_menu().await?;
        
        match choice.as_str() {
            "1" => {
                let conn_str = get_input("Enter connection string (postgresql://user:pass@host:port/db)").await?;
                let tool_result = service
                    .call_tool(CallToolRequestParam {
                        name: "register".into(),
                        arguments: Some(object!({
                            "conn_str": conn_str
                        })),
                    })
                    .await?;
                conn_id = tool_result.content[0].raw.as_text().unwrap().text.clone();
                println!("Connection registered with ID: {}", conn_id);
            },
            "2" => {
                let query = get_input("Enter CREATE TABLE query").await?;
                let tool_result = service
                    .call_tool(CallToolRequestParam {
                        name: "create_table".into(),
                        arguments: Some(object!({
                            "conn_id": conn_id,
                            "query": query
                        })),
                    })
                    .await?;
                println!("Table created: {:#?}", tool_result);
            },
            "3" => {
                let query = get_input("Enter INSERT query").await?;
                let tool_result = service
                    .call_tool(CallToolRequestParam {
                        name: "insert".into(),
                        arguments: Some(object!({
                            "conn_id": conn_id,
                            "query": query
                        })),
                    })
                    .await?;
                println!("Data inserted: {:#?}", tool_result);
            },
            "4" => {
                let query = get_input("Enter SELECT query").await?;
                let tool_result = service
                    .call_tool(CallToolRequestParam {
                        name: "query".into(),
                        arguments: Some(object!({
                            "conn_id": conn_id,
                            "query": query
                        })),
                    })
                    .await?;
                println!("Query result: {:#?}", tool_result);
            },
            "5" => {
                let schema = get_input("Enter schema name (default: public)").await?;
                let tool_result = service
                    .call_tool(CallToolRequestParam {
                        name: "list_tables".into(),
                        arguments: Some(object!({
                            "conn_id": conn_id,
                            "schema": schema
                        })),
                    })
                    .await?;
                println!("Tables: {:#?}", tool_result);
            },
            "6" => {
                let table = get_input("Enter table name").await?;
                let tool_result = service
                    .call_tool(CallToolRequestParam {
                        name: "describe".into(),
                        arguments: Some(object!({
                            "conn_id": conn_id,
                            "table": table
                        })),
                    })
                    .await?;
                println!("Table description: {:#?}", tool_result);
            },
            "7" => {
                let query = get_input("Enter UPDATE query").await?;
                let tool_result = service
                    .call_tool(CallToolRequestParam {
                        name: "update".into(),
                        arguments: Some(object!({
                            "conn_id": conn_id,
                            "query": query
                        })),
                    })
                    .await?;
                println!("Data updated: {:#?}", tool_result);
            },
            "8" => {
                let query = get_input("Enter DELETE query").await?;
                let tool_result = service
                    .call_tool(CallToolRequestParam {
                        name: "delete".into(),
                        arguments: Some(object!({
                            "conn_id": conn_id,
                            "query": query
                        })),
                    })
                    .await?;
                println!("Data deleted: {:#?}", tool_result);
            },
            "9" => {
                let query = get_input("Enter CREATE INDEX query").await?;
                let tool_result = service
                    .call_tool(CallToolRequestParam {
                        name: "create_index".into(),
                        arguments: Some(object!({
                            "conn_id": conn_id,
                            "query": query
                        })),
                    })
                    .await?;
                println!("Index created: {:#?}", tool_result);
            },
            "10" => {
                let index = get_input("Enter index name").await?;
                let tool_result = service
                    .call_tool(CallToolRequestParam {
                        name: "drop_index".into(),
                        arguments: Some(object!({
                            "conn_id": conn_id,
                            "index": index
                        })),
                    })
                    .await?;
                println!("Index dropped: {:#?}", tool_result);
            },
            "11" => {
                let table = get_input("Enter table name").await?;
                let tool_result = service
                    .call_tool(CallToolRequestParam {
                        name: "drop_table".into(),
                        arguments: Some(object!({
                            "conn_id": conn_id,
                            "table": table
                        })),
                    })
                    .await?;
                println!("Table dropped: {:#?}", tool_result);
            },
            "12" => {
                let tool_result = service
                    .call_tool(CallToolRequestParam {
                        name: "unregister".into(),
                        arguments: Some(object!({
                            "conn_id": conn_id
                        })),
                    })
                    .await?;
                println!("Connection unregistered: {:#?}", tool_result);
                conn_id.clear();
            },
            "13" => {
                if !conn_id.is_empty() {
                    service
                        .call_tool(CallToolRequestParam {
                            name: "unregister".into(),
                            arguments: Some(object!({
                                "conn_id": conn_id
                            })),
                        })
                        .await?;
                }
                service.cancel().await?;
                break;
            },
            _ => println!("Invalid choice, please try again"),
        }
    }

    Ok(())
}