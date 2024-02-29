use std::{collections::HashMap, fs};

use reqwest;
use serde_json::{json, Value};
use tokio::test;
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone, Deserialize, Serialize)]
pub struct MarkRecord {
    item: Item,
    comment_text: Option<String>,
    rating_grade: Option<usize>,
}

#[derive(Debug,Clone, Serialize,Deserialize)]
pub struct Item {
    uuid: String,
}

pub async fn download_user_rating(token:String) -> Result<bool, Box<dyn std::error::Error>> {
    let mut page = 1;
    let mut all_collections:Vec<MarkRecord> = Vec::new();
    
    let client = reqwest::Client::new();

    loop{
        // 拼接url
        let url = format!("https://neodb.social/api/me/shelf/complete?page={}", page);

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
            let collections: Vec<MarkRecord> = serde_json::from_value(body_json["data"].clone())?;
            all_collections.extend(collections);
            //检查是否还有数据
            if page < body_json["pages"].as_u64().unwrap() as usize{
                // 绿色字体
                println!("\x1b[32m downloaded {} user rating\x1b[0m", all_collections.len());
                page += 1;
            }else{
                break;
            }
        } 
        else {
            // 打印错误信息
            println!("Failed to download user rating: {}", response.status());
            println!("Response is {}",response.text().await?);
            println!("current page is {}",page);
            return Ok(false);
        }
    }
        // 将所有数据写入文件
        let serialized_data = serde_json::to_string(&all_collections)?;
        fs::write("neodb_user_rating.json", serialized_data)?;
        Ok(true)
}

pub fn read_user_rating() -> Result<Vec<MarkRecord>, Box<dyn std::error::Error>> {
    // 读取文件并处理可能的IO错误
    let file = fs::read_to_string("neodb_user_rating.json")?;

    // 将文件内容解析为JSON
    let collections:Vec<MarkRecord> = serde_json::from_str(&file)?;

    // 过滤出评分为10分
    let result:Vec<MarkRecord> = collections.iter()
                            .filter(|item| item.rating_grade == Some(10))
                            .cloned()
                            .collect();

    Ok(result)
}

// 创建目录并返回目录id
pub async fn create_index(name: &str, token: String) -> Result<String, reqwest::Error> {
    let url = "https://neodb.social/api/me/collection/";
    let authorization_header = format!("Bearer {}",token);
    let title = format!("{}的十分榜单",name);

    let body = json!({
        "title": title,
        "brief": "",
        "visibility": 0// 0:公开，1:仅关注者，2:仅自己
    });

    let client = reqwest::Client::new();
    //发送请求
    let res = client
        .post(url)
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
        let id = body["uuid"].to_string();
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
pub async fn add_subject(index_id:String,token:String,collection_vec:Vec<MarkRecord>) -> Result<bool,reqwest::Error>{
    let client = reqwest::Client::new();
    let url = format!("https://neodb.social/api/me/collection/{}/item/",index_id);
    let authorization_header = format!("Bearer {}",token);
    let mut count = 0;

    // 遍历collection_vec,发送请求
    for (_,record) in collection_vec.iter().enumerate(){
        let mut body = json!({
            "item_uuid" : record.item.uuid,
            "note": "" //该值不能缺省
        });

        // 检查 comment_text 是否存在
        if let Some(comment_text) = &record.comment_text {
            // 如果 comment_text 存在，则添加 note 字段
            body["note"] = json!(comment_text);
        }

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
            println!("\x1b[32mAdd subject {} successful!\x1b[0m", record.item.uuid);
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


