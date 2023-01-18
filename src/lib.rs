//! bevy_state_tool包是bevy包的依赖包，一个微型构建状态的包。
//! 使用时需要添加bevy包与iyes_loopless包
//! 例子：
//! ```toml
//! bevy = { version = "0.9.1", features = ["dynamic"] }
//! iyes_loopless="0.9.1"
//! bevy_state_tool="0.1.0"
//! ```
//! 定义结构体
//! 必须实现enter,ready,update,exit,init函数
//! 其中除了init函数外其他函数遵循bevy定义的函数格式，参数可以自定义
//! ```
//! #[derive(Clone, Eq, PartialEq, Debug, Hash,GameStates)]
//! enum AppState {
//!     Start,
//!     In,
//! }
//! impl Start {
//!     fn enter() {
//!         println!("已经进入状态")
//!     }
//!     fn ready() {
//!         println!("已经开始初始加载")
//!     }
//!     fn update() {
//!
//!     }
//!     fn exit() {
//!         println!("切换时我将显示")
//!     }
//!     fn init(app: &mut App) {
//!         println!("使用app对插件加载额外的信息")
//!     }
//! }
//! impl In{
//!     /*
//!      */
//! }
//! ```
//! 
//! main函数中的实现
//! ```
//! use bevy::prelude::*;
//! use iyes_loopless::prelude::*;
//! fn main(){
//!     //加载初始状态
//!     App::new()/* */.add_plugin(AppState::Start).run();
//!     
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/** 该过程宏会自动根据枚举生成对印结构体 */
#[proc_macro_derive(GameStates)]
pub fn game_states(input: proc_macro::TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut expanded = quote! {};
    let enum_name = &input.ident;
    let mut loading_expanded = quote! {};
    //构建enum
    //对数据进行处理
    match input.data {
        //如果类型是枚举体
        syn::Data::Enum(ref data_struct) => {
            for variant in &data_struct.variants {
                let variant_name = &variant.ident;
                expanded = quote! {
                    #expanded
                    pub struct #variant_name;

                    impl Plugin for #variant_name{
                        fn build(&self, app: &mut App) {
                            app.add_startup_system(Self::ready).add_enter_system(#enum_name::#variant_name,Self::enter)
                            .add_system(Self::update.run_in_state(#enum_name::#variant_name))
                            .add_exit_system(#enum_name::#variant_name,Self::exit);
                            Self::init(app);
                        }
                    }
                };
                loading_expanded = quote! {
                    #loading_expanded.add_plugin(#variant_name)
                };
            }
        }
        _ => (),
    }
    expanded = quote! {
        #expanded

        //这里生成默认的Plugin
        impl Plugin for #enum_name{
            fn build(&self,app:&mut App){
                app.add_loopless_state(*self)#loading_expanded;
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

//创建一个Attri
