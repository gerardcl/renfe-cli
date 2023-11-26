use pyo3::{
    exceptions::{PyConnectionError, PyValueError},
    pyclass, pymethods, PyResult,
};
use scraper::{Html, Selector};

#[pyclass]
pub struct Renfe {
    stations: Vec<String>,
}

#[pymethods]
impl Renfe {
    #[new]
    pub fn new() -> PyResult<Self> {
        println!("Loading stations from Renfe web");
        let response = match ureq::get("https://www.renfe.com/content/renfe/es/en/viajar/informacion-util/horarios/app-horarios.html").call() {
            Ok(response) => { response },
            Err(_) => { return Err(PyConnectionError::new_err("something wrong")) }
        };

        let parsed_html = Html::parse_document(&response.into_string().unwrap());

        let selector = &Selector::parse(r#"#O > option"#).unwrap();

        let stations: Vec<String> = parsed_html
            .select(selector)
            .flat_map(|el| el.text())
            .map(|t| t.to_string())
            .collect();

        Ok(Renfe {
            stations: stations[1..].to_vec(),
        })
    }

    pub fn stations_match(&self, station: String) -> PyResult<Vec<&String>> {
        let found: Vec<&String> = self
            .stations
            .iter()
            .filter(|s| s.contains(&station))
            .collect();
        Ok(found)
    }

    pub fn filter_station(&self, station: String) -> PyResult<String> {
        match self.stations_match(station.clone()) {
            Ok(v) if v.len() == 1 => {
                println!(
                    "Provided input '{}' station matches with '{}'...continue",
                    station, v[0]
                );
                Ok(v[0].to_owned())
            }
            Ok(v) => Err(PyValueError::new_err(format!(
                "Provided input '{station}' station does not match one '{v:?}'"
            ))),
            Err(e) => Err(e),
        }
    }
}
