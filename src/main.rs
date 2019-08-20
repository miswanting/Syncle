#[macro_use]
extern crate clap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
// use std::collections::HashMap;
// use clap::App;
/*
Syncle - 同步工具
用法：
    syncle [field] action [-args] [target]
领域：
    folder
    group

动作：
    add
    remove
    scan
    sync

添加同步文件夹：syncle folder add
默认组搜索：syncle scan
添加同步组：syncle group add
syncle remove
syncle
*/
// 常量
const CONFIG_NAME: &str = "config.yaml";
const DATABASE_NAME: &str = "database.json";
const SYNCDB_NAME: &str = "SyncDB.json";

// 配置文件结构
#[derive(Debug, Serialize, Deserialize)]
struct Config {}

// 主数据库结构（记录组信息）
#[derive(Debug, Serialize, Deserialize)]
struct Database {
    groups: Vec<Group>,
}

// 文件数据库结构
#[derive(Debug, Serialize, Deserialize)]
struct SyncDB {}

#[derive(Debug, Serialize, Deserialize)]
struct Group {
    name: String,
    folders: Vec<Folder>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Folder {
    name: String,
    path: String,
}

fn main() {
    let mut cf = Config {};
    // let mut default_map:HashMap<String,Group>=HashMap.new();
    // default_map.insert("default".to_string(),folders: Vec::new())
    let mut db = Database {
        // groups:default_map,
        groups: vec![Group {
            name: "default".to_string(),
            folders: Vec::new(),
        }],
    };
    let cf_path = Path::new(CONFIG_NAME);
    let db_path = Path::new(DATABASE_NAME);

    if cf_path.exists() {
        fs::remove_file(cf_path).unwrap();
    }
    if db_path.exists() {
        fs::remove_file(db_path).unwrap();
    }

    if cf_path.exists() {
        // 读取配置
        let file_content = fs::read_to_string(CONFIG_NAME).unwrap();
        cf = serde_yaml::from_str(&file_content).unwrap();
    } else {
        // 生成配置
        let file_content = serde_yaml::to_string(&cf).unwrap();
        fs::write(CONFIG_NAME, file_content).unwrap();
    }
    if db_path.exists() {
        // 读取数据库
        let file_content = fs::read_to_string(DATABASE_NAME).unwrap();
        db = serde_json::from_str(&file_content).unwrap();
    } else {
        // 生成数据库
        let file_content = serde_json::to_string(&db).unwrap();
        fs::write(DATABASE_NAME, file_content).unwrap();
    }

    let matches = clap_app!(Syncle =>
        (version: "v0.1.0-190820")
        (author: "Miswanting <453542772@qq.com>")
        (about: "A Sync Manager.")
        (@subcommand add =>
            (about: "add folder to default group.")
            (version: "v0.1.0-190820")
            (author: "Miswanting <453542772@qq.com>")
            (@arg TARGET: +required "target folder")
        )
        (@subcommand remove =>
            (about: "remove folder from default group.")
            (version: "v0.1.0-190820")
            (author: "Miswanting <453542772@qq.com>")
        )
        (@subcommand scan =>
            (about: "scan default group.")
            (version: "v0.1.0-190820")
            (author: "Miswanting <453542772@qq.com>")
        )
        (@subcommand sync =>
            (about: "sync default group.")
            (version: "v0.1.0-190820")
            (author: "Miswanting <453542772@qq.com>")
        )
    )
    .get_matches();

    match matches.subcommand() {
        ("add", Some(target)) => {
            let target_path = Path::new(target.value_of("TARGET").unwrap());
            let target_db = target_path.join(SYNCDB_NAME);
            if target_db.exists() {
                // 目标数据库存在，读取
            } else {
                // 目标数据库不存在，生成
            }
            println!("{:?}", target_db);
        }
        ("remove", Some(sub_m)) => {}
        ("scan", Some(sub_m)) => {}
        ("sync", Some(sub_m)) => {}
        _ => {
            println!("Syncle");
            println!();
            loop {
                print!("> ");
                io::stdout().flush().unwrap();
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).unwrap();
                buffer = buffer.to_string();
                let cmd:Vec<&str>=buffer.split_whitespace().collect();
                println!("{:?}", cmd);
            }
        }
    }
}
