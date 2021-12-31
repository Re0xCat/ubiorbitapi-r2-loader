use std::fs::OpenOptions;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_char;
use std::path::PathBuf;

use derive_new::new;
use widestring::U16CString;

#[derive(Debug, Default)]
pub struct Inner {
    pub request_id: u32,
    pub savegame_reader: Box<SavegameReader>,
    pub savegame_writer: Box<SavegameWriter>,
}

#[derive(Debug, Default)]
pub struct OrbitClient {
    inner: Box<Inner>,
}

impl OrbitClient {
    pub fn get_next_request_id(&mut self) -> u32 {
        self.inner.request_id += 1;
        self.inner.request_id
    }
}

impl Deref for OrbitClient {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for OrbitClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug, new)]
pub struct SavegameInfo {
    pub id: u32,
    pub size: u32,
    pub name: U16CString,
}

#[derive(Debug, Default, new)]
pub struct SavegameReader {
    pub path: PathBuf,
}

#[derive(Debug, new)]
pub struct SavegameWriter {
    pub id: u32,
    pub path: PathBuf,
    pub options: OpenOptions,
}

impl Default for SavegameWriter {
    fn default() -> Self {
        Self {
            id: 0,
            path: Default::default(),
            options: OpenOptions::new(),
        }
    }
}

pub struct IGetSavegameListListener {
    pub callback: *const extern "stdcall" fn(
        request_id: u32,
        savegame_info_list: *const Box<SavegameInfo>,
        list_size: u32,
    ),
}

pub struct IGetSavegameWriterListener {
    pub callback: *const extern "stdcall" fn(
        request_id: u32,
        unk: u32,
        savegame_writer: *const SavegameWriter,
    ),
}

pub struct IGetSavegameReaderListener {
    pub callback: *const extern "stdcall" fn(
        request_id: u32,
        unk: u32,
        savegame_reader: *const SavegameReader,
    ),
}

pub struct IRemoveSavegameListener {
    pub callback: *const extern "stdcall" fn(request_id: u32, removed: bool),
}

pub struct IGetLoginDetailsListener {
    pub callback: *const extern "stdcall" fn(
        request_id: u32,
        account_id: *const c_char,
        username: *const c_char,
        password: *const c_char,
    ),
}

pub struct ISavegameReadListener {
    pub callback: *const extern "stdcall" fn(request_id: u32, bytes_read: u32),
}

pub struct ISavegameWriteListener {
    pub callback: *const extern "stdcall" fn(request_id: u32, bytes_written: u32),
}
