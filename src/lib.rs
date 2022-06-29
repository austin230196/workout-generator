use std::env;



pub struct Cacher<T> {
    pub calculate: T,
    pub value: Option<u32>
}


//implementations
impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    
    pub fn new(cal: T) -> Cacher<T> {
        //this function initializes Cacher
        Self {
            calculate: cal,
            value: None
        }
    } 


    pub fn get_value(&mut self, arg: u32) -> u32 {
        //this checks if the value is set and if it is
        if let Some(v) = self.value {
            v
        } else {
            //at this point the self.value is None
            //so we set the value
            //check intensity value
            if arg < 1 || arg > 10 {
                panic!("Intensity level must be in between 1 and 10");
            }

            let v = (self.calculate)(arg);
            self.value = Some(v);
            v
        }
    }
}



pub mod helpers {
    use super::*;
    pub fn get_intensity() -> Result<u32, &'static str> {
        let args = env::args().collect::<Vec<String>>();
        if args.len() == 1 {
            Err("Specify an intensity level")
        }else {
            //at this point the user has enough args we just need the second one
            let intensity = &args[1];
            let intensity = intensity.parse::<u32>().unwrap();
            Ok(intensity)
        }
    }
}
