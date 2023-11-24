use pyo3::prelude::*;

use headless_chrome::{Browser, LaunchOptions};
use scraper::{ElementRef, Html, Selector};
use std::time::Duration;

trait VecParser {
    fn texts_parser(&self, selector: Selector) -> Vec<String>;
    fn alts_parser(&self, selector: Selector) -> Vec<String>;
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
    fn alts_parser(&self, selector: Selector) -> Vec<String> {
        self.select(&selector)
            .flat_map(|el| el.value().attr("alt"))
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

#[pyfunction]
pub fn search_timetable(
    origin: String,
    destination: String,
    day: String,
    month: String,
    year: String,
) -> Vec<Vec<String>> {
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

    println!("loadingn new tab");
    let tab = browser.new_tab().unwrap();
    println!("got new tab");

    tab.navigate_to("https://www.renfe.com/es/es/viajar/informacion-util/horarios")
        .unwrap();
    println!("navigating");

    // let _jpeg_data = tab.capture_screenshot(
    //     Page::CaptureScreenshotFormatOption::Jpeg,
    //     None,
    //     None,
    //     true)?;

    // let img = image::load_from_memory(&_jpeg_data).expect("Data from stdin could not be decoded.");
    // print(&img, &Config::default()).expect("Image printing failed.");
    tab.wait_until_navigated()
        .unwrap()
        .wait_for_elements_by_xpath(r#"//*[@id="O"]"#)
        .unwrap();

    tab.find_element_by_xpath(r#"//*[@id="O"]"#)
        .unwrap()
        .click()
        .unwrap();
    println!("got input bar");

    tab.type_str(&origin).unwrap().press_key("Enter").unwrap();
    println!("search");

    tab.find_element_by_xpath(r#"//*[@id="D"]"#)
        .unwrap()
        .click()
        .unwrap();
    println!("got input bar");

    tab.type_str(&destination)
        .unwrap()
        .press_key("Enter")
        .unwrap();
    println!("search");

    tab.find_element_by_xpath(r#"//*[@id="DF"]"#)
        .unwrap()
        .click()
        .unwrap();
    println!("got input bar");

    tab.type_str(&day).unwrap().press_key("Enter").unwrap();
    println!("search");

    tab.find_element_by_xpath(r#"//*[@id="MF"]"#)
        .unwrap()
        .click()
        .unwrap();
    println!("got input bar");

    tab.type_str(&month).unwrap().press_key("Enter").unwrap();
    println!("search");

    tab.find_element_by_xpath(r#"//*[@id="AF"]"#)
        .unwrap()
        .click()
        .unwrap();
    println!("got input bar");

    let elem = tab.type_str(&year).unwrap().press_key("Enter").unwrap();
    println!("searching");

    elem.press_key("Tab").unwrap().press_key("Enter").unwrap();

    println!("search");

    // wait on navigating to search result page
    tab.wait_until_navigated()
        .unwrap()
        .wait_for_elements_by_xpath(r#"//*[@id="contenedor"]"#)
        .unwrap();

    println!("got html");

    let html = tab
        .find_element_by_xpath(r#"//*[@id="contenedor"]"#)
        .unwrap()
        .get_content()
        .unwrap();

    let parsed_html = Html::parse_document(&html);

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

    tracks
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
    use crate::{print_timetable, search_timetable};

    #[test]
    fn test_search_timetable() -> Result<(), Box<dyn std::error::Error>> {
        print_timetable(search_timetable(
            "Girona".to_owned(),
            "Barcelona".to_owned(),
            "28".to_owned(),
            "Nov".to_owned(),
            "2023".to_owned(),
        ));

        Ok(())
    }
}
