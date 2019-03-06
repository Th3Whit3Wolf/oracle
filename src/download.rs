use crate::{distro::distro_menu, os::Os};

use std::{
    fs,
    io::{self, copy, Read},
    path::Path,
};

use exitfailure::ExitFailure;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Url;
use reqwest::{header, Client};
use select::document::Document;
use select::predicate::Name;

pub struct DownloadProgress<R> {
    inner: R,
    progress_bar: ProgressBar,
}

impl<R: Read> Read for DownloadProgress<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf).map(|n| {
            self.progress_bar.inc(n as u64);
            n
        })
    }
}

pub fn download(iso: &str, neofetch: &str, array: &[Os; 42]) -> Result<(), ExitFailure> {
    let url = Url::parse(iso)?;
    let client = Client::new();

    let total_size = {
        let resp = client.head(url.as_str()).send()?;
        if resp.status().is_success() {
            resp.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0)
        } else {
            return Err(failure::err_msg(format!(
                "Couldn't download URL: {}. Error: {:?}",
                url,
                resp.status(),
            ))
            .into());
        }
    };

    let mut request = client.get(url.as_str());
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
                 .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                 .progress_chars("#>-"));

    let file = Path::new(
        url.path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or("tmp.bin"),
    );

    if file.exists() {
        let size = file.metadata()?.len() - 1;
        request = request.header(header::RANGE, format!("bytes={}-", size));
        pb.inc(size);
    }

    let mut source = DownloadProgress {
        progress_bar: pb,
        inner: request.send()?,
    };

    let mut dest = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file)?;

    let _ = copy(&mut source, &mut dest)?;

    println!(
        "Download of '{}' has been completed.",
        file.to_str().unwrap()
    );
    distro_menu(neofetch, array);

    Ok(())
}

pub fn download_size(iso: &str) -> Result<(u64), ExitFailure> {
    let url = Url::parse(iso)?;
    let client = Client::new();
    let resp = client.head(url.as_str()).send()?;

    let total_size = {
        if resp.status().is_success() {
            resp.headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0)
        } else {
            return Err(failure::err_msg(format!(
                "Couldn't download URL: {}. Error: {:?}",
                url,
                resp.status(),
            ))
            .into());
        }
    };
    let total_size = total_size / 1024 / 1024;
    Ok(total_size)
}

pub fn links2(url: &str, end: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::with_capacity(10);
    let mut z: Vec<String> = Vec::with_capacity(10);
    let resp = reqwest::get(url).expect("No Internets");
    assert!(resp.status().is_success());

    Document::from_read(resp)
        .expect("No Internet")
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| v.push(x.to_string()));
    for x in &v {
        if x.ends_with(end) {
            z.push(x.to_string());
        }
    }
    z
}
