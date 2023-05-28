extern crate uptime_lib;

fn main() {
    let minute: f64 = 60.0;
    let day: f64 = 24.0;
    match uptime_lib::get() {
        Ok(utime) => {
            let uptime = uptime.as_secs_f64();
            let hours = ((uptime/ minute / minute) - ((uptime / minute / minute / day).floor() * day)).floor();
            let mins = ((uptime / minute) - ((uptime / minute / minute).floor() * minute)).floor();
            let seconds = ((uptime) - ((uptime / minute).floor() * minute)).floor();
            let days = (uptime / minute / minute / day).floor();


            print!("Uptime: {} days,", days);
            print!(" {} hours,", hours);
            print!(" {} minutes,", mins);
            println!(" {} seconds", seconds);
        }
        Err(err) => {
            eprintln!("uptime: {}", err);
            std::process::exit(1);
        }
    }
}
