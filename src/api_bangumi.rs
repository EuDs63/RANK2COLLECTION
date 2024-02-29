use std::{fmt::format, fs};

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::test;

pub async fn search(name:&str) -> Result<String,reqwest::Error>{
    let url = "https://api.bgm.tv/v0/search/subjects?limit=1";

    let body = json!({
        "keyword": name,
        "sort": "rank",
        "filter": {
            "type": [4],
            "nsfw": false
        }
    });


    let client = reqwest::Client::new();
    //发送请求
    let res = client
        .post(url)
        .header("User-Agent", "EuDs63/steam2record")
        .json(&body)
        .send()
        .await?;
    // 解析请求
    let body:Value = res.text().await?.parse().unwrap();
    //println!("{}",body);
    let id = body["data"][0]["id"].to_string();
    Ok(id)
}

#[derive(Debug,Clone, Deserialize, Serialize)]
pub struct CollectionItem {
    subject_id: usize,
    subject_type: usize,
    rate: usize,
}

pub async fn download_user_rating(name: &str,token:String) -> Result<bool, Box<dyn std::error::Error>> {
    let limit = 100;
    let mut offset = 0;
    let mut all_collections:Vec<CollectionItem> = Vec::new();
    
    let client = reqwest::Client::new();

    loop{
        // 拼接url
        let url = format!("https://api.bgm.tv/v0/users/{}/collections?limit={}&offset={}", name,limit,offset);

        let authorization_header = format!("Bearer {}",token);

        // 发起GET请求并等待响应
        let response = client
                        .get(&url)
                        .header(reqwest::header::AUTHORIZATION,authorization_header)
                        .header(reqwest::header::USER_AGENT, "EuDs63/RANK2COLLECTION")
                        .send()
                        .await?;

        // 检查响应状态
        if response.status().is_success() {
            let body_json: serde_json::Value = response.json().await?;
            let collections: Vec<CollectionItem> = serde_json::from_value(body_json["data"].clone())?;
            all_collections.extend(collections);
            //检查是否还有数据
            if all_collections.len() < body_json["total"].as_u64().unwrap() as usize{
                // 绿色字体
                println!("\x1b[32m downloaded {} user rating\x1b[0m", all_collections.len());
                // bangumi的offset指的是偏移值，每次请求100条数据，所以每次请求后offset+100
                offset += limit;
            }else{
                break;
            }
        } 
        else {
            // 打印错误信息
            println!("Failed to download user rating: {}", response.status());
            println!("Response is {}",response.text().await?);
            println!("current offset is {}",offset);
            return Ok(false);
        }
    }
        // 将所有数据写入文件
        let serialized_data = serde_json::to_string(&all_collections)?;
        fs::write("bangumi_user_rating.json", serialized_data)?;
        Ok(true)
}


// 读取bangumi_user_rating.json并输出10分评分
pub fn read_user_rating() -> Result<Vec<CollectionItem>, Box<dyn std::error::Error>> {
    // 读取文件并处理可能的IO错误
    let file = fs::read_to_string("bangumi_user_rating.json")?;

    // 将文件内容解析为JSON
    let collections:Vec<CollectionItem> = serde_json::from_str(&file)?;

    // 过滤出评分为10分
    let result:Vec<CollectionItem> = collections.iter()
                            .filter(|item| item.rate == 10)
                            .cloned()
                            .collect();

    // // 遍历JSON数组并输出评分
    // for item in collections.iter() {
    //     if item.rate  == 10 {
    //         println!("{}", item.subject_id);
    //     }
    // }

    Ok(result)
}

// 创建目录并返回目录id
pub async fn create_index(name: &str, token: String) -> Result<u64, reqwest::Error> {
    let url = "https://api.bgm.tv/v0/indices";
    let authorization_header = format!("Bearer {}",token);
    let title = format!("{}的十分榜单",name);

    let body = json!({
        "title": title,
    });

    let client = reqwest::Client::new();
    //发送请求
    let res = client
        .post(url)
        .header("User-Agent", "EuDs63/steam2record")
        .header("Authorization",authorization_header)
        .header("Content-Type","application/json")
        .json(&body)
        .send()
        .await?;

    // 获取响应状态码
    let status_code = res.status();

    // 根据状态码进行判断
    if status_code.is_success() {
        let body:Value = res.text().await?.parse().unwrap();
        let id = body["id"].as_u64().unwrap();
        // 绿色字体
        println!("\x1b[32mCreate index {} successful!\x1b[0m",id);
        return Ok(id);
    }
    // 创建失败
    // 红色字体
    println!("\x1b[31mCreate index failed!\x1b[0m");
    println!("Request failed! Status code: {}", status_code);
    println!("Response is {}",res.text().await?);
    panic!("Create index failed!");
}

// 添加subject到index
pub async fn add_subject(index_id:u64,token:String,collection_vec:Vec<CollectionItem>) -> Result<bool,reqwest::Error>{
    let client = reqwest::Client::new();
    let url = format!("https://api.bgm.tv/v0/indices/{}/subjects",index_id);
    let authorization_header = format!("Bearer {}",token);
    let mut count = 0;

    // 遍历collection_vec,发送请求
    for (i,item) in collection_vec.iter().enumerate(){
        let body = json!({
            "subject_id" : item.subject_id,
            "sort": i
        });

        //发送请求
        let res = client
            .post(&url)
            .header("User-Agent", "EuDs63/steam2record")
            .header("Authorization",&authorization_header)
            .json(&body)
            .send()
            .await?;

        // 获取响应状态码
        let status_code = res.status();

        // 根据状态码进行判断
        if status_code.is_success() {
            // 绿色字体
            println!("\x1b[32mAdd subject {} successful!\x1b[0m", item.subject_id);
            count += 1;
        }else{
            println!("Request failed! Status code: {}", status_code);
            println!("Response is {}",res.text().await?);
            return Ok(false);
        }
    }
    println!("Add {} subjects to index {} successful!",count,index_id);
    Ok(true)
}



#[test]
async fn test_search_function(){
    let name = "Squirrelmageddon!";
    let id  = search(name).await.unwrap();
    assert_eq!(id,"null".to_string());
}

#[test]
async fn test_create_indices(){
    let config = crate::app_config::AppConfig::from_file("config.toml").unwrap();
    let token = config.bangumi_token;
    let name = config.bangumi_username;
    let result = create_index(&name, token).await.unwrap();
    assert_eq!(result,56190);
}


