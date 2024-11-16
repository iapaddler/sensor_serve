use chrono::Local;
use libc::{c_double, c_int};
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;

const PERIOD: u64 = 3;
const DATA_SIZE: usize = 100;
const DATA_FILE: &str = "sensor.dat";

#[repr(C)]
pub struct sensor_data_t {
    pub temperature: c_double,
    pub pressure: c_double,
}

//#[link(name = "rsd", kind = "static")]
extern "C" {
    fn getSensorData(sdata: &sensor_data_t) -> c_int;
}

fn debug(msg: String) {
    let args: Vec<String> = env::args().collect();
    //println!("num args: {}", args.len());

    if args.len() >= 3 {
        let arg = match args.get(1) {
            Some(cmd) => cmd,
            None => {
                eprintln!("Unknown arguments provided");
                return;
            }
        };

        let dcmd = String::from("-d");
        if arg == dcmd.as_str() {
            let dval = match args.get(2) {
                Some(val) => val,
                None => {
                    eprintln!("Unknown arguments provided!");
                    return;
                }
            };

            let gdbg = match dval.parse::<u64>() {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("Unable to parse number from argument: {}", e);
                    return;
                }
            };

            if gdbg > 0 {
                println!("{}", msg);
            }
        }
    }
}

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

    debug(format!("First line deleted from {}", file_path));
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
    writeln!(
        file,
        "{:.2} {:.2} {}",
        sdata.pressure, sdata.temperature, now
    )?;

    debug(format!("Data appended to {}", DATA_FILE));

    match count_lines(DATA_FILE) {
        Ok(line_count) => {
            debug(format!("The file has {} lines.", line_count));
            if line_count >= DATA_SIZE {
                match remove_oldest(DATA_FILE) {
                    Ok(()) => debug(format!(
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

fn get_sensor_data() -> u32 {
    debug(format!("Get sensor data"));

    let res: u32;
    let sdata = sensor_data_t {
        temperature: 0.0,
        pressure: 0.0,
    };

    unsafe {
        getSensorData(&sdata);
    }

    debug(format!(
        "Sensor Data: Temp {} Pressure {}",
        sdata.temperature, sdata.pressure
    ));

    match write_sensor_data(&sdata) {
        Ok(rslt) => {
            debug(format!("Update Sensor data"));
            res = rslt;
        }
        Err(e) => {
            eprintln!("Failed to update sensor data: {}", e);
            res = 1;
        }
    };

    res
}

fn main() {
    loop {
        get_sensor_data();
        thread::sleep(Duration::from_secs(PERIOD));
    }
}
