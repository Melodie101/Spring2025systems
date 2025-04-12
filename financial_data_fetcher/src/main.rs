
use serde::Deserialize;
use ureq;
use serde_json;
use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;
use std::{thread, time::Duration};

pub trait Pricing {
    fn fetch_price(& mut self) -> Result<(), String>;
    fn save_to_file(&self) -> Result<(), String>;


}
#[derive(Deserialize, Debug)]
pub struct Bitcoin {
    pub price_usd: f64,
    pub timestamp: String,
}

impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        let resp_result = ureq::get(url).call();

        match resp_result {
            Ok(resp) => {
                let json: serde_json::Value = resp.into_json().map_err(|e| e.to_string())?;
                self.price_usd = json["bitcoin"]["usd"]
                .as_f64()
                .ok_or("Failed to parse Bitcoin price")?;
                self.timestamp = Utc::now().to_rfc3339();
                Ok(())
            },
            Err(ureq::Error::Status(429, _)) => {
                Err("Bitcoin: 429 Too Many Requests. Skipping this cycle.".to_string())
            },
            Err(e) => Err(format!("Bitcoin: Request failed: {}", e)),
        }
       

      
    }

    fn save_to_file(&self) -> Result<(), String> {
        let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("bitcoin.txt")
        .map_err(|e| e.to_string())?;

        writeln!(file, "{}: ${}", self.timestamp, self.price_usd)
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct Ethereum {
    pub price_usd: f64,
    pub timestamp: String,
}

impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        let resp_result = ureq::get(url).call();

        match resp_result {
            Ok(resp) => {
                let json: serde_json::Value = resp.into_json().map_err(|e| e.to_string())?;
                self.price_usd = json["ethereum"]["usd"]
                    .as_f64()
                    .ok_or("Failed to parse ETH price")?;
                self.timestamp = Utc::now().to_rfc3339();
                Ok(())
            },
            Err(ureq::Error::Status(429, _)) => {
                Err("Ethereum: 429 Too Many Requests. Skipping this cycle.".to_string())
            },
            Err(e) => Err(format!("Ethereum: Request failed: {}", e)),
        }
    }   

    fn save_to_file(&self) -> Result<(), String> {
        let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("ethereum.txt")
        .map_err(|e| e.to_string())?;

        writeln!(file, "{}: ${}", self.timestamp, self.price_usd)
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct SP500 {
    pub price_usd: f64,
    pub timestamp: String,
}

impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m";
        let resp = ureq::get(url)
        .call()
        .map_err(|e| e.to_string())?;

        let json: serde_json::Value = resp.into_json().map_err(|e| e.to_string())?;

        self.price_usd = json["chart"]["result"][0]["indicators"]["quote"][0]["close"][0]
        .as_f64()
        .ok_or("Failed to parse S&P 500 price")?;

        self.timestamp = Utc::now().to_rfc3339();
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), String> {
        let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("sp500.txt")
        .map_err(|e| e.to_string())?;

        writeln!(file, "{}: ${}", self.timestamp, self.price_usd)
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}



fn main(){
    let mut bitcoin = Bitcoin {
        price_usd: 0.0,
        timestamp: String::new(),
    };

    let mut ethereum = Ethereum {
        price_usd: 0.0,
        timestamp: String::new(),
    };

    let mut sp500 = SP500 {
        price_usd: 0.0,
        timestamp: String::new(),
    };


   loop {
    if let Err(e) = bitcoin.fetch_price().and_then(|_| bitcoin.save_to_file()) {
        eprintln!("Bitcoin Error: {}", e);
    }

    if let Err(e) = ethereum.fetch_price().and_then(|_| ethereum.save_to_file()) {
        eprintln!("Ethereum Error: {}", e);
    }

    if let Err(e) = sp500.fetch_price().and_then(|_| sp500.save_to_file()) {
        eprintln!("S&P500 Error: {}", e);
    }

    println!("Data fetched and saved. Sleeping for 10 seconds...");
    thread::sleep(Duration::from_secs(10));

   }
}

