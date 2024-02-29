use csv::ReaderBuilder;
use std::{error::Error, fs::File};
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

    // let csv_path = "steam-library.csv";

    // let file = File::open(csv_path).expect("Couldn't open file");
    // let mut reader = ReaderBuilder::new().from_reader(file);

    // 遍历表头，获取索引
    // let headers = reader.headers().expect("no header");
    // let name_index = headers.iter().position(|r| r == "game").unwrap();
    // let hours_index = headers.iter().position(|r| r == "hours").unwrap();

    //记录总数
    // let mut neodb_count = 0;
    // let mut bangumi_count = 0;

    // 遍历
    // for result in reader.records() {
    //     let record = result.expect("Couldn't get record");
    //     // 处理 CSV 记录中的数据
    //     if let (Some(name), Some(hours)) = (record.get(name_index), record.get(hours_index)) {
    //         // 判断是否启用neodb
    //         match config.neodb_enable {
    //             true => {
    //                 if api_neodb::operate(name, hours, config.neodb_token.to_string()).await.unwrap()  {
    //                     neodb_count += 1;
    //                 }
    //             },
    //             _ => {
                    
    //             }
                
    //         }
    //         // 判断是否启用bangumi
    //         match config.bangumi_enable {
    //             true => {
    //                 if api_bangumi::operate(name, hours, config.bangumi_token.to_string()).await.unwrap() {
    //                     // 累加
    //                     bangumi_count += 1;
    //                 }
    //             },
    //             _ => {
                    
    //             }
    //         }

    //     }
    // }
    // println!("{} games have been marked on neodb", neodb_count);
    // println!("{} games have been marked on bangumi", bangumi_count);

    Ok(())
}
