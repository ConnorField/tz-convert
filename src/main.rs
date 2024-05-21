use chrono::prelude::*;
use prettytable::{Table, row};
use colored::*;
use dialoguer::Input;
use chrono::NaiveDateTime;

fn main() {
    let local_time = Local::now();

    let timezones = vec![
        ("UTC", chrono_tz::UTC),
        ("US/Eastern", chrono_tz::US::Eastern),
        ("US/Central", chrono_tz::US::Central),
        ("US/Mountain", chrono_tz::US::Mountain),
        ("US/Pacific", chrono_tz::US::Pacific),
        ("Europe/London", chrono_tz::Europe::London),
        ("Europe/Berlin", chrono_tz::Europe::Berlin),
        ("Asia/Tokyo", chrono_tz::Asia::Tokyo),
        ("Australia/Sydney", chrono_tz::Australia::Sydney),
    ];

    let mut table = Table::new();
    table.add_row(row![
        "Timezone".bold().underline().cyan().to_string(),
        "Current Time".bold().underline().cyan().to_string()
    ]);

    table.add_row(row![
        "Local time".bold().green().to_string(),
        local_time.format("%Y-%m-%d %H:%M:%S %Z").to_string()
    ]);

    for (name, tz) in &timezones {
        let tz_time = local_time.with_timezone(tz);
        table.add_row(row![
            name.bold().blue().to_string(),
            tz_time.format("%Y-%m-%d %H:%M:%S %Z").to_string()
        ]);
    }

    table.printstd();

    // Prompt the user to enter a time
    let input_time: String = Input::new()
        .with_prompt("Enter a time in your local timezone (YYYY-MM-DD HH:MM, e.g., 2024-05-21 15:30)")
        .interact_text()
        .unwrap();

    // Parse the user input
    let naive_time = match NaiveDateTime::parse_from_str(&input_time, "%Y-%m-%d %H:%M") {
        Ok(dt) => dt,
        Err(_) => {
            eprintln!("{}", "Invalid time format. Please use YYYY-MM-DD HH:MM.".bold().red().to_string());
            return;
        }
    };

    // Convert the naive datetime to local datetime
    let user_time = Local.from_local_datetime(&naive_time).unwrap();

    // Create a new table for the specified time
    let mut specified_table = Table::new();
    specified_table.add_row(row![
        "Timezone".bold().underline().cyan().to_string(),
        "Specified Time".bold().underline().cyan().to_string()
    ]);

    specified_table.add_row(row![
        "Local time".bold().green().to_string(),
        user_time.format("%Y-%m-%d %H:%M:%S %Z").to_string()
    ]);

    for (name, tz) in &timezones {
        let tz_time = user_time.with_timezone(tz);
        specified_table.add_row(row![
            name.bold().blue().to_string(),
            tz_time.format("%Y-%m-%d %H:%M:%S %Z").to_string()
        ]);
    }

    println!("\nTime for the specified input:");
    specified_table.printstd();
}
