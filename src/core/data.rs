use std::collections::HashMap;
use std::fmt;

pub struct Series<T> {
    name: String,
    data: Vec<T>
}

impl<T> Series<T> where T: fmt::Display + Clone {
    pub fn new(name: String, data: Vec<T>) -> Self {
        Series {name, data}
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn set(&mut self, index: usize, value: T) {
        if index < self.data.len() {
            self.data[index] = value;
        }
    }

    pub fn show(&self) {
        println!("Series: {}", self.name);
        for (index, value) in self.data.iter().enumerate() {
            println!("{}: {}", index, value)
        }
    }
}

pub struct DataFrame {
    columns: HashMap<String, Series<f64>>,
}

impl DataFrame {
    pub fn new() -> Self {
        DataFrame {
            columns: HashMap::new(),
        }
    }

    pub fn add_column(&mut self, name: String, column: Series<f64>) {
        self.columns.insert(name, column);
    }

    pub fn remove_column(&mut self, name: &str) -> Option<Series<f64>> {
        self.columns.remove(name)
    }

    pub fn get_column(&self, name: &str) -> Option<&Series<f64>> {
        self.columns.get(name)
    }

    pub fn show(&self) {
        for (name, series) in &self.columns {
            println!("Column: {}", name);
            series.show();
            println!("");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_series_new() {
        let series = Series::new("Test Series".to_string(), vec![1.0, 2.0, 3.0]);
        assert_eq!(series.data.len(), 3);
    }

    #[test]
    fn test_series_get() {
        let series = Series::new("Test Series".to_string(), vec![1.0, 2.0, 3.0]);
        assert_eq!(*series.get(1).unwrap(), 2.0);
    }

    #[test]
    fn test_dataframe_add_and_get_column() {
        let mut df = DataFrame::new();
        let series = Series::new("Test Column".to_string(), vec![1.0, 2.0, 3.0]);
        df.add_column("Test Column".to_string(), series);
        assert!(df.get_column("Test Column").is_some());
    }
}
