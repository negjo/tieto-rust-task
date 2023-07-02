/// Tietoevry's Entry Rust Assignment
///
/// Highest and lowest inflation rates in the history of the Czech and Slovak Republics & future values.
///
/// A rich factory owner looks back 30 years and asks whether it was wise to leave half of his monetary wealth in the Czech Republic and half in Slovak Republic after the division of Czechoslovakia.
/// 
/// You have received code that is missing some parts. Please fill them in so that the output of your code is the same as the output of the binary provided.
///
/// © Jakub Maly <jakub.maly@tietoevry.com>, 2023
///

// This helps to print large numbers nicely.
// Note: You will need to cargo add this as well.
extern crate thousands;
use thousands::Separable;

// Data to use (1993 - 2022):
// Note: Do not change this part if you want to achieve the same results.
const STARTING_YEAR: usize = 1993;
const NUMBER_OF_YEARS: usize = 30;
const CZECH_REP_INF_RATES: [f64; NUMBER_OF_YEARS] = [
    0.28, 0.1, 0.091, 0.088, 0.085, 0.107, 0.021, 0.039, 0.047, 0.018, 0.001, 0.028, 0.019, 0.025,
    0.028, 0.063, 0.01, 0.015, 0.019, 0.033, 0.014, 0.004, 0.003, 0.007, 0.025, 0.021, 0.028,
    0.032, 0.038, 0.151,
];
const SLOVAK_REP_INF_RATES: [f64; NUMBER_OF_YEARS] = [
    0.232, 0.134, 0.099, 0.058, 0.061, 0.067, 0.106, 0.12, 0.073, 0.033, 0.085, 0.075, 0.027,
    0.045, 0.028, 0.046, 0.016, 0.01, 0.039, 0.036, 0.014, -0.001, -0.003, -0.005, 0.013, 0.025,
    0.027, 0.019, 0.032, 0.128,
];


mod inflation_tracker{
    // We want our hash map linked - we might ask you later why :)
    // Note: You will need to cargo add this.
    extern crate linked_hash_map;
    use linked_hash_map::LinkedHashMap;

    /// Structure of linked hash map and several helper variables that will be used together as an inflation tracker.
    pub struct InflationTracker {
        /// Name of the country
        country_name: String,
        /// Linked hash map for storing yearly value of inflation rate values
        yearly_val: LinkedHashMap<usize, f64>,
    }

    impl InflationTracker {
        /// Returns a new tracker with the given country name.
        ///
        /// # Arguments
        ///
        /// * `name` - Name of the country
        ///
        /// # Returns
        ///
        /// * The new tracker
        ///
        pub fn new(name: &str) -> Self {
            InflationTracker{
                country_name: name.to_string(),
                yearly_val: LinkedHashMap::new(),
            }
        }

        /// Inserts new value to the hash map and if needed updates the highest and the lowest variables as well.
        ///
        /// # Arguments
        ///
        /// * `year` - Year of measurement to be used as a key
        /// * `value` - Value of the measured inflation rate
        ///
        pub fn insert(&mut self, year: usize, value: f64) {
            self.yearly_val.insert(year, value);
        }

        /// Clears the hash map and sets all variables to default values.
        pub fn clear(&mut self) {
            self.yearly_val.clear();
        }

        /// Changes the name of the country.
        ///
        /// # Arguments
        ///
        /// * `new_name` - New name of the country
        ///
        pub fn change_country_name(&mut self, new_name: &str) {
            self.country_name = new_name.to_string();
        }

        /// Returns the year and value of the maximum yearly inflation.
        ///
        /// # Returns
        ///
        /// * The year of the maximum yearly inflation
        /// * The value of the maximum yearly inflation
        ///
        pub fn get_max(&self) -> (usize, f64) {
            let max = (self.yearly_val.iter().max_by(|x, y| (x.1).partial_cmp(&y.1).unwrap())).unwrap();
            return (*max.0, *max.1);
        }

        /// Returns the year and value of the minimum yearly inflation.
        ///
        /// # Returns
        ///
        /// * The year of the minimum yearly inflation
        /// * The value of the minimum yearly inflation
        ///
        pub fn get_min(&self) -> (usize, f64) {
            let min = self.yearly_val.iter().min_by(|x, y| (x.1).partial_cmp(&y.1).unwrap()).unwrap();
            return (*min.0, *min.1);
        }

        /// Returns the future value of money for a given year range
        ///
        /// # Arguments
        ///
        /// * `money` - Original value
        /// * `start_year` - Year when money was worth 100%
        /// * `end_year` - Year for which we want to know the value
        ///
        ///
        /// # Returns
        ///
        /// * The future value of money for a given year
        ///
        /// Or displays a warning if:
        ///     No data is available
        ///     Start year is not in the data
        ///     End year -1 is not in the data
        ///     Start year is equal or greater than End year
        pub fn get_future_val(&self, money: f64, start_year: usize, end_year: usize) -> f64 {
            let mut final_value = money;
            if self.yearly_val.len() == 0{
                println!("Warning: No data is available!\n");
            }
            if start_year < *self.yearly_val.front().unwrap().0{
                println!("Warning: Start year is outside available data range!\n");
            }
            if end_year-1 >*self.yearly_val.back().unwrap().0{
                println!("Warning: End year is outside available data range!\n");
            }
            if start_year >= end_year {
                println!("Warning: Start year is equal or greater than end year!\n");
            }
            for i in &self.yearly_val {
                if (i.0 >= &start_year) && (i.0 < &end_year) {
                    final_value = final_value * (1.0-i.1);
                }
            }
            return final_value;
        }

        /// Prints stored data as:
        ///
        /// Data for country_name are:
        /// year_0 | rate_0
        /// year_N | rate_N
        ///
        /// Or displays a warning if no data is available to print.
        pub fn print_data(&self) {
            if self.yearly_val.len() == 0 {
                println!("Warning: Found no data to print!\n");
                return;
            }

            println!("Data for {} are:", self.country_name);
            for (year, val) in &self.yearly_val{
                println!("{} | {:.2}%", year, val*100.0);
            }
        }
    }
}


/// Rounds a number to two decimal points.
///
/// # Arguments
///
/// * `num` - Number to round
///
/// # Returns
///
/// * The rounded number
///
fn round2(num: f64) -> f64 {
    (num * 100.0).round() / 100.0
}

fn main() {
    // Initialize the tracker.
    let mut tracker = inflation_tracker::InflationTracker::new("Czech Republic");

    // Insert the data for the first country.
    let mut cur_year = 0;
    while cur_year < NUMBER_OF_YEARS{
        tracker.insert(STARTING_YEAR+cur_year, CZECH_REP_INF_RATES[cur_year]);
        cur_year += 1;
    }

    // Print inserted data.
    tracker.print_data();

    // Print the maximum value of inflation rate and the year for the first country in percentage.
    let max = tracker.get_max();
    println!("Maximum was {}: {:.2}%", max.0, max.1 * 100.0);

    // Print the minimum value of inflation rate and the year for the first country in percentage.
    let min = tracker.get_min();
    println!("Minimum was {}: {:.2}%", min.0, min.1 * 100.0);

    // Print the result of saving 5,000,000 in cash for 30 years.
    // Do not forget to save this for later evaluation.
    let end_money_czech = round2(tracker.get_future_val(5_000_000.0, STARTING_YEAR, STARTING_YEAR+30));
    println!("Saving 5,000,000 for 30 years in 1993 with no interest rate would mean having {} in 2023\n", end_money_czech.separate_with_commas());

    // Clear the tracker.
    // Do not create a new one! reuse reuse reuse :)
    tracker.clear();

    // Check that it was cleared - print_data.
    tracker.print_data();

    // Insert the data for the second country.
    // Do not forget to change the name!
    tracker.change_country_name("Slovak Republic");
    let mut cur_year = 0;
    while cur_year < NUMBER_OF_YEARS{
        tracker.insert(STARTING_YEAR+cur_year, SLOVAK_REP_INF_RATES[cur_year]);
        cur_year += 1;
    }
    
    // Print inserted data.
    tracker.print_data();

    // Print the maximum value of inflation rate and the year for the first country in percentage.
    let max = tracker.get_max();
    println!("Maximum was {}: {:.2}%", max.0, max.1 * 100.0);

    // Print the minimum value of inflation rate and the year for the first country in percentage.
    let min = tracker.get_min();
    println!("Minimum was {}: {:.2}%", min.0, min.1 * 100.0);

    // Print the result of saving 5,000,000 in cash for 30 years.
    // Do not forget to save this for later evaluation.
    let end_money_slovak = round2(tracker.get_future_val(5_000_000.0, STARTING_YEAR, STARTING_YEAR+30));
    println!("Saving 5,000,000 for 30 years in 1993 with no interest rate would mean having {} in 2023\n", end_money_slovak.separate_with_commas());

    // Print which country has had higher inflation in the last 30 years.
    if end_money_czech > end_money_slovak {
        println!("Slovak Republic had a higher inflation rate than Czech Republic!");
    }
    else if end_money_czech < end_money_slovak{
        println!("Czech Republic had a higher inflation rate than Slovak Republic!"); 
    }
    else {
        println!("Czech Republic and Slovak Republic had the same inflation rate!");
    }
}

// BONUS POINTS:

// Get the maximum and minimum without using any helper variable.
// Hint: Take a look at `max_by()` and `min_by()` hash map functions.

// Move implementation to a separate module.
// Hint: There is reason for `pub` keywords.
