pub fn calculate_moon_phase(year: i32, month: u32, day: u32) -> (f64, &'static str) {
    let lp = 2551443; // 平均朔望月の長さ（秒）
    let new_moon = 592500; // 基準となる新月のタイムスタンプ（1970年1月7日 20:35 UTC）

    let timestamp = chrono::NaiveDate::from_ymd_opt(year, month, day)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();

    let phase = ((timestamp - new_moon) % lp) as f64 / lp as f64;
    let phase = if phase < 0.0 { phase + 1.0 } else { phase };

    let phase_name = match (phase * 8.0).round() as u8 {
        0 => "新月",
        1 => "三日月",
        2 => "上弦の月",
        3 => "十三夜",
        4 => "満月",
        5 => "十六夜",
        6 => "下弦の月",
        7 => "二十六夜",
        _ => "不明",
    };

    (phase * 100.0, phase_name)
}
