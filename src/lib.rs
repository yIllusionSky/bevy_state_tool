//! bevy_state_tool包是bevy包的依赖包，一个微型构建状态的包。
//! 使用时需要添加bevy包与iyes_loopless包
//! example for Cargo.toml：
//! ```toml
//! bevy = { version = "0.9.1", features = ["dynamic"] }
//! iyes_loopless="0.9.1"
//! bevy_state_tool="0.1.0"
//! ```
//!
//! example for lib.rs
//! ```
//! pub mod test;
//! use bevy::prelude::*;
//! use iyes_loopless::prelude::*;
//! use bevy_state_tool::GameStates;
//! 
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, GameStates)]
//! pub enum GameState {
//! GameIn,
//! GamePause,
//! StartUi,
//! ChooseGame,
//! AssetLoadingSpecial,
//! }
//! ```
//! 
//! example for main.rs
//! ```
//! use bevy::prelude::*;
//! use iyes_loopless::prelude::*;
//! use bevy_state_tool::GameStates;
//! fn main()
//! {
//!     App::new().add_plugin(test1::Test::A).run();
//! }
//! ```
#![feature(fs_try_exists)]
use std::io::{Read, Write};

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::DeriveInput;
/** 该过程宏会自动根据枚举生成对印结构体 */
#[proc_macro_derive(GameStates)]
pub fn game_states(input: proc_macro::TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name: &str = &input.ident.to_string();
    let path_name: &str = &enum_name.to_ascii_lowercase();
    //如果没有这个文件夹
    if !std::fs::try_exists(String::from("src/") + path_name).unwrap() {
        /* 创建目录以及子文件夹 */
        std::fs::create_dir_all(String::from("src/") + path_name + "/config").unwrap();

        /* 创建mod.rs文件 */
        let mut mod_rs_file = std::fs::File::options()
            .write(true)
            .create(true)
            .open(String::from("src/") + path_name + "/mod.rs")
            .unwrap();
        /* 创建这个文件 */
        let mut file = std::fs::File::options()
            .write(true)
            .create(true)
            .open(String::from("src/") + path_name + "/config/info.txt")
            .unwrap();

        /* 创建加载文件 */
        let mut init_file = std::fs::File::options()
            .write(true)
            .create(true)
            .open(String::from("src/") + path_name + "/config/init.txt")
            .unwrap();
        writeln!(init_file, "app.add_loopless_state(*self)").unwrap();

        //对数据进行处理
        match input.data {
            //如果类型是枚举体
            syn::Data::Enum(ref data_struct) => {
                for variant in &data_struct.variants {
                    //结构体名字
                    let variant_name: &str = &variant.ident.to_string();
                    let struct_file_name: &str = &variant_name.to_ascii_lowercase();
                    //加载记录
                    writeln!(file, "{}", variant_name).unwrap();
                    writeln!(init_file, ".add_plugin({})", variant_name).unwrap();
                    writeln!(mod_rs_file, "mod {};", struct_file_name).unwrap();
                    writeln!(mod_rs_file, "use {}::{};", struct_file_name, variant_name).unwrap();
                    create_file(enum_name, variant_name).unwrap();
                }
            }
            _ => (),
        }
        let over_string = include_str!("../template_mod.txt").replace("$", enum_name);
        write!(mod_rs_file, "{}", over_string).unwrap();
        file.flush().unwrap();
        init_file.flush().unwrap();
        mod_rs_file.flush().unwrap();
    } else {
        /* 打开这个文件 */
        let mut file = std::fs::File::options()
            .read(true)
            .write(true)
            .open(String::from("src/") + path_name + "/config/info.txt")
            .unwrap();

        /* 打开mod.rs文件 */
        let mut mod_rs_file = std::fs::File::options()
            .append(true)
            .open(String::from("src/") + path_name + "/mod.rs")
            .unwrap();

        /* 打开加载文件 */
        let mut init_file = std::fs::File::options()
            .append(true)
            .open(String::from("src/") + path_name + "/config/init.txt")
            .unwrap();

        /* 加载文件里面的内容 */
        let mut file_str = String::new();
        file.read_to_string(&mut file_str).unwrap();
        let arr = file_str.split('\n').collect::<Vec<_>>();
        match input.data {
            //如果类型是枚举体
            syn::Data::Enum(ref data_struct) => {
                for variant in &data_struct.variants {
                    //结构体名字
                    let variant_name: &str = &variant.ident.to_string();
                    let struct_file_name: &str = &variant_name.to_ascii_lowercase();
                    //如果不包括该结构体的名字

                    if !arr.contains(&variant_name) {
                        //加载记录
                        writeln!(file, "{}", variant_name).unwrap();
                        writeln!(init_file, ".add_plugin({})", variant_name).unwrap();
                        writeln!(mod_rs_file, "mod {};", struct_file_name).unwrap();
                        writeln!(mod_rs_file, "use {}::{};", struct_file_name, variant_name).unwrap();
                        create_file(enum_name, variant_name).unwrap();
                    }
                }
            }
            _ => (),
        }
        file.flush().unwrap();
        init_file.flush().unwrap();
        mod_rs_file.flush().unwrap();
    }

    TokenStream::new()
}

//创建文件
#[inline]
fn create_file(path: &str, file_name: &str) -> std::io::Result<()> {
    let mut struct_file = std::fs::File::create(
        String::from("src/")
            + &path.to_ascii_lowercase()
            + "/"
            + &file_name.to_ascii_lowercase()
            + ".rs",
    )?;
    let over_str = include_str!("../template.txt")
        .replace("$", path)
        .replace("~", file_name);
    write!(struct_file, "{}", over_str)
}
