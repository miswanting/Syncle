#[macro_use]
extern crate clap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
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
struct Database {}

// 文件数据库结构
#[derive(Debug, Serialize, Deserialize)]
struct SyncDB {}

struct Group {
    path: Path,
}

fn main() {
    let mut cf = Config {};
    let mut db = Database {};
    let cf_path = Path::new(CONFIG_NAME);
    let db_path = Path::new(DATABASE_NAME);
    if cf_path.exists() {
        // 读取配置
    } else {
        // 生成配置
        let file_content = serde_yaml::to_string(&cf).unwrap();
        fs::write(CONFIG_NAME, file_content).unwrap();
    }
    let matches = clap_app!(Syncle =>
        (version: "v0.1.0-190820")
        (author: "Miswanting <453542772@qq.com>")
        (about: "A Sync Manager.")
        (@subcommand add =>
            (about: "add folder to default group")
            (version: "v0.1.0-190820")
            (author: "Miswanting <453542772@qq.com>")
            (@arg TARGET: +required "target folder")
        )
        (@subcommand remove =>
            (about: "remove folder from default group")
            (version: "v0.1.0-190820")
            (author: "Miswanting <453542772@qq.com>")
        )
        (@subcommand scan =>
            (about: "scan default group")
            (version: "v0.1.0-190820")
            (author: "Miswanting <453542772@qq.com>")
        )
        (@subcommand sync =>
            (about: "sync default group")
            (version: "v0.1.0-190820")
            (author: "Miswanting <453542772@qq.com>")
        )
    )
    .get_matches();

    match matches.subcommand() {
        ("add", Some(sub_m)) => println!("{:?}", sub_m),
        ("remove", Some(sub_m)) => {}
        ("scan", Some(sub_m)) => {}
        ("sync", Some(sub_m)) => {}
        _ => {}
    }
}
