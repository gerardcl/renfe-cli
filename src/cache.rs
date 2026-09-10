use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use directories::BaseDirs;
use gtfs_structures::{Exception, Gtfs, TransferType};
use jiff::civil::Date;
use reqwest::StatusCode;
use reqwest::blocking::{Client, Response};
use reqwest::header::{ETAG, IF_NONE_MATCH};
use serde::{Deserialize, Serialize};

use crate::router::{StopTime, Transfer, Trip};

const CACHE_FORMAT_VERSION: u8 = 2;

#[derive(Clone, Copy)]
pub(crate) enum Feed {
  Default,
  Cercanias,
}

impl Feed {
  fn url(self) -> &'static str {
    match self {
      Self::Default => "https://ssl.renfe.com/gtransit/Fichero_AV_LD/google_transit.zip",
      Self::Cercanias => "https://ssl.renfe.com/ftransit/Fichero_CER_FOMENTO/fomento_transit.zip",
    }
  }

  fn cache_name(self) -> &'static str {
    match self {
      Self::Default => "default",
      Self::Cercanias => "cercanias",
    }
  }

  fn loading_message(self) -> &'static str {
    match self {
      Self::Default => {
        "Loading default GTFS data from Renfe web - Alta velocidad, Larga distancia y Media distancia"
      },
      Self::Cercanias => "Loading Cercanías GTFS data from Renfe web - long load time",
    }
  }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct TransitData {
  pub(crate) stations: HashMap<String, TransitStation>,
  pub(crate) trips: Vec<ServiceTrip>,
  pub(crate) transfers: Vec<Transfer>,
  pub(crate) calendar: HashMap<String, ServiceCalendar>,
  pub(crate) calendar_dates: HashMap<String, Vec<ServiceCalendarDate>>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct TransitStation {
  pub(crate) name: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ServiceTrip {
  pub(crate) service_id: String,
  pub(crate) trip: Trip,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ServiceCalendar {
  pub(crate) monday: bool,
  pub(crate) tuesday: bool,
  pub(crate) wednesday: bool,
  pub(crate) thursday: bool,
  pub(crate) friday: bool,
  pub(crate) saturday: bool,
  pub(crate) sunday: bool,
  pub(crate) start_date: Date,
  pub(crate) end_date: Date,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ServiceCalendarDate {
  pub(crate) date: Date,
  pub(crate) added: bool,
}

struct CachePaths {
  data: PathBuf,
  etag: PathBuf,
}

pub(crate) fn load(feed: Feed) -> Result<TransitData, String> {
  let paths = cache_paths(feed);
  let cached_etag = paths
    .as_ref()
    .and_then(|paths| fs::read_to_string(&paths.etag).ok())
    .filter(|etag| !etag.trim().is_empty());

  // Renfe's download server currently ignores conditional headers over HTTP/2.
  let client = Client::builder()
    .http1_only()
    .build()
    .map_err(|error| format!("Error creating HTTP client: {error}"))?;
  let mut request = client.get(feed.url());
  if let Some(etag) = cached_etag.as_deref() {
    request = request.header(IF_NONE_MATCH, etag.trim());
  }

  let response = request
    .send()
    .map_err(|error| format!("Error checking Renfe GTFS data: {error}"))?;

  if response.status() == StatusCode::NOT_MODIFIED
    && let Some(paths) = &paths
  {
    match read_cache(&paths.data) {
      Ok(data) => {
        println!("Loading processed GTFS data from cache");
        data.print_stats();
        return Ok(data);
      },
      Err(error) => {
        eprintln!("Cached GTFS data could not be read ({error}); downloading it again");
        let response = client
          .get(feed.url())
          .send()
          .map_err(|error| format!("Error downloading Renfe GTFS data: {error}"))?;
        return download_and_cache(feed, response, Some(paths));
      },
    }
  }

  download_and_cache(feed, response, paths.as_ref())
}

fn download_and_cache(
  feed: Feed,
  response: Response,
  paths: Option<&CachePaths>,
) -> Result<TransitData, String> {
  let response = response
    .error_for_status()
    .map_err(|error| format!("Error downloading Renfe GTFS data: {error}"))?;
  let etag = response
    .headers()
    .get(ETAG)
    .and_then(|value| value.to_str().ok())
    .map(str::to_owned);
  let body = response
    .bytes()
    .map_err(|error| format!("Error reading Renfe GTFS data: {error}"))?;

  println!("{}", feed.loading_message());
  let cursor = std::io::Cursor::new(body);
  let gtfs =
    Gtfs::from_reader(cursor).map_err(|error| format!("Error parsing GTFS zip: {error}"))?;
  gtfs.print_stats();
  let data = TransitData::from(gtfs);

  if let Some(paths) = paths
    && let Err(error) = write_cache(paths, &data, etag.as_deref())
  {
    eprintln!("Processed GTFS data could not be cached: {error}");
  }

  Ok(data)
}

fn cache_paths(feed: Feed) -> Option<CachePaths> {
  let root = std::env::var_os("RENFE_CLI_CACHE_DIR")
    .map(PathBuf::from)
    .or_else(|| BaseDirs::new().map(|directories| directories.cache_dir().join("renfe-cli")))?;
  let directory = root.join(format!("v{CACHE_FORMAT_VERSION}"));
  Some(CachePaths {
    data: directory.join(format!("{}.bin", feed.cache_name())),
    etag: directory.join(format!("{}.etag", feed.cache_name())),
  })
}

fn read_cache(path: &Path) -> Result<TransitData, String> {
  let bytes = fs::read(path).map_err(|error| error.to_string())?;
  bincode::deserialize(&bytes).map_err(|error| error.to_string())
}

fn write_cache(paths: &CachePaths, data: &TransitData, etag: Option<&str>) -> Result<(), String> {
  let directory = paths
    .data
    .parent()
    .ok_or_else(|| "cache path has no parent directory".to_owned())?;
  fs::create_dir_all(directory).map_err(|error| error.to_string())?;
  let bytes = bincode::serialize(data).map_err(|error| error.to_string())?;
  atomic_write(&paths.data, &bytes)?;

  if let Some(etag) = etag {
    atomic_write(&paths.etag, etag.as_bytes())?;
  } else if paths.etag.exists() {
    fs::remove_file(&paths.etag).map_err(|error| error.to_string())?;
  }
  Ok(())
}

fn atomic_write(path: &Path, contents: &[u8]) -> Result<(), String> {
  let temporary = path.with_extension(format!("tmp-{}", std::process::id()));
  let mut file = fs::File::create(&temporary).map_err(|error| error.to_string())?;
  file
    .write_all(contents)
    .and_then(|()| file.sync_all())
    .map_err(|error| error.to_string())?;
  if let Err(error) = fs::rename(&temporary, path) {
    if path.exists() {
      fs::remove_file(path).map_err(|remove_error| remove_error.to_string())?;
      fs::rename(&temporary, path).map_err(|rename_error| rename_error.to_string())
    } else {
      Err(error.to_string())
    }
  } else {
    Ok(())
  }
}

impl TransitData {
  fn print_stats(&self) {
    println!("GTFS data:");
    println!("  Stops: {}", self.stations.len());
    println!("  Trips: {}", self.trips.len());
  }
}

impl From<Gtfs> for TransitData {
  fn from(gtfs: Gtfs) -> Self {
    let transfers = gtfs
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
    let stations = gtfs
      .stops
      .iter()
      .map(|(id, stop)| {
        (
          id.clone(),
          TransitStation {
            name: stop.name.clone().unwrap_or_else(|| id.clone()),
          },
        )
      })
      .collect();
    let trips = gtfs
      .trips
      .values()
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
        (stop_times.len() >= 2).then(|| ServiceTrip {
          service_id: trip.service_id.clone(),
          trip: Trip {
            id: trip.id.clone(),
            name: gtfs
              .get_route(&trip.route_id)
              .ok()
              .and_then(|route| route.short_name.clone().or_else(|| route.long_name.clone()))
              .or_else(|| trip.trip_short_name.clone())
              .unwrap_or_else(|| trip.route_id.clone()),
            stop_times,
          },
        })
      })
      .collect();

    Self {
      stations,
      trips,
      transfers,
      calendar: gtfs
        .calendar
        .into_iter()
        .map(|(service_id, calendar)| {
          (
            service_id,
            ServiceCalendar {
              monday: calendar.monday,
              tuesday: calendar.tuesday,
              wednesday: calendar.wednesday,
              thursday: calendar.thursday,
              friday: calendar.friday,
              saturday: calendar.saturday,
              sunday: calendar.sunday,
              start_date: calendar.start_date,
              end_date: calendar.end_date,
            },
          )
        })
        .collect(),
      calendar_dates: gtfs
        .calendar_dates
        .into_iter()
        .map(|(service_id, dates)| {
          (
            service_id,
            dates
              .into_iter()
              .map(|calendar_date| ServiceCalendarDate {
                date: calendar_date.date,
                added: calendar_date.exception_type == Exception::Added,
              })
              .collect(),
          )
        })
        .collect(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn processed_cache_round_trips() {
    let data = TransitData {
      stations: HashMap::from([(
        "station".into(),
        TransitStation {
          name: "Station".into(),
        },
      )]),
      trips: vec![ServiceTrip {
        service_id: "service".into(),
        trip: Trip {
          id: "trip".into(),
          name: "Train".into(),
          stop_times: vec![
            StopTime {
              stop_id: "station".into(),
              arrival: 100,
              departure: 100,
            },
            StopTime {
              stop_id: "destination".into(),
              arrival: 200,
              departure: 200,
            },
          ],
        },
      }],
      transfers: Vec::new(),
      calendar: HashMap::from([(
        "service".into(),
        ServiceCalendar {
          monday: true,
          tuesday: true,
          wednesday: true,
          thursday: true,
          friday: true,
          saturday: false,
          sunday: false,
          start_date: Date::new(2026, 1, 1).unwrap(),
          end_date: Date::new(2026, 12, 31).unwrap(),
        },
      )]),
      calendar_dates: HashMap::from([(
        "service".into(),
        vec![ServiceCalendarDate {
          date: Date::new(2026, 6, 1).unwrap(),
          added: false,
        }],
      )]),
    };

    let encoded = bincode::serialize(&data).unwrap();
    let decoded: TransitData = bincode::deserialize(&encoded).unwrap();

    assert_eq!(decoded.stations["station"].name, "Station");
    assert_eq!(decoded.trips[0].trip.stop_times[1].arrival, 200);
    assert_eq!(
      decoded.calendar["service"].end_date,
      Date::new(2026, 12, 31).unwrap()
    );
    assert!(!decoded.calendar_dates["service"][0].added);
  }
}
