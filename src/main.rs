use chrono::prelude::*;
use chrono_tz::Tz;
use std::env;

fn main() {
    let local_time = Local::now();
    println!("Local time: {}", local_time);

    let timezones = vec![
        chrono_tz::UTC,
        chrono_tz::Europe::London,
        chrono_tz::US::Eastern,
        chrono_tz::US::Central,
        chrono_tz::US::Mountain,
        chrono_tz::US::Pacific,
        chrono_tz::Europe::Berlin,
        chrono_tz::Asia::Tokyo,
        chrono_tz::Australia::Sydney,
    ];

    for tz in timezones {
        let tz_time = local_time.with_timezone(&tz);
        println!("Time in {}: {}", tz, tz_time);
    }

    if let Ok(tz_str) = env::var("TZ") {
        match tz_str.parse::<Tz>() {
            Ok(system_tz) => {
                let system_time = local_time.with_timezone(&system_tz);
                println!("System configured timezone ({}) time: {}", system_tz, system_time);
            },
            Err(_) => println!("Failed to parse system timezone."),
        }
    } else {
        println!("TZ environment variable is not set.");
    }
}
