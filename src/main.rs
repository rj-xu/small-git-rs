use std::env;

fn main() {
    let version = git2::Version::get();
    let (major, minor, patch) = version.libgit2_version();
    println!("libgit2 版本: {}.{}.{}", major, minor, patch);

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: {} <参数>", args[0]);
        std::process::exit(1);
    }

    let cmd = &args[1];
    match cmd.as_str() {
        "hello" => println!("你好"),
        "help" => println!("帮助"),
        _ => println!("未知参数 {}", cmd),
    }
}
