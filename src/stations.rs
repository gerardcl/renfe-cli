use pyo3::{pyclass, pyfunction, pymethods, PyResult};
use scraper::{Html, Selector};

#[pyfunction]
pub fn load_stations() -> Vec<String> {
    println!("loading stations from Renfe web");
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

#[pyclass]
pub struct Renfe {
    stations: Vec<String>,
}

#[pymethods]
impl Renfe {
    #[new]
    pub fn new() -> PyResult<Self> {
        Ok(Renfe {
            stations: load_stations(),
        })
    }

    fn check(&self, station: String) -> PyResult<Vec<&String>> {
        let found: Vec<&String> = self
            .stations
            .iter()
            .filter(|s| s.contains(&station))
            .collect();
        Ok(found)
    }
}
