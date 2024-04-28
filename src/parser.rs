use regex::Regex;

#[derive(Debug)]
pub struct Measurement {
    name: String,
    value: i128,
}

impl Measurement {
    pub fn new(name: String, value: i128) -> Self {
        Self { name, value }
    }
}

#[derive(Debug)]
struct Measurements {
    measurements: Vec<Measurement>,
}

impl Measurements {
    fn from(buf: &str) -> Self {
        let re = Regex::new(r"([A-Za-z\.]+):([0-9]+)\|g").unwrap(); // TODO remove unwrap

        let a = buf.split("\n").map(|line| {
            let captures = re.captures(line);

            captures.map(|captures| {
                Measurement::new(
                    captures.get(1).unwrap().as_str().to_string(),
                    i128::from_str_radix(captures.get(2).unwrap().as_str(), 10).unwrap(),
                )
            })
        });

        Measurements {
            measurements: a
                .filter(|val| val.is_some())
                .map(|val| val.unwrap())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: &str = "metric.test:1|g\nmetric.test:2|g\nmetric.test:3|g\nmetric.test:4|g\nmetric.test:5|g\nmetric.test:6|g\nmetric.test:7|g\nmetric.test:8|g\nmetric.test:9|g\nmetric.test:10|g\nmetric.test:11|g\nmetric.test:12|g\nmetric.test:13|g\nmetric.test:14|g\nmetric.test:15|g\n";
    const B: &str = "metric.test:16|g\nmetric.test:17|g\nmetric.test:18|g\nmetric.test:19|g\nmetric.test:20|g\nmetric.test:21|g\nmetric.test:22|g\nmetric.test:23|g\nmetric.test:24|g\nmetric.test:25|g\nmetric.test:26|g\nmetric.test:27|g\nmetric.test:28|g\nmetric.test:29|g\nmetric.test:30|g\n";
    const C: &str = "metric.test:91|g\nmetric.test:92|g\nmetric.test:93|g\nmetric.test:94|g\nmetric.test:95|g\nmetric.test:96|g\nmetric.test:97|g\nmetric.test:98|g\nmetric.test:99|g\nmetric.test:100|g\nmetric.test:101|g\nmetric.test:102|g\nmetric.test:103|g\nmetric.test:104|g\n";

    #[test]
    fn parse() {
        println!("{:?}", Measurements::from(A));
        println!("{:?}", Measurements::from(B));
        println!("{:?}", Measurements::from(C));
    }
}
