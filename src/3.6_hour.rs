fn main() {
    time_to_seconds(2, 1, 1);
    seconds_to_hour(7261);
}

fn time_to_seconds(hour: u64, minute: u64, second: u64) -> u64 {
    let final_seconds = second + (minute * 60) + (hour * 3600);
    // println!("{}", final_seconds);
    final_seconds
}

fn seconds_to_hour(seconds: u64) -> (u64, u64, u64) {
    let hour = seconds / 3600;
    let minute = (seconds % 3600 ) / 60;
    let second = (seconds % 3600) % 60;
    // println!("{} {} {}",hour, minute, second);
    (hour, minute, second)
}
