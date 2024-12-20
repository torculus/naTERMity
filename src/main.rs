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

extern crate terminal_size;
use terminal_size::{Width, Height, terminal_size};

extern crate termion;
use termion::{color, cursor, clear};

use std::{env,thread,time};
use chrono::{Datelike, Local};
use rand::Rng;

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
      1 => {noel_day = 6}, 
      _ => {noel_day = 25}, 
    }

    let five_min = time::Duration::from_secs(300);

    let mut dt: chrono::DateTime<Local>;
    let mut selected: u32;

   loop {
      //clear the screen on each iteration
      println!("{}",termion::clear::All);

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

      selected = 3;

      print_scene(selected);
      thread::sleep(five_min);
   }
}

fn print_scene(selected: u32) {

    let size = terminal_size();
    let term_w: u16;
    let term_h: u16;

    if let Some((Width(w), Height(h))) = size {
        term_w = w;
	term_h = h;
    } else {
	term_w = 0;
	term_h = 0;
        println!("Unable to get terminal size");
    }

    print_stable_manger(term_w, term_h);

    match selected {
      0 => {},
      1 => {print_mary_joseph(term_w, term_h);},
      2 => {print_mary_joseph(term_w, term_h);
      		print_jesus(term_w, term_h);},
      _ => {print_mary_joseph(term_w, term_h);
      		print_jesus(term_w, term_h);
		print_magi(term_w, term_h);},
    }

    loop { //twinkle stars in sky every second
	print_sky(term_w);
	print_star(selected, term_w);
	thread::sleep(time::Duration::from_secs(1));
    }

}

fn print_stable_manger(width: u16, height: u16) {
    let brown = color::Fg(color::Rgb(139,69,19));
    let reset = color::Fg(color::Reset);

    println!("{goto}{brown}./^\\.{reset}",
    	goto = cursor::Goto(width/2-2, height-11));
    
    for i in 1..5 { //stable roof
        println!("{goto}{brown}.%%.{goto2}.%%.{reset}",
		goto = cursor::Goto(width/2-2-3*i,height-11+i),
		goto2 = cursor::Goto(width/2-1+3*i,height-11+i));
    }

    for j in 1..7 { //stable walls
        println!("{goto}{brown}##{goto2}##{reset}",
		goto = cursor::Goto(width/2-13,height-7+j),
		goto2 = cursor::Goto(width/2+12,height-7+j));
    }

    //print manger
    println!("{goto}{brown}{straw}{reset}",
    	goto = cursor::Goto(width/2-3, height-2),
    	straw = r#"\"""""/"#);
    println!("{goto}{brown}{legs}{reset}",
    	goto = cursor::Goto(width/2-3, height-1),
    	legs = r#"/ \ / \"#);

}

fn print_sky(width: u16) {
    let mut rng = rand::thread_rng();

    //clear the sky
    println!("{goto}{clear}",
    	goto = cursor::Goto(width,4),
	clear = clear::BeforeCursor);

    for _i in 1..10 {
        let x = rng.gen_range(0..width-1);
        let y = rng.gen_range(0..4);
	//set a star at (x,y)
	println!("{goto}{white}*{reset}",
		goto = cursor::Goto(x,y),
		white = color::Fg(color::White),
		reset = color::Fg(color::Reset));
    }
}

fn print_star(selected: u32, width: u16) {
    let yellow = color::Fg(color::Yellow);
    let reset = color::Fg(color::Reset);

    match selected {
      0 => { println!("{goto}{yellow}*{reset}",//         *
      		goto = cursor::Goto(width/2+2, 1));//   *
	     println!("{goto}{yellow}*{reset}",//     *
	     	goto = cursor::Goto(width/2, 2));
	     println!("{goto}{yellow}*{reset}",
	     	goto = cursor::Goto(width/2-2, 3));},
      1 => { println!("{goto}{yellow}.{reset}",//       .
      		goto = cursor::Goto(width/2, 1));//   . * .
	     println!("{goto}{yellow}. * .{reset}",//   .
	     	goto = cursor::Goto(width/2-2, 2));
	     println!("{goto}{yellow}.{reset}",
	     	goto = cursor::Goto(width/2, 3));},
      _ => { println!("{goto}{yellow}:{reset}",//       :
      		goto = cursor::Goto(width/2, 1));//  .. * ..
	     println!("{goto}{yellow}.. * ..{reset}",// :
	     	goto = cursor::Goto(width/2-3, 2));
	     println!("{goto}{yellow}:{reset}",
	     	goto = cursor::Goto(width/2, 3));},
    }

    loop {
        println!("{goto}{yellow}/ | \\{reset}",
		goto = cursor::Goto(width/2-2,4));
	thread::sleep(time::Duration::from_secs(1));
        println!("{goto}{yellow}/  |  \\{reset}",
		goto = cursor::Goto(width/2-3,5));
	thread::sleep(time::Duration::from_secs(1));
        println!("{goto}{yellow}/   |   \\{reset}",
		goto = cursor::Goto(width/2-4,6));
	thread::sleep(time::Duration::from_secs(1));

	//clear the rays
        println!("{goto}{clear}{goto2}{clear2}{goto3}{clear3}",
		goto = cursor::Goto(1,4),
		clear = clear::CurrentLine,
		goto2 = cursor::Goto(1,5),
		clear2 = clear::CurrentLine,
		goto3 = cursor::Goto(1,6),
		clear3 = clear::CurrentLine);
	thread::sleep(time::Duration::from_secs(1));
    }

}

fn print_mary_joseph(width: u16, height: u16) {
    let blue = color::Fg(color::Blue);
    let brown = color::Fg(color::Rgb(139,69,19));
    let green = color::Fg(color::Green);
    let reset = color::Fg(color::Reset);

    //print Mary
    println!("{goto}{blue}.@{reset}",
    	goto = cursor::Goto(width/2-6, height-4));
    println!("{goto}{blue}%%#{reset}",
    	goto = cursor::Goto(width/2-6, height-3));
    println!("{goto}{blue}%%{reset}",
    	goto = cursor::Goto(width/2-6, height-2));
    println!("{goto}{blue}%%%{reset}",
    	goto = cursor::Goto(width/2-7, height-1));

    //print Joseph
    println!("{goto}{brown}? {green}@{reset}",
    	goto = cursor::Goto(width/2+5, height-5));
    println!("{goto}{brown}|{green}#%\\{reset}",
    	goto = cursor::Goto(width/2+5, height-4));
    println!("{goto}{brown}| {green}%%%{reset}",
    	goto = cursor::Goto(width/2+5, height-3));
    println!("{goto}{brown}| {green}%%%{reset}",
    	goto = cursor::Goto(width/2+5, height-2));
    println!("{goto}{brown}|  {green}%%%{reset}",
    	goto = cursor::Goto(width/2+5, height-1));
}

fn print_jesus(width: u16, height: u16) {
    let blue = color::Fg(color::Blue);
    let reset = color::Fg(color::Reset);

    println!("{goto}{blue}@###{reset}",
    	goto = cursor::Goto(width/2-2, height-3));
}

fn print_magi(width: u16, height: u16) {
    let brown = color::Fg(color::Rgb(139,69,19));
    let green = color::Fg(color::Green);
    let reset = color::Fg(color::Reset);

    //print Magi
    println!("{goto}{brown}@   %{reset}",
    	goto = cursor::Goto(width/2+31, height-6));
    println!("{goto}{green}@    @    @   {brown}#%%%%%%{reset}",
    	goto = cursor::Goto(width/2+18, height-5));
    println!("{goto}{brown}#%%\\ #%%\\ #%%\\ %%%%%%%%{reset}",
    	goto = cursor::Goto(width/2+17, height-4));
    println!("{goto}{brown}%%%  %%%  %%%  #%%%%={reset}",
    	goto = cursor::Goto(width/2+18, height-3));
    println!("{goto}{brown}%%%  %%%  %%%  =    ={reset}",
    	goto = cursor::Goto(width/2+18, height-2));
    println!("{goto}{brown}%%%  %%%  %%% .=   .={reset}",
    	goto = cursor::Goto(width/2+18, height-1));
}
