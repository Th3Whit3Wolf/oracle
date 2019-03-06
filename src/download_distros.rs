use crate::{
    download::{download_size, links2},
    download_menu::download_menu,
    os::Os,
    work::{art, confirmation, inlp},
};

use select::{document::Document, predicate::Name};

pub fn get_versions(url: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::with_capacity(10);
    let mut z: Vec<String> = Vec::with_capacity(10);
    let resp = reqwest::get(url).expect("No internet bro");
    assert!(resp.status().is_success());

    Document::from_read(resp)
        .unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| v.push(x.to_string()));

    for x in &v {
        if x.ends_with('/') && (x.starts_with('1') || x.starts_with('2') || x.starts_with('3')) {
            let q = str::replace(&x, '/', "");
            z.push(q);
        }
    }
    z
}

pub fn version_as_string(version: &[String]) -> String {
    let join = version.join(", ");
    join.to_string()
}

pub fn version_to_iso(url: &str, version: &str) -> String {
    if url == "http://releases.ubuntu.com/" {
        let string = url.to_string() + "/" + version + "/";
        let out: Vec<String> = links2(&string, ".iso");
        let temp = &out[0].to_string();
        url.to_string() + "/" + version + "/" + &temp
    } else {
        let string = url.to_string() + "/" + version + "/release/";
        let out: Vec<String> = links2(&string, ".iso");
        let temp = &out[0].to_string();
        url.to_string() + "/" + version + "/release/" + &temp
    }
}

pub fn beta_to_iso(url: &str, version: &str) -> String {
    if url == "http://releases.ubuntu.com/" {
        let string = url.to_string() + "/" + version + "/";
        let out: Vec<String> = links2(&string, ".iso");
        let temp = &out[0].to_string();
        url.to_string() + "/" + version + "/" + &temp
    } else {
        let string = url.to_string() + "/" + version + "/beta/";
        let out: Vec<String> = links2(&string, ".iso");
        let temp = &out[0].to_string();
        url.to_string() + "/" + version + "/beta/" + &temp
    }
}

pub fn vers_dl_size(url: &str, version: &str) -> String {
    if url == "http://releases.ubuntu.com/" {
        let string = url.to_string() + "/" + version + "/";
        let out: Vec<String> = links2(&string, ".iso");
        let temp = out[0].to_string();
        let temp = url.to_string() + "/" + version + "/" + &temp;
        let size = download_size(&temp).expect("No Internets");
        let sizes = size.to_string();
        if size < 1000 {
            sizes + " MB"
        } else {
            "".to_string() + &sizes[..1] + "," + &sizes[1..4] + " MB"
        }
    } else {
        let string = url.to_string() + "/" + version + "/release/";
        let out: Vec<String> = links2(&string, ".iso");
        let temp = out[0].to_string();
        let temp = url.to_string() + "/" + version + "/release/" + &temp;
        let size = download_size(&temp).expect("No Internets");
        let sizes = size.to_string();
        if size < 1000 {
            sizes + " MB"
        } else {
            "".to_string() + &sizes[..1] + "," + &sizes[1..4] + " MB"
        }
    }
}

pub fn beta_dl_size(url: &str, version: &str) -> String {
    if url == "http://releases.ubuntu.com/" {
        let string = url.to_string() + "/" + version + "/";
        let out: Vec<String> = links2(&string, ".iso");
        let temp = out[0].to_string();
        let temp = url.to_string() + "/" + version + "/" + &temp;
        let size = download_size(&temp).expect("No Internets");
        let sizes = size.to_string();
        if size < 1000 {
            sizes + " MB"
        } else {
            "".to_string() + &sizes[..1] + "," + &sizes[1..4] + " MB"
        }
    } else {
        let string = url.to_string() + "/" + version + "/beta/";
        let out: Vec<String> = links2(&string, ".iso");
        let temp = out[0].to_string();
        let temp = url.to_string() + "/" + version + "/beta/" + &temp;
        let size = download_size(&temp).expect("No Internets");
        let sizes = size.to_string();
        if size < 1000 {
            sizes + " MB"
        } else {
            "".to_string() + &sizes[..1] + "," + &sizes[1..4] + " MB"
        }
    }
}

pub fn download_ubuntu(url: &str, os: &Os, neofetch: &str, array: &[Os; 42]) {
    let vers_list = get_versions(&url);
    let version = version_as_string(&get_versions(&url));
    let fs = "N/A";
    art(os, &version, fs, false);
    if vers_list.len() > 1 {
        println!("What version would you like to download?");

        let mut q = 0;
        while q != vers_list.len() {
            println!("{}.) {}", q + 1, vers_list[q]);
            q += 1;
        }
        println!(
            "{}.) Go Back\n{}.) Refresh",
            vers_list.len() + 1,
            vers_list.len() + 2
        );

        let input = inlp();

        // Expected Value
        if input > 0 && input < vers_list.len() + 1 {
            let size = vers_dl_size(url, &vers_list[input - 1]);
            let download_url = version_to_iso(url, &vers_list[input - 1]);
            art(os, &vers_list[input - 1], &size, false);
            confirmation(
                "Are you sure want to download this version?",
                &download_url,
                &download_ubuntu,
                url,
                os,
                neofetch,
                array,
            );
        }

        if input == vers_list.len() + 1 {
            let size = beta_dl_size(url, &vers_list[input - 1]);
            let download_url = beta_to_iso(url, &vers_list[input - 1]);
            art(os, &vers_list[input - 1], &size, false);
            confirmation(
                "Are you sure want to download this version?",
                &download_url,
                &download_ubuntu,
                url,
                os,
                neofetch,
                array,
            );
        }

        // Out of bounds
        if input < 1 || input > vers_list.len() + 2 {
            println!("That's not an option");
        }

        // Return to previous menu
        if input == vers_list.len() + 1 {
            download_menu(neofetch, array);
        }

        // Refresh menu
        if input == vers_list.len() + 2 {
            download_ubuntu(url, os, neofetch, array);
        }
    } else {
        let size = vers_dl_size(url, &vers_list[0]);
        art(os, &vers_list[0], &size, false);
    }
}
