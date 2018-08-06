//use std::path::Path;
//use std::{thread, time};
//use std::sync::mpsc::{self, TryRecvError};
use std::fs;

use std::time::SystemTime;

//extern crate fs_extra;
//use fs_extra::dir::*;
//use fs_extra::error::*;

/*fn example_copy() -> Result<()> {
    let path_from = Path::new("./temp");
    let path_to = path_from.join("out");
    let test_folder = path_from.join("test_folder");
    let dir = test_folder.join("dir");
    let sub = dir.join("sub");
    let file1 = dir.join("file1.txt");
    let file2 = sub.join("file2.txt");

    //create_all(&sub, true)?;
    //create_all(&path_to, true)?;
    fs_extra::file::write_all(&file1, "content1")?;

    assert!(dir.exists());
    assert!(sub.exists());
    assert!(file1.exists());
    assert!(file2.exists());

    let mut options = CopyOptions::new();
    options.buffer_size = 10240000;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let handler = |process_info: TransitProcess| {
            tx.send(process_info).unwrap();
            //thread::sleep(time::Duration::from_millis(500));
            fs_extra::dir::TransitProcessResult::ContinueOrAbort
        };
        copy_with_progress(&test_folder, &path_to, &options, handler).unwrap();
    });

    loop {
        match rx.try_recv() {
            Ok(process_info) => {
                println!(
                    "{} of {} bytes",
                    process_info.copied_bytes, process_info.total_bytes
                );
            }
            Err(TryRecvError::Disconnected) => {
                println!("finished");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    }
    Ok(())
}*/

fn fs_object_age(path: &str) -> u64 {
    const SECS_IN_DAY: u64 = 86400;
    let ppath = std::path::Path::new(path);
    //example_copy();
    let metadata = fs::metadata(ppath)
        .expect("Err: Creation timestamp is not supported on this platform. Aborting!");
    let time = metadata.created().unwrap();
    match SystemTime::now().duration_since(time) {
        Ok(n) => return (n.as_secs() / SECS_IN_DAY) as u64,
        Err(_) => panic!("Err: Cant calculate age of file. Aborting!"),
    }
}

fn iterate_dir(suffix: &str) /*-> io::Result<Vec<PathBuf>>*/
{
    const WRK_PATH: &str = "./temp/test_folder"; //change to "./" for prod!!!

    let path = fs::read_dir(&WRK_PATH).expect("Err: Cant read drectory contents. Aborting!");
    path.filter_map(Result::ok)
        .filter_map(|d| {
            d.path().to_str().and_then(|f| {
                if (f.ends_with(suffix) && (!f.ends_with("exe"))) {
                    Some(d)
                } else {
                    None
                }
            })
        })
        .for_each(|f| {
            println!(
                "{:?}{:?}",
                f.path(),
                fs_object_age(f.path().to_str().unwrap())
            )
        });

    //for path in paths {
    //    println!("Name: {}", path.unwrap().path().display())
    // }
}

fn main() {
    //example_copy();
    //println!("{}", fs_object_age("./temp/test_folder/dir/sub/file2.txt"));
    iterate_dir("");
}
