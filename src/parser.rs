use regex::Regex;

/// A single measurement value.
#[derive(Debug, Clone, PartialEq)]
pub struct Measurement {
    pub(crate) name: String,
    pub(crate) value: f64,
}

impl Measurement {
    pub fn new(name: String, value: f64) -> Self {
        Self { name, value }
    }
}

/// Create a group of measurements from a string. The string must contain valid statsd line format.
/// No aggregation of the measurements is performed. See the [`dedup`] method for that.
///
/// # Example
///
/// ```text
/// example.metric:5|g\nexample.metric:6|g\n
/// ```
pub fn from_statsd_line(buf: &str) -> Vec<Measurement> {
    let re = Regex::new(r"([A-Za-z\.]+):([0-9]+)\|g").unwrap(); // TODO remove unwrap

    buf.split('\n')
        .filter_map(|line| {
            let captures = re.captures(line);

            captures.map(|captures| {
                Measurement::new(
                    captures.get(1).unwrap().as_str().to_string(),
                    captures.get(2).unwrap().as_str().parse().unwrap(), // TODO remove unwrap
                )
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_gauage() {
        const NAME: &str = "metric.test";
        const STR: &str = "metric.test:1|g\nmetric.test:2|g\nmetric.test:3|g\nmetric.test:4|g\nmetric.test:5|g\nmetric.test:6|g\nmetric.test:7|g\nmetric.test:8|g\nmetric.test:9|g\nmetric.test:10|g\nmetric.test:11|g\nmetric.test:12|g\nmetric.test:13|g\nmetric.test:14|g\nmetric.test:15|g\n";

        let mut expected_measurments: Vec<Measurement> = Vec::new();
        for i in 1..16 {
            expected_measurments.push(Measurement::new(NAME.to_string(), i as f64));
        }

        let measurements = super::from_statsd_line(STR);

        assert_eq!(measurements, expected_measurments);
    }
}
