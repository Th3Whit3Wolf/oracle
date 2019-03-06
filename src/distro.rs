use crate::{
    download_menu::download_menu,
    os::Os,
    work::{find_distro_name, find_distros, find_version, fs, inlp, iso_to_pp, unknown_distro},
};

use termion::{clear, cursor};

use std::process;

pub fn distro_menu(neofetch: &str, array: &[Os; 42]) {
    print!("{}{}{}", clear::All, cursor::Goto(1, 1), neofetch);
    let g_d = find_distros();
    if g_d.trim_right().is_empty() {
        println!("It looks like you don't have any Operating Systems downloaded...\n1.) Reload List \n2.) Exit");
        let input = inlp();

        match input {
            1 => distro_menu(neofetch, array),
            2 => process::exit(0x0100),
            _ => println!("That's not an option"),
        }
    } else {
        let distros: Vec<&str> = g_d.split_whitespace().collect();
        println!(
            "{:^90}",
            "What Operating System would you like to make a Live USB with?"
        );

        let mut i = 0;
        while i != distros.len() {
            if i < 9 {
                println!("0{}.) {}", i + 1, iso_to_pp(distros[i]));
                i += 1;
            } else {
                println!("{}.) {}", i + 1, iso_to_pp(distros[i]));
                i += 1;
            }
        }

        if distros.len() < 9 {
            if distros.len() == 8 {
                println!(
                    "0{}.) Download\n{}.) Reload\n{}.) EXIT",
                    distros.len() + 1,
                    distros.len() + 2,
                    distros.len() + 3
                );
            }
            if distros.len() == 7 {
                println!(
                    "0{}.) Download\n0{}.) Reload\n{}.) EXIT",
                    distros.len() + 1,
                    distros.len() + 2,
                    distros.len() + 3
                );
            }
            if distros.len() < 7 {
                println!(
                    "0{}.) Download\n0{}.) Reload\n0{}.) EXIT",
                    distros.len() + 1,
                    distros.len() + 2,
                    distros.len() + 3
                );
            }
        } else {
            println!(
                "{}.) Download\n{}.) Reload\n{}.) EXIT",
                distros.len() + 1,
                distros.len() + 2,
                distros.len() + 3
            );
        }

        let input = inlp();

        if input > 0 && input < distros.len() + 1 {
            print!("{}{}", clear::All, cursor::Goto(1, 1));
            let file_in: &str = &distros[input - 1].to_string();
            let version: &str = &find_version(&distros[input - 1]);
            let fs: &str = &fs(&distros[input - 1]);
            match find_distro_name(distros[input - 1]).trim_right() {
                "androidr" => array[3].print_distro_art(file_in, version, fs, neofetch, &array),
                "archlinux" => array[8].print_distro_art(file_in, version, fs, neofetch, &array),
                "elementaryos" => {
                    array[17].print_distro_art(file_in, version, fs, neofetch, &array)
                }
                "Fedora" => array[18].print_distro_art(file_in, version, fs, neofetch, &array),
                "kali" => array[19].print_distro_art(file_in, version, fs, neofetch, &array),
                "kubuntu" => array[20].print_distro_art(file_in, version, fs, neofetch, &array),
                "lubuntu" => array[21].print_distro_art(file_in, version, fs, neofetch, &array),
                "manjaro" => array[22].print_distro_art(file_in, version, fs, neofetch, &array),
                "linuxmintxfce" | "linuxmintmate" | "linuxmintcinnamon" | "linuxmintkde" => {
                    array[23].print_distro_art(file_in, version, fs, neofetch, &array)
                }
                "qubes" => array[24].print_distro_art(file_in, version, fs, neofetch, &array),
                "rhel" => array[25].print_distro_art(file_in, version, fs, neofetch, &array),
                "ubuntu" => array[26].print_distro_art(file_in, version, fs, neofetch, &array),
                "ubuntubudgie" => {
                    array[27].print_distro_art(file_in, version, fs, neofetch, &array)
                }
                "ubuntumate" => array[28].print_distro_art(file_in, version, fs, neofetch, &array),
                "ubuntustudio" => {
                    array[29].print_distro_art(file_in, version, fs, neofetch, &array)
                }
                "xubuntu" => array[30].print_distro_art(file_in, version, fs, neofetch, &array),
                "void" | "voidlxqt" | "voidcinnamon" | "voidlxde" | "voidxfce"
                | "voidenlightenment" => {
                    array[31].print_distro_art(file_in, version, fs, neofetch, &array)
                }
                _ => unknown_distro(fs, neofetch, &iso_to_pp(distros[input - 1]), file_in, array),
            }
        }
        // Out of bounds
        if input < 1 || input > distros.len() + 3 {
            println!("That's not an option");
        }
        // Downloads
        if input == distros.len() + 1 {
            download_menu(neofetch, array);
        }
        // Refresh
        if input == distros.len() + 2 {
            distro_menu(neofetch, array);
        }
        // Exit
        if input == distros.len() + 3 {
            process::exit(0x0100);
        }
    }
}
