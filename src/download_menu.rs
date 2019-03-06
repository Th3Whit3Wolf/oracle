use crate::{distro::distro_menu, download_distros::download_ubuntu, os::Os, work::inlp};

use termion::{clear, cursor};

use std::process;

pub fn download_menu(neofetch: &str, array: &[Os; 42]) {
    println!("{}{}{}", clear::All, cursor::Goto(1, 1), neofetch);
    println!("1.) Ubuntu \n2.) Lubuntu \n3.) Kubuntu \n4.) Xubuntu \n5.) Ubuntu Mate \n6.) Ubuntu Studio \n7.) Ubuntu Budgie \n8.) Go Back \n9.) Exit\n");
    let input = inlp();

    match input {
        1 => {
            download_ubuntu("http://releases.ubuntu.com/", &array[26], neofetch, array);
        }
        2 => {
            download_ubuntu(
                "http://cdimage.ubuntu.com/lubuntu/releases/",
                &array[26],
                neofetch,
                array,
            );
        }
        3 => {
            download_ubuntu(
                "http://cdimage.ubuntu.com/kubuntu/releases/",
                &array[20],
                neofetch,
                array,
            );
        }
        4 => {
            download_ubuntu(
                "http://cdimage.ubuntu.com/xubuntu/releases/",
                &array[30],
                neofetch,
                array,
            );
        }
        5 => {
            download_ubuntu(
                "http://cdimage.ubuntu.com/ubuntu-mate/releases/",
                &array[13],
                neofetch,
                array,
            );
        }
        6 => {
            download_ubuntu(
                "http://cdimage.ubuntu.com/ubuntustudio/releases/",
                &array[29],
                neofetch,
                array,
            );
        }
        7 => {
            download_ubuntu(
                "http://cdimage.ubuntu.com/ubuntu-budgie/releases/",
                &array[27],
                neofetch,
                array,
            );
        }
        8 => distro_menu(neofetch, array),
        9 => process::exit(0x0100),
        _ => println!("That's not an option"),
    }
}
