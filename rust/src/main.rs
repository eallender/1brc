use std::collections::HashMap;
use std::env::args;
use std::fmt::Write;
use std::fs::{read_to_string, write};

fn round(x: f64) -> f64 {
    (x * 10.0).ceil() / 10.0
}

fn read_stations(filepath: String, stations: &mut HashMap<String, (f64, f64, f64, f64)>) {
    for line in read_to_string(filepath).unwrap().lines() {
        let items: Vec<&str> = line.split_terminator(";").collect();
        let name = items[0].to_string();
        let value: f64 = items[1].parse::<f64>().unwrap();

        if stations.contains_key(&name) {
            let mut current = stations.get(&name).unwrap().clone();
            if value < current.0 {
                current.0 = value
            } else if value > current.1 {
                current.1 = value
            }

            current.2 += value;
            current.3 += 1.0;

            stations.insert(name, current);
        } else {
            stations.insert(name, (value, value, value, 1.0));
        }
    }
}

fn write_values(filepath: String, stations: &HashMap<String, (f64, f64, f64, f64)>) {
    let mut keys: Vec<&String> = stations.keys().collect();
    keys.sort();
    let mut line: String = "{".to_string();
    let length = keys.len();
    for (i, station) in keys.into_iter().enumerate() {
        let values = stations[station];
        let average = round(values.2 / values.3);
        write!(
            line,
            "{}={:.1}/{:.1}/{:.1}",
            station, values.0, average, values.1
        )
        .unwrap();

        if i != length - 1 {
            let _ = write!(line, ", ");
        }
    }
    let _ = write!(line, "}}\n");

    let _ = write(filepath, line);
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        println!("Usage: {:?} data.txt", args[0]);
        return;
    }

    let filepath = &args[1];
    let mut stations: HashMap<String, (f64, f64, f64, f64)> = HashMap::new(); // string to (min, max, sum, count)

    read_stations(filepath.clone(), &mut stations);
    write_values("./out.txt".to_string(), &stations);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read;

    #[test]
    fn test_measurements_1() {
        let filepath = "../test/samples/measurements-1.txt".to_string();
        let out_path = "../test/out/measurements-1.out".to_string();
        let mut stations: HashMap<String, (f64, f64, f64, f64)> = HashMap::new(); // string to (min, max, sum, count)

        read_stations(filepath, &mut stations);
        write_values(out_path.clone(), &stations);

        let expected = read("../test/samples/measurements-1.out".to_string()).unwrap();
        let result = read(out_path).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_measurements_2() {
        let filepath = "../test/samples/measurements-2.txt".to_string();
        let out_path = "../test/out/measurements-2.out".to_string();
        let mut stations: HashMap<String, (f64, f64, f64, f64)> = HashMap::new(); // string to (min, max, sum, count)

        read_stations(filepath, &mut stations);
        write_values(out_path.clone(), &stations);

        let expected = read("../test/samples/measurements-2.out".to_string()).unwrap();
        let result = read(out_path).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_measurements_3() {
        let filepath = "../test/samples/measurements-3.txt".to_string();
        let out_path = "../test/out/measurements-3.out".to_string();
        let mut stations: HashMap<String, (f64, f64, f64, f64)> = HashMap::new(); // string to (min, max, sum, count)

        read_stations(filepath, &mut stations);
        write_values(out_path.clone(), &stations);

        let expected = read("../test/samples/measurements-3.out".to_string()).unwrap();
        let result = read(out_path).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_measurements_10() {
        let filepath = "../test/samples/measurements-10.txt".to_string();
        let out_path = "../test/out/measurements-10.out".to_string();
        let mut stations: HashMap<String, (f64, f64, f64, f64)> = HashMap::new(); // string to (min, max, sum, count)

        read_stations(filepath, &mut stations);
        write_values(out_path.clone(), &stations);

        let expected = read("../test/samples/measurements-10.out".to_string()).unwrap();
        let result = read(out_path).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_measurements_20() {
        let filepath = "../test/samples/measurements-20.txt".to_string();
        let out_path = "../test/out/measurements-20.out".to_string();
        let mut stations: HashMap<String, (f64, f64, f64, f64)> = HashMap::new(); // string to (min, max, sum, count)

        read_stations(filepath, &mut stations);
        write_values(out_path.clone(), &stations);

        let expected = read("../test/samples/measurements-20.out".to_string()).unwrap();
        let result = read(out_path).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_measurements_10000_unique_keys() {
        let filepath = "../test/samples/measurements-10000-unique-keys.txt".to_string();
        let out_path = "../test/out/measurements-10000-unique-keys.out".to_string();
        let mut stations: HashMap<String, (f64, f64, f64, f64)> = HashMap::new(); // string to (min, max, sum, count)

        read_stations(filepath, &mut stations);
        write_values(out_path.clone(), &stations);

        let expected =
            read("../test/samples/measurements-10000-unique-keys.out".to_string()).unwrap();
        let result = read(out_path).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_measurements_boundaries() {
        let filepath = "../test/samples/measurements-boundaries.txt".to_string();
        let out_path = "../test/out/measurements-boundaries.out".to_string();
        let mut stations: HashMap<String, (f64, f64, f64, f64)> = HashMap::new(); // string to (min, max, sum, count)

        read_stations(filepath, &mut stations);
        write_values(out_path.clone(), &stations);

        let expected = read("../test/samples/measurements-boundaries.out".to_string()).unwrap();
        let result = read(out_path).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_measurements_complex_utf8() {
        let filepath = "../test/samples/measurements-complex-utf8.txt".to_string();
        let out_path = "../test/out/measurements-complex-utf8.out".to_string();
        let mut stations: HashMap<String, (f64, f64, f64, f64)> = HashMap::new(); // string to (min, max, sum, count)

        read_stations(filepath, &mut stations);
        write_values(out_path.clone(), &stations);

        let expected = read("../test/samples/measurements-complex-utf8.out".to_string()).unwrap();
        let result = read(out_path).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_measurements_dot() {
        let filepath = "../test/samples/measurements-dot.txt".to_string();
        let out_path = "../test/out/measurements-dot.out".to_string();
        let mut stations: HashMap<String, (f64, f64, f64, f64)> = HashMap::new(); // string to (min, max, sum, count)

        read_stations(filepath, &mut stations);
        write_values(out_path.clone(), &stations);

        let expected = read("../test/samples/measurements-dot.out".to_string()).unwrap();
        let result = read(out_path).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_measurements_rounding() {
        let filepath = "../test/samples/measurements-rounding.txt".to_string();
        let out_path = "../test/out/measurements-rounding.out".to_string();
        let mut stations: HashMap<String, (f64, f64, f64, f64)> = HashMap::new(); // string to (min, max, sum, count)

        read_stations(filepath, &mut stations);
        write_values(out_path.clone(), &stations);

        let expected = read("../test/samples/measurements-rounding.out".to_string()).unwrap();
        let result = read(out_path).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_measurements_short() {
        let filepath = "../test/samples/measurements-short.txt".to_string();
        let out_path = "../test/out/measurements-short.out".to_string();
        let mut stations: HashMap<String, (f64, f64, f64, f64)> = HashMap::new(); // string to (min, max, sum, count)

        read_stations(filepath, &mut stations);
        write_values(out_path.clone(), &stations);

        let expected = read("../test/samples/measurements-short.out".to_string()).unwrap();
        let result = read(out_path).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_measurements_shortest() {
        let filepath = "../test/samples/measurements-shortest.txt".to_string();
        let out_path = "../test/out/measurements-shortest.out".to_string();
        let mut stations: HashMap<String, (f64, f64, f64, f64)> = HashMap::new(); // string to (min, max, sum, count)

        read_stations(filepath, &mut stations);
        write_values(out_path.clone(), &stations);

        let expected = read("../test/samples/measurements-shortest.out".to_string()).unwrap();
        let result = read(out_path).unwrap();
        assert_eq!(expected, result);
    }
}
