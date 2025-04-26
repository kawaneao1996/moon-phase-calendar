use chrono::{Datelike, Local, NaiveDate};
use dioxus::prelude::*;

fn main() {
    // dioxusのデスクトップランチャーを起動
    // dioxus_desktop::launch(App);
    dioxus::launch(App);
}

#[derive(Debug, Clone)]
struct CalendarState {
    current_year: i32,
    current_month: u32,
}

impl Default for CalendarState {
    fn default() -> Self {
        let today = Local::now();
        Self {
            current_year: today.year(),
            current_month: today.month(),
        }
    }
}

fn App() -> Element {
    let mut calendar_state = use_signal(|| CalendarState::default());

    let today = Local::now();
    let today_year = today.year();
    let today_month = today.month();
    let today_day = today.day();

    let year = calendar_state().current_year;
    let month = calendar_state().current_month;

    let month_name = match month {
        1 => "1月",
        2 => "2月",
        3 => "3月",
        4 => "4月",
        5 => "5月",
        6 => "6月",
        7 => "7月",
        8 => "8月",
        9 => "9月",
        10 => "10月",
        11 => "11月",
        12 => "12月",
        _ => "",
    };

    let first_day_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 30,
    };

    // 月曜日を0、日曜日を6とする（chrono::Weekdayは月曜が0、日曜が6）
    let first_day_weekday = first_day_of_month.weekday().num_days_from_monday();

    rsx! {
        div {
            style: "font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px;",

            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",

                button {
                    onclick: move |_| {
                        calendar_state.write().current_month = if calendar_state().current_month == 1 {
                            calendar_state.write().current_year -= 1;
                            12
                        } else {
                            calendar_state().current_month - 1
                        };
                    },
                    "前月"
                }

                h1 {
                    style: "text-align: center; margin: 0;",
                    "{year}年{month_name}"
                }

                button {
                    onclick: move |_| {
                        calendar_state.write().current_month = if calendar_state().current_month == 12 {
                            calendar_state.write().current_year += 1;
                            1
                        } else {
                            calendar_state().current_month + 1
                        };
                    },
                    "翌月"
                }
            }

            div {
                style: "display: grid; grid-template-columns: repeat(7, 1fr); text-align: center;",

                // 曜日の表示
                div { style: "padding: 10px; font-weight: bold; color: red;", "日" }
                div { style: "padding: 10px; font-weight: bold;", "月" }
                div { style: "padding: 10px; font-weight: bold;", "火" }
                div { style: "padding: 10px; font-weight: bold;", "水" }
                div { style: "padding: 10px; font-weight: bold;", "木" }
                div { style: "padding: 10px; font-weight: bold;", "金" }
                div { style: "padding: 10px; font-weight: bold; color: blue;", "土" }

                // 前月の空白部分
                for _ in 0..first_day_weekday {
                    div { style: "padding: 10px;" }
                }

                // 日付の表示
                for day in 1..=days_in_month {
                    {
                        let is_today = year == today_year && month == today_month && day == today_day;
                        let day_of_week = (first_day_weekday + (day - 1) as u32) % 7;

                        let style = if is_today {
                            "padding: 10px; background-color: #ffeb3b; border-radius: 50%; font-weight: bold;"
                        } else if day_of_week == 0 { // 日曜
                            "padding: 10px; color: red;"
                        } else if day_of_week == 6 { // 土曜
                            "padding: 10px; color: blue;"
                        } else {
                            "padding: 10px;"
                        };

                        rsx!{
                            div {
                                style: style,
                                "{day}"
                            }
                        }
                    }
                }
            }
        }
    }
}
