// 导入所需的宏库
use std::collections::HashMap;

// 声明一个名为 `my_macro` 的宏
macro_rules! my_macro {
    // 定义宏的模式和替换代码
    ($name:expr, {$($key:expr => $value:expr),*}) => {
        {
            // 创建一个 HashMap
            let mut map = HashMap::new();

            // 将键值对添加到 HashMap 中
            $(
                map.insert($key, $value);
            )*

            // 打印 HashMap 的内容
            println!("Name: {}", $name);
            for (key, value) in map.iter() {
                println!("{}: {}", key, value);
            }
        };
    };
}

fn main() {
    // 使用宏来生成代码
    my_macro!("Xiaowen", {
        "age" => "25",
        "city" => "Beijing",
        "country" => "China"
    });
}
