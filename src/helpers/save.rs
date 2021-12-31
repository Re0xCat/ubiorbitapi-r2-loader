use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::{Path, PathBuf};
use std::{env, fs};

use anyhow::{anyhow, Result};
use fs::OpenOptions;
use if_chain::if_chain;

use super::manifest::{get_manifest_path, read_manifest, write_manifest};
use crate::consts::SAVE_FILE_EXTENSION;
use crate::global::CONFIG;
use crate::models::manifest::Manifest;

#[inline]
pub fn get_saves_path() -> Result<PathBuf> {
    let path = match CONFIG.orbit.saves.as_str() {
        "<default>" => env::current_dir()?.join("Saves"),
        "<roaming>" => dirs::config_dir()
            .ok_or_else(|| anyhow!("Unknown config dir!"))?
            .join("OrbitEmu")
            .join(&CONFIG.orbit.name)
            .join("Saves"),
        _ => PathBuf::from(&CONFIG.orbit.saves),
    };

    Ok(path)
}

#[inline]
pub fn get_save_path(id: u32) -> Result<PathBuf> {
    let file = format!("{}.{}", id, SAVE_FILE_EXTENSION);
    let path = get_saves_path()?.join(file);

    Ok(path)
}

#[inline]
pub fn get_saves() -> Result<Vec<(u32, String, u64)>> {
    let mut saves = Vec::new();

    let saves_path = get_saves_path()?;
    let manifest_path = get_manifest_path()?;

    if !saves_path.exists() || !manifest_path.exists() {
        return Ok(saves);
    }

    let manifest = read_manifest()?;

    for entry in fs::read_dir(&saves_path)? {
        let entry = entry?;
        let path = entry.path();
        let is_file = path.is_file();

        if_chain! {
            if let Some(ext) = path.extension();
            if let Some(ext) = ext.to_str();

            if is_file && ext == SAVE_FILE_EXTENSION;
            if let Some(stem) = path.file_stem();

            let save_id = stem.to_string_lossy().parse::<u32>()?;
            if let Some(save) = manifest.saves.iter().find(|save| save.id == save_id);

            then {
                let id = save.id as u32;
                let name = save.name.clone();
                let size = fs::metadata(&path)?.len();

                saves.push((id, name, size));
            }
        }
    }

    Ok(saves)
}

#[inline]
pub fn read_save(
    path: &Path,
    num_of_bytes_to_read: usize,
    offset: u64,
) -> Result<(Vec<u8>, usize)> {
    let mut file = OpenOptions::new().read(true).open(path)?;

    file.seek(SeekFrom::Start(offset))?;

    let mut buffer = vec![0u8; num_of_bytes_to_read];
    let read_bytes = file.read(&mut buffer)?;

    Ok((buffer, read_bytes))
}

#[inline]
pub fn write_save(path: &Path, options: &OpenOptions, buffer: &[u8]) -> Result<()> {
    if let Some(path) = path.parent() {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
    }

    let mut file = options.open(path)?;
    file.write_all(buffer)?;

    Ok(())
}

#[inline]
pub fn remove_save(id: u32) -> Result<()> {
    let path = get_save_path(id)?;
    let manifest = read_manifest()?;

    fs::remove_file(path)?;

    let saves = manifest
        .saves
        .iter()
        .cloned()
        .filter(|save| save.id != id)
        .collect();

    write_manifest(&Manifest { saves })?;

    Ok(())
}
