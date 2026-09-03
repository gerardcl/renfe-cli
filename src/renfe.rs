use chrono::{NaiveTime, TimeDelta, Timelike};
use gtfs_structures::{Gtfs, TransferType};
use jiff::civil::{Date, Weekday};
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};
use std::io::Read;

use crate::router::{StopTime, Transfer, Trip, find_journeys};

#[pyclass]
pub struct Renfe {
  gtfs: Gtfs,
  schedules: Vec<Schedule>,
}

// Struct to hold the schedule details
#[pyclass]
pub struct Schedule {
  train_type: String,
  service_departure: u32,
  departure_time: NaiveTime,
  arrival_time: NaiveTime,
  duration: TimeDelta,
  transfers: usize,
}

// Struct to hold the station name and ID
#[pyclass(skip_from_py_object)]
#[derive(Debug, Clone)]
pub struct Station {
  pub name: String,
  pub id: String,
}

#[pymethods]
impl Renfe {
  #[new]
  pub fn new(cercanias: bool) -> PyResult<Self> {
    let mut res = reqwest::blocking::get(
            match cercanias {
                false => {
                    println!("Loading default GTFS data from Renfe web - Alta velocidad, Larga distancia y Media distancia");
                    "https://ssl.renfe.com/gtransit/Fichero_AV_LD/google_transit.zip"
                },
                true => {
                    println!("Loading Cercanías GTFS data from Renfe web - long load time");
                    "https://ssl.renfe.com/ftransit/Fichero_CER_FOMENTO/fomento_transit.zip"
                },
            },
        )
        .expect("Error downloading GTFS zip file");
    let mut body = Vec::new();
    res.read_to_end(&mut body)?;
    let cursor = std::io::Cursor::new(body);

    let gtfs = Gtfs::from_reader(cursor).expect("Error parsing GTFS zip");

    gtfs.print_stats();

    Ok(Renfe {
      gtfs,
      schedules: Vec::new(),
    })
  }

  pub fn all_stations(&self) -> PyResult<Vec<Station>> {
    let stations: Vec<Station> = self
      .gtfs
      .stops
      .iter()
      .map(|s| Station {
        name: s.1.name.clone().unwrap(),
        id: s.1.id.clone(),
      })
      .collect();
    Ok(stations)
  }

  pub fn stations_match(&self, station: String) -> PyResult<Vec<Station>> {
    let found: Vec<Station> = self
      .gtfs
      .stops
      .iter()
      .filter(|s| {
        s.1
          .name
          .clone()
          .unwrap()
          .to_lowercase()
          .contains(&station.to_lowercase())
      })
      .map(|s| Station {
        name: s.1.name.clone().unwrap(),
        id: s.1.id.clone(),
      })
      .collect();
    Ok(found)
  }

  pub fn filter_station(&self, station: String) -> PyResult<Station> {
    match self.stations_match(station.clone()) {
      Ok(v) if v.len() == 1 => {
        println!(
          "Provided input '{}' does a match with '{:?}'",
          station, v[0]
        );
        Ok(v[0].clone())
      },
      Ok(v) => Err(PyValueError::new_err(format!(
        "Provided input '{station}' does match with '{v:?}' -> There must be ONLY one match"
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
    // the date for which schedules are needed
    let date = i16::try_from(year)
      .ok()
      .zip(i8::try_from(month).ok())
      .zip(i8::try_from(day).ok())
      .and_then(|((year, month), day)| Date::new(year, month, day).ok());
    let date = match date {
      Some(date) => date,
      None => {
        return Err(PyValueError::new_err(format!(
          "Provided date '{year}-{month}-{day}' does not exist"
        )));
      },
    };

    let trips: Vec<_> = gtfs
      .trips
      .values()
      .filter(|trip| is_service_active(gtfs, &trip.service_id, date))
      .filter_map(|trip| {
        let stop_times: Vec<_> = trip
          .stop_times
          .iter()
          .filter_map(|stop_time| {
            Some(StopTime {
              stop_id: stop_time.stop.id.clone(),
              arrival: stop_time.arrival_time?,
              departure: stop_time.departure_time?,
            })
          })
          .collect();
        (stop_times.len() >= 2).then(|| Trip {
          id: trip.id.clone(),
          name: gtfs
            .get_route(&trip.route_id)
            .ok()
            .and_then(|route| route.short_name.clone().or_else(|| route.long_name.clone()))
            .or_else(|| trip.trip_short_name.clone())
            .unwrap_or_else(|| trip.route_id.clone()),
          stop_times,
        })
      })
      .collect();
    let transfers: Vec<_> = gtfs
      .stops
      .values()
      .flat_map(|stop| {
        stop.transfers.iter().map(|transfer| Transfer {
          from_stop_id: stop.id.clone(),
          to_stop_id: transfer.to_stop_id.clone(),
          duration: (transfer.transfer_type != TransferType::Impossible)
            .then(|| transfer.min_transfer_time.unwrap_or(5 * 60)),
        })
      })
      .collect();

    let mut schedules: Vec<_> = find_journeys(
      &trips,
      &transfers,
      origin_station_id,
      destination_station_id,
    )
    .into_iter()
    .filter_map(|journey| {
      let departure = journey.departure();
      let arrival = journey.arrival();
      Some(Schedule {
        train_type: journey
          .legs
          .iter()
          .map(|leg| leg.name.as_str())
          .collect::<Vec<_>>()
          .join(" → "),
        service_departure: departure,
        departure_time: seconds_to_time(departure)?,
        arrival_time: seconds_to_time(arrival)?,
        duration: TimeDelta::seconds(i64::from(arrival.saturating_sub(departure))),
        transfers: journey.transfers(),
      })
    })
    .collect();

    // Sort schedules by departure_time
    schedules.sort_by_key(|schedule| schedule.service_departure);

    if sorted {
      println!("sorting timetable by duration");
      schedules.sort_by_key(|schedule| schedule.duration);
    }

    self.schedules = schedules;

    Ok(())
  }

  pub fn print_timetable(&self) {
    if self.schedules.is_empty() {
      println!("\nNo schedules available...won't print timetable.");
    } else {
      println!("\n================================TIMETABLE================================");
      println!(
        "  {0: <22} | {1: <9} | {2: <7} | {3: <8} | {4: <9}",
        "Trains", "Departure", "Arrival", "Duration", "Transfers"
      );
      for track in &self.schedules {
        println!("-------------------------------------------------------------------------");
        println!(
          "  {0: <22} |   {1: <7} |  {2: <7} |  {3: <7} |     {4: <4}",
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
          ),
          track.transfers
        );
      }
      println!("=========================================================================");
    }
  }
}

fn seconds_to_time(seconds: u32) -> Option<NaiveTime> {
  NaiveTime::from_hms_opt((seconds / 3600) % 24, seconds % 3600 / 60, seconds % 60)
}

// Helper function to check if a service is active on a given date
fn is_service_active(gtfs: &Gtfs, service_id: &str, date: Date) -> bool {
  // First check the `calendar.txt`
  if let Some(calendar) = gtfs.calendar.get(service_id) {
    let weekday = match date.weekday() {
      Weekday::Monday => calendar.monday,
      Weekday::Tuesday => calendar.tuesday,
      Weekday::Wednesday => calendar.wednesday,
      Weekday::Thursday => calendar.thursday,
      Weekday::Friday => calendar.friday,
      Weekday::Saturday => calendar.saturday,
      Weekday::Sunday => calendar.sunday,
    };

    if weekday && date >= calendar.start_date && date <= calendar.end_date {
      // this should never happen - but a check is for free
      if let Some(calendar_dates) = gtfs.calendar_dates.get(service_id) {
        for date_override in calendar_dates {
          if date_override.date == date {
            return !(date_override.exception_type == gtfs_structures::Exception::Deleted);
          }
        }
      }
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
