use scraper::{Html, Selector};
use std::collections::HashMap;
use std::time::Duration;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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


    println!("第一階段 ➔ 🌐 正在獲取安全 Session 憑證簽章...");
    let init_resp = client.get(init_url).send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
    if !init_resp.status().is_success() {
        eprintln!("❌ 無法連線至公開資訊觀測站，狀態碼: {}", init_resp.status());
        return Ok(());
    }
    tokio::time::sleep(Duration::from_millis(600)).await;

    println!("第二階段 ➔ 🚀 正在發送 POST 請求並清洗網頁數據...");
    
    let mut params = HashMap::new();
    params.insert("step", "1");
    params.insert("step00", "0");
    params.insert("firstin", "ture"); 
    params.insert("off", "1");
    params.insert("TYPEK", "all");
    
    // 🎯 終極修復核心：後端要求必須有這三個欄位！
    params.insert("year", "115");
    params.insert("month", "08");
    params.insert("day", "24");

    let res_obj = client
        .post(api_url)
        .form(&params)
        .send()
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    let response_text = res_obj.text().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
    //println!("response_text = {}", response_text );

    // 2. 建立本地不亂碼 CSV 檔案
    let csv_path = "mops_today_captured.csv";
    let file = std::fs::File::create(csv_path)?;
    let mut buffered_file = std::io::BufWriter::new(file);
    use std::io::Write;
    buffered_file.write_all(b"\xEF\xBB\xBF")?; 
    let mut csv_writer = csv::Writer::from_writer(buffered_file);
    csv_writer.write_record(&["發言日期", "發言時間", "公司代號", "公司簡稱", "主旨"])?;

    println!("\n=================================== 📊 成功獲取今日重大訊息 ===================================");
    println!("{:<10}\t{:<10}\t{:<8}\t{:<10}\t{}", "發言日期", "發言時間", "公司代號", "公司簡稱", "主旨");
    println!("---------------------------------------------------------------------------------------------");

    // 3. 使用 scraper 精準解析 HTML 標籤
    let document = Html::parse_document(&response_text);
    
    // 鎖定所有表格行 tr，以及裡面的單元格 td
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let mut found_count = 0;

    for row in document.select(&row_selector) {
        let cells: Vec<String> = row
            .select(&cell_selector)
            .map(|cell| {
                // 🎯 核心修正 1：將單元格內部的文字抽取出來，並徹底拔除網頁上的 "&nbsp;" 空白轉義字元
                cell.text()
                    .collect::<Vec<_>>()
                    .join("")
                    .replace("&nbsp;", "")
                    .trim()
                    .to_string()
            })
            .collect();

        // 🎯 核心修正 2：根據您提供的 HTML 結構，前五個欄位正是我們需要的核心數據
        if cells.len() >= 5 {
            let date = &cells[0];     // "115/08/24"
            let time = &cells[1];     // "16:04:51"
            let code = &cells[2];     // "7850" (原先被包在 <pre> 中，現在已被純文字化)
            let name = &cells[3];     // "寶泰生醫"
            let subject = &cells[4];  // "本公司受邀參加由寬量國際舉辦之..."

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
    println!("📈 今日數據提取完畢！全市場共成功篩選並自動導出了 {} 筆即時重大訊息紀錄。", found_count);
    if found_count > 0 {
        println!("💾 完整試算表報表已完美保存至本地：{}", csv_path);
    }
    println!("=============================================================================================\n");

    Ok(())
}