use crate::error::Error;
use std::ffi::OsStr;
use std::io::Cursor;
use std::path::PathBuf;
use std::{fs, io};

pub async fn download_css_archive(url: &str, css_path: &str) -> Result<(), Error> {
    let extract_path = PathBuf::from(css_path);
    let response = reqwest::get(url).await?.error_for_status()?;
    let content = Cursor::new(response.bytes().await?);
    let mut archive = zip::ZipArchive::new(content)?;
    let mut selected: Vec<_> = Vec::new();

    for n in archive.file_names() {
        let path = PathBuf::from(n);
        let extension = path.extension().unwrap_or_default();
        let file_stem = path
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        if extension.eq(OsStr::new("css"))
            && file_stem.starts_with("pico.classless")
            && !file_stem.contains("conditional")
            && file_stem.contains("min")
        {
            println!("{}", path.to_string_lossy());
            selected.push(path);
        }
    }

    if !extract_path.is_dir() {
        fs::create_dir(extract_path.as_path()).expect("could not create directory")
    }

    for path in selected {
        let filename = path.file_name().unwrap().to_str().unwrap();
        let out_path = extract_path.join(filename);
        let index = archive
            .index_for_path(path)
            .expect("unable to get index for path");
        let mut data = archive.by_index(index)?;
        let mut out_file = fs::File::create(out_path)?;
        io::copy(&mut data, &mut out_file)?;
    }

    Ok(())
}
