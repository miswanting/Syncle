#[macro_use]
extern crate clap;
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
fn main() {
    // App::new("Syncle")
    //     .version("v0.1.0-190820")
    //     .author("Miswanting <453542772@qq.com>")
    //     .about("A Sync Manager.")
    //     .get_matches();
    let matches = clap_app!(Syncle =>
        (version: "v0.1.0-190820")
        (author: "Miswanting <453542772@qq.com>")
        (about: "A Sync Manager.")
        (@subcommand add =>
            (about: "add folder to default group")
            (version: "v0.1.0-190820")
            (author: "Miswanting <453542772@qq.com>")
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
    
}
