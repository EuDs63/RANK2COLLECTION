use std::error::Error;
use tokio;

mod app_config;
mod api_neodb;
mod api_bangumi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 读取配置文件
    let config = app_config::AppConfig::from_file("config.toml").unwrap();

    // 判断是否启用neodb
    match config.neodb_enable {
        true => {
            println!("neodb is enabled");
            // 判断是否下载neodb记录
            if config.neodb_download {
                println!("neodb game info will be downloaded");
                if api_neodb::download_user_rating(config.neodb_token.to_string()).await.unwrap() {
                    println!("neodb user rating info has been downloaded");
                }else{
                    // 红色字体
                    println!("\x1b[31m neodb game info download failed\x1b[0m");
                }
            }
            // 创建目录
            let index_id = api_neodb::create_index(&config.neodb_username, config.neodb_token.to_string()).await.unwrap();

            let collection_vec = api_neodb::read_user_rating().unwrap();

            // 添加用户评分到目录
            if api_neodb::add_subject(index_id, config.neodb_token.to_string(), collection_vec).await.unwrap() {
                // 绿色字体
                println!("neodb user rating has been added to index");
            }else{
                // 红色字体
                println!("\x1b[31m neodb user rating add failed\x1b[0m");
            }
        },
        _ => {
            println!("neodb is disabled");
        }
    }

    // 判断是否启用bangumi
    match config.bangumi_enable {
        true => {
            println!("bangumi is enabled");
            // 判断是否下载用户评分记录
            if config.bangumi_download {
                println!("bangumi user rating info will be downloaded");
                if api_bangumi::download_user_rating(&config.bangumi_username,config.bangumi_token.to_string()).await.unwrap() {
                    println!("bangumi user rating has been downloaded");
                }else{
                    // 红色字体
                    println!("\x1b[31m bangumi user rating download failed\x1b[0m");
                }
            }
            // 创建目录
            let index_id = api_bangumi::create_index(&config.bangumi_username,config.bangumi_token.to_string()).await.unwrap();

            let collection_vec = api_bangumi::read_user_rating().unwrap();

            // 添加用户评分到目录
            if api_bangumi::add_subject(index_id, config.bangumi_token, collection_vec).await.unwrap() {
                // 绿色字体
                println!("bangumi user rating has been added to index");
            }else{
                // 红色字体
                println!("\x1b[31m bangumi user rating add failed\x1b[0m");
            }
        },
        _ => {
            println!("bangumi is disabled");
        }
    }

    Ok(())
}
