use pyo3::prelude::*;

use std::error::Error;
use std::fs;
use std::time::Duration;
use anyhow::Result;
use scraper::{Html, Selector, ElementRef};
use viuer::{Config, print};
use headless_chrome::{Browser, LaunchOptions};
use headless_chrome::protocol::cdp::Page;
use image;


#[pyfunction]
fn browse_renfe() {
    let browser = Browser::new(
        LaunchOptions {
            headless: true,
            sandbox: true,
            enable_gpu: false,
            enable_logging: false,
            idle_browser_timeout: Duration::from_secs(30),
            window_size: Some((1920,1080)),
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
        }
    ).unwrap();

    println!("loadingn new tab");
    let tab = browser.new_tab().unwrap();
    println!("got new tab");

    tab.navigate_to("https://www.renfe.com/es/es/viajar/informacion-util/horarios").unwrap();
    println!("navigating");

    // let _jpeg_data = tab.capture_screenshot(
    //     Page::CaptureScreenshotFormatOption::Jpeg,
    //     None,
    //     None,
    //     true)?;

    // let img = image::load_from_memory(&_jpeg_data).expect("Data from stdin could not be decoded.");
    // print(&img, &Config::default()).expect("Image printing failed.");

    tab.wait_until_navigated().unwrap().find_element_by_xpath(r#"//*[@id="O"]"#).unwrap().click().unwrap();
    println!("got input bar");

    tab.type_str("Girona").unwrap().press_key("Enter").unwrap();
    println!("search");


    tab.wait_until_navigated().unwrap().find_element_by_xpath(r#"//*[@id="D"]"#).unwrap().click().unwrap();
    println!("got input bar");

    tab.type_str("Barcelona").unwrap().press_key("Enter").unwrap();
    println!("search");

    tab.wait_until_navigated().unwrap().find_element_by_xpath(r#"//*[@id="DF"]"#).unwrap().click().unwrap();
    println!("got input bar");

    tab.type_str("23").unwrap().press_key("Enter").unwrap();
    println!("search");


    tab.wait_until_navigated().unwrap().find_element_by_xpath(r#"//*[@id="MF"]"#).unwrap().click().unwrap();
    println!("got input bar");

    tab.type_str("Nov").unwrap().press_key("Enter").unwrap();
    println!("search");


    tab.wait_until_navigated().unwrap().find_element_by_xpath(r#"//*[@id="AF"]"#).unwrap().click().unwrap();
    println!("got input bar");

    let elem = tab.type_str("2023").unwrap().press_key("Enter").unwrap();
    println!("searching");

    elem.press_key("Tab").unwrap().press_key("Enter").unwrap();

    println!("search");

    // wait on navigating to search result page
    tab.wait_until_navigated().unwrap().wait_for_elements_by_xpath(r#"//*[@id="contenedor"]"#).unwrap();

    println!("got html");

    let html = tab.wait_until_navigated().unwrap().find_element_by_xpath(r#"//*[@id="contenedor"]"#).unwrap().get_content().unwrap();

    let parsed_html = Html::parse_document(&html);

    let resum_selector = make_selector(r#"tr.odd"#);
    let total_tracks = parsed_html.select(&resum_selector);
    // println!("#trajectes: {:?}", &total_tracks.count());

    for track in total_tracks {
        println!("track:");
        let columns_selector: Selector = make_selector(r#"td"#);
        let columns = track.texts_parser(columns_selector);
        for (idx, column) in columns.iter().enumerate() {
            if (0..4).contains(&idx) {
                println!("#sortida: {:?}", &column);

            }
        }
    }

    // Ok(())
}

// Convenience function to avoid unwrap()ing all the time
fn make_selector(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

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

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn renfe_cli(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(browse_renfe, m)?)?;

    Ok(())
}
