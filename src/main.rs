use std::env;
use std::path::Path;
use std::process::{Command, exit};

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查是否提供了文件路径
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        exit(1);
    }

    // 读取文件路径
    let file_path = &args[1];

    // 使用 Path 模块获取文件名并替换扩展名
    let path = Path::new(file_path);
    let avi_input = path.file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_owned() + ".avi";
    let avi_output = path.file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_owned() + ".avi";
    let output_file_name = path.file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_owned() + ".mp4";

    // 如果输出文件名为空，退出
    if output_file_name.is_empty() {
        eprintln!("Failed to determine output file name.");
        exit(1);
    }

    // 调用外部命令 binkvideo64.exe BinkConv <input>
    let output = Command::new("C:\\Program Files (x86)\\RADVideo\\radvideo64.exe")
        .arg("BinkConv")
        .arg(file_path)
        .arg(avi_output)
        .output()
        .expect("Failed to execute binkvideo64 command");

    // 检查第一个命令执行结果
    if output.status.success() {
        println!("binkvideo64 command executed successfully.");
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("stdout: {}", stdout);

        // 第二个命令 ffmpeg -y -i <input> -c:v libx264 -crf 23 -preset medium -pix_fmt yuv420p -vf "pad=ceil(iw/2)*2:ceil(ih/2)*2" <output>
        let ffmpeg_output = Command::new("ffmpeg")
            .arg("-y")
            .arg("-i")
            .arg(avi_input)
            .arg("-c:v")
            .arg("libx264")
            .arg("-crf")
            .arg("23")
            .arg("-preset")
            .arg("medium")
            .arg("-pix_fmt")
            .arg("yuv420p")
            .arg("-vf")
            .arg("pad=ceil(iw/2)*2:ceil(ih/2)*2")
            .arg(&output_file_name)
            .output()
            .expect("Failed to execute ffmpeg command");

        // 检查第二个命令执行结果
        if ffmpeg_output.status.success() {
            println!("ffmpeg command executed successfully.");
            let ffmpeg_stdout = String::from_utf8_lossy(&ffmpeg_output.stdout);
            println!("ffmpeg stdout: {}", ffmpeg_stdout);
        } else {
            eprintln!("ffmpeg command failed to execute.");
            let ffmpeg_stderr = String::from_utf8_lossy(&ffmpeg_output.stderr);
            eprintln!("ffmpeg stderr: {}", ffmpeg_stderr);
            exit(1);
        }

    } else {
        eprintln!("binkvideo64 command failed to execute.");
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("stderr: {}", stderr);
        exit(1);
    }
}
