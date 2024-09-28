use chrono::{Datelike, NaiveDate, NaiveTime, TimeDelta, Timelike};
use gtfs_structures::Gtfs;
use pyo3::{exceptions::PyValueError, pyclass, pymethods, PyResult};

#[pyclass]
pub struct Renfe {
    gtfs: Gtfs,
    schedules: Vec<Schedule>,
}

// Struct to hold the schedule details
#[pyclass]
pub struct Schedule {
    train_type: String,
    origin_stop_name: String,
    destination_stop_name: String,
    departure_time: NaiveTime,
    arrival_time: NaiveTime,
    duration: TimeDelta,
}

#[pymethods]
impl Renfe {
    #[new]
    pub fn new() -> PyResult<Self> {
        println!("Loading GTFS data from Renfe web");
        let gtfs_zip_path = "./google_transit.zip"; // path to the GTFS zip file

        let gtfs = Gtfs::from_path(gtfs_zip_path) // from_url("https://ssl.renfe.com/gtransit/Fichero_AV_LD/google_transit.zip")
            .expect("Error parsing GTFS zip");

        // gtfs.print_stats();

        Ok(Renfe {
            gtfs,
            schedules: Vec::new(),
        })
    }

    pub fn stations_match(&self, station: String) -> PyResult<Vec<(String, String)>> {
        let found: Vec<(String, String)> = self
            .gtfs
            .stops
            .iter()
            .filter(|s| s.1.name.clone().unwrap().contains(&station))
            .map(|s| (s.1.name.clone().unwrap(), s.1.id.clone()))
            .collect();
        Ok(found)
    }

    pub fn filter_station(&self, station: String) -> PyResult<(String, String)> {
        match self.stations_match(station.clone()) {
            Ok(v) if v.len() == 1 => {
                println!(
                    "Provided input '{}' station matches with '{}'...continue",
                    station, v[0].0
                );
                Ok(v[0].clone())
            }
            Ok(v) => Err(PyValueError::new_err(format!(
                "Provided input '{station}' station does not match one '{v:?}'"
            ))),
            Err(e) => Err(e),
        }
    }

    // Function to get train schedules between an origin and a destination on a given date
    pub fn set_train_schedules(
        &mut self,
        origin_station_id: &str,
        destination_station_id: &str,
        day: u32,
        month: u32,
        year: i32,
        sorted: bool,
    ) -> PyResult<()> {
        let gtfs = &self.gtfs;
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap(); // the date for which schedules are needed

        let mut schedules = Vec::new();

        // Loop through each trip to find ones active on the given date
        for trip in gtfs.trips.values() {
            // Check if the trip's service is active on the given date
            if is_service_active(gtfs, &trip.service_id, date) {
                // Filter stop times for the trip
                let stop_times: Vec<_> = trip.stop_times.clone();

                // Find the origin and destination stops in the trip's stop times
                let origin_stop = stop_times.iter().find(|st| st.stop.id == origin_station_id);
                let destination_stop = stop_times
                    .iter()
                    .find(|st| st.stop.id == destination_station_id);

                // If the trip includes both origin and destination, and origin is before destination
                if let (Some(origin), Some(destination)) = (origin_stop, destination_stop) {
                    if origin.stop_sequence < destination.stop_sequence {
                        let time_origin = origin.departure_time.unwrap();
                        let time_destination = destination.arrival_time.unwrap();
                        let departure_time = NaiveTime::from_hms_opt(
                            time_origin / 3600,
                            time_origin % 3600 / 60,
                            time_origin % 60,
                        )
                        .unwrap();
                        let arrival_time = NaiveTime::from_hms_opt(
                            time_destination / 3600,
                            time_destination % 3600 / 60,
                            time_destination % 60,
                        )
                        .unwrap();
                        let duration = arrival_time.signed_duration_since(departure_time);

                        schedules.push(Schedule {
                            train_type: gtfs
                                .get_route(&trip.route_id)
                                .unwrap()
                                .short_name
                                .clone()
                                .unwrap(),
                            origin_stop_name: gtfs.stops[&origin.stop.id].name.clone().unwrap(),
                            destination_stop_name: gtfs.stops[&destination.stop.id]
                                .name
                                .clone()
                                .unwrap(),
                            departure_time,
                            arrival_time,
                            duration,
                        });
                    }
                }
            }
        }

        // Sort schedules by departure_time
        schedules.sort_by_key(|schedule| schedule.departure_time);

        if sorted {
            println!("sorting timetable by duration");
            schedules.sort_by(|a, b| a.duration.cmp(&b.duration));
        }

        self.schedules = schedules;

        Ok(())
    }

    pub fn print_timetable(&self) {
        println!("=========================TIMETABLE=========================");
        println!(
            "{0: <12} |   {1: <10} |   {2: <10} |   {3: <12}",
            "Train", "Departure", "Arrival", "Duration"
        );
        for track in &self.schedules {
            println!("-----------------------------------------------------------");
            println!(
                "{0: <12} |    {1: <9} |    {2: <9} |    {3: <10}",
                track.train_type,
                format!(
                    "{:02}:{:02}",
                    track.departure_time.hour(),
                    track.departure_time.minute() % 60
                ),
                format!(
                    "{:02}:{:02}",
                    track.arrival_time.hour(),
                    track.arrival_time.minute() % 60
                ),
                format!(
                    "{:02}:{:02}",
                    track.duration.num_hours(),
                    track.duration.num_minutes() % 60
                )
            );
        }
        println!("===========================================================");
    }
}

// Helper function to check if a service is active on a given date
fn is_service_active(gtfs: &Gtfs, service_id: &str, date: NaiveDate) -> bool {
    // First check the `calendar.txt`
    if let Some(calendar) = gtfs.calendar.get(service_id) {
        let weekday = match date.weekday() {
            chrono::Weekday::Mon => calendar.monday,
            chrono::Weekday::Tue => calendar.tuesday,
            chrono::Weekday::Wed => calendar.wednesday,
            chrono::Weekday::Thu => calendar.thursday,
            chrono::Weekday::Fri => calendar.friday,
            chrono::Weekday::Sat => calendar.saturday,
            chrono::Weekday::Sun => calendar.sunday,
        };

        if weekday && date >= calendar.start_date && date <= calendar.end_date {
            return true;
        }
    }

    // Then check the `calendar_dates.txt` for exceptions
    if let Some(calendar_dates) = gtfs.calendar_dates.get(service_id) {
        for date_override in calendar_dates {
            if date_override.date == date {
                return date_override.exception_type == gtfs_structures::Exception::Added;
            }
        }
    }

    false
}
