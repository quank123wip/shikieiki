use std::fs::File;
use std::io::{Read, Seek};


pub fn find_blankless_end (mut f: File) -> std::io::Result<u64>{
    let len = f.seek(std::io::SeekFrom::End(0))?;

    const CHUNK_SIZE: u32 = 10240;

    let mut cnt = 1;
    

    Ok(loop {
        let s = if len < (CHUNK_SIZE * cnt) as u64 { 0 } else { len - (CHUNK_SIZE * cnt) as u64 };
        let mut buffer = String::new();
        let k = f.by_ref();
        k.seek(std::io::SeekFrom::Start(s)).unwrap();
        k.take(CHUNK_SIZE as u64).read_to_string(&mut buffer).unwrap();
        let mut flag = true;
        let mut pos = 0;
        for (i, v) in buffer.chars().rev().enumerate() {
            if v != ' ' && v != '\n' {
                flag = false;
                pos = i;
                break;
            }
        }
        if flag {
            cnt += 1;
        } else {
            break len - (CHUNK_SIZE * (cnt - 1)) as u64 - pos as u64;
        }
    })
}