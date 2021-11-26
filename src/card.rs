use rand::Rng;
use rand::rngs::ThreadRng;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub enum CardType {
    Visa,
    Master,
    AmericanExpress(AmericanExpressCard),
    Discover,
    Custom(i64),
}

#[derive(Clone)]
pub enum AmericanExpressCard {
    T4,
    T7,
}

impl CardType {
    pub fn from_string(v: &String) -> Result<CardType, String> {
        match v.to_lowercase().as_ref() {
            "visa" | "v" => {
                Ok(CardType::Visa)
            }
            "master" | "m" => {
                Ok(CardType::Master)
            }
            "american_express_4" | "americanexpress_4" | "american_4" | "am_4" => {
                Ok(CardType::AmericanExpress(AmericanExpressCard::T4))
            }
            "american_express_7" | "americanexpress_7" | "american_7" | "am_7" => {
                Ok(CardType::AmericanExpress(AmericanExpressCard::T7))
            }
            "discover" | "d" => {
                Ok(CardType::Discover)
            }
            _ => {
                if v.parse::<i64>().is_ok() {
                    Ok(CardType::Custom(v.parse::<i64>().unwrap()))
                } else {
                    Err("Invalid Card Type".to_string())
                }
            }
        }
    }

    fn int_val(&self) -> i64 {
        match self {
            CardType::Visa => 4,
            CardType::Master => 5,
            CardType::AmericanExpress(am) => {
                match am {
                    AmericanExpressCard::T4 => 34,
                    AmericanExpressCard::T7 => 37
                }
            }
            CardType::Discover => 6,
            CardType::Custom(v) => v.clone()
        }
    }
}

pub struct Card {
    number: Option<i64>,
    length: i64,
    ctype: CardType,
}


impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut type_name = String::new();
        match self.ctype.clone() {
            CardType::Visa => type_name = String::from("Visa"),
            CardType::Master => type_name = String::from("Master"),
            CardType::AmericanExpress(t) => {
                match t {
                    AmericanExpressCard::T4 => type_name = String::from("American Express v4"),
                    AmericanExpressCard::T7 => type_name = String::from("American Express v7")
                }
            }
            CardType::Discover => type_name = String::from("discover"),
            CardType::Custom(i) => type_name = format!("custom({})", i)
        }
        if let Some(i) = self.number {
            write!(f, "type: {}, number: {}", type_name, i)
        } else {
            write!(f, "type: {}, number: {}", type_name, "None_Generated")
        }
    }
}

impl Card {
    pub fn from(length: i64, card: CardType) -> Card {
        Card { length, ctype: card, number: None }
    }

    pub fn is_valid(&self) -> Result<bool, String> {
        let mut num: i64 = 0;
        if let Some(i) = self.number {
            num = i;
        } else {
            return Err("num is not set".to_string());
        }
        let mut num: i64 = num.clone();
        let mut new_num: i64 = 0;
        let mut add_num: i64 = 0;
        let mut first = true;
        let mut temp_num: i64 = 0;
        while num > 0 {
            temp_num *= 10;
            temp_num += num % 10;
            num = num / 10;
        }
        if temp_num % 10 == 3 {
            if (temp_num / 10) % 10 == 4 || (temp_num / 10) % 10 == 7 {
                first = false;
            }
        }
        num = temp_num;
        while num > 0 {
            add_num = num % 10;
            num /= 10;
            if first {
                add_num *= 2;
                if add_num > 9 {
                    add_num = add_num / 10 + (add_num % 10);
                }
            }
            first = !first;
            new_num += add_num;
        }
        return Ok(new_num % 10 == 0);
    }


    pub fn generate_number(&mut self, rng: &mut ThreadRng) {
        let mut t = self.ctype.clone().int_val();
        let glength = self.length - numlength(self.ctype.clone().int_val());
        let mut beg = 1;
        for _ in 0..glength - 1 {
            beg *= 10;
        }
        t *= beg * 10;
        let mut seed = rng.gen_range(beg..beg * 10);
        let mut v = t + seed;
        self.number = Some(v);
        while !self.is_valid().unwrap() {
            seed = rng.gen_range(beg..beg * 10);
            v = t + seed;
            self.number = Some(v);
        }
    }
}


fn numlength(n: i64) -> i64 {
    let mut n = n;
    if n == 0 {
        return 1;
    }
    let mut l: i64 = 0;

    while n > 0 {
        n = n / 10;
        l += 1;
    }
    return l;
}