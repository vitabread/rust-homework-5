// 代碼展開

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::collections::HashMap;
fn main() {
    {
        let mut map = HashMap::new();
        map.insert("age", "25");
        map.insert("city", "Beijing");
        map.insert("country", "China");
        {
            ::std::io::_print(format_args!("Name: {0}\n", "Xiaowen"));
        };
        for (key, value) in map.iter() {
            {
                ::std::io::_print(format_args!("{0}: {1}\n", key, value));
            };
        }
    };
}
