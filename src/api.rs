#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::OpenOptions;
use std::os::raw::c_char;
use std::path::PathBuf;
use std::{ptr, slice};

use anyhow::Result;
use cxxabi::cxxabi;
use fnlog::fn_debug;
use log::error;
use once_cell::sync::Lazy;
use thiscall::{get_this_ptr_cxx, set_this_ptr_cxx};
use ustr::Ustr;
use widestring::{U16CStr, U16CString};

use crate::global::CONFIG;
use crate::helpers::alloc::alloc;
use crate::helpers::manifest::{read_manifest, write_manifest};
use crate::helpers::save::{get_save_path, get_saves, read_save, remove_save, write_save};
use crate::models::manifest::Save;
use crate::types::{
    IGetLoginDetailsListener, IGetSavegameListListener, IGetSavegameReaderListener,
    IGetSavegameWriterListener, IRemoveSavegameListener, ISavegameReadListener,
    ISavegameWriteListener, OrbitClient, SavegameInfo, SavegameReader, SavegameWriter,
};

static ACCOUNT_ID: Lazy<Ustr> = Lazy::new(|| Ustr::from(&CONFIG.orbit.profile.account_id.as_str()));
static USERNAME: Lazy<Ustr> = Lazy::new(|| Ustr::from(&CONFIG.orbit.profile.username.as_str()));
static PASSWORD: Lazy<Ustr> = Lazy::new(|| Ustr::from(&CONFIG.orbit.profile.password.as_str()));

macro_rules! thiscall {
    ($ty: expr, $body: expr) => {
        set_this_ptr_cxx($ty as u32);
        $body
    };
}

#[inline(never)]
#[cxxabi(name = "??0OrbitClient@orbitclient@mg@@QAE@XZ", ctor = true)]
fn orbit_client_ctor() -> *const OrbitClient {
    fn_debug!("__CALL__");
    alloc(OrbitClient::default())
}

#[inline(never)]
#[cxxabi(
    name = "?StartProcess@OrbitClient@orbitclient@mg@@QAEXPAG00@Z",
    ctor = false
)]
fn orbit_client_start_process(
    client: *const OrbitClient,
    unk0: *const u16,
    unk1: *const u16,
    unk2: *const u16,
) {
    fn_debug!("__CALL__");
}

#[inline(never)]
#[cxxabi(
    name = "?StartLauncher@OrbitClient@orbitclient@mg@@QAE_NIIPBD0@Z",
    ctor = false
)]
fn orbit_client_start_launcher(
    client: *const OrbitClient,
    unk0: u32,
    unk1: u32,
    unk2: *const c_char,
    unk3: *const c_char,
) -> bool {
    fn_debug!("__CALL__");
    return false;
}

#[inline(never)]
#[cxxabi(
    name = "?GetSavegameList@OrbitClient@orbitclient@mg@@QAEXIPAVIGetSavegameListListener@23@I@Z",
    ctor = false
)]
fn orbit_client_get_savegame_list(
    client: *mut OrbitClient,
    request_id: u32,
    savegame_list_listener_callback: *const IGetSavegameListListener,
    product_id: u32,
) {
    fn_debug!("__CALL__");

    let callback = unsafe { (*savegame_list_listener_callback).callback };

    if callback.is_null() {
        return;
    }

    let result = || -> Result<Vec<Box<SavegameInfo>>> {
        let saves = get_saves()?;
        let mut save_info_list = Vec::new();

        for (id, name, size) in saves {
            let size = size as u32;
            let u16name = U16CString::from_str(name)?;

            save_info_list.push(Box::new(SavegameInfo {
                id,
                size,
                name: u16name,
            }));
        }

        Ok(save_info_list)
    }();

    match result {
        Ok(list) => unsafe {
            let saves = list.as_ptr();
            let size = list.len() as u32;

            if size == 0 {
                thiscall!(savegame_list_listener_callback, {
                    (*callback)(request_id, ptr::null(), 0);
                });
            } else {
                thiscall!(savegame_list_listener_callback, {
                    (*callback)(request_id, saves, size as u32);
                });
            }
        },
        Err(err) => error!("{}", err),
    }
}

#[inline(never)]
#[cxxabi(
    name = "?GetSavegameWriter@OrbitClient@orbitclient@mg@@QAEXIPAVIGetSavegameWriterListener@23@II_N@Z",
    ctor = false
)]
fn orbit_client_get_savegame_writer(
    client: *mut OrbitClient,
    request_id: u32,
    savegame_writer_listener_callback: *const IGetSavegameWriterListener,
    product_id: u32,
    save_game_id: u32,
    open: bool,
) {
    fn_debug!("__CALL__");

    let callback = unsafe { (*savegame_writer_listener_callback).callback };

    if callback.is_null() {
        return;
    }

    let result = (|| -> Result<PathBuf> {
        let path = get_save_path(save_game_id)?;
        Ok(path)
    })();

    match result {
        Ok(file) => unsafe {
            let client = &mut (*client);
            let options = if open {
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .to_owned()
            } else {
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .to_owned()
            };
            let writer = Box::new(SavegameWriter::new(save_game_id, file, options));

            client.savegame_writer = writer;

            thiscall!(savegame_writer_listener_callback, {
                (*callback)(request_id, 0, client.savegame_writer.as_ref());
            });
        },
        Err(err) => error!("{}", err),
    }
}

#[inline(never)]
#[cxxabi(
    name = "?GetSavegameReader@OrbitClient@orbitclient@mg@@QAEXIPAVIGetSavegameReaderListener@23@II@Z",
    ctor = false
)]
fn orbit_client_get_savegame_reader(
    client: *mut OrbitClient,
    request_id: u32,
    savegame_reader_listener_callback: *const IGetSavegameReaderListener,
    product_id: u32,
    save_game_id: u32,
) {
    fn_debug!("__CALL__");

    let callback = unsafe { (*savegame_reader_listener_callback).callback };

    if callback.is_null() {
        return;
    }

    let result = (|| -> Result<PathBuf> {
        let path = get_save_path(save_game_id)?;
        Ok(path)
    })();

    match result {
        Ok(file) => unsafe {
            let client = &mut (*client);
            let reader = Box::new(SavegameReader::new(file));

            client.savegame_reader = reader;

            thiscall!(savegame_reader_listener_callback, {
                (*callback)(request_id, 0, client.savegame_reader.as_ref());
            });
        },
        Err(err) => error!("{}", err),
    }
}

#[inline(never)]
#[cxxabi(
    name = "?RemoveSavegame@OrbitClient@orbitclient@mg@@QAEXIPAVIRemoveSavegameListener@23@II@Z",
    ctor = false
)]
fn orbit_client_remove_savegame(
    client: *const OrbitClient,
    request_id: u32,
    remove_savegame_listener_callback: *const IRemoveSavegameListener,
    product_id: u32,
    save_game_id: u32,
) {
    fn_debug!("__CALL__");

    let callback = unsafe { (*remove_savegame_listener_callback).callback };

    if callback.is_null() {
        return;
    }

    let result = (|| -> Result<()> {
        remove_save(save_game_id)?;
        Ok(())
    })();

    match result {
        Ok(_) => unsafe {
            thiscall!(remove_savegame_listener_callback, {
                (*callback)(request_id, true);
            });
        },
        Err(err) => error!("{}", err),
    }
}

#[inline(never)]
#[cxxabi(
    name = "?GetLoginDetails@OrbitClient@orbitclient@mg@@QAEXIPAVIGetLoginDetailsListener@23@@Z",
    ctor = false
)]
fn orbit_client_get_login_details(
    client: *const OrbitClient,
    request_id: u32,
    login_details_listener_callback: *const IGetLoginDetailsListener,
) {
    fn_debug!("__CALL__");

    let callback = unsafe { (*login_details_listener_callback).callback };

    if callback.is_null() {
        return;
    }

    unsafe {
        let account_id = ACCOUNT_ID.as_ptr();
        let username = USERNAME.as_ptr();
        let password = PASSWORD.as_ptr();

        thiscall!(login_details_listener_callback, {
            (*callback)(
                request_id,
                account_id as *const i8,
                username as *const i8,
                password as *const i8,
            );
        });
    }
}

#[inline(never)]
#[cxxabi(
    name = "?GetRequestUniqueId@OrbitClient@orbitclient@mg@@QAEIXZ",
    ctor = false
)]
fn orbit_client_get_request_unique_id(client: *mut OrbitClient) -> u32 {
    fn_debug!("__CALL__");

    unsafe {
        return (*client).get_next_request_id();
    }
}

#[inline(never)]
#[cxxabi(
    name = "?GetInstallationErrorNum@OrbitClient@orbitclient@mg@@QAEIXZ",
    ctor = false
)]
fn orbit_client_get_installation_error_string(client: *const OrbitClient) -> u16 {
    fn_debug!("__CALL__");
    return 0;
}

#[inline(never)]
#[cxxabi(
    name = "?GetInstallationErrorString@OrbitClient@orbitclient@mg@@QAEPAGPBD@Z",
    ctor = false
)]
fn orbit_client_get_installation_error_num(client: *const OrbitClient) -> *const u16 {
    fn_debug!("__CALL__");
    return ptr::null();
}

#[inline(never)]
#[cxxabi(name = "?Update@OrbitClient@orbitclient@mg@@QAEXXZ", ctor = false)]
fn orbit_client_update(client: *const OrbitClient) {
    fn_debug!("__CALL__");
}

#[inline(never)]
#[cxxabi(name = "??1OrbitClient@orbitclient@mg@@QAE@XZ", ctor = false)]
fn orbit_client_dtor(client: *mut OrbitClient) {
    fn_debug!("__CALL__");

    unsafe {
        Box::from_raw(client);
    }
}

#[inline(never)]
#[cxxabi(
    name = "?GetSavegameId@SavegameInfo@orbitclient@mg@@QAEIXZ",
    ctor = false
)]
fn savegame_info_get_savegame_id(save_game_info: *const Box<SavegameInfo>) -> u32 {
    fn_debug!("__CALL__");

    unsafe {
        return (*save_game_info).id;
    }
}

#[inline(never)]
#[cxxabi(name = "?GetSize@SavegameInfo@orbitclient@mg@@QAEIXZ", ctor = false)]
fn savegame_info_get_size(save_game_info: *const Box<SavegameInfo>) -> u32 {
    fn_debug!("__CALL__");

    unsafe {
        return (*save_game_info).size;
    }
}

#[inline(never)]
#[cxxabi(name = "?GetName@SavegameInfo@orbitclient@mg@@QAEPBGXZ", ctor = false)]
fn savegame_info_get_name(save_game_info: *const Box<SavegameInfo>) -> *const u16 {
    fn_debug!("{:#?}", unsafe { &(*save_game_info) });

    unsafe {
        return (*save_game_info).name.as_ptr();
    }
}

#[inline(never)]
#[cxxabi(
    name = "?Read@SavegameReader@orbitclient@mg@@QAEXIPAVISavegameReadListener@23@IPAXI@Z",
    ctor = false
)]
fn savegame_reader_read(
    save_game_reader: *const SavegameReader,
    request_id: u32,
    savegame_read_listener_callback: *const ISavegameReadListener,
    offset: u32,
    buffer: *mut c_char,
    number_of_bytes: u32,
) {
    fn_debug!("{:#?}", unsafe { &(*save_game_reader) });

    let callback = unsafe { (*savegame_read_listener_callback).callback };

    if callback.is_null() {
        return;
    }

    let reader = unsafe { &(*save_game_reader) };

    let result = (|| -> Result<(Vec<u8>, usize)> {
        let (data, size) = read_save(&reader.path, number_of_bytes as usize, offset as u64)?;
        Ok((data, size))
    })();

    match result {
        Ok((data, size)) => unsafe {
            ptr::copy(data.as_ptr() as *const c_char, buffer, size);

            thiscall!(savegame_read_listener_callback, {
                (*callback)(request_id, size as u32);
            });
        },
        Err(err) => error!("{}", err),
    }
}

#[inline(never)]
#[cxxabi(name = "?Close@SavegameReader@orbitclient@mg@@QAEXXZ", ctor = false)]
fn savegame_reader_close(save_game_reader: *const SavegameReader) {
    fn_debug!("__CALL__");
}

#[inline(never)]
#[cxxabi(
    name = "?Write@SavegameWriter@orbitclient@mg@@QAEXIPAVISavegameWriteListener@23@PAXI@Z",
    ctor = false
)]
fn savegame_writer_write(
    save_game_writer: *const SavegameWriter,
    request_id: u32,
    savegame_write_listener_callback: *const ISavegameWriteListener,
    buffer: *const c_char,
    number_of_bytes: u32,
) {
    fn_debug!("{:#?}", unsafe { &(*save_game_writer) });

    let callback = unsafe { (*savegame_write_listener_callback).callback };

    if callback.is_null() {
        return;
    }

    let writer = unsafe { &(*save_game_writer) };

    let result = (|| -> Result<()> {
        let buffer =
            unsafe { slice::from_raw_parts(buffer as *const u8, number_of_bytes as usize) };
        let _ = write_save(&writer.path, &writer.options, buffer)?;
        Ok(())
    })();

    match result {
        Ok(_) => unsafe {
            thiscall!(savegame_write_listener_callback, {
                (*callback)(request_id, number_of_bytes as u32);
            });
        },
        Err(err) => error!("{}", err),
    }
}

#[inline(never)]
#[cxxabi(
    name = "?SetName@SavegameWriter@orbitclient@mg@@QAE_NPAG@Z",
    ctor = false
)]
fn savegame_writer_set_name(save_game_writer: *const SavegameWriter, name: *const u16) -> bool {
    fn_debug!("{:#?}", unsafe { &(*save_game_writer) });

    let writer = unsafe { &(*save_game_writer) };

    let result = (|| -> Result<()> {
        let u16str = unsafe { U16CStr::from_ptr_str(name) };
        let u16name = u16str.to_string()?;

        let mut manifest = read_manifest().unwrap_or_default();

        match manifest.saves.iter_mut().find(|save| save.id == writer.id) {
            Some(save) => {
                save.name = u16name;
            }
            None => manifest.saves.push(Save {
                id: writer.id,
                name: u16name,
            }),
        }

        let _ = write_manifest(&manifest)?;
        Ok(())
    })();

    match result {
        Ok(_) => return true,
        Err(err) => error!("{}", err),
    }

    return false;
}

#[inline(never)]
#[cxxabi(name = "?Close@SavegameWriter@orbitclient@mg@@QAEX_N@Z", ctor = false)]
fn savegame_writer_close(save_game_writer: *const SavegameWriter) {
    fn_debug!("__CALL__");
}
