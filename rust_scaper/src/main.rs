use std::error;
use std::fmt;

use reqwest;
use scraper;

// inner_html() returns the inner html, what is between the <> signs
// value() returns the html element itself

/* Formatting, {:?} : separates the name of the thing being formatted from the next thing,
which is the formatting options. The formatting option ? triggers the use of std::fmt::Debug
implementation, which is a trait that can be defined on structs on enums to allow for debug
printing. The default formatting is using the Display trait.

{:?} formats the "next" value passed to a formatting macro and supports anything that
implement std::fmt::Debug.

It's very common to use #[derive(Debug)] on structs and enums to get a default Debug
implementation. */

#[derive(Debug)]
pub struct SimpleError {
    message: String,
}

impl SimpleError {
    fn new(message: &str) -> SimpleError {
        SimpleError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for SimpleError {
    fn description(&self) -> &str {
        &self.message
    }
}

trait Stock {
    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn is_in_stock(&self) -> bool;
    fn get_status(&self) -> String;
}

#[derive(PartialEq, Debug)]
enum StockStatus {
    Unknown,
    InStock,
    OutOfStock,
}

impl ToString for StockStatus {
    fn to_string(&self) -> String {
        match self {
            StockStatus::Unknown => "unknown".to_owned(),
            StockStatus::InStock => "in stock".to_owned(),
            StockStatus::OutOfStock => "out of stock".to_owned(),
        }
    }
}

#[derive(Debug)]
struct StrandbergGuitarsCom {
    stock_status: StockStatus,
    name: Option<String>,
    url: String,
}

impl StrandbergGuitarsCom {
    fn new(url: String) -> StrandbergGuitarsCom {
        StrandbergGuitarsCom {
            stock_status: StockStatus::Unknown,
            name: None,
            url,
        }
    }
}

impl Stock for StrandbergGuitarsCom {
    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.stock_status = StockStatus::Unknown;

        let response = reqwest::blocking::get(&self.url)?.text()?;
        let document = scraper::Html::parse_document(&response);
        let availability_selector =
            scraper::Selector::parse("div.woocommerce-variation-availability>p")?;

        let selections: Vec<String> = document
            .select(&availability_selector)
            .map(|e| e.inner_html())
            .collect();

        if selections.len() > 1 {
            return Err(Box::new(SimpleError::new(&format!(
                "Found more than 1 selection for {:?}",
                self
            ))));
        }

        self.stock_status = match selections.get(0).unwrap().as_str() {
            "Out of stock" => StockStatus::OutOfStock,
            _ => StockStatus::InStock,
        };

        Ok(())
    }

    fn is_in_stock(&self) -> bool {
        return self.stock_status == StockStatus::InStock;
    }

    fn get_status(&self) -> String {
        match self.name {
            Some(name) => format!("Status for {} is {}", name, self.stock_status.to_string()),
            None => format!(
                "Status for product with url {} is {}",
                self.url,
                self.stock_status.to_string()
            ),
        }
    }
}

fn main() {
    // let amber_guitar = StrandbergGuitarsCom::new(
    //     "Amber guitar".to_owned(),
    //     "https://strandbergguitars.com/eu/product/boden-standard-nx-6-charcoal/".to_owned(),
    // );

    // if amber_guitar.is_in_stock().unwrap() {
    //     println!("{}", amber_guitar.get_in_stock_message());
    // } else {
    //     println!("The guitar is not in stock")
    // }
}
