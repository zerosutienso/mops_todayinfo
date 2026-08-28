
//[dependencies]
//tokio = { version = "1", features = ["full"] }
//reqwest = { version = "0.12", features = ["cookies", "json"] }
//scraper = "0.19"
//# 🎯 核心新增：CSV 編碼儲存套件
//csv = "1.3"
//regex = "1.10"
//# 🎯 核心新增：日期時間處理套件
//chrono = "0.4"


// 🎯 終極修復核心 1：改用新版升級後的核心數據路由 ajax_t05st01
//let init_url = "https://mopsov.twse.com.tw/mops/web/t05st02";
//let api_url = "https://mopsov.twse.com.tw/mops/web/ajax_t05st02";

//[dependencies]
//tokio = { version = "1", features = ["full"] }
//reqwest = { version = "0.12", features = ["cookies", "json"] }
//scraper = "0.19"
//csv = "1.3"
//chrono = "0.4"


use scraper::{Html, Selector};
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::path::Path;
use chrono::{Datelike, Local, Timelike}; // 🎯 引入 Timelike 用於判斷小時

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 🎯 初始化記憶體快取雜湊集合
    let mut visited_records: HashSet<String> = HashSet::new();
    
    // 🎯 紀錄當前的日期字串，用來比對是否跨日
    let mut current_date_str = String::new();

    let csv_path_prefix = "mops_report";

    println!("=== 🇹🇼 臺灣公開資訊觀測站 智慧巡檢（外部動態讀檔版）啟動 ===");

    // 🎯 核心無限循環監控：每 10 分鐘自動執行一次
    loop {
        // -----------------------------------------------------------------
        // 🎯 核心新增：每次循環都重新讀取外部文字檔，實現免編譯即時生效
        // -----------------------------------------------------------------
        let subscribed_stocks = load_config_file("stocks.txt");
        let keywords = load_config_file("keywords.txt");

        println!("📝 [載入設定] 訂閱個股: {:?} | 追蹤關鍵字: {:?}", subscribed_stocks, keywords);

        // 1. 自動獲取今天日期（自動換算為臺灣民國年）
        let now = Local::now();
        let query_year = (now.year() - 1911).to_string();
        let query_month = format!("{:02}", now.month());
        let query_day = format!("{:02}", now.day());
        
        // 組合當天的比對標籤，例: "115_08_24"
        let check_date_str = format!("{}_{}_{}", query_year, query_month, query_day);

<<<<<<< HEAD
        // 跨日自動更換檔案與清空快取
        if current_date_str != check_date_str {
            println!("\n📅 ──────────────────────────────────────────────────");
            if !current_date_str.is_empty() {
                println!("🌅 偵測到跨日！自動清空昨日記憶...");
                visited_records.clear(); 
            }
            current_date_str = check_date_str;
            println!("📅 今日鎖定日期：民國 {} 年 {} 月 {} 日", query_year, query_month, query_day);
            println!("────────────────────────────────────────────────────\n");
        }

        // 檔名自動依需求綁定當天年月日
        let csv_path = format!("{}_{}_{}_{}.csv", csv_path_prefix, query_year, query_month, query_day);

        // 冷啟動記憶功能
        if visited_records.is_empty() && Path::new(&csv_path).exists() {
            println!("📂 偵測到今日歷史 CSV 檔案，正在重建記憶索引以防止重複抓取...");
=======
        // --------------------------------────────────────-----------------
        // 🎯 跨日自動切換：當日期變更為新的一天 (New Day)
        // -----------------------------------------------------------------
        if current_date_str != check_date_str {
            println!("\n📅 ──────────────────────────────────────────────────");
            if !current_date_str.is_empty() {
                println!("🌅 偵測到日期跨日更換！自動為新的一天清空記憶體快取記錄...");
                visited_records.clear(); // 🎯 跨日徹底清空舊快取
            }
            current_date_str = check_date_str;
            println!("📅 今日鎖定巡檢日期：民國 {} 年 {} 月 {} 日", query_year, query_month, query_day);
            println!("────────────────────────────────────────────────────\n");
        }

        // 🎯 檔名完美依需求 1 綁定當天動態年月日
        let csv_path = format!("mops_report_{}_{}_{}.csv", query_year, query_month, query_day);

        // -----------------------------------------------------------------
        // 🎯 冷啟動記憶核心：開機時，若當天 CSV 檔案已存在，自動讀取並重建索引
        // -----------------------------------------------------------------
        if visited_records.is_empty() && Path::new(&csv_path).exists() {
            println!("📂 偵測到今日歷史備份 CSV 檔案已存在，正在自動讀取重建記憶體索引...");
>>>>>>> 7e436d9347b2b72a88a57a44f67054fd3d6cd442
            let file = OpenOptions::new().read(true).open(&csv_path)?;
            let mut csv_reader = csv::Reader::from_reader(file);
            let mut history_count = 0;
            // 逐行讀取 CSV 資料 (欄位順序: 發言日期, 發言時間, 公司代號, 公司簡稱, 主旨)
            for result in csv_reader.records() {
                if let Ok(record) = result {
                    // 確保欄位長度正確才解析，避免讀取到空白行損壞
                    if record.len() >= 5 {
<<<<<<< HEAD
=======
                        // 讀取 CSV 中的公司代號與發言時間
>>>>>>> 7e436d9347b2b72a88a57a44f67054fd3d6cd442
                        let time = record[1].trim();
                        let code = record[2].trim();
                        // 還原唯一識別碼格式: 公司代號_發言時間
                        let unique_key = format!("{}_{}", code, time);
                        visited_records.insert(unique_key);
                        history_count += 1;
                    }
                }
            }
<<<<<<< HEAD
            println!("✅ 索引重建成功！已同步過濾 {} 筆今日已抓取訊息。", history_count);
        }

        // 初始化當天專屬的 CSV 檔案 (加入防亂碼頭)
=======
            println!("✅ 今日索引重建成功！已自動同步過濾 {} 筆歷史重大訊息。", history_count);
        }

        // 初始化當天專屬的 CSV 檔案（若不存在則建立並加入不亂碼 BOM 頭與表頭）
>>>>>>> 7e436d9347b2b72a88a57a44f67054fd3d6cd442
        if !Path::new(&csv_path).exists() {
            println!("📝 正在建立今日專屬 CSV 資料庫：{}", csv_path);
            let file = std::fs::File::create(&csv_path)?;
            let mut buffered_file = std::io::BufWriter::new(file);
            use std::io::Write as _;
            buffered_file.write_all(b"\xEF\xBB\xBF")?; // 寫入 Excel 識別專用的 BOM 頭
            let mut csv_writer = csv::Writer::from_writer(buffered_file);
<<<<<<< HEAD
            csv_writer.write_record(&["發言日期", "發言時間", "公司代號", "公司簡稱", "主旨"])?;
=======
            // 🎯 欄位最尾端加入「詳細內文連結」欄位
            csv_writer.write_record(&["發言日期", "發言時間", "公司代號", "公司簡稱", "主旨", "詳細內文連結"])?;
>>>>>>> 7e436d9347b2b72a88a57a44f67054fd3d6cd442
            csv_writer.flush()?;
        }

        println!("🕒 [{}] 正在發起常規巡檢...", now.format("%Y-%m-%d %H:%M:%S"));

<<<<<<< HEAD
        // 2. 執行抓取任務
        match fetch_and_parse_mops(&query_year, &query_month, &query_day, &mut visited_records, &csv_path, &subscribed_stocks, &keywords).await {
            Ok(new_count) => {
                if new_count > 0 {
                    println!("✨ 巡檢完畢！本次發現 {} 筆全新重大訊息！", new_count);
                } else {
                    println!("😴 巡檢完畢。暫無新訊息更新。");
=======
        // 2. 執行非同步爬取與增量數據清洗任務
        match fetch_and_parse_mops(&query_year, &query_month, &query_day, &mut visited_records, &csv_path).await {
            Ok(new_count) => {
                if new_count > 0 {
                    println!("✨ 巡檢完畢！本次發現並實時更新了 {} 筆全新的重大訊息！", new_count);
                } else {
                    println!("😴 巡檢完畢。全市場暫無新重大訊息更新。");
>>>>>>> 7e436d9347b2b72a88a57a44f67054fd3d6cd442
                }
            },
            Err(e) => {
                eprintln!("⚠️ 本次巡檢發生網路或解析錯誤: {}，系統將於下個週期自動重試。", e);
                // 這裡加入防禦性冷卻休眠，防止因為斷網或連線過快被封鎖時，狂暴循環撞伺服器
                tokio::time::sleep(Duration::from_secs(10)).await;
            },
        }

<<<<<<< HEAD
        // 智慧時段判定：晚上 21:00 至 隔天 07:00 休眠 30 分鐘，其餘時段 10 分鐘
        let current_hour = now.hour();
        let sleep_duration = if current_hour >= 21 || current_hour < 7 {
            println!("🌙 夜間非尖峰時段 (21:00 - 07:00)，巡檢週期為【 30 分鐘 】");
            Duration::from_secs(1800) 
        } else {
            println!("☀️ 日間高峰時段，巡檢週期為【 10 分鐘 】");
            Duration::from_secs(600)  
=======
        // 3. 定時器核心：執行緒安全休眠 10 分鐘 (600 秒)
        // -----------------------------------------------------------------
        // 🎯 智慧時段判定核心：晚上 21:00 至 隔天清晨 07:00 休眠 30 分鐘，其餘時段 10 分鐘
        // -----------------------------------------------------------------
        let current_hour = now.hour();
        let sleep_duration = if current_hour >= 21 || current_hour < 7 {
            println!("🌙 當前處於夜間非尖峰時段 (21:00 - 07:00)，巡檢週期自動調整為【 30 分鐘 】");
            Duration::from_secs(1800) // 30 分鐘 = 1800 秒
        } else {
            println!("☀️ 當前處於日間股市交易高峰時段，巡檢週期維持【 10 分鐘 】");
            Duration::from_secs(600)  // 10 分鐘 = 600 秒
>>>>>>> 7e436d9347b2b72a88a57a44f67054fd3d6cd442
        };

        println!("💤 進入休眠，下一次巡檢將在等待後自動啟動...\n-----------------------------------------------------");
        tokio::time::sleep(sleep_duration).await;
    }
}

<<<<<<< HEAD
/// 輔助函數：讀取設定檔並過濾掉空白行與註解
fn load_config_file(file_name: &str) -> HashSet<String> {
    let mut set = HashSet::new();
    if let Ok(content) = fs::read_to_string(file_name) {
        for line in content.lines() {
            let trimmed = line.trim();
            // 過濾掉空白行或以 # 開頭的註解行
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                set.insert(trimmed.to_string());
            }
        }
    } else {
        // 如果檔案不存在，自動建立一個空的預設檔案，方便使用者未來修改
        let _ = fs::write(file_name, "# 請在此處每行輸入一個項目\n");
        println!("⚠️ 找不到 {}，已自動為您建立初始設定檔範本。", file_name);
    }
    set
}

=======
>>>>>>> 7e436d9347b2b72a88a57a44f67054fd3d6cd442
/// 核心爬取、清洗與增量過濾函數
async fn fetch_and_parse_mops(
    year: &str, 
    month: &str, 
    day: &str, 
    visited: &mut HashSet<String>,
<<<<<<< HEAD
    csv_path: &str,
    subscribed_stocks: &HashSet<String>,
    keywords: &HashSet<String>
=======
    csv_path: &str
>>>>>>> 7e436d9347b2b72a88a57a44f67054fd3d6cd442
) -> Result<usize, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;


    // 🎯 終極修復核心 1：改用新版升級後的核心數據路由 ajax_t05st01
    let init_url = "https://mopsov.twse.com.tw/mops/web/t05st02";
    let api_url = "https://mopsov.twse.com.tw/mops/web/ajax_t05st02";


    // 第一階段：獲取 Session 憑證
    let init_resp = client.get(init_url).send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
    if !init_resp.status().is_success() {
        return Err(format!("無法連線至網頁端，狀態碼: {}", init_resp.status()).into());
    }
    tokio::time::sleep(Duration::from_millis(600)).await;

    // 第二階段：發送完備的控制項 POST 表單參數
    let mut params = HashMap::new();
    params.insert("step", "1");               
    params.insert("step00", "0");             
    params.insert("firstin", "ture"); // 完美對齊網頁原始碼特有的拼字錯誤 "ture"
    params.insert("off", "1");                
    params.insert("TYPEK", "all");            
    
    // 傳遞經由時區換算出的精準參數，且值設定為空字串，防止後端伺服器噴出「年度未輸入」錯誤
    params.insert("year", year);        
    params.insert("month", month);      
    params.insert("day", day);         

    let res_obj = client.post(api_url).form(&params).send().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
    let response_text = res_obj.text().await.map_err(|e| Box::new(e) as Box<dyn Error>)?;
<<<<<<< HEAD
    //println!("response_text = {}", response_text);
=======
>>>>>>> 7e436d9347b2b72a88a57a44f67054fd3d6cd442
    
    // 載入 DOM 樹進行網頁數據格子清洗
    let document = Html::parse_document(&response_text);
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    // 以「追加模式 (Append)」打開當天專屬的 CSV 檔案
    let file = OpenOptions::new().write(true).append(true).open(csv_path)?;
    let mut csv_writer = csv::Writer::from_writer(file);

    let mut new_records_count = 0;

    for row in document.select(&row_selector) {
<<<<<<< HEAD
=======
        // 🎯 核心升級：同時提取內部的隱藏 inputs 欄位（用來獲取詳細內文所需跳轉的加密參數，如 i / TYPEK / pgname）
        let mut input_params: HashMap<String, String> = HashMap::new();
        let sub_input_selector = Selector::parse("input[type='hidden']").unwrap();
        for input in row.select(&sub_input_selector) {
            if let (Some(name), Some(value)) = (input.value().attr("name"), input.value().attr("value")) {
                input_params.insert(name.to_string(), value.to_string());
            }
        }

>>>>>>> 7e436d9347b2b72a88a57a44f67054fd3d6cd442
        let cells: Vec<String> = row
            .select(&cell_selector)
            .map(|cell| {
                cell.text()
                    .collect::<Vec<_>>()
                    .join("")
                    .replace("&nbsp;", "")
                    .trim()
                    .to_string()
            })
            .collect();

        if cells.len() >= 5 {
            let date = &cells[0];     // 發言日期 (例: 115/08/24)
            let time = &cells[1];     // 發言時間 (例: 16:04:51)
            let code = &cells[2];     // 公司代號 (例: 7850)
            let name = &cells[3];     // 公司簡稱 (例: 寶泰生醫)
            let subject = &cells[4];  // 主旨

            // 數據特徵校準
            if date.contains('/') && code.chars().all(|c| c.is_digit(10)) && !code.is_empty() {
                
                // 建立唯一識別碼
                let unique_key = format!("{}_{}", code, time);

                // 增量過濾核心：只有當天沒重複的資料才進行寫入與列印
                if !visited.contains(&unique_key) {
<<<<<<< HEAD
                    visited.insert(unique_key.clone());

                    // 實時追加寫入 CSV (維持純文字乾淨格式)
                    csv_writer.write_record(&[date, time, code, name, subject])?;

                    let truncated_subject = if subject.chars().count() > 35 {
                        format!("{}...", subject.chars().take(35).collect::<String>())
                    } else {
                        subject.to_string()
                    };
                    
                    // 🎯 核心高亮邏輯：判定是否符合訂閱股票或觸發關鍵字
                    let is_subscribed_stock = subscribed_stocks.contains(code);
                    let has_keyword = keywords.iter().any(|kw| subject.contains(kw));

                    if is_subscribed_stock && has_keyword {
                        // 🔥 雙重命中：黑字紅底 (極度危險或重要)
                        println!(
                            "\x1b[30;41m 💣 [重要!! 追蹤個股特選重訊] {}\t{}\t{}\t{:<10}\t{} \x1b[0m",
                            date, time, code, name, truncated_subject
                        );
                    } else if is_subscribed_stock {
                        // 🌟 命中股票：黑字黃底
                        println!(
                            "\x1b[30;43m 🚨 [訂閱個股新重訊] {}\t{}\t{}\t{:<10}\t{} \x1b[0m",
                            date, time, code, name, truncated_subject
                        );
                    } else if has_keyword {
                        // 💡 僅命中關鍵字：藍字黑底 (全市場焦點關注)
                        println!(
                            "\x1b[1;36m 🔍 [全市場焦點關鍵字] {}\t{}\t{}\t{:<10}\t{} \x1b[0m",
                            date, time, code, name, truncated_subject
                        );
                    } else {
                        // 無命中的常規普通顯示
                        println!(" 🆕 [新重訊] {}\t{}\t{}\t{:<10}\t{}", date, time, code, name, truncated_subject);
                    }
                    
=======
                    visited.insert(unique_key);

                    // -----------------------------------------------------------------
                    // 🎯 核心功能新增：動態推導並構造 Excel 專用的內嵌超連結公式
                    // -----------------------------------------------------------------
                    // 提取網頁表單中的動態序列參數，若提取不到則自動補齊安全預設值
                    let param_i = input_params.get("i").cloned().unwrap_or_else(|| "173".to_string());
                    let param_typek = input_params.get("TYPEK").cloned().unwrap_or_else(|| "rotc".to_string());
                    let param_h1732 = input_params.get("h1732").cloned().unwrap_or_else(|| "".to_string());
                    let param_h1733 = input_params.get("h1733").cloned().unwrap_or_else(|| "".to_string());

                    // 完美組裝官方 2026 最新版 RWD 重訊公告內文直連網址介面
                    let detail_url = format!(
                        "https://twse.com.tw?encodeURIComponent=1&step=1&firstin=1&off=1&pgname=t05st02&co_id={}&i={}&TYPEK={}&h1732={}&h1733={}",
                        code, param_i, param_typek, param_h1732, param_h1733
                    );

                    // 轉化為 Excel 專屬的自動公式字符串，格式為: =HYPERLINK("網址", "顯示名稱")
                    let excel_hyperlink_formula = format!(
                        "=HYPERLINK(\"{}\", \"點我開啟詳細重訊公文\")",
                        detail_url
                    );
                    // -----------------------------------------------------------------
                    // 實時追加寫入當天 CSV（內含超連結欄位）
                    csv_writer.write_record(&[date, time, code, name, subject, &excel_hyperlink_formula])?;
                    let truncated_subject = 
                        if subject.chars().count() > 35 {
                            format!("{}...", subject.chars().take(35).collect::<String>())
                        } else {
                            subject.to_string()
                        };
                    println!(" 🆕 [新重訊] {}\t{}\t{}\t{:<10}\t{}", date, time, code, name, truncated_subject);
>>>>>>> 7e436d9347b2b72a88a57a44f67054fd3d6cd442
                    new_records_count += 1;
                }
            }
        }
    }

    csv_writer.flush()?;
    Ok(new_records_count)
<<<<<<< HEAD
}
=======
}
>>>>>>> 7e436d9347b2b72a88a57a44f67054fd3d6cd442
