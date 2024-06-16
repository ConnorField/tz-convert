use chrono::prelude::*;
use chrono::NaiveDateTime;
use colored::*;
use dialoguer::{Confirm, Input};
use prettytable::{row, Table};
use std::process::Command;

fn main() {
    let local_time = Local::now();

    let us_timezones = vec![
        ("US/Eastern", chrono_tz::US::Eastern),
        ("US/Central", chrono_tz::US::Central),
        ("US/Mountain", chrono_tz::US::Mountain),
        ("US/Pacific", chrono_tz::US::Pacific),
        ("US/Alaska", chrono_tz::US::Alaska),
        ("US/Hawaii", chrono_tz::US::Hawaii),
    ];

    let europe_timezones = vec![
        ("Europe/London", chrono_tz::Europe::London),
        ("Europe/Berlin", chrono_tz::Europe::Berlin),
        ("Europe/Paris", chrono_tz::Europe::Paris),
        ("Europe/Moscow", chrono_tz::Europe::Moscow),
    ];

    let asia_timezones = vec![
        ("Asia/Tokyo", chrono_tz::Asia::Tokyo),
        ("Asia/Shanghai", chrono_tz::Asia::Shanghai),
        ("Asia/Kolkata", chrono_tz::Asia::Kolkata),
        ("Asia/Dubai", chrono_tz::Asia::Dubai),
    ];

    let australia_timezones = vec![
        ("Australia/Sydney", chrono_tz::Australia::Sydney),
        ("Australia/Melbourne", chrono_tz::Australia::Melbourne),
        ("Australia/Perth", chrono_tz::Australia::Perth),
    ];

    fn clear_screen() {
        if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
        } else {
            Command::new("clear").status().unwrap();
        }
    }

    fn display_timezones(
        local_time: DateTime<Local>,
        category: &str,
        timezones: &Vec<(&str, chrono_tz::Tz)>,
    ) {
        let mut table = Table::new();
        table.add_row(row![
            format!("Timezone ({})", category)
                .bold()
                .underline()
                .cyan()
                .to_string(),
            "Current Time".bold().underline().cyan().to_string()
        ]);

        for (name, tz) in timezones {
            let tz_time = local_time.with_timezone(tz);
            table.add_row(row![
                name.bold().blue().to_string(),
                tz_time.format("%Y-%m-%d %H:%M:%S %Z").to_string()
            ]);
        }

        table.printstd();
    }

    println!("{}", "Current Times:".bold().cyan().to_string());
    display_timezones(local_time, "US", &us_timezones);
    display_timezones(local_time, "Europe", &europe_timezones);
    display_timezones(local_time, "Asia", &asia_timezones);
    display_timezones(local_time, "Australia", &australia_timezones);

    let convert_time = Confirm::new()
        .with_prompt("Do you want to convert a time?".bold().yellow().to_string())
        .interact()
        .unwrap();

    if convert_time {
        let example_time = local_time.format("%Y-%m-%d %H:%M").to_string();

        let input_time: String = Input::new()
            .with_prompt(format!(
                "Enter a time to convert to other timezones\nExample: {}",
                example_time
            ))
            .interact_text()
            .unwrap();

        let naive_time = match NaiveDateTime::parse_from_str(&input_time, "%Y-%m-%d %H:%M") {
            Ok(dt) => dt,
            Err(_) => {
                eprintln!(
                    "{}",
                    "Invalid time format. Please use YYYY-MM-DD HH:MM."
                        .bold()
                        .red()
                        .to_string()
                );
                return;
            }
        };

        let user_time = Local.from_local_datetime(&naive_time).unwrap();

        clear_screen();

        println!(
            "{}",
            format!("There's the conversion for {}", input_time)
                .bold()
                .cyan()
                .to_string()
        );

        println!("{}", "US Timezones:".bold().cyan().to_string());
        display_timezones(user_time, "US", &us_timezones);

        println!("{}", "Europe Timezones:".bold().cyan().to_string());
        display_timezones(user_time, "Europe", &europe_timezones);

        println!("{}", "Asia Timezones:".bold().cyan().to_string());
        display_timezones(user_time, "Asia", &asia_timezones);

        println!("{}", "Australia Timezones:".bold().cyan().to_string());
        display_timezones(user_time, "Australia", &australia_timezones);
    }
}
