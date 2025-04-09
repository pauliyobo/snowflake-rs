use arrow::util::pretty::pretty_format_batches;
use snowflake_api::{QueryResult, SnowflakeApi};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv().ok();
    let mut api = SnowflakeApi::from_env()?;
    let sql = r#"SELECT SUM(1) AS views FROM DOCET_DATA.AAA__SERVING.PB_BE_EVENTS__POST_VIEW WHERE "post_id"=?"#;
    let req = api.prepare(sql).await?.bind("pjlvfvnacjnuxctyr2hs4ikwtla").build();
    let res = api.exec_request(req).await?;
    if let QueryResult::Arrow(a) = res {
        println!("{}", pretty_format_batches(&a)?);
    }
    api.close_session().await?;
    Ok(())
}
