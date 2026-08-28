
//[dependencies]
//tokio = { version = "1", features = ["full"] }
//reqwest = { version = "0.12", features = ["cookies", "json"] }
//scraper = "0.19"
//# 🎯 核心新增：CSV 編碼儲存套件
//csv = "1.3"
//regex = "1.10"
//# 🎯 核心新增：日期時間處理套件
//chrono = "0.4"

use scraper::{Html, Selector};
use std::collections::HashMap;
use std::time::Duration;
use std::error::Error;
use chrono::{Datelike, Local}; // 🎯 引入日期時間功能

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ---------------------------------------------------------
    // 🎯 核心自動化：獲取今天系統日期並自動換算為民國年格式
    // ---------------------------------------------------------
    let now = Local::now();
    
    // 西元年減去 1911 即為民國年
    let tw_year = now.year() - 1911; 
    let query_year = tw_year.to_string();
    
    // 格式化月份與日期，確保其為雙碼字串（例如 "08"、"24"）
    let query_month = format!("{:02}", now.month());
    let query_day = format!("{:02}", now.day());

    println!("=== 🇹🇼 臺灣公開資訊觀測站 當日重訊自動巡檢 ===");
    println!("📅 自動鎖定今日日期（民國格式）：{} 年 {} 月 {} 日", query_year, query_month, query_day);
    // ---------------------------------------------------------

    // 1. 初始化自動記憶 Cookie 憑證的 HTTP 客戶端
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    // 🎯 終極修復核心 1：改用新版升級後的核心數據路由 ajax_t05st01
    let init_url = "https://mopsov.twse.com.tw/mops/web/t05st02";
    let api_url = "https://mopsov.twse.com.tw/mops/web/ajax_t05st02";

     println!("\n第一階段 ➔ 🌐 正在獲取安全 Session 憑證簽章...");
    let init_resp = client.get(init_url)
        .send()
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        
    if !init_resp.status().is_success() {
        eprintln!("❌ 無法連線至公開資訊觀測站，狀態碼: {}", init_resp.status());
        return Ok(());
    }
    tokio::time::sleep(Duration::from_millis(600)).await;

    println!("第二階段 ➔ 🚀 完全複製前端表單參數，發送核心 POST 請求 (查詢日期: {}/{}/{})...", query_year, query_month, query_day);
    
    // 🎯 100% 複製您的 HTML 核心校驗參數
    let mut params = HashMap::new();
    params.insert("step", "1");               
    params.insert("step00", "0");             
    params.insert("firstin", "ture");         // 完美復刻網頁特有錯字 "ture"
    params.insert("off", "1");                
    params.insert("TYPEK", "all");            
    
    // 注入剛剛由終端機輸入的自訂年月日
    params.insert("year", &query_year);        
    params.insert("month", &query_month);      
    params.insert("day", &query_day);          

    // 發送核心 POST 大數據請求
    let res_obj = client
        .post(api_url)
        .form(&params)
        .send()
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    let response_text = res_obj
        .text()
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    // 4. 建立本地 CSV 檔案寫入器 (帶有 UTF-8 BOM 確保 Excel 打開不亂碼)
    let csv_path = format!("mops_report_{}_{}_{}.csv", query_year, query_month, query_day);
    let file = std::fs::File::create(&csv_path)?;
    
    use std::io::Write as _;
    let mut buffered_file = std::io::BufWriter::new(file);
    buffered_file.write_all(b"\xEF\xBB\xBF")?; 
    let mut csv_writer = csv::Writer::from_writer(buffered_file);
    csv_writer.write_record(&["發言日期", "發言時間", "公司代號", "公司簡稱", "主旨"])?;

    println!("\n=================================== 📊 成功獲取今日重大訊息 ===================================");
    println!("{:<10}\t{:<10}\t{:<8}\t{:<10}\t{}", "發言日期", "發言時間", "公司代號", "公司簡稱", "主旨");
    println!("---------------------------------------------------------------------------------------------");

    // 3. 載入 DOM 樹進行網頁數據清洗
    let document = Html::parse_document(&response_text);
    let row_selector = Selector::parse("table tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();
    let mut found_count = 0;

    for row in document.select(&row_selector) {
        let cells: Vec<String> = row
            .select(&cell_selector)
            .map(|cell| {
                // 將單元格內部的文字抽取出來，並徹底拔除網頁上的 "&nbsp;" 空白轉義字元
                cell.text()
                    .collect::<Vec<_>>()
                    .join("")
                    .replace("&nbsp;", "")
                    .trim()
                    .to_string()
            })
            .collect();

        // 確保基本數據欄位長度符合重訊列特徵
        if cells.len() >= 5 {
            let date = &cells[0];     
            let time = &cells[1];     
            let code = &cells[2];     
            let name = &cells[3];     
            let subject = &cells[4];  

            // 🎯 核心驗證：確保第一欄有日期斜線，且第三欄公司代號為純數字（過濾掉表格標頭欄位）
            if date.contains('/') && code.chars().all(|c| c.is_digit(10)) && !code.is_empty() {
                
                // 同步寫入 CSV
                csv_writer.write_record(&[date, time, code, name, subject])?;

                // 限制終端機畫面上的主旨顯示長度，保持排版整齊
                let truncated_subject = if subject.chars().count() > 35 {
                    format!("{}...", subject.chars().take(35).collect::<String>())
                } else {
                    subject.to_string()
                };

                // 完美在畫面上輸出
                println!("{}\t{}\t{}\t{:<10}\t{}", date, time, code, name, truncated_subject);
                found_count += 1;
            }
        }
    }

    csv_writer.flush()?;

    println!("---------------------------------------------------------------------------------------------");
    println!("📈 數據提取完畢。民國 {}/{}/{} 全市場共成功篩選並自動導出了 {} 筆即時重大訊息紀錄。", query_year, query_month, query_day, found_count);
    if found_count > 0 {
        println!("💾 試算表報表已完美生成：{}", csv_path);
    }
    println!("=============================================================================================\n");

    Ok(())
}