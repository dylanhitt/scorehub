extern crate pwr_hd44780;

use clap::Parser;
use pwr_hd44780::Hd44780;
use pwr_hd44780::frontends::Direct;
use std::process;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short, action)]
    off: bool,
}


fn main() {
    let team1 = Team {
        name: String::from("BAL"),
        score: 1,
        player: String::from("J. Mateo"),
    };

    let team2 = Team {
        name: String::from("WAS"),
        score: 0,
        player: String::from("V. Robles"),
    };

    let scoreboard = ScoreBoard {
        outs: 2,
        inning: 8,
        away: team2,
        home: team1,
        isTop: true,
    };
   
    let args = Args::parse();

    

    let mut display = create_lcd();
    
    if args.off {
        display.off().unwrap();
        process::exit(0);
    }

    display.update_board(scoreboard).unwrap();
}

struct LCDDisplay {
    lcd: Direct,
}

fn create_lcd() -> LCDDisplay {
    let lcd_bus = pwr_hd44780::I2CBus::new("/dev/i2c-1", 0x27).unwrap();
    let lcd = pwr_hd44780::DirectLcd::new(Box::new(lcd_bus), 20, 4).unwrap();
    LCDDisplay {
        lcd
    }
}

impl LCDDisplay {
    fn off(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.lcd.clear()?;
        self.lcd.set_backlight(false)?;

        Ok(())
    }

    fn update_board(&mut self, sb: ScoreBoard) -> Result<(), Box<dyn std::error::Error>> {
        self.lcd.clear()?;
        self.lcd.print_at(0, 0, format!("{}", sb.away.name))?;
        self.lcd.print_at(0, 4, format!("{}", sb.away.score))?;

        self.lcd.print_at(1, 6, format!("{}", sb.inning))?;

        self.lcd.print_at(2, 0, format!("{}", sb.home.name))?;
        self.lcd.print_at(2, 4, format!("{}", sb.home.score))?;

        Ok(())
    }
}

struct Team {
    name: String,
    score: u8,
    player: String,
}

struct ScoreBoard {
    outs: u8,
    inning: u8,
    away: Team,
    home: Team,
    isTop: bool,
}

