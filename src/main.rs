use chrono::prelude::*;
use chrono_tz::Tz;
use prettytable::{Table, row};
use colored::*;
use std::fs;
use std::path::PathBuf;

fn get_local_timezone() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        if let Ok(tz) = std::env::var("TZ") {
            return Some(tz);
        }
    }

    #[cfg(unix)]
    {
        if let Ok(link) = fs::read_link("/etc/localtime") {
            if let Some(tz) = link.to_str() {
                if tz.starts_with("/usr/share/zoneinfo/") {
                    let tz = &tz["/usr/share/zoneinfo/".len()..];
                    return Some(tz.to_string());
                }
            }
        }

        let tz_path = PathBuf::from("/etc/timezone");
        if tz_path.exists() {
            if let Ok(tz) = fs::read_to_string(tz_path) {
                return Some(tz.trim().to_string());
            }
        }
    }

    None
}

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
}
