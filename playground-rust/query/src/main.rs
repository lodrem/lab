use datafusion::error::Result;
use datafusion::prelude::*;
use sqlparser::ast::Statement;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    parse_sql();

    let ctx = SessionContext::new();
    ctx.register_json("lab", "test/lab.json", NdJsonReadOptions::default())
        .await?;

    let df = ctx.sql("SELECT * FROM users").await?;

    df.show().await?;

    Ok(())
}

fn parse_sql() {
    let dialect = PostgreSqlDialect {};

    let queries = [
        "SELECT * FROM users",
        "INSERT INTO users (id, name, created_at) VALUES (gen_random_uuid(), 'ccc', NOW()) RETURNING *",
    ];

    for q in queries {
        let ast: Vec<Statement> = Parser::parse_sql(&dialect, q).unwrap();
        println!("{} => {:?}", q, ast);
    }
}
