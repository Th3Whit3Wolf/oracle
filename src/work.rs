use crate::{distro::distro_menu, download::download, os::Os};

use {
    regex::Regex,
    termion::{clear, color, cursor},
    walkdir::{DirEntry, WalkDir},
};

use std::{
    borrow::Cow,
    ffi::OsStr,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    process::{self, Command, Stdio},
    thread, time,
};

// wait 3/4 sec
pub fn pause() {
    let half_sec = time::Duration::from_millis(750);
    thread::sleep(half_sec);
}

// Input Loop
pub fn inlp() -> usize {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That was not a number, try again");
                continue;
            }
        };
        return input;
    }
}

// Launch Neofetch and capture output
pub fn neofetch() -> String {
    let nf = Command::new("sh")
        .arg("-c")
        .arg("neofetch")
        .output()
        .expect("failed to execute neofetch");

    if nf.status.success() {
        let nf = String::from_utf8_lossy(&nf.stdout);
        nf.to_string()
    } else {
        "".to_string()
    }
}

// Run dd and stream output
pub fn dd(s: String) -> std::process::ExitStatus {
    let mut dd = Command::new("sh")
        .arg("-c")
        .arg(s)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    dd.wait().unwrap()
}

// Get file size
pub fn fs(st: &str) -> String {
    let fs = fs::metadata(st).unwrap().len();
    let math = fs / 1024 / 1024;
    let string = math.to_string();
    if math < 1000 {
        string + " MB"
    } else {
        "".to_string() + &string[..1] + "," + &string[1..4] + " MB"
    }
}

// Parse path for file name
pub fn basename(path: &'_ str) -> Cow<'_, str> {
    let mut pieces = path.rsplit('/');
    match pieces.next() {
        Some(p) => p.into(),
        None => path.into(),
    }
}

// Uppercase first letter in &str
pub fn to_uppercase(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Find all .iso & .img files
pub fn find_distros() -> String {
    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
    }

    let mut v: Vec<String> = Vec::with_capacity(10);
    let ext1 = Some(OsStr::new("iso"));
    let ext2 = Some(OsStr::new("img"));

    match dirs::home_dir() {
        Some(path) => {
            let walker = WalkDir::new(&path).min_depth(1).max_depth(4).into_iter();
            for entry in walker.filter_entry(|e| !is_hidden(e)) {
                let entry = entry.unwrap();
                if entry.path().extension() == ext1 || entry.path().extension() == ext2 {
                    let iso_path = entry.path().display().to_string();
                    v.push(iso_path);
                }
            }
            let join = v.join("\n");
            join.to_string()
        }
        None => {
            let walker = WalkDir::new("/home/").min_depth(1).max_depth(4).into_iter();
            for entry in walker.filter_entry(|e| !is_hidden(e)) {
                let entry = entry.unwrap();
                if entry.path().extension() == ext1 || entry.path().extension() == ext2 {
                    let iso_path = entry.path().display().to_string();
                    v.push(iso_path);
                }
            }
            let join = v.join("\n");
            join.to_string()
        }
    }
}

// Parses for version
pub fn find_version(s: &str) -> String {
    let letters = Regex::new(r"[[:alpha:]]").unwrap();
    let s = str::replace(&s, ".iso", "");
    let s = str::replace(&s, "-amd64", "");
    let s = str::replace(&s, "-desktop", "");
    let s = str::replace(&s, "-live", "");
    let s = str::replace(&s, "-Live", "");
    let s = str::replace(&s, "-x86_64", "");
    let s = str::replace(&s, "-", " ");
    let s = str::replace(&s, "server", "");
    let s = basename(&s);
    let s = letters.replace_all(&s, "");
    s.to_string()
}

// Parses Distro file into something more pleasing to human eyes
pub fn iso_to_pp(s: &str) -> String {
    let ubuntu_serv =
        Regex::new(r"(?P<D>[[:alpha:]]) (?P<V>\d{2}\.\d{2}\.\d{1}) (?P<S>Server)").unwrap();
    let void = Regex::new(r"(?P<D>[[:alpha:]]) (?P<V>\d{8}) (?P<S>Cinnamon)").unwrap();
    let s = str::replace(&s, ".iso", "");
    let s = str::replace(&s, "dfly", "DragonflyBSD");
    let s = str::replace(&s, "Win10", "Windows 10");
    let s = str::replace(&s, "archlinux", "Arch Linux");
    let s = str::replace(&s, "_intel_34", "");
    let s = str::replace(&s, "_English_x64", "");
    let s = str::replace(&s, "-amd64", "");
    let s = str::replace(&s, "_amd64", "");
    let s = str::replace(&s, "-desktop", "");
    let s = str::replace(&s, "-live", "");
    let s = str::replace(&s, "-Live", "");
    let s = str::replace(&s, "-x86_64", "");
    let s = str::replace(&s, "-64bit", "");
    let s = str::replace(&s, "_REL", "");
    let s = str::replace(&s, "mate", "MATE");
    let s = str::replace(&s, "kde", "KDE");
    let s = str::replace(&s, "xfce", "XFCE");
    let s = str::replace(&s, "lxqt", "LXQT");
    let s = str::replace(&s, "cinnamon", "Cinnamon");
    let s = str::replace(&s, "budgie", "Budgie");
    let s = str::replace(&s, "kylin", " Kylin");
    let s = str::replace(&s, "studio", " Studio");
    let s = str::replace(&s, "-dvd", "");
    let s = str::replace(&s, "-v2", "");
    let s = str::replace(&s, "musl", "Musl");
    let s = str::replace(&s, "-", " ");
    let s = str::replace(&s, "_", " ");
    let s = str::replace(&s, "server", "Server");
    let s = str::replace(&s, "standard", "Standard");
    let s = str::replace(&s, "os", "OS");
    let s = basename(&s);
    let s = to_uppercase(&s);
    let s = ubuntu_serv.replace_all(&s, "$D $S $V ");
    let s = void.replace_all(&s, "$D $S $V ");
    s.to_string()
}

// Parses file into something that can predictably matched on
pub fn find_distro_name(s: &str) -> String {
    let ubuntu_serv =
        Regex::new(r"(?P<D>[[:alpha:]]) (?P<V>\d{2}\.\d{2}\.\d{1}) (?P<S>Server)").unwrap();
    let dist_name = Regex::new(r"[0-9]").unwrap();
    let s = str::replace(&s, ".iso", "");
    let s = str::replace(&s, "-amd64", "");
    let s = str::replace(&s, "-desktop", "");
    let s = str::replace(&s, "-live", "");
    let s = str::replace(&s, "-Live", "");
    let s = str::replace(&s, "-x86_64", "");
    let s = str::replace(&s, "-dvd", "");
    let s = str::replace(&s, "-musl", "");
    let s = str::replace(&s, "-Workstation", "");
    let s = str::replace(&s, "-stable", "");
    let s = str::replace(&s, "-64bit", "");
    let s = str::replace(&s, "-v2", "");
    let s = str::replace(&s, "-", "");
    let s = str::replace(&s, "server", "");
    let s = str::replace(&s, ".", "");
    let s = basename(&s);
    let s = ubuntu_serv.replace_all(&s, "$D");
    let s = dist_name.replace_all(&s, "");
    s.to_string()
}

// Get list of all detected USBs
pub fn usb() -> std::string::String {
    let usb = Command::new("sh")
        .arg("-c")
        .arg("lsblk -o NAME,SIZE,TYPE,VENDOR | grep -e disk | grep -v ATA | grep -v nvme ")
        .output()
        .expect("failed to execute process");
    let usb = String::from_utf8_lossy(&usb.stdout);
    usb.to_string()
}

// Yes or no function
pub fn confirmation(
    info: &str,
    value1: &str,
    f: &Fn(&str, &Os, &str, &[Os; 42]),
    url: &str,
    os: &Os,
    neofetch: &str,
    array: &[Os; 42],
) {
    println!("{} \n1.) Yes \n2.) No", info);
    let input = inlp();

    match input {
        1 => {
            println!("{}", value1);
            download(&value1, neofetch, array).expect("No Internet bro");
        }
        2 => {
            f(url, os, neofetch, array);
        }
        _ => {
            println!("\nInappropriate respone, please pick \n1.)Yes 2.) No");
        }
    }
}

// For .iso files that aren't recongnized by the program
// This looks really ugly to print everything while only locking stdio once without
pub fn unknown_distro(fs: &str, neofetch: &str, distro: &str, file_in: &str, array: &[Os; 42]) {
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    println!(
        "{}        #####                                   {}Operating System: {}\n{}       #######                                  {}OS Type: Unknown\n{}       ##{}O{}#{}O{}##                                  {}File Size: {}\n{}       #{}#####{}#                                  {}Version: Unknown\n{}     ##{}##{}###{}##{}##                                {}Based On: Unknown\n{}    #{}##########{}##                               {}Devloper: Unknown\n{}   #{}############{}##                              {}Origin: Unknown\n{}   #{}############{}###                             {}Architecture: Unknown\n{}  ##{}#{}###########{}##{}#                             {}Desktop: Unknown\n{}######{}#{}#######{}#{}######                           {}Package Manager: Unknown\n{}#######{}#{}#####{}#{}#######                           {}Initial Release: Unknown\n{}  #####{}#######{}#####                             {}Release Cycle: Unknown",
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        distro,
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        fs,
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(195, 159, 0)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(195, 159, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(195, 159, 0)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(195, 159, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(195, 159, 0)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(195, 159, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(195, 159, 0)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(195, 159, 0)),
        color::Fg(color::Rgb(255, 255, 255)),
        color::Fg(color::Rgb(195, 159, 0)),
        color::Fg(color::Rgb(0, 0, 0)),
        color::Fg(color::Rgb(195, 159, 0)),
        color::Fg(color::Rgb(255, 255, 255))
    );
    print!("\n{}", color::Fg(color::Reset));

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
            1 => unknown_distro(fs, neofetch, distro, file_in, array),
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
                            unknown_distro(fs, neofetch, distro, file_in, array);
                        } else {
                            println!("Installation Failed... \n{}", dd);
                        }
                        unknown_distro(fs, neofetch, distro, file_in, array);
                    }
                    2 => {
                        print!("{}{}", clear::All, cursor::Goto(1, 1));
                        unknown_distro(fs, neofetch, distro, file_in, array);
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
            unknown_distro(fs, neofetch, distro, file_in, array);
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

pub fn art(os: &Os, version: &str, fs: &str, test: bool) {
    if !test {
        print!("{}{}", clear::All, cursor::Goto(1, 1));
    }
    let f = File::open(os.path).unwrap();
    let file = BufReader::new(&f);
    for (num, line) in file.lines().enumerate() {
        let l = line.unwrap();

        if num <= 1 {
            let v: Vec<&str> = l.split_terminator("${c}").collect();
            let mut i = 1;
            println!();
            while i != v.len() {
                if i % 2 > 0 {
                    print!("{}{}", color::Fg(color::Rgb(os.x, os.y, os.z)), v[i - 1]);
                } else {
                    print!("{}{}", color::Fg(color::Rgb(os.xx, os.yy, os.zz)), v[i - 1]);
                };
                i += 1;
            }
            if v.len() > 1 {
                print!("{}{}", color::Fg(color::Rgb(255, 255, 255)), v[i - 1]);
            } else if i % 2 > 0 {
                print!("{}{}", color::Fg(color::Rgb(os.x, os.y, os.z)), v[i - 1]);
            } else {
                print!("{}{}", color::Fg(color::Rgb(os.xx, os.yy, os.zz)), v[i - 1]);
            }
        }

        if num == 2 {
            let v: Vec<&str> = l.split_terminator("${c}").collect();
            let mut i = 1;
            println!();
            while i != v.len() + 1 {
                if i % 2 > 0 {
                    print!("{}{}", color::Fg(color::Rgb(os.x, os.y, os.z)), v[i - 1]);
                } else {
                    print!("{}{}", color::Fg(color::Rgb(os.xx, os.yy, os.zz)), v[i - 1]);
                };
                i += 1;
            }
            print!("{} {}", color::Fg(color::Rgb(255, 255, 255)), fs);
        }

        if num == 3 {
            let v: Vec<&str> = l.split_terminator("${c}").collect();
            let mut i = 1;
            println!();
            while i != v.len() + 1 {
                if i % 2 > 0 {
                    print!("{}{}", color::Fg(color::Rgb(os.x, os.y, os.z)), v[i - 1]);
                } else {
                    print!("{}{}", color::Fg(color::Rgb(os.xx, os.yy, os.zz)), v[i - 1]);
                };
                i += 1;
            }
            print!("{} {}", color::Fg(color::Rgb(255, 255, 255)), version);
        }

        if num > 3 {
            let v: Vec<&str> = l.split_terminator("${c}").collect();
            let mut i = 1;
            println!();
            while i != v.len() {
                if i % 2 > 0 {
                    print!("{}{}", color::Fg(color::Rgb(os.x, os.y, os.z)), v[i - 1]);
                } else {
                    print!("{}{}", color::Fg(color::Rgb(os.xx, os.yy, os.zz)), v[i - 1]);
                };
                i += 1;
            }
            if v.len() > 1 {
                print!("{}{}", color::Fg(color::Rgb(255, 255, 255)), v[i - 1]);
            } else if i % 2 > 0 {
                print!("{}{}", color::Fg(color::Rgb(os.x, os.y, os.z)), v[i - 1]);
            } else {
                print!("{}{}", color::Fg(color::Rgb(os.xx, os.yy, os.zz)), v[i - 1]);
            }
        }
    }
    if !test {
        print!("\n{}", color::Fg(color::Reset));
    }
}
