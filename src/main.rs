use chrono::Local;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;
use utils::sensor_data_t;

const PERIOD: u64 = 300;
const DATA_SIZE: usize = 100;
const DATA_FILE: &str = "sensor.dat";

// remove the first/oldest line in the sensor data file
fn remove_oldest(file_path: &str) -> io::Result<()> {
    // Read all lines except the first
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().skip(1).collect::<Result<_, _>>()?;

    // Update the file with the content excluding the first line
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    utils::debug(format!("First line deleted from {}", file_path));
    Ok(())
}

// Count the lines in the sensor data file
fn count_lines(file_path: &str) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Count the number of lines
    let line_count = reader.lines().count();
    Ok(line_count)
}

/*
 * Append a sensor reading and time tag to the end of the data file
 * Count the number of lines in the data file
 * If there are >= 100 line then remove the first/oldest line & write back
*/
fn write_sensor_data(sdata: &sensor_data_t) -> Result<u32, io::Error> {
    // Open the file in append mode, creating it if it doesn't exist
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(DATA_FILE)?;

    // Current local date and time
    let now = Local::now();

    // Write text to the end of the file
    writeln!(file, "{:.2} {:.2} {now}", sdata.pressure, sdata.temperature)?;

    utils::debug(format!("Data appended to {}", DATA_FILE));

    match count_lines(DATA_FILE) {
        Ok(line_count) => {
            utils::debug(format!("The file has {} lines.", line_count));
            if line_count >= DATA_SIZE {
                match remove_oldest(DATA_FILE) {
                    Ok(()) => utils::debug(format!(
                        "Removed oldest data from sensor data with {} lines.",
                        line_count
                    )),
                    Err(e) => eprintln!("Failed to remove oldest data: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Failed to count lines: {}", e),
    };

    Ok(0)
}

fn do_sensor_data() -> u32 {
    let res: u32;
    let sdata = utils::get_sensor_data();

    utils::debug(format!(
        "do_sensor_data: Temp {} Pressure {}",
        sdata.temperature, sdata.pressure
    ));

    match write_sensor_data(&sdata) {
        Ok(rslt) => {
            utils::debug(format!("Update Sensor data"));
            res = rslt;
        }
        Err(e) => {
            eprintln!("Failed to update sensor data: {}", e);
            res = 1;
        }
    };

    res
}

#[tokio::main]
async fn main() {
    // Install a ctl-c handler
    utils::ctl_c_handler();

    // Start a thread that gets sensor data and notifies on a period
    tokio::spawn(async move {
        utils::update_and_notify().await;
    });

    // Delay before taking another measurement
    thread::sleep(Duration::from_secs(30));

    loop {
        do_sensor_data();
        thread::sleep(Duration::from_secs(PERIOD));
    }
}
