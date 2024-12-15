/* naTERMity - an animated nativity terminal screensaver
 * Copyright (C) 2024 Benjamin S Osenbach
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

extern crate term_size;
use term_size;

extern crate termion;
use termion::{color, style, cursor, clear};

use std::{env,thread,time};
use chrono::{Datelike, Local};

fn main() {

   let args: Vec<String> = env::args().collect();

   let mut opts: Options = Options::new();
   opts.optflag("h", "help", "print this help menu");
   opts.optflag("j", "july", "set Christmas in July mode");
   opts.optflag("o", "orthodox", "set Orthodox calendar mode");

   let matches = match opts.parse(&args[1..]) {
       Ok(m) => { m },
       Err(f) => { panic!("{}", f.to_string()) },
   };
   
   let mut noel_month: u32;
   if matches.opt_present("j") {
   	noel_month = 7;
   } else if matches.opt_present("o") {
   	noel_month = 1;
   } else {
   	noel_month = 12;
   }

   let mut noel_day: u32;
   match noel_month {
     1 => {noel_day = 6}, 
     _ => {noel_day = 25}, 
   }

   let five_min = time::Duration::from_secs(300);

   let mut dt: chrono::DateTime<Local> = Local::now();
   let mut selected: u32;

   loop {
      //clear the screen on each iteration
      println!("{}", termion::clear::All);

      dt = Local::now();

      if dt.month() == noel_month {
        if dt.day() < noel_day - 1 { //before Christmas Eve
            selected = 0;
        } else if dt.day() == noel_day - 1 { //Christmas Eve
            selected = 1;
        } else if dt.day() == noel_day { //Christmas day
            selected = 2;
        } else {
            selected = 3; //Magi visit
        }
      } else {
        selected = 0;
      }

      print_scene(selected);
      thread::sleep(five_min);
   }
}

fn print_scene(selected: u32) {
    let Some((term_w, term_h)) = term_size::dimensions();
    if (term_w < 30 || (term_h < 10) {
    	//pass
    }

    let stars: [&str; 3] = ["
             *          *             *         *         *
  *                        *        *               *                *
         *      *                *         *                  *",
"
	     *          *           .           *         *
  *                        *      . * .             *                *
         *      *                   .      *                  *",
"
	     *          *           :           *         *
  *                        *     .. * ..            *                *
         *      *                   :      *                  *"];
 
	let manger: &str = 
"                                  ./^\\.
                               .%%.   .%%.
                           .%%.           .%%.
                        .%%.                 .%%.
                     .%%.                       .%%.
                      ##                         ##
                      ##                         ##
                      ##                         ##
                      ##                         ##
                      ##         \\\"\"\"\"\"/         ##
                      ##         / \\ / \\         ##";
   
    if selected == 0 {
      println!("{white}{}", stars[0],
         white = color::Fg(color::White));
      println!("{brown}{manger}",
         brown = color::Fg(color::Rgb(139,69,19)));
   } else if selected == 1 {
      println!("{white}{}", stars[1],
         white = color::Fg(color::White));
      println!("{brown}{manger}",
         brown = color::Fg(color::Rgb(139,69,19)));
   } else if selected == 2 {
      println!("{white}{}", stars[2],
         white = color::Fg(color::White));
      println!("{brown}{manger}",
         brown = color::Fg(color::Rgb(139,69,19)));
   } else if selected == 3 {
      println!("{white}{}", stars[2],
         white = color::Fg(color::White));
      println!("{brown}{manger}",
         brown = color::Fg(color::Rgb(139,69,19)));
   }
   
}

fn prep_sky(selected: u32) {
    a
}
