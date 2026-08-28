//[dependencies]
//headless_chrome = "1.0"
//scraper = "0.19"
//regex = "1.10"


use headless_chrome::{Browser, LaunchOptions};
use scraper::{Html, Selector};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 啟動瀏覽器（建議先開啟畫面觀察是否被驗證碼阻擋）
    let options = LaunchOptions { headless: false, ..LaunchOptions::default() };
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;
    
    // 🎯 核心修正：不走 index 首頁，直接導向「即時重大訊息」的專屬功能路由
    let target_url = "https://mops.twse.com.tw/mops/web/t05sr01_1";
    println!("🌐 正在直接跳轉至即時重大訊息功能頁: {}", target_url);
    tab.navigate_to(target_url)?;
    
    // 2. 給予充足時間讓 AJAX 容器與後台表格加載完畢
    println!("⏳ 正在等待重訊表格載入完成...");
    std::thread::sleep(Duration::from_secs(6));

    // 3. 獲取渲染完畢後的真實 HTML
    let page_content = tab.get_content()?;
    let document = Html::parse_document(&page_content);

    // 4. 定義表格選擇器
    // MOPS 的數據行通常被包在 class 含有 report_cont 或 odd/even 的 table 之中
    let row_selector = Selector::parse("table tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    println!("\n=================================== 📊 成功解析：即時重大訊息 ===================================");
    println!("{:<8}\t{:<10}\t{:<10}\t{:<10}\t{}", "公司代號", "公司簡稱", "發言日期", "發言時間", "主旨");
    println!("---------------------------------------------------------------------------------------------");

    let mut found_count = 0;

    // 5. 開始清洗並處理每一行資料
    for row in document.select(&row_selector) {
        // 取出該 tr 行內所有的 td 單元格文字
        let cells: Vec<String> = row
            .select(&cell_selector)
            .map(|cell| cell.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();

        // 💡 關鍵寬鬆清洗匹配：
        // 畫面上標準重訊行包含：公司代號(0)、公司簡稱(1)、發言日期(2)、發言時間(3)、主旨(4)
        if cells.len() >= 5 {
            let code = &cells[0];
            let name = &cells[1];
            let date = &cells[2];
            let time = &cells[3];
            let subject = &cells[4];

            // 🎯 防錯校準驗證：
            // 1. 公司代號必須全部是數字 (例如 6610)
            // 2. 公司代號長度通常為 4 碼 (上市櫃) 或 6 碼 (部分公開發行)
            // 3. 發言日期應該包含斜線（如 115/08/24 或 112/01/24）
            if code.chars().all(|c| c.is_digit(10)) && (code.len() == 4 || code.len() == 6) && date.contains('/') {
                // 格式化主旨輸出，避免過長換行
                let truncated_subject = if subject.chars().count() > 40 {
                    format!("{}...", subject.chars().take(40).collect::<String>())
                } else {
                    subject.to_string()
                };

                println!("{}\t{:<10}\t{}\t{}\t{}", code, name, date, time, truncated_subject);
                found_count += 1;
            }
        }
    }

    println!("---------------------------------------------------------------------------------------------");
    println!("📈 本次成功解析抓取到 {} 筆即時重大訊息紀錄。", found_count);
    println!("=============================================================================================\n");

    if found_count == 0 {
        println!("⚠️ 警告：仍未抓取到數據。請確認當前打開的瀏覽器視窗是否出現了「圖形驗證碼」或「請先點擊確認」的阻擋彈窗？");
    }

    Ok(())
}