use regex::Regex;

/// A single measurement value.
#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    name: String,
    value: f64,
}

impl Measurement {
    pub fn new(name: String, value: f64) -> Self {
        Self { name, value }
    }
}

/// A group of measurements.
#[derive(Debug, PartialEq)]
pub struct Measurements {
    measurements: Vec<Measurement>,
}

impl Measurements {
    pub fn new(measurements: Vec<Measurement>) -> Self {
        Self { measurements }
    }

    fn from(buf: &str) -> Self {
        let re = Regex::new(r"([A-Za-z\.]+):([0-9]+)\|g").unwrap(); // TODO remove unwrap

        let measurements = buf
            .split("\n")
            .map(|line| {
                let captures = re.captures(line);

                captures.map(|captures| {
                    Measurement::new(
                        captures.get(1).unwrap().as_str().to_string(),
                        captures.get(2).unwrap().as_str().parse().unwrap(), // TODO remove unwrap
                    )
                })
            })
            .filter_map(|measurement| measurement)
            .collect();

        Measurements::new(measurements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_gauage() {
        const NAME: &str = "metric.test";
        const STR: &str = "metric.test:1|g\nmetric.test:2|g\nmetric.test:3|g\nmetric.test:4|g\nmetric.test:5|g\nmetric.test:6|g\nmetric.test:7|g\nmetric.test:8|g\nmetric.test:9|g\nmetric.test:10|g\nmetric.test:11|g\nmetric.test:12|g\nmetric.test:13|g\nmetric.test:14|g\nmetric.test:15|g\n";

        let mut measurments: Vec<Measurement> = Vec::new();
        for i in 1..16 {
            measurments.push(Measurement::new(NAME.to_string(), i as f64));
        }

        assert_eq!(Measurements::from(STR), Measurements::new(measurments));
    }
}
