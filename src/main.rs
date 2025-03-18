/* naTERMity - an animated nativity terminal screensaver
 * Copyright (C) 2024-2025 Benjamin S Osenbach
 *
 * Inspired by ChristBASHTree (https://github.com/sergiolepore/ChristBASHTree)
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */
extern crate getopts;
use getopts::Options;

extern crate crossterm;
use crossterm::{cursor, queue, QueueableCommand, style, terminal};
use crate::crossterm::style::Stylize;
use crossterm::style::Color;
use crossterm::cursor::MoveTo;
use crossterm::cursor::Hide;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::Clear;
use crossterm::execute;
use crossterm::style::PrintStyledContent;
use std::io::stdout;
use crossterm::terminal::size;
use crate::crossterm::ExecutableCommand;

use chrono::{Datelike, Local};
use rand::Rng;
use std::{env, io, thread, time};

const brown: Color = Color::Rgb { r: 139, g: 69, b: 19 };

fn main() {
    let mut stdout = stdout();
    terminal::enable_raw_mode();
    stdout.execute(EnterAlternateScreen);
    stdout.execute(Hide);

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts: Options = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("j", "july", "set Christmas in July mode");
    opts.optflag("o", "orthodox", "set Orthodox calendar mode");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let noel_month: u32;
    if matches.opt_present("j") {
        noel_month = 7;
    } else if matches.opt_present("o") {
        noel_month = 1;
    } else {
        noel_month = 12;
    }

    let noel_day: u32;
    match noel_month {
        1 => noel_day = 7,
        _ => noel_day = 25,
    }

    let five_min = time::Duration::from_secs(300);

    let mut dt: chrono::DateTime<Local>;
    let mut selected: u32;

    loop {
        //clear the screen on each iteration
        stdout.execute(Clear(terminal::ClearType::All));

        dt = Local::now();

        if dt.month() == noel_month {
            match dt.day() {
                //before Christmas Eve
                1..23 => selected = 1,
                //Christmas Eve to 12 days of Christmas
                24..25 => selected = 2,
                //Epiphany
                _ => selected = 3,
            }
        } else {
            selected = 0;
        }

        print_scene(selected);
        thread::sleep(five_min);
    }
}

fn print_scene(selected: u32) {
    let size = size();
    let (term_w, term_h) = match size {
        Ok(size) => size,
	Err(error) => panic!("{}", error.to_string()),
    };

    print_stable_manger(term_w, term_h);

    match selected {
        0 => {}
        1 => {
            print_mary_joseph(term_w, term_h);
        }
        2 => {
            print_mary_joseph(term_w, term_h);
            print_jesus(term_w, term_h);
        }
        _ => {
            print_mary_joseph(term_w, term_h);
            print_jesus(term_w, term_h);
            print_magi(term_w, term_h);
        }
    }

    let mut round: u16 = 1;
    loop {
        //twinkle stars in sky every second
        print_sky(term_w);
        print_star(selected, term_w);

        if selected > 1 {
            print_rays(term_w, round);
            round = (round + 1) % 4;
        }

        thread::sleep(time::Duration::from_secs(1));
    }
}

fn print_stable_manger(width: u16, height: u16) {
    let mut stdout = stdout();
    queue!(stdout,
    	MoveTo(width / 2 - 2, height - 11),
	PrintStyledContent("./^\\.".with(brown))
	);

    for i in 1..5 {
        //stable roof
	queue!(stdout,
	    MoveTo(width / 2 - 2 - 3 * i, height - 11 + i),
	    PrintStyledContent(".%%.".with(brown)),
	    MoveTo(width / 2 - 2 + 3 * i, height - 11 + i),
	    PrintStyledContent(".%%.".with(brown))
	    );
    }

    for j in 1..7 {
        //stable walls
	queue!(stdout,
	    MoveTo(width / 2 - 13, height - 7 + j),
	    PrintStyledContent("##".with(brown)),
	    MoveTo(width / 2 + 12, height - 7 + j),
	    PrintStyledContent("##".with(brown))
	    );
    }

    //print manger
    queue!(stdout,
	MoveTo(width / 2 - 3, height - 2),
	PrintStyledContent(r#"""""""#.yellow()),
	MoveTo(width / 2 - 3, height - 1),
	PrintStyledContent(r#"/ \ / \"#.with(brown))
	);
}

fn print_sky(width: u16) {
    let mut rng = rand::thread_rng();
    let mut stdout = stdout();

    //clear the sky
    execute!(stdout,
	MoveTo(width, 4),
	Clear(terminal::ClearType::FromCursorUp)
	);

    for _i in 1..10 {
        //generating (0,0) will throw an error: Goto is one-based
        let x = rng.gen_range(1..width - 1);
        let y = rng.gen_range(1..4);

        //set a star at (x,y)
	queue!(stdout,
	    MoveTo(x, y),
	    PrintStyledContent("*".white())
	    );
    }
}

fn print_star(selected: u32, width: u16) {
    let mut stdout = stdout();
    match selected {
        0 => {
	    queue!(stdout,
		MoveTo(width / 2 + 2, 1),
		PrintStyledContent("*".yellow()),
		MoveTo(width / 2, 2),
		PrintStyledContent("*".yellow()),
		MoveTo(width / 2 - 2, 3),
		PrintStyledContent("*".yellow())
		);
        }
        1 => {
	    queue!(stdout,
		MoveTo(width / 2, 1),
		PrintStyledContent(".".yellow()),
		MoveTo(width / 2 - 2, 2),
		PrintStyledContent(". * .".yellow()),
		MoveTo(width / 2, 3),
		PrintStyledContent(".".yellow())
		);
        }
        _ => {
	    queue!(stdout,
		MoveTo(width / 2, 1),
		PrintStyledContent(":".yellow()),
		MoveTo(width / 2 - 3, 2),
		PrintStyledContent(".. * ..".yellow()),
		MoveTo(width / 2, 3),
		PrintStyledContent(":".yellow())
		);
        }
    }
}

fn print_mary_joseph(width: u16, height: u16) {
    let mut stdout = stdout();

    //print Mary
    queue!(stdout,
	MoveTo(width / 2 - 6, height - 4),
	PrintStyledContent(".@".blue()),
	MoveTo(width / 2 - 6, height - 3),
	PrintStyledContent("%%#".blue()),
	MoveTo(width / 2 - 6, height - 2),
	PrintStyledContent("%%".blue()),
	MoveTo(width / 2 - 7, height - 1),
	PrintStyledContent("%%%".blue())
	);

    queue!(stdout,
	// Joseph's staff
	MoveTo(width / 2 + 5, height - 5),
	PrintStyledContent("?".with(brown)),
	MoveTo(width / 2 + 5, height - 4),
	PrintStyledContent("|".with(brown)),
	MoveTo(width / 2 + 5, height - 3),
	PrintStyledContent("|".with(brown)),
	MoveTo(width / 2 + 5, height - 2),
	PrintStyledContent("|".with(brown)),
	MoveTo(width / 2 + 5, height - 1),
	PrintStyledContent("|".with(brown)),
	// Joseph's body
	MoveTo(width / 2 + 7, height - 5),
	PrintStyledContent("@".green()),
	MoveTo(width / 2 + 6, height - 4),
	PrintStyledContent("#%".green()),
	MoveTo(width / 2 + 7, height - 3),
	PrintStyledContent("%%%".green()),
	MoveTo(width / 2 + 7, height - 2),
	PrintStyledContent("%%%".green()),
	MoveTo(width / 2 + 8, height - 1),
	PrintStyledContent("%%%".green())
	);
}

fn print_jesus(width: u16, height: u16) {
    let mut stdout = stdout();

    queue!(stdout,
	MoveTo(width / 2 - 2, height - 3),
	PrintStyledContent("@##".white())
	);
}

fn print_magi(width: u16, height: u16) {
    let purple = Color::Rgb { r: 159, g: 43, b: 154 };
    let orange = Color::Rgb { r: 255, g: 87, b: 51 };
    let mut stdout = stdout();

    //print Magi
    /*println!(
        "{goto}{yellow}_     _     _  {brown}@   %{reset}",
        goto = cursor::Goto(width / 2 + 18, height - 6)
    );
    println!(
        "{goto}{red}@     {purple}@     {orange}@   {brown}#%%%%%%{reset}",
        goto = cursor::Goto(width / 2 + 18, height - 5)
    );
    println!("{goto}{yellow}# {red}%%\\ {green}^ {purple}%%\\ {purple}o {orange}%%\\ {brown}%%%%%%%%{reset}",
    	goto = cursor::Goto(width/2+16, height-4));
    println!(
        "{goto}{yellow}#{red}#%%% {green}#{purple}#%%% {purple}#{orange}#%%%  {brown}#%%%%={reset}",
        goto = cursor::Goto(width / 2 + 16, height - 3)
    );
    println!(
        "{goto}{red}%%%   {purple}%%%   {orange}%%%  {brown}=    ={reset}",
        goto = cursor::Goto(width / 2 + 18, height - 2)
    );
    println!(
        "{goto}{red}%%%   {purple}%%%   {orange}%%% {brown}.=   .={reset}",
        goto = cursor::Goto(width / 2 + 18, height - 1)
    );*/
}

fn print_rays(width: u16, round: u16) {
    let mut stdout = stdout();

    queue!(stdout,
	MoveTo(width / 2 - 1 - round, 4 + round),
	PrintStyledContent("/".yellow()),
	MoveTo(width / 2, 4 + round),
	PrintStyledContent("|".yellow()),
	MoveTo(width / 2 + 1 + round, 4 + round),
	PrintStyledContent("\\".yellow())
	);

    if round == 0 {
        //clear the rays
	queue!(stdout,
	    MoveTo(width, 7),
	    PrintStyledContent("\\".yellow())
	    );
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}
