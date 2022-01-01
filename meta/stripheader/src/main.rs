use std::env;
use std::ffi::OsString;
use std::fs::{self, FileType};
use std::fs::File;
use std::path::Path;
use std::io::{Read, Seek, SeekFrom, BufReader, copy};
use anyhow::Result;
use tempfile::NamedTempFile;
use walkdir::WalkDir;

const NES : u32 = 0x4e45531a; // NES<EOF>
const FDS : u32 = 0x4644531a; // FDS<EOF>
const UNIF : u32 = 0x554e4946; // UNIF
const LYNX: u32 = 0x4C594E58; // LYNX
const LYNX2: [u8; 3] = [0x42, 0x53, 0x39];
const A7800 : [u8; 0x9] = [0x41, 0x54, 0x41, 0x52, 0x49, 0x37, 0x38, 0x30,0x30]; // ATARI7800

const NES_HEADER_SIZE: u64 = 0x10;
const A7800_HEADER_SIZE: u64 = 0x80;
const LYNX_HEADER_SIZE: u64 = 0x40;
fn strip(mut f: File, size: u64, original_name: &dyn AsRef<Path>) -> Result<()> {
    println!("stripping header of size {} from {}", size, original_name.as_ref().as_os_str().to_string_lossy());
    f.seek(SeekFrom::Start(size))?;
    let filelen = f.metadata()?.len();
    let mut bufread = BufReader::new(&mut f);

    let mut output = NamedTempFile::new_in(".")?;
    let result = copy(&mut bufread, &mut output)?;
    if result != filelen - size {
        return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "mismatched file length").into())
    }

    // close the file handle
    drop(f);

    let mut target = OsString::from(original_name.as_ref().as_os_str());
    target.push(".bak");
    fs::rename(original_name, &target)?;
    output.persist(original_name)?;
    Ok(())
}

fn main() -> Result<()> {
    for entry in WalkDir::new(".").min_depth(1).max_depth(1) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path();
            if path.ends_with(".bak") {
                continue;
            }
            let mut buf = [0u8; 4];
            let mut file = File::open(&path)?;
            file.read_exact(&mut buf)?;
            let header = u32::from_be_bytes(buf);
            match header {
                NES => strip(file, NES_HEADER_SIZE, &path)?,
                FDS => strip(file, NES_HEADER_SIZE, &path)?,
                LYNX => strip(file, LYNX_HEADER_SIZE, &path)?,
                _ => {
                    check_atari(file, &path)?;
                }
            }
        }
    }
    Ok(())
}

fn check_atari(mut file: File, path: &Path) -> Result<()> {
    // a7800
    file.seek(SeekFrom::Start(1))?;
    let mut buf = [0u8; 0x9];
    file.read_exact(&mut buf)?;
    if buf == A7800 {
        return strip(file, A7800_HEADER_SIZE, &path)
    } 
    // lynx com
    file.seek(SeekFrom::Start(0x6))?;
    let mut buf = [0u8; 0x3];
    file.read_exact(&mut buf)?;
    if buf == LYNX2 {
        return strip(file, LYNX_HEADER_SIZE, &path)
    }
    Ok(())
}