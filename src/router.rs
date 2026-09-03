use std::collections::{HashMap, HashSet};

pub const MAX_TRANSFERS: usize = 3;
const MAX_LEGS: usize = MAX_TRANSFERS + 1;
const DEFAULT_TRANSFER_TIME: u32 = 5 * 60;
const TRANSFER_PENALTY: u32 = 15 * 60;
const MAX_JOURNEY_TIME: u32 = 24 * 60 * 60;

#[derive(Clone, Debug)]
pub struct StopTime {
  pub stop_id: String,
  pub arrival: u32,
  pub departure: u32,
}

#[derive(Clone, Debug)]
pub struct Trip {
  pub id: String,
  pub name: String,
  pub stop_times: Vec<StopTime>,
}

#[derive(Clone, Debug)]
pub struct Transfer {
  pub from_stop_id: String,
  pub to_stop_id: String,
  /// `None` represents a GTFS transfer explicitly marked as impossible.
  pub duration: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Leg {
  pub trip_id: String,
  pub name: String,
  pub from_stop_id: String,
  pub to_stop_id: String,
  pub departure: u32,
  pub arrival: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Journey {
  pub legs: Vec<Leg>,
}

impl Journey {
  pub fn departure(&self) -> u32 {
    self.legs.first().map_or(0, |leg| leg.departure)
  }

  pub fn arrival(&self) -> u32 {
    self.legs.last().map_or(0, |leg| leg.arrival)
  }

  pub fn transfers(&self) -> usize {
    self.legs.len().saturating_sub(1)
  }

  fn score(&self) -> u32 {
    self
      .arrival()
      .saturating_sub(self.departure())
      .saturating_add(self.transfers() as u32 * TRANSFER_PENALTY)
  }
}

#[derive(Clone)]
struct Label {
  arrival: u32,
  legs: Vec<Leg>,
}

/// Finds a useful timetable with a round-based public transport search.
///
/// Each round adds one train leg, which makes the transfer limit a property of
/// the algorithm rather than a filter applied after potentially unbounded
/// exploration. For each possible departure from the origin, the result with
/// the best travel-time/transfer score is retained.
pub fn find_journeys(
  trips: &[Trip],
  transfers: &[Transfer],
  origin: &str,
  destination: &str,
) -> Vec<Journey> {
  let departure_times: HashSet<u32> = trips
    .iter()
    .flat_map(|trip| {
      trip
        .stop_times
        .iter()
        .take(trip.stop_times.len().saturating_sub(1))
        .filter(|stop_time| stop_time.stop_id == origin)
        .map(|stop_time| stop_time.departure)
    })
    .collect();

  let mut journeys: Vec<_> = departure_times
    .into_iter()
    .filter_map(|departure| find_journey(trips, transfers, origin, destination, departure))
    .collect();

  journeys.sort_by_key(|journey| (journey.departure(), journey.arrival()));
  journeys.dedup_by(|a, b| a.legs == b.legs);
  remove_dominated(journeys)
}

fn find_journey(
  trips: &[Trip],
  transfers: &[Transfer],
  origin: &str,
  destination: &str,
  departure: u32,
) -> Option<Journey> {
  let mut previous = HashMap::from([(
    origin.to_owned(),
    Label {
      arrival: departure,
      legs: Vec::new(),
    },
  )]);
  let mut candidates = Vec::new();

  for round in 0..MAX_LEGS {
    let boarding_labels = if round == 0 {
      previous.clone()
    } else {
      labels_after_transfers(&previous, transfers)
    };
    let mut current: HashMap<String, Label> = HashMap::new();

    for trip in trips {
      let mut boarded: Option<Label> = None;

      for stop_time in &trip.stop_times {
        if boarded.is_none() {
          if let Some(label) = boarding_labels.get(&stop_time.stop_id) {
            let changes_trip = label.legs.last().is_none_or(|leg| leg.trip_id != trip.id);
            let can_board = if round == 0 {
              label.arrival == stop_time.departure
            } else {
              label.arrival <= stop_time.departure
            };
            if changes_trip && can_board {
              let mut label = label.clone();
              label.legs.push(Leg {
                trip_id: trip.id.clone(),
                name: trip.name.clone(),
                from_stop_id: stop_time.stop_id.clone(),
                to_stop_id: stop_time.stop_id.clone(),
                departure: stop_time.departure,
                arrival: stop_time.departure,
              });
              boarded = Some(label);
            }
          }
          continue;
        }

        let mut label = boarded.clone().expect("checked above");
        label.arrival = stop_time.arrival;
        if label.arrival.saturating_sub(departure) > MAX_JOURNEY_TIME {
          continue;
        }
        if let Some(leg) = label.legs.last_mut() {
          leg.to_stop_id = stop_time.stop_id.clone();
          leg.arrival = stop_time.arrival;
        }
        keep_earliest(&mut current, stop_time.stop_id.clone(), label.clone());

        if stop_time.stop_id == destination {
          candidates.push(Journey { legs: label.legs });
        }
      }
    }

    if current.is_empty() {
      break;
    }
    previous = current;
  }

  candidates
    .into_iter()
    .min_by_key(|journey| (journey.score(), journey.arrival()))
}

fn labels_after_transfers(
  previous: &HashMap<String, Label>,
  transfers: &[Transfer],
) -> HashMap<String, Label> {
  let mut labels = HashMap::new();

  for (stop_id, label) in previous {
    let explicit_same_stop = transfers
      .iter()
      .find(|transfer| transfer.from_stop_id == *stop_id && transfer.to_stop_id == *stop_id);
    let duration = explicit_same_stop
      .map(|transfer| transfer.duration)
      .unwrap_or(Some(DEFAULT_TRANSFER_TIME));
    if let Some(duration) = duration {
      let mut same_stop = label.clone();
      same_stop.arrival = same_stop.arrival.saturating_add(duration);
      keep_earliest(&mut labels, stop_id.clone(), same_stop);
    }
  }

  for transfer in transfers {
    if transfer.from_stop_id != transfer.to_stop_id
      && let (Some(label), Some(duration)) =
        (previous.get(&transfer.from_stop_id), transfer.duration)
    {
      let mut transferred = label.clone();
      transferred.arrival = transferred.arrival.saturating_add(duration);
      keep_earliest(&mut labels, transfer.to_stop_id.clone(), transferred);
    }
  }

  labels
}

fn keep_earliest(labels: &mut HashMap<String, Label>, stop_id: String, candidate: Label) {
  if labels
    .get(&stop_id)
    .is_none_or(|existing| candidate.arrival < existing.arrival)
  {
    labels.insert(stop_id, candidate);
  }
}

fn remove_dominated(journeys: Vec<Journey>) -> Vec<Journey> {
  journeys
    .iter()
    .enumerate()
    .filter(|(index, journey)| {
      !journeys.iter().enumerate().any(|(other_index, other)| {
        index != &other_index
          && other.departure() >= journey.departure()
          && other.arrival() <= journey.arrival()
          && other.transfers() <= journey.transfers()
          && (other.departure() > journey.departure()
            || other.arrival() < journey.arrival()
            || other.transfers() < journey.transfers())
      })
    })
    .map(|(_, journey)| journey.clone())
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn trip(id: &str, stops: &[(&str, u32, u32)]) -> Trip {
    Trip {
      id: id.into(),
      name: id.into(),
      stop_times: stops
        .iter()
        .map(|(stop_id, arrival, departure)| StopTime {
          stop_id: (*stop_id).into(),
          arrival: *arrival,
          departure: *departure,
        })
        .collect(),
    }
  }

  #[test]
  fn finds_a_three_transfer_journey() {
    let trips = vec![
      trip("A", &[("O", 0, 0), ("1", 100, 100)]),
      trip("B", &[("1", 400, 400), ("2", 500, 500)]),
      trip("C", &[("2", 800, 800), ("3", 900, 900)]),
      trip("D", &[("3", 1_200, 1_200), ("X", 1_300, 1_300)]),
    ];

    let journeys = find_journeys(&trips, &[], "O", "X");

    assert_eq!(journeys.len(), 1);
    assert_eq!(journeys[0].transfers(), MAX_TRANSFERS);
  }

  #[test]
  fn does_not_search_beyond_three_transfers() {
    let trips = vec![
      trip("A", &[("O", 0, 0), ("1", 100, 100)]),
      trip("B", &[("1", 400, 400), ("2", 500, 500)]),
      trip("C", &[("2", 800, 800), ("3", 900, 900)]),
      trip("D", &[("3", 1_200, 1_200), ("4", 1_300, 1_300)]),
      trip("E", &[("4", 1_600, 1_600), ("X", 1_700, 1_700)]),
    ];

    assert!(find_journeys(&trips, &[], "O", "X").is_empty());
  }

  #[test]
  fn requires_time_to_make_a_connection() {
    let trips = vec![
      trip("A", &[("O", 0, 0), ("1", 100, 100)]),
      trip("too-soon", &[("1", 200, 200), ("X", 250, 250)]),
      trip("possible", &[("1", 400, 400), ("X", 500, 500)]),
    ];

    let journeys = find_journeys(&trips, &[], "O", "X");

    assert_eq!(journeys[0].legs[1].trip_id, "possible");
  }

  #[test]
  fn each_timetable_entry_starts_at_its_actual_departure() {
    let trips = vec![
      trip("slow", &[("O", 0, 0), ("X", 1_200, 1_200)]),
      trip("fast", &[("O", 1_000, 1_000), ("X", 1_500, 1_500)]),
    ];

    let journeys = find_journeys(&trips, &[], "O", "X");

    assert_eq!(journeys.len(), 2);
    assert_eq!(journeys[0].legs[0].trip_id, "slow");
    assert_eq!(journeys[1].legs[0].trip_id, "fast");
  }

  #[test]
  fn respects_an_explicitly_impossible_same_stop_transfer() {
    let trips = vec![
      trip("A", &[("O", 0, 0), ("1", 100, 100)]),
      trip("B", &[("1", 1_000, 1_000), ("X", 1_100, 1_100)]),
    ];
    let transfers = vec![Transfer {
      from_stop_id: "1".into(),
      to_stop_id: "1".into(),
      duration: None,
    }];

    assert!(find_journeys(&trips, &transfers, "O", "X").is_empty());
  }

  #[test]
  fn penalizes_a_transfer_that_saves_little_time() {
    let trips = vec![
      trip("direct", &[("O", 0, 0), ("X", 1_000, 1_000)]),
      trip("A", &[("O", 0, 0), ("1", 100, 100)]),
      trip("B", &[("1", 400, 400), ("X", 900, 900)]),
    ];

    let journeys = find_journeys(&trips, &[], "O", "X");

    assert_eq!(journeys[0].legs[0].trip_id, "direct");
    assert_eq!(journeys[0].transfers(), 0);
  }
}
