use std::borrow::BorrowMut;
use std::error;
use std::fmt;

use reqwest;
use scraper;
use scraper::ElementRef;

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

fn get_single_element<'a>(
    document: &'a scraper::Html,
    selector: &scraper::Selector,
) -> Result<scraper::ElementRef<'a>, Box<dyn std::error::Error>> {
    let elements: Vec<ElementRef> = document.select(&selector).collect();

    if elements.len() != 1 {
        return Err(Box::new(SimpleError::new(&format!(
            "Found {} selection(s) for {:?}",
            elements.len(),
            selector
        ))));
    }

    Ok(*elements.get(0).unwrap())
}

impl Stock for StrandbergGuitarsCom {
    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.stock_status = StockStatus::Unknown;

        let response = reqwest::blocking::get(&self.url)?.text()?;
        let document = scraper::Html::parse_document(&response);

        let name_selector = scraper::Selector::parse("div.product-info-wrapper>h1")?;
        let stock_selector = scraper::Selector::parse("div.woocommerce-variation-availability>p")?;

        let name_element: ElementRef = get_single_element(&document, &name_selector)?;
        let stock_element: ElementRef = get_single_element(&document, &stock_selector)?;

        self.name = Some(name_element.inner_html());
        self.stock_status = match stock_element.inner_html().as_str() {
            "Out of stock" => StockStatus::OutOfStock,
            "In stock" | "Only 1 in stock" | "Only 2 in stock" => StockStatus::InStock,
            _ => StockStatus::Unknown,
        };

        Ok(())
    }

    fn is_in_stock(&self) -> bool {
        return self.stock_status == StockStatus::InStock;
    }

    fn get_status(&self) -> String {
        match &self.name {
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
    let mut products: Vec<StrandbergGuitarsCom> = vec![];
    products.push(StrandbergGuitarsCom::new(
        "https://strandbergguitars.com/eu/product/boden-standard-nx-6-amber/".to_string(),
    ));
    products.push(StrandbergGuitarsCom::new(
        "https://strandbergguitars.com/eu/product/boden-standard-nx-6-amber-refurb/".to_string(),
    ));

    products.iter_mut().for_each(|product| {
        if let Err(error) = product.update() {
            println!("Error occurred while updating, {:?}", error);
        }
        println!("{}", product.get_status())
    })
}
