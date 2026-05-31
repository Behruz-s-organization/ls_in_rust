mod decode;

use decode::{format_permissions, format_date};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::{fs};
use owo_colors::OwoColorize;
use clap::{Parser};
use uzers::{get_user_by_uid, get_group_by_gid};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command()]
    file_name: Option<String>,

    #[arg(short, long)]
    all: bool,

    #[arg(short, long)]
    list: bool,
}

struct FileInfo {
    name: String,
    permissions: String,
    // links: u64,
    size: u64,
    date: String,
    owner: String,
    owner_group: String,
    is_dir: bool,
    nlink: u64,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let file_name = args.file_name.unwrap_or(String::from("."));
    let all = args.all;
    let list = args.list;

    for entry in fs::read_dir(file_name)? {
        let dir = entry?;
        let file_name = dir.file_name();
        let name = file_name.to_string_lossy();
        let metadata = dir.metadata()?;

        let file_info = FileInfo {
            name: name.to_string(),
            permissions: format_permissions(metadata.permissions().mode()),
            date: format_date(metadata.created()?),
            size: metadata.size(),
            owner: match get_user_by_uid(metadata.uid()) {
                Some(user) => user.name().to_string_lossy().to_string(),
                None => String::from("No user")
            },
            owner_group: match get_group_by_gid(metadata.gid()) {
                Some(group) => group.name().to_string_lossy().to_string(),
                None => String::from("No group")
            },
            is_dir: metadata.is_dir(),
            nlink: metadata.nlink(),
        };

        match file_info.is_dir {
            true => print_folder(file_info, all, list),
            false => print_file(file_info, all, list),
        }
    };
    Ok(())
}


fn print_folder(file_info: FileInfo, all: bool, list: bool) {
    if list && !all {
        if file_info.name.starts_with(".") {
            return
        } else {
            println!(
                "{:<10} {} {:<8} {:<8} {:>6} {} {}/",
                file_info.permissions,
                file_info.nlink,
                file_info.owner,
                file_info.owner_group,
                file_info.size,
                file_info.date,
                file_info.name.blue().bold()
            )
        }
    } else if all && !list{
        println!(
            "{:<10} {} {:<8} {:<8} {:>6} {} {}/",
            file_info.permissions,
            file_info.nlink,
            file_info.owner,
            file_info.owner_group,
            file_info.size,
            file_info.date,
            file_info.name.blue().bold()
        );
    } else if list && all {
        println!(
            "{:<10} {} {:<8} {:<8} {:>6} {} {}/",
            file_info.permissions,
            file_info.nlink,
            file_info.owner,
            file_info.owner_group,
            file_info.size,
            file_info.date,
            file_info.name.blue().bold()
        );
    } else {
        if file_info.name.starts_with(".") {
            return
        } else {
            print!(
                "{}/   ",
                file_info.name.blue().bold()
            );
        }
    };
}


fn print_file(file_info: FileInfo, all: bool, list: bool) {
    if list && !all {
        if file_info.name.starts_with(".") {
            return
        } else {
            println!(
                "{:<10} {} {:<8} {:<8} {:>6} {} {}",
                file_info.permissions,
                file_info.nlink,
                file_info.owner,
                file_info.owner_group,
                file_info.size,
                file_info.date,
                file_info.name.italic()
            )
        }
    } else if all && !list {
        println!(
            "{:<10} {} {:<8} {:<8} {:>6} {} {}",
            file_info.permissions,
            file_info.nlink,
            file_info.owner,
            file_info.owner_group,
            file_info.size,
            file_info.date,
            file_info.name.italic()
        );
    } else if list && all {
        println!(
            "{:<10} {} {:<8} {:<8} {:>6} {} {}",
            file_info.permissions,
            file_info.nlink,
            file_info.owner,
            file_info.owner_group,
            file_info.size,
            file_info.date,
            file_info.name.italic()
        );
    } else {
        if file_info.name.starts_with(".") {
            return
        } else {
            print!(
                "{}   ",
                file_info.name.italic()
            );
        }
    };
}
