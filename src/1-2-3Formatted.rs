use std::fmt::{Display, Formatter, Result};


struct City {
    name : &'static str,
    lat : f32,
    lon : f32,
}

impl Display for City {
    // f is formatted string buffer
    fn fmt(&self, f : &mut Formatter) -> Result{
        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'E'} else {'W'};

        write!(f,"{}: {:.3}°{} {:.3}°{}", 
            self.name, self.lat.abs(), lat_c, self.lat.abs(), lon_c)
    }
}

fn main(){
    let city = City{
        name : "korea",
        lat : 32.3,
        lon : -20.4
    };

    println!("{}", city);
}
