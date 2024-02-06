use headless_chrome::{Browser, LaunchOptions};
use pyo3::{pyfunction, PyResult};
use scraper::{ElementRef, Html, Selector};
use std::{collections::HashMap, thread::sleep, time::Duration};

trait VecParser {
    fn texts_parser(&self, selector: Selector) -> Vec<String>;
}

impl VecParser for ElementRef<'_> {
    fn texts_parser(&self, selector: Selector) -> Vec<String> {
        self.select(&selector)
            .flat_map(|el| el.text())
            .map(|t| t.to_string())
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect()
    }
}

// Convenience function to avoid unwrap()ing all the time
fn make_selector(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

fn to_renfe_day(day: String) -> String {
    let first_digit: u8 = day.parse::<u8>().unwrap() / 10;
    let last_digit: u8 = day.parse::<u8>().unwrap() % 10;
    let mut renfe_day: String = String::new();
    if day.starts_with('0') {
        day
    } else {
        for _ in 0..last_digit + 1 {
            renfe_day += &first_digit.to_string();
        }
        renfe_day
    }
}

fn to_renfe_month(month: String) -> String {
    let months: HashMap<&str, &str> = HashMap::from([
        ("1", "Ene"),
        ("2", "Feb"),
        ("3", "Mar"),
        ("4", "Abr"),
        ("5", "May"),
        ("6", "Jun"),
        ("7", "Jul"),
        ("8", "Ago"),
        ("9", "Sep"),
        ("10", "Oct"),
        ("11", "Nov"),
        ("12", "Dec"),
    ]);
    months[month.as_str()].to_owned()
}

fn get_duration_from_renfe_string(s: &str) -> u8 {
    let splits: Vec<&str> = s.split(' ').collect();
    if s.contains('h') {
        let hours = splits[0].parse::<u8>().unwrap();
        let minutes = splits[2].parse::<u8>().unwrap();
        hours * 60 + minutes
    } else {
        splits[0].parse::<u8>().unwrap()
    }
}

#[pyfunction]
pub fn search_timetable(
    origin: String,
    destination: String,
    day: String,
    month: String,
    year: String,
    wait: u64,
    sorted: bool,
) -> PyResult<Vec<Vec<String>>> {
    println!("loading headless chrome browser");
    let browser = Browser::new(LaunchOptions {
        headless: true,
        sandbox: true,
        enable_gpu: false,
        enable_logging: false,
        idle_browser_timeout: Duration::from_secs(30),
        window_size: Some((1920, 1080)),
        path: None,
        user_data_dir: None,
        port: None,
        ignore_certificate_errors: true,
        extensions: Vec::new(),
        process_envs: None,
        fetcher_options: Default::default(),
        args: Vec::new(),
        disable_default_args: false,
        proxy_server: None,
    })
    .unwrap();

    let tab = browser.new_tab().unwrap();
    tab.set_default_timeout(Duration::from_secs(wait));

    println!("navigating to renfe timetable search page");
    tab.navigate_to("https://www.renfe.com/es/es/viajar/informacion-util/horarios")
        .unwrap()
        .wait_until_navigated()
        .unwrap();

    println!("waiting for search page");
    tab.wait_until_navigated()
        .unwrap()
        .wait_for_elements_by_xpath(r#"//*[@id="O"]"#)
        .unwrap();

    // let _jpeg_data = tab
    //     .capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)
    //     .unwrap();
    // std::fs::write("./screenshot1.jpg", _jpeg_data)?;

    println!("adding origin station");
    tab.find_element_by_xpath(r#"//*[@id="O"]"#)
        .unwrap()
        .type_into(&origin)
        .unwrap();

    println!("adding destination station");
    tab.find_element_by_xpath(r#"//*[@id="D"]"#)
        .unwrap()
        .type_into(&destination)
        .unwrap();

    println!("adding day");
    tab.find_element_by_xpath(r#"//*[@id="DF"]"#)
        .unwrap()
        .type_into(&to_renfe_day(day))
        .unwrap();

    println!("adding month");
    tab.find_element_by_xpath(r#"//*[@id="MF"]"#)
        .unwrap()
        .type_into(&to_renfe_month(month))
        .unwrap();

    println!("adding year");
    tab.find_element_by_xpath(r#"//*[@id="AF"]"#)
        .unwrap()
        .type_into(&year)
        .unwrap();

    println!("searching timetable");

    tab.find_element_by_xpath(r#"//*[@id="seleccion"]/fieldset/div[3]/button"#)
        .unwrap()
        .click()
        .unwrap();

    // wait on navigating and prepare search in result page
    let html = tab.wait_until_navigated().unwrap();
    println!("got timetable page");

    println!("wait for timetable iframe");
    sleep(Duration::from_secs(wait));

    let table_content = html
        .wait_for_elements_by_xpath(r#"//*[@id="contenedor"]"#)
        .unwrap()
        .first()
        .unwrap()
        .get_content()
        .unwrap();

    println!("loading timetable");

    let parsed_html = Html::parse_document(&table_content);

    let resum_selector = make_selector(r#"tr.odd"#);
    let total_tracks = parsed_html.select(&resum_selector);
    // println!("#trajectes: {:?}", &total_tracks.count());

    let mut tracks: Vec<Vec<String>> = Vec::new();
    for track in total_tracks {
        let columns_selector: Selector = make_selector(r#"td"#);
        let columns = track.texts_parser(columns_selector);
        let mut row = Vec::<String>::with_capacity(4);
        for (idx, column) in columns.iter().enumerate() {
            if idx == 0 {
                let train = column
                    .trim_start_matches(char::is_numeric)
                    .trim()
                    .to_owned();
                // println!("#sortida: {:?}", &train);
                row.push(train);
            }
            if (1..4).contains(&idx) {
                let timing = column.trim().to_owned();
                // println!("#sortida: {:?}", &timing);
                row.push(timing);
            }
        }
        tracks.push(row);
    }

    if sorted {
        println!("sorting timetable");
        tracks.sort_by(|a, b| {
            get_duration_from_renfe_string(&a[3]).cmp(&get_duration_from_renfe_string(&b[3]))
        });
    }

    Ok(tracks)
}

#[pyfunction]
pub fn print_timetable(tracks: Vec<Vec<String>>) {
    println!("=========================TIMETABLE=========================");
    println!(
        "{0: <12} |   {1: <10} |   {2: <10} | {3: <12}",
        "Train", "Departure", "Arrival", "Duration"
    );
    for track in tracks {
        println!("-----------------------------------------------------------");
        println!(
            "{0: <12} |    {1: <9} |    {2: <9} | {3: <12}",
            track[0], track[1], track[2], track[3]
        );
    }
    println!("===========================================================");
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Utc};

    use crate::{print_timetable, search_timetable};

    #[test]
    fn test_search_and_print_timetable() -> Result<(), Box<dyn std::error::Error>> {
        let now = Utc::now();

        print_timetable(search_timetable(
            "Girona".to_owned(),
            "Barcelona".to_owned(),
            now.day().to_string(),
            now.month().to_string(),
            now.year().to_string(),
            15,
            false,
        )?);

        Ok(())
    }
}
