use std::{fs::OpenOptions, io::Write, thread, time::Duration};

trait Pricing {
    fn fetch_to_price(&mut self);
    fn save_to_file(&self);
}

struct Bitcoin{
    price: f64,
}

struct Ethereum{
    price: f64,
}

struct SP500{
    price: f64,
}

impl Pricing for Bitcoin {
    fn fetch_to_price(&mut self){
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        let res = ureq::get(url).call();
        
        // Handling the result to get the response and then into_string on Response
        if let Ok(response) = res {
            if let Ok(body) = response.into_string() {
                let parsed: serde_json::Value = serde_json::from_str(&body).unwrap_or_default();
                if let Some(price) = parsed["bitcoin"]["usd"].as_f64() {
                    self.price = price;
                } else {
                    println!("Failed to parse Bitcoin price.");
                }
            } else {
                println!("Failed to read Bitcoin response body.");
            }
        } else {
            println!("Bitcoin API error: {:?}", res.err());
        }
    }

    fn save_to_file(&self){
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("bitcoin.txt")
            .unwrap();
        writeln!(file, "Bitcoin: ${:.2}", self.price).unwrap();
    }
}

impl Pricing for Ethereum {
    fn fetch_to_price(&mut self){
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        let res = ureq::get(url).call();
        
        if let Ok(response) = res {
            if let Ok(body) = response.into_string() {
                let parsed: serde_json::Value = serde_json::from_str(&body).unwrap_or_default();
                if let Some(price) = parsed["ethereum"]["usd"].as_f64() {
                    self.price = price;
                } else {
                    println!("Failed to parse Ethereum price.");
                }
            } else {
                println!("Failed to read Ethereum response body.");
            }
        } else {
            println!("Ethereum API error: {:?}", res.err());
        }
    }

    fn save_to_file(&self){
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("ethereum.txt")
            .unwrap();
        writeln!(file, "Ethereum: ${:.2}", self.price).unwrap();
    }
}


impl Pricing for SP500 {
    fn fetch_to_price(&mut self){
        self.price = 4000.0;
    }
    

    fn save_to_file(&self){
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("sp500.txt")
            .unwrap();
        writeln!(file, "S&P 500: ${:.2}", self.price).unwrap();
    }
}

fn main(){
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin { price: 0.0 }),
        Box::new(Ethereum { price: 0.0 }),
        Box::new(SP500 { price: 0.0 }),
    ];

    loop {
        for asset in assets.iter_mut(){
            asset.fetch_to_price();
            asset.save_to_file();
        }
        println!("Prices fetched and saved. Sleeping...");
        thread::sleep(Duration::from_secs(10));
    }
}