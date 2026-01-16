use std::{fs, io};
use chrono::NaiveDate;
use serde_json::Value;

fn task1_week(client: &mut untis::Client) -> Result<(), untis::Error>
{
  let mut timetable = client.own_timetable_current_week()?;

  if timetable.is_empty() {
    println!("No timetable found!");
    return Ok(());
  }

  timetable.sort_by(|a, b| {
    a.date.cmp(&b.date).then(a.start_time.cmp(&b.start_time))
  });

  for lesson in timetable.iter() {
    let day = lesson.date.format("%A").to_string();
    let start = lesson.start_time.format("%H:%M").to_string();
    let end = lesson.end_time.format("%H:%M").to_string();
    let lesson_names: Vec<&str> = lesson.subjects.iter().map(|s| s.name.as_str()).collect();
    let lessons = lesson_names.join("|");
    let teacher_names: Vec<&str> = lesson.teachers.iter().map(|t| t.name.as_str()).collect();
    let teachers = teacher_names.join(",");

    println!("{day} {start}-{end} {lessons} {teachers}")
  }

  Ok(())
}

fn task2_progress(client: &mut untis::Client) -> Result<(), untis::Error>
{
  let start_date = untis::Date(NaiveDate::from_ymd_opt(2024, 9, 2).unwrap());
  let today = untis::Date::today();
  let yesterday = untis::Date(untis::Date::today().checked_sub_days(chrono::Days::new(1)).unwrap());
  let end_date = untis::Date(NaiveDate::from_ymd_opt(2025, 6, 27).unwrap());
  
  let lessons_past = client.own_timetable_between(&start_date, &yesterday)?.len();
  let lessons_future = client.own_timetable_between(&today, &end_date)?.len();
  let lessons_total = lessons_past + lessons_future;
  let percent = (lessons_past as f64 / lessons_total as f64) * 100.0;
  
  println!("{}/{}  Unterrichtsstunden, {:.2}%", lessons_past, lessons_total, percent);
  Ok(())
}

fn main() -> Result<(), untis::Error> {
  let school = untis::schools::get_by_name("Spengergasse")?;

  let file = fs::File::open("login.json").expect("Could not open JSON file with login information!");
  let reader = io::BufReader::new(file);
  let login_json:Value = serde_json::from_reader(reader).unwrap();
  let username = login_json["username"].as_str().unwrap();
  let password = login_json["password"].as_str().unwrap();

  let mut client = school.client_login(username, password)?;
  task1_week(&mut client)?;
  task2_progress(&mut client)?;
  
  Ok(())
}
