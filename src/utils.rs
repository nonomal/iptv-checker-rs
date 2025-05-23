use crate::config::get_others;
use lazy_static::lazy_static;
use rand::distr::Alphanumeric;
use rand::Rng;
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::process::Command;

/// 获取输出文件名，如果未指定则生成随机文件名
pub fn get_out_put_filename(folder: &str, output_file: String) -> String {
    let mut filename = output_file.clone();
    if output_file.is_empty() {
        filename = format!("{}{}", folder, get_random_output_filename());
    }
    filename
}

/// 生成随机的输出文件名
fn get_random_output_filename() -> String {
    let rng = rand::rng();

    let random_string: String = rng
        .sample_iter(Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    format!("{}.m3u", random_string)
}

/// 读取PID文件内容
fn read_pid_contents(pid_file: String) -> Result<String, Error> {
    let mut f = File::open(pid_file)?;
    let mut contents = String::default();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

/// 检查指定PID的进程是否存在
pub fn check_process(pid: u32) -> Result<bool, Error> {
    let status = Command::new("ps").arg("-p").arg(pid.to_string()).output();
    Ok(status.unwrap().status.success())
}

/// 检查文件是否存在
pub fn file_exists(file_path: &String) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        metadata.is_file()
    } else {
        false
    }
}

lazy_static! {
    // 匹配数字和方括号中的内容
    static ref RE: Regex = Regex::new(r"(?m)(\d+\s)?\[\w+\]").unwrap(); // 仅编译一次
    // 匹配开头的数字
    static ref RegexPrefixNum:  Regex = Regex::new(r"^\d+\s*").unwrap();
    // open cc
    // static ref Translator:OpenCC = OpenCC::new(DefaultConfig::T2S).unwrap();
}

pub fn replace_char(mut str: String) -> String {
    let others_config = get_others().expect("TODO: panic message");

    if !others_config.replace_chars.is_empty() {
        for i in others_config.replace_chars.iter() {
            str = str.replace(i.name.as_str(), i.replace.as_str());
        }
    }

    return str;
}

/// 清理频道名称中的特殊字符和标记
pub fn remove_other_char(str: String) -> String {
    let mut res_str = str.to_string();
    // 移除开头的数字和方括号标记
    let result = RE.captures_iter(&str);
    for mat in result {
        if mat.len() >= 1 {
            res_str = res_str.replace(mat.get(0).unwrap().as_str(), "");
        }
    }
    // 移除特定的频道标记
    let mut rename_channel_list: Vec<&str> = vec![
        "[geo-blocked]",
        "[ipv6]",
        "hevc",
        "50 fps",
        "[not 24/7]",
        " (600p) ",
    ];
    let others_config = get_others().expect("TODO: panic message");
    if others_config.replace_chars.len() != 0 {
        for i in others_config.replace_empty.iter() {
            rename_channel_list.push(i);
        }
    }
    for change in rename_channel_list {
        res_str = res_str.replace(change, "")
    }

    let binding = res_str.to_string();
    // 移除开头的数字
    let pre_num_result = RegexPrefixNum.captures_iter(&binding);
    for mat in pre_num_result {
        if mat.len() >= 1 {
            res_str = res_str.replace(mat.get(0).unwrap().as_str(), "");
        }
    }

    res_str
}

/// 繁体转简体（目前未实现）
pub fn translator_t2s(str: &str) -> String {
    // Translator.convert(str)
    str.to_string()
}

/// 测试模块
#[cfg(test)]
mod tests {
    use crate::common::util::parse_normal_str;
    use crate::utils::{remove_other_char, translator_t2s};

    #[tokio::test]
    async fn parse_data_normal () {
        let data = parse_normal_str(String::from(r#"#EXTM3U
#EXTINF:-1 tvg-name="CCTV5(backup)" tvg-id="378823" tvg-country="中国大陆" tvg-language="国语" tvg-logo="https://epg.pw/media/images/channel/2025/01/25/large/20250125001815951580_60.jpg" group-title="运动",cctv5-体育
https://stream1.freetv.fun/8c0a0439191a3ba401897378bc2226a7edda1e571cb356ac7c7f4c15f6a2f380.m3u8"#));
        for i in data.get_list() {
            println!("{}", i.get_extend().unwrap().group_title);
            println!("{}", i.get_extend().unwrap().tv_logo);
            println!("{}", i.get_extend().unwrap().tv_language);
            println!("{}", i.get_extend().unwrap().tv_country);
            println!("{}", i.get_extend().unwrap().tv_id);
            println!("{}", i.get_extend().unwrap().user_agent);
            println!("{}", i.get_extend().unwrap().tv_name);
        }
    }

    #[tokio::test]
    async fn test_str() {
        println!("{}", remove_other_char("213123 [HD]这是1".to_string()));
        println!("{}", remove_other_char("[HD]这是2".to_string()));
        println!("{}", remove_other_char("[HD]cctv3".to_string()));
        println!("{}", remove_other_char("[bd]cctv4".to_string()));
        println!("{}", remove_other_char("2323 cctv5".to_string()));
        println!(
            "{}",
            remove_other_char("2323 cctv6[geo-blocked]".to_string())
        );

        println!("{}", translator_t2s("FTV (民視) (720p) [Not 24/7]"));
    }
}

/// 检查文件夹是否存在
pub fn folder_exists(file_path: &String) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        metadata.is_dir()
    } else {
        false
    }
}

/// 检查并清理已存在的PID文件
pub fn check_pid_exits(pid_name: &String) {
    if file_exists(pid_name) {
        let num = read_pid_num(pid_name).expect("获取pid失败");
        let has_process = check_process(num).expect("检查pid失败");
        if has_process {
            kill_process(num);
        }
    }
}

/// 终止指定PID的进程
fn kill_process(pid: u32) {
    let _output = Command::new("kill")
        .arg("-9")
        .arg(pid.to_string())
        .output()
        .expect("Failed to execute command");
}

/// 从PID文件中读取进程ID
pub fn read_pid_num(pid_name: &String) -> Result<u32, Error> {
    match read_pid_contents(pid_name.clone()) {
        Ok(contents) => {
            let mut n_contents = contents;
            n_contents = n_contents.replace('\n', "");
            match n_contents.parse::<u32>() {
                Ok(num) => Ok(num),
                Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
            }
        }
        Err(e) => Err(e),
    }
}

/// 创建文件夹，如果已存在则不做任何操作
pub fn create_folder(folder_name: &String) -> Result<(), Error> {
    if !folder_exists(folder_name) {
        fs::create_dir(folder_name)
    } else {
        Ok(())
    }
}
