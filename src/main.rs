extern crate clap;
use clap::App;
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
fn main() {
    App::new("Syncle")
        .version("0.1.0")
        .author("Miswanting")
        .about("[DES]")
        .get_matches();
}
