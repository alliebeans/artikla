use chrono::{DateTime, Utc, Local};

pub fn get_weekday_swedish(datetime: DateTime<Utc>) -> String {
    let weekday: String = match format!("{}", datetime.format("%u")).parse::<i32>().unwrap() {
        1 => String::from("Måndag"),
        2 => String::from("Tisdag"),
        3 => String::from("Onsdag"),
        4 => String::from("Torsdag"),
        5 => String::from("Fredag"),
        6 => String::from("Lördag"),
        7 => String::from("Söndag"),
        _ => String::from("Okänd"),
    };
    return format!("{}", weekday);
}
pub fn get_month_name_swedish(datetime: DateTime<Utc>) -> String {
    let month_name: String = match format!("{}", datetime.format("%B")).as_str() {
        "January" => String::from("januari"),
        "February" => String::from("februari"),
        "Mars" => String::from("mars"),
        "April" => String::from("april"),
        "May" => String::from("maj"),
        "June" => String::from("juni"),
        "July" => String::from("juli"),
        "August" => String::from("augusti"),
        "September" => String::from("september"),
        "October" => String::from("oktober"),
        "November" => String::from("november"),
        "December" => String::from("december"),
        _ => String::from("okänd"),
    };
    return format!("{}", month_name);
}
pub fn get_today_swedish(datetime: DateTime<Local>) -> String {
    let utc: DateTime<Utc> = DateTime::from(datetime);
    let weekday = get_weekday_swedish(utc);
    let month_name = get_month_name_swedish(utc);
    let day = format!("{} ", datetime.format("%-e"));
    let year = format!("{} ", datetime.format("%Y"));

    return format!("{} {} {} {}", weekday, day, month_name, year);
}