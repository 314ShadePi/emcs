use asking::error::Processing;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::io::{Cursor, Read, Write};
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    let javatest = Command::new("java").arg("--version").output();
    match javatest {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
            println!("Please install java");
            std::process::exit(1);
        }
    }


    let versions = vec![
        "1.12.2".to_string(),
        "1.13".to_string(),
        "1.13.1".to_string(),
        "1.13.2".to_string(),
        "1.14".to_string(),
        "1.14.1".to_string(),
        "1.14.2".to_string(),
        "1.14.3".to_string(),
        "1.14.4".to_string(),
        "1.15".to_string(),
        "1.15.1".to_string(),
        "1.15.2".to_string(),
        "1.16".to_string(),
        "1.16.1".to_string(),
        "1.16.2".to_string(),
        "1.16.3".to_string(),
        "1.16.4".to_string(),
        "1.16.5".to_string(),
        "1.17".to_string(),
        "1.17.1".to_string(),
        "1.18".to_string(),
        "1.18.1".to_string(),
        "1.18.2".to_string(),
        "1.19".to_string(),
        "1.19.1".to_string(),
    ];
    loop {
        let eula_accepted = asking::yn()
            .message("Please agree to the MINECRAFT END USER LICENSE AGREEMENT at https://account.mojang.com/documents/minecraft_eula before continuing. [y/N]")
            .default_value(false)
            .ask();

        match async_std::task::block_on(eula_accepted) {
            Ok(true) => break,
            Ok(false) => {
                println!("You have to agree to the MINECRAFT END USER LICENSE AGREEMENT at https://account.mojang.com/documents/minecraft_eula before continuing.");
                continue;
            }
            Err(_processing) => {
                println!("You have to agree to the MINECRAFT END USER LICENSE AGREEMENT at https://account.mojang.com/documents/minecraft_eula before continuing.");
                continue;
            }
        }
    }

    let version = asking::select_with_msg(versions.clone(), "Please select a valid version").message("input the minecraft version you want your server to be(Ex. 1.16.3, 1.12.2, 1.13 Etc.(Supports from 1.12.2 and up))\n").ask();

    let version = match async_std::task::block_on(version) {
        Ok(version) => string_to_static_str(version),
        Err(_processing) => {
            println!("You have to select a version before continuing.");
            return;
        }
    };

    let url = match version {
        "1.12.2" => "https://launcher.mojang.com/v1/objects/886945bfb2b978778c3a0288fd7fab09d315b25f/server.jar",
        "1.13" => "https://launcher.mojang.com/v1/objects/d0caafb8438ebd206f99930cfaecfa6c9a13dca0/server.jar",
        "1.13.1" => "https://launcher.mojang.com/v1/objects/fe123682e9cb30031eae351764f653500b7396c9/server.jar",
        "1.13.2" => "https://launcher.mojang.com/v1/objects/3737db93722a9e39eeada7c27e7aca28b144ffa7/server.jar",
        "1.14" => "https://launcher.mojang.com/v1/objects/f1a0073671057f01aa843443fef34330281333ce/server.jar",
        "1.14.1" => "https://launcher.mojang.com/v1/objects/ed76d597a44c5266be2a7fcd77a8270f1f0bc118/server.jar",
        "1.14.2" => "https://launcher.mojang.com/v1/objects/808be3869e2ca6b62378f9f4b33c946621620019/server.jar",
        "1.14.3" => "https://launcher.mojang.com/v1/objects/d0d0fe2b1dc6ab4c65554cb734270872b72dadd6/server.jar",
        "1.14.4" => "https://launcher.mojang.com/v1/objects/3dc3d84a581f14691199cf6831b71ed1296a9fdf/server.jar",
        "1.15" => "https://launcher.mojang.com/v1/objects/e9f105b3c5c7e85c7b445249a93362a22f62442d/server.jar",
        "1.15.1" => "https://launcher.mojang.com/v1/objects/4d1826eebac84847c71a77f9349cc22afd0cf0a1/server.jar",
        "1.15.2" => "https://launcher.mojang.com/v1/objects/bb2b6b1aefcd70dfd1892149ac3a215f6c636b07/server.jar",
        "1.16" => "https://launcher.mojang.com/v1/objects/a0d03225615ba897619220e256a266cb33a44b6b/server.jar",
        "1.16.1" => "https://launcher.mojang.com/v1/objects/a412fd69db1f81db3f511c1463fd304675244077/server.jar",
        "1.16.2" => "https://launcher.mojang.com/v1/objects/c5f6fb23c3876461d46ec380421e42b289789530/server.jar",
        "1.16.3" => "https://launcher.mojang.com/v1/objects/f02f4473dbf152c23d7d484952121db0b36698cb/server.jar",
        "1.16.4" => "https://launcher.mojang.com/v1/objects/35139deedbd5182953cf1caa23835da59ca3d7cd/server.jar",
        "1.16.5" => "https://launcher.mojang.com/v1/objects/1b557e7b033b583cd9f66746b7a9ab1ec1673ced/server.jar",
        "1.17" => "https://launcher.mojang.com/v1/objects/0a269b5f2c5b93b1712d0f5dc43b6182b9ab254e/server.jar",
        "1.17.1" => "https://launcher.mojang.com/v1/objects/a16d67e5807f57fc4e550299cf20226194497dc2/server.jar",
        "1.18" => "https://launcher.mojang.com/v1/objects/3cf24a8694aca6267883b17d934efacc5e44440d/server.jar",
        "1.18.1" => "https://launcher.mojang.com/v1/objects/125e5adf40c659fd3bce3e66e67a16bb49ecc1b9/server.jar",
        "1.18.2" => "https://launcher.mojang.com/v1/objects/c8f83c5655308435b3dcf03c06d9fe8740a77469/server.jar",
        "1.19" => "https://launcher.mojang.com/v1/objects/e00c4052dac1d59a1188b2aa9d5a87113aaf1122/server.jar",
        "1.19.1" => "https://piston-data.mojang.com/v1/objects/8399e1211e95faa421c1507b322dbeae86d604df/server.jar",
        _ => {
            println!("Invalid version");
            return;
        }

    };
    let directory = asking::text().message("Input the directory where you want to save your server relative to current directory(Ex. C:\\Server\\ on windows, /server/ on linux)\n").ask();
    let directory = match async_std::task::block_on(directory) {
        Ok(directory) => string_to_static_str(directory),
        Err(_processing) => {
            println!("You have to select a directory before continuing.");
            return;
        }
    };
    let directory = match directory {
        "" => {
            println!("You have to select a directory before continuing.");
            return;
        }
        _ => directory,
    };

    let directory_creation_res = fs::create_dir_all(&directory);
    match directory_creation_res {
        Ok(_) => {
            println!("Directory created");
        }
        Err(e) => {
            println!("Failed to create directory: {}", e);
            return;
        }
    }

    println!("Downloading server...");
    let response = reqwest::get(url).await.unwrap();
    let file_name: String = directory.clone().to_string() + "/server.jar";
    let mut file = std::fs::File::create(&file_name).unwrap();
    let mut content =  Cursor::new(response.bytes().await.unwrap());
    std::io::copy(&mut content, &mut file).unwrap();
    println!("Server downloaded to {}", &file_name);

    println!("Starting server to create files...");
    let command = Command::new("java")
        .args(["-Xmx1024M", "-Xms1024M", "-jar", "./server.jar", "nogui"])
        .current_dir(directory)
        .output();

    match command {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            println!("Failed to start server: {}", e);
            std::process::exit(1);
        }
    }

    let mut eula = fs::File::open(Path::new(&directory).join("eula.txt")).unwrap();
    let mut eula_file = String::new();
    eula.read_to_string(&mut eula_file).unwrap();
    let eulafile = eula_file.replace("eula=false", "eula=true");
    let mut eula = fs::File::create(Path::new(&directory).join("eula.txt")).unwrap();
    eula.write_all(eulafile.as_bytes()).unwrap();

    println!("Starting server...");
    println!("To stop the server, type \"stop\"");
    let command = Command::new("java")
        .args(["-Xmx1024M", "-Xms1024M", "-jar", "./server.jar", "nogui"])
        .current_dir(directory)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn().unwrap()
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output.")).unwrap();

    let reader = BufReader::new(command);

    reader.lines().filter_map(|line| line.ok()).for_each(|line| println!("{}", line));

    if cfg!(windows) {
        let file_name = directory.clone().to_string() + "/run.cmd";
        let mut file = fs::File::create(file_name).unwrap();
        let content = format!("java -Xmx1024M -Xms1024M -jar ./server.jar nogui");
        file.write_all(content.as_bytes()).unwrap();
    } else {
        let file_name = directory.clone().to_string() + "/run.sh";
        let mut file = fs::File::create(&file_name).unwrap();
        let content = format!("java -Xmx1024M -Xms1024M -jar ./server.jar nogui");
        file.write_all(content.as_bytes()).unwrap();
        let chmod = Command::new("chmod")
            .args(["+x", string_to_static_str(file_name)])
            .current_dir(directory)
            .output();
        match chmod {
            Ok(_) => {
                println!("run.sh created");
            }
            Err(e) => {
                println!("Failed to create run.sh: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
