use chrono::{Datelike, NaiveDate    , NaiveTime};
use gtfs_structures::Gtfs;

fn main() {
    // Set input parameters
    let gtfs_zip_path = "./google_transit.zip"; // path to the GTFS zip file
    let origin_station_id = "79300"; // Girona - the ID of the origin station
    let destination_station_id = "71801"; // Barcelona Sants - the ID of the destination station
    let date = NaiveDate::from_ymd_opt(2024, 9, 22).unwrap(); // the date for which schedules are needed

    // Load the GTFS zip file
    let gtfs = Gtfs::from_path(gtfs_zip_path) // from_url("https://ssl.renfe.com/gtransit/Fichero_AV_LD/google_transit.zip")
        .expect("Error parsing GTFS zip");

    gtfs.print_stats();
    // println!("{:?}", gtfs.stops);

    // Get timetable
    let mut schedules = get_train_schedules(&gtfs, origin_station_id, destination_station_id, date);

    // Sort schedules by departure_time
    schedules.sort_by_key(|schedule| schedule.departure_time);
    
    // Print the results
    for schedule in schedules {
        println!(
            "Trip {} from {} to {} - departure at {} - arrival at {} - with duration {}",
            schedule.trip_id,
            schedule.origin_stop_name,
            schedule.destination_stop_name,
            schedule.departure_time,
            schedule.arrival_time,
            schedule.duration
        );
    }
}

// Function to get train schedules between an origin and a destination on a given date
fn get_train_schedules(
    gtfs: &Gtfs,
    origin_station_id: &str,
    destination_station_id: &str,
    date: NaiveDate,
) -> Vec<Schedule> {
    let mut schedules = Vec::new();

    // Loop through each trip to find ones active on the given date
    for trip in gtfs.trips.values() {
        // Check if the trip's service is active on the given date
        if is_service_active(&gtfs, &trip.service_id, date) {
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
                        trip_id: gtfs.get_route(&trip.route_id).unwrap().short_name.clone().unwrap(),
                        origin_stop_name: gtfs.stops[&origin.stop.id].name.clone().unwrap(),
                        destination_stop_name: gtfs.stops[&destination.stop.id]
                            .name
                            .clone()
                            .unwrap(),
                        departure_time,
                        arrival_time,
                        duration: format!(
                            "{:02}:{:02}",
                            duration.num_hours(),
                            duration.num_minutes() % 60
                        ),
                    });
                }
            }
        }
    }

    schedules
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

// Struct to hold the schedule details
struct Schedule {
    trip_id: String,
    origin_stop_name: String,
    destination_stop_name: String,
    departure_time: NaiveTime,
    arrival_time: NaiveTime,
    duration: String
}
