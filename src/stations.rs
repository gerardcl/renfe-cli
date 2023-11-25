use pyo3::pyfunction;
use scraper::{Html, Selector};

#[pyfunction]
pub fn load_stations() -> Vec<String> {
    let response = match ureq::get("https://www.renfe.com/content/renfe/es/en/viajar/informacion-util/horarios/app-horarios.html").call() {
        Ok(response) => { response },
        Err(_) => { panic!("something wrong") }
    };

    let parsed_html = Html::parse_document(&response.into_string().unwrap());

    let selector = &Selector::parse(r#"#O > option"#).unwrap();

    let stations: Vec<String> = parsed_html
        .select(selector)
        .flat_map(|el| el.text())
        .map(|t| t.to_string())
        .collect();

    stations[1..].to_vec()
}
