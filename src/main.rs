use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};
use std::fs;
use std::thread::sleep;


fn conv( bik_path: &String){
    let exe_path = env::current_exe().expect("Failed to get current exe path");
    let exe_dir = exe_path.parent().expect("Failed to get exe directory");

    let path = Path::new(bik_path);
    let file_stem = path.file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("");

    // 构建输出文件路径
    let avi_output_path = exe_dir.join(format!("{}.avi", file_stem));
    let mp4_output_path = exe_dir.join(format!("{}.mp4", file_stem));

    // 调用外部命令 binkvideo64.exe BinkConv <input> <avi_output>
    let output = Command::new("C:\\Program Files (x86)\\RADVideo\\radvideo64.exe")
        .arg("BinkConv")
        .arg(bik_path)
        .arg(avi_output_path.to_str().unwrap())
        .arg("/#")
        .output()
        .expect("Failed to execute binkvideo64 command");

    // 检查第一个命令执行结果
    if output.status.success() {
        println!("binkvideo64 command executed successfully.");
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("stdout: {}", stdout);

        // 第二个命令 ffmpeg -y -i <avi_output> -c:v libx264 -crf 23 -preset medium -pix_fmt yuv420p -vf "pad=ceil(iw/2)*2:ceil(ih/2)*2" <mp4_output>
        let ffmpeg_output = Command::new("ffmpeg")
            .arg("-y")
            .arg("-i")
            .arg(avi_output_path.to_str().unwrap())
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
            .arg(mp4_output_path.to_str().unwrap())
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        exit(1);
    }


    let file_path = &args[1];
    // dict or .bik
    if (file_path.ends_with(".bik")) {
        conv(file_path);

    }
    else {
        let entries = fs::read_dir(file_path)
            .expect("Failed to read directory");
        let bik_path = Path::new(file_path);
        for entry in entries{
            if let Ok(entry) = entry{
                let file_name = entry.file_name();

                let bik_path = bik_path.join(file_name);

                if (bik_path.to_string_lossy().ends_with(".bik")){
                    conv(&bik_path.to_string_lossy().to_string());
                }
                println!("file_path:{}",bik_path.to_string_lossy());
            }
        }
    }
}
