//use std::path::Path;
//use std::{thread, time};
//use std::sync::mpsc::{self, TryRecvError};
use std::fs;

use std::time::SystemTime;

use std::collections::HashMap;
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
        .expect("Err: File metadata is not supported on this platform. Aborting!");
    let time = metadata
        .created()
        .expect("Err: Creation timestamp is not supported on this platform. Aborting!");
    match SystemTime::now().duration_since(time) {
        Ok(n) => return (n.as_secs() / SECS_IN_DAY) as u64,
        Err(_) => panic!("Err: Cant calculate age of file. Aborting!"),
    }
}

fn iterate_dir(
    suffix: &str,
    xuffix: &str,
    xxuffix: &str,
) -> std::collections::HashMap<u64, String> {
    //suffix - directory MUST have this suffix (can be empty -> all dirs are in list except filtered by next params)
    //xuffix & xxuffix - directory MUST NOT have this suffixes, so in resulting list will be all directories mtching suffix except xuffix and xxuffix
    const WRK_PATH: &str = "./temp/test_folder"; //change to "./" for prod!!!

    let mut result = HashMap::new();

    let path = fs::read_dir(&WRK_PATH).expect("Err: Cant read drectory contents. Aborting!");
    path.filter_map(Result::ok)
        .filter_map(|e| {
            if e.path()
                .metadata()
                .expect("Err: File metadata is not supported on this platform. Aborting!")
                .file_type()
                .is_dir()
            {
                Some(e)
            } else {
                None
            }
        })
        .filter_map(|d| {
            d.path().to_str().and_then(|f| {
                if f.ends_with(suffix) && (!f.ends_with(".exe")) && (!f.ends_with(xuffix))
                    && (!f.ends_with(xxuffix))
                {
                    Some(d)
                } else {
                    None
                }
            })
        })
        .for_each(|f| {
            //            println!(
            //                "{:?}{:?}",
            //                f.path(),
            //                fs_object_age(f.path().to_str().unwrap())
            //            );
            result.insert(
                fs_object_age(f.path().to_str().unwrap()),
                f.path().into_os_string().into_string().unwrap(),
            );
        });

    return result;
}

fn main() {
    //    for (key, value) in iterate_dir(
    //        "__month",
    //        "$dummy$_$that$_$never$_$can$_$be$_$met$",
    //        "$dummy$_$that$_$never$_$can$_$be$_$met$",
    //    ) {
    //        println!("{}: {}", key, value);
    //    }
    //
    //    println!("----------------------");
    //    for (key, value) in iterate_dir("", "5", "4").iter().min() {
    //        println!("{}: {}", key, value);
    //    }
    //

    //
    //
    //Find most recent file without marks
    //
    //
    let mut ddayslast: u64 = std::u64::MAX;
    let mut dfilepath: String = "".to_string();

    for (key, value) in iterate_dir("", "_month", "_week").iter().min() {
        ddayslast = *key;
        dfilepath = value.clone().to_string();
    }
    if ddayslast != 0 {
        panic!(
            "Err: last daily created {} days ago.  Exitting...",
            ddayslast
        );
    }
    println!("{} {}", ddayslast, dfilepath);
    //
    //
    //Found OK
    //Let's find latest file with mark month
    //
    //
    let mut mdayslast: u64 = std::u64::MAX;
    let mut mfilepath: String = "".to_string();

    for (key, value) in iterate_dir("_month", "_week", "$dummy$_$that$_$never$_$can$_$be$_$mEt$")
        .iter()
        .min()
    {
        mdayslast = *key;
        mfilepath = value.clone().to_string();
    }
    //
    //
    //Let's find latest file with mark week
    //
    //
    let mut wdayslast: u64 = std::u64::MAX;
    let mut wfilepath: String = "".to_string();

    for (key, value) in iterate_dir("_week", "_month", "$dummy$_$that$_$never$_$can$_$be$_$mEt$")
        .iter()
        .min()
    {
        wdayslast = *key;
        wfilepath = value.clone().to_string();
    }

    //If monthly file not found (date not MAX) _OR_ found and it is older than 31 day
    if (mdayslast == std::u64::MAX) || (mdayslast >= 31) {
        //if weekly file found
        if wdayslast != std::u64::MAX {
            //rename weekly file to month
            //
            println!(
                "Renaming {} to {} !",
                &wfilepath,
                &format!("{}{}", wfilepath, "__month")
            );
            fs::rename(&wfilepath, &format!("{}{}", wfilepath, "__month"))
                .expect("Cannot rename dir!");
        } else {
            //rename daily file to month
            //
            println!(
                "Renaming {} to {} !",
                &dfilepath,
                &format!("{}{}", dfilepath, "__month")
            );
            fs::rename(&dfilepath, &format!("{}{}", dfilepath, "__month"))
                .expect("Cannot rename dir!");
        }
    }

    //
    //
    //Find most recent file without marks once again
    //
    //
    let mut ddayslast: u64 = std::u64::MAX;
    let mut dfilepath: String = "".to_string();

    for (key, value) in iterate_dir("", "_month", "_week").iter().min() {
        ddayslast = *key;
        dfilepath = value.clone().to_string();
    }

    //
    //
    //Let's find latest file with mark week once again
    //
    //
    let mut wdayslast: u64 = std::u64::MAX;
    let mut wfilepath: String = "".to_string();

    for (key, value) in iterate_dir("_week", "_month", "$dummy$_$that$_$never$_$can$_$be$_$mEt$")
        .iter()
        .min()
    {
        wdayslast = *key;
        wfilepath = value.clone().to_string();
    }

    //If weekly file not found (date not MAX) _OR_ found and it is older than 7 day
    if (wdayslast == std::u64::MAX) || (wdayslast >= 7) {
        //if weekly file found
        if ddayslast != std::u64::MAX {
            //rename daily file to week
            //
            println!(
                "Renaming {} to {} !",
                &dfilepath,
                &format!("{}{}", dfilepath, "__week")
            );
            fs::rename(&dfilepath, &format!("{}{}", dfilepath, "__week"))
                .expect("Cannot rename dir!");
        } else {
            //panic with no more daily copies to store
            //
            println!("NO DAY COPIES!")
        }
    }

    //if dayslast != 0 {
    //    panic!(
    //        "Err: last daily created {} days ago.  Exitting...",
    //        dayslast
    //    );
    //}
    println!("{} {}", mdayslast, mfilepath);
}
