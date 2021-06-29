use clap::{App, Arg};

const HELP_TEXT: &str = "Number of cores to enable. 
Must be within 1 <= ... <= number of cores present.";

#[cfg(target_os="linux")]
fn main() {
    let matches = App::new("torpid")
        .version("0.1.0")
        .author("Nipun Kumar <nipunkumar@outlook.com>")
        .about("Turn off some CPU cores")
        .arg(
            Arg::with_name("NUM_CPU_CORES")
                .help(HELP_TEXT)
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let num = matches
        .value_of("NUM_CPU_CORES")
        .expect("Missing required argument NUM_CPU_CORES")
        .parse::<usize>()
        .expect("Argument NUM_CPU_CORES should be a positive integer");
    if num < 1 {
        panic!("Argument NUM_CPU_CORES should be a positive integer");
    }

    let contents =
        std::fs::read_to_string("/sys/devices/system/cpu/present").expect("Could not read present cpus");

    let parts: Vec<&str> = contents.split("-").collect();
    if parts.len() != 2 {
        eprintln!(
            "Unexpected contents {} in /sys/devices/system/cpu/present",
            &contents
        );
        return;
    }
    let _min = parts[0].trim().parse::<usize>().unwrap();
    let max = parts[1].trim().parse::<usize>().unwrap();

    if num > max + 1 {
        panic!("Argument NUM_CPU_CORES should be <= {}", max + 1);
    }

    // switch on cores 0-(num - 1)
    // switch off cores num-max
    let mut enable_count = 0;
    for idx in 0..=max {
        let path = &format!("/sys/devices/system/cpu/cpu{}/online", idx);
        // the boot core will not have a file to enable/disable
        // should be core 0 usually
        if std::path::Path::new(path).exists() {
            // if we have enough cores enabled, disable current one
            // else enable it
            let flag = if enable_count < num {
                enable_count = enable_count + 1;
                "1"
            } else {
                "0"
            };
            std::fs::write(path, flag).expect("Could not write to CPU online status file");
        } else {
            enable_count = enable_count + 1;
        }
    }
}
