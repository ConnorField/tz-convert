use chrono::prelude::*;
use chrono_tz::Tz;
use prettytable::{Table, row, cell};
use colored::*;
use std::env;

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

    for (name, tz) in timezones {
        let tz_time = local_time.with_timezone(&tz);
        table.add_row(row![
            name.bold().blue().to_string(),
            tz_time.format("%Y-%m-%d %H:%M:%S %Z").to_string()
        ]);
    }

    table.printstd();

    if let Ok(tz_str) = env::var("TZ") {
        match tz_str.parse::<Tz>() {
            Ok(system_tz) => {
                let system_time = local_time.with_timezone(&system_tz);
                println!(
                    "\n{}{}{}",
                    "System configured timezone (".bold().yellow().to_string(),
                    tz_str.bold().yellow().to_string(),
                    format!(
                        ") time: {}",
                        system_time.format("%Y-%m-%d %H:%M:%S %Z")
                    ).bold().yellow().to_string()
                );
            },
            Err(_) => println!("{}", "Failed to parse system timezone.".bold().red().to_string()),
        }
    } else {
        println!("{}", "TZ environment variable is not set.".bold().red().to_string());
    }
}
