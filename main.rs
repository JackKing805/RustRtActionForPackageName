extern crate crossterm;

use std::{
    env::{self, args},
    f32::consts::E,
    fs,
    os::unix::thread,
    process::{Command, Output},
    result,
    thread::Thread,
    vec,
};

use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;

use std::io::Write;

use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    execute,
    style::{
        Attribute, Color, Print, PrintStyledContent, ResetColor, SetAttribute, SetBackgroundColor,
        SetForegroundColor, Stylize,
    },
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetSize},
};
use std::io::stdout;

fn main() {
    let args: Vec<String> = env::args().collect();
    let removeFirstArgs = args.split_first();
    if let Some(a) = removeFirstArgs {
        let realComment = a.1;
        for (index, item) in realComment.iter().enumerate() {
            if item.starts_with("-") {
                if item == "-package" {
                    let packageName = realComment.get(index + 1).expect("请提供包名");
                    onPackageNameParsed(packageName);
                } else {
                    panic!("错误指令")
                }
            }
        }
    }
}

fn onPackageNameParsed(packageName: &String) {
    // adb shell ps -A | grep com.callapp.theapp.callthemes
    // let aa = Command::new(format!("adb shell ps -A | {}",packageName)).output();
    let usePackageName = format!("{}", packageName);
    let _ = spawn(move || loop {
        loop {
            let aa = Command::new("adb")
                .arg("shell")
                .arg("ps")
                .arg("-A")
                .arg("|")
                .arg("grep")
                .arg(&usePackageName)
                .output();
            match aa {
                Ok(a) => {
                    if a.status.success() {
                        let str = String::from_utf8(a.stdout).unwrap();
                        let array = str.split("\n");

                        let mut vec_process: Vec<String> = vec![];
                        for ele in array {
                            if !ele.trim().is_empty() {
                                vec_process.push(ele.to_string());
                            }
                        }

                        if !&vec_process.is_empty() {
                            let first = vec_process.first().unwrap();
                            let psplit = first.split(" ");

                            let mut vec_plist: Vec<String> = vec![];
                            for ele in psplit {
                                if !ele.trim().is_empty() {
                                    vec_plist.push(ele.to_string());
                                }
                            }

                            let pid = &vec_plist.first().unwrap();
                            loop {
                                let pid_command = Command::new("adb")
                                    .arg("shell")
                                    .arg("ps")
                                    .arg("-A")
                                    .arg("|")
                                    .arg("grep")
                                    .arg(pid)
                                    .output();

                                match pid_command {
                                    Ok(pid) => {
                                        if pid.status.success() {
                                            let result = String::from_utf8(pid.stdout).unwrap();
                                            let array2 = result.split("\n");

                                            let mut vec_process2: Vec<String> = vec![];
                                            for ele in array2 {
                                                if !ele.trim().is_empty() {
                                                    vec_process2.push(ele.to_string());
                                                }
                                            }
                                            let mut pringString = String::new();
                                            for ele in vec_process2 {
                                                pringString.push_str(format!("{}\n", ele).as_str());
                                            }
                                            execute!(
                                                stdout(),
                                                Clear(ClearType::FromCursorUp),
                                                Print(pringString.yellow())
                                            )
                                            .unwrap();
                                        } else {
                                            break;
                                        }
                                    }
                                    Err(_err) => {
                                        break;
                                    }
                                }
                                sleep(Duration::from_secs(1));
                            }
                        }
                    }else{
                        execute!(
                            stdout(),
                            Clear(ClearType::FromCursorUp),
                            Print("进程不存在\n".yellow())
                        )
                        .unwrap();
                    }
                }
                Err(_err) => {
                    execute!(
                        stdout(),
                        Clear(ClearType::FromCursorUp),
                        Print("进程不存在\n".yellow())
                    )
                    .unwrap();
                }
            }
            sleep(Duration::from_millis(500));
        }
    })
    .join();
}
