use chrono::{NaiveTime, TimeDelta, Timelike};
use jiff::civil::{Date, Weekday};
use pyo3::{PyResult, exceptions::PyRuntimeError, exceptions::PyValueError, pyclass, pymethods};
use std::collections::HashMap;

use crate::cache::{Feed, ServiceCalendar, ServiceCalendarDate, TransitData};
use crate::router::{Journey, Trip, find_journeys};

#[pyclass]
pub struct Renfe {
  data: TransitData,
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
  transfers: Vec<ScheduleTransfer>,
}

#[derive(Debug, PartialEq, Eq)]
struct ScheduleTransfer {
  location: String,
  arrival_time: NaiveTime,
  departure_time: NaiveTime,
  duration: TimeDelta,
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
    let feed = if cercanias {
      Feed::Cercanias
    } else {
      Feed::Default
    };
    let data = crate::cache::load(feed).map_err(PyRuntimeError::new_err)?;

    Ok(Renfe {
      data,
      schedules: Vec::new(),
    })
  }

  pub fn all_stations(&self) -> PyResult<Vec<Station>> {
    let stations: Vec<Station> = self
      .data
      .stations
      .iter()
      .map(|(id, station)| Station {
        name: station.name.clone(),
        id: id.clone(),
      })
      .collect();
    Ok(stations)
  }

  pub fn stations_match(&self, station: String) -> PyResult<Vec<Station>> {
    let found: Vec<Station> = self
      .data
      .stations
      .iter()
      .filter(|(_, candidate)| {
        candidate
          .name
          .to_lowercase()
          .contains(&station.to_lowercase())
      })
      .map(|(id, station)| Station {
        name: station.name.clone(),
        id: id.clone(),
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
    let data = &self.data;
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

    let trips: Vec<Trip> = data
      .trips
      .iter()
      .filter(|trip| {
        is_service_active(&data.calendar, &data.calendar_dates, &trip.service_id, date)
      })
      .map(|trip| trip.trip.clone())
      .collect();

    let mut schedules: Vec<_> = find_journeys(
      &trips,
      &data.transfers,
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
        transfers: schedule_transfers(&journey, |stop_id| {
          data
            .stations
            .get(stop_id)
            .map(|stop| stop.name.clone())
            .unwrap_or_else(|| stop_id.to_owned())
        }),
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
          track.transfers.len()
        );
        for (index, transfer) in track.transfers.iter().enumerate() {
          println!(
            "      Transfer {} at {}: arrive {}, depart {} ({})",
            index + 1,
            transfer.location,
            format_time(transfer.arrival_time),
            format_time(transfer.departure_time),
            format_duration(transfer.duration)
          );
        }
      }
      println!("=========================================================================");
    }
  }
}

fn schedule_transfers(
  journey: &Journey,
  stop_name: impl Fn(&str) -> String,
) -> Vec<ScheduleTransfer> {
  journey
    .legs
    .windows(2)
    .filter_map(|legs| {
      let arriving_leg = &legs[0];
      let departing_leg = &legs[1];
      let arrival_location = stop_name(&arriving_leg.to_stop_id);
      let departure_location = stop_name(&departing_leg.from_stop_id);
      let location = if arriving_leg.to_stop_id == departing_leg.from_stop_id {
        arrival_location
      } else {
        format!("{arrival_location} → {departure_location}")
      };

      Some(ScheduleTransfer {
        location,
        arrival_time: seconds_to_time(arriving_leg.arrival)?,
        departure_time: seconds_to_time(departing_leg.departure)?,
        duration: TimeDelta::seconds(i64::from(
          departing_leg.departure.saturating_sub(arriving_leg.arrival),
        )),
      })
    })
    .collect()
}

fn format_time(time: NaiveTime) -> String {
  format!("{:02}:{:02}", time.hour(), time.minute())
}

fn format_duration(duration: TimeDelta) -> String {
  format!(
    "{:02}:{:02}",
    duration.num_hours(),
    duration.num_minutes() % 60
  )
}

fn seconds_to_time(seconds: u32) -> Option<NaiveTime> {
  NaiveTime::from_hms_opt((seconds / 3600) % 24, seconds % 3600 / 60, seconds % 60)
}

// Helper function to check if a service is active on a given date
fn is_service_active(
  calendar: &HashMap<String, ServiceCalendar>,
  calendar_dates: &HashMap<String, Vec<ServiceCalendarDate>>,
  service_id: &str,
  date: Date,
) -> bool {
  // First check the `calendar.txt`
  if let Some(calendar) = calendar.get(service_id) {
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
      if let Some(calendar_dates) = calendar_dates.get(service_id) {
        for date_override in calendar_dates {
          if date_override.date == date {
            return date_override.added;
          }
        }
      }
      return true;
    }
  }

  // Then check the `calendar_dates.txt` for exceptions
  if let Some(calendar_dates) = calendar_dates.get(service_id) {
    for date_override in calendar_dates {
      if date_override.date == date {
        return date_override.added;
      }
    }
  }

  false
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::router::Leg;

  fn leg(name: &str, from: &str, to: &str, departure: u32, arrival: u32) -> Leg {
    Leg {
      trip_id: name.into(),
      name: name.into(),
      from_stop_id: from.into(),
      to_stop_id: to.into(),
      departure,
      arrival,
    }
  }

  #[test]
  fn describes_transfer_times_durations_and_locations() {
    let journey = Journey {
      legs: vec![
        leg("A", "origin", "central", 8 * 3_600, 9 * 3_600),
        leg("B", "central", "north", 9 * 3_600 + 15 * 60, 10 * 3_600),
        leg(
          "C",
          "south",
          "destination",
          10 * 3_600 + 30 * 60,
          11 * 3_600,
        ),
      ],
    };

    let transfers = schedule_transfers(&journey, |id| match id {
      "central" => "Central Station".into(),
      "north" => "North Station".into(),
      "south" => "South Station".into(),
      _ => id.into(),
    });

    assert_eq!(
      transfers,
      vec![
        ScheduleTransfer {
          location: "Central Station".into(),
          arrival_time: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
          departure_time: NaiveTime::from_hms_opt(9, 15, 0).unwrap(),
          duration: TimeDelta::minutes(15),
        },
        ScheduleTransfer {
          location: "North Station → South Station".into(),
          arrival_time: NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
          departure_time: NaiveTime::from_hms_opt(10, 30, 0).unwrap(),
          duration: TimeDelta::minutes(30),
        },
      ]
    );
  }
}
