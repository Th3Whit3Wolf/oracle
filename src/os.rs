use crate::{
    distro::distro_menu,
    work::{art, dd, inlp, iso_to_pp, pause, usb},
};

use termion::{clear, cursor};

use std::{io, process};

pub struct Os {
    pub path: &'static str,
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub xx: u8,
    pub yy: u8,
    pub zz: u8,
}

impl Os {
    pub fn print_distro_art(
        &self,
        file_in: &str,
        version: &str,
        fs: &str,
        neofetch: &str,
        array: &[Os; 42],
    ) {
        art(&self, version, fs, false);

        let usb = usb();
        let usbz: Vec<&str> = usb.lines().collect();
        let mut usb_name: Vec<String> = Vec::with_capacity(3);
        let mut usb_path: Vec<String> = Vec::with_capacity(3);

        #[allow(unused_variables)]
        let usb_temp: Vec<&str> = Vec::with_capacity(3);

        for x in &usbz {
            let mut usb_temp: Vec<&str> = x.split_whitespace().collect();
            usb_path.push("/dev/".to_string() + usb_temp[0]);
            usb_name.push(usb_temp[3].to_string() + " " + usb_temp[1]);
            usb_temp.clear();
        }

        if usbz.is_empty() {
            println!(
                "It looks like you have no USBs plugged in...\n1.) Reload \n2.) Go Back\n3.) Exit"
            );

            let input = inlp();

            match input {
                1 => Os::print_distro_art(self, file_in, version, fs, neofetch, array),
                2 => distro_menu(neofetch, array),
                3 => process::exit(0x0100),
                _ => println!("That's not an option"),
            }
        } else {
            println!("What USB would you like to make a like to make your Live USB with?");
            let mut i = 0;
            while i != usbz.len() {
                println!("{}.) {}", i + 1, usb_name[i]);
                i += 1;
            }
            println!(
                "{}.) Refresh\n{}.) Go Back\n{}.) Exit",
                usb_path.len() + 1,
                usb_path.len() + 2,
                usb_path.len() + 3
            );

            let input = inlp();

            if input > 0 && input < usb_path.len() + 1 {
                let out_file: &str = &usb_path[input - 1];
                let device: String = usb_name[input - 1].to_string();
                println!("Would you like to install to {}\n1.) yes\n2.) no", device);
                loop {
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("failed to read input");

                    let input: usize = match input.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("There were only two numbers to choose from... and you still fucked it up");
                            continue;
                        }
                    };
                    let dd_arg: String = "sudo dd bs=4M if=".to_string()
                        + file_in
                        + " of="
                        + out_file
                        + " status=progress oflag=sync";
                    match input {
                        1 => {
                            let dd = dd(dd_arg);
                            print!("{}{}", clear::All, cursor::Goto(1, 1));
                            if dd.success() {
                                println!("{} was successfully installed!", iso_to_pp(file_in));
                                pause();
                                Os::print_distro_art(self, file_in, version, fs, neofetch, array);
                            } else {
                                println!("Installation Failed... \n{}", dd);
                            }
                            Os::print_distro_art(self, file_in, version, fs, neofetch, array);
                        }
                        2 => {
                            print!("{}{}", clear::All, cursor::Goto(1, 1));
                            Os::print_distro_art(self, file_in, version, fs, neofetch, array);
                        }
                        _ => {
                            println!("\nInappropriate respone, please pick \n1.)Yes 2.) No");
                        }
                    }
                }
            }
            // Out of bounds
            if input < 1 || input > usb_path.len() + 3 {
                println!("That's not an option");
            }
            // Refresh
            if input == usb_path.len() + 1 {
                print!("{}{}", clear::All, cursor::Goto(1, 1));
                Os::print_distro_art(self, file_in, version, fs, neofetch, array);
            }
            // Go Back
            if input == usb_path.len() + 2 {
                distro_menu(neofetch, array);
            }
            // Exit
            if input == usb_path.len() + 3 {
                process::exit(0x0100);
            }
        }
    }
}
