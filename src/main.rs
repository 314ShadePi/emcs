use c314_utils::prelude::ToStr;
use error_stack::{Context, IntoReport, Report, Result, ResultExt};
use inquire::{self, Confirm, Select, Text};
use std::fmt;
use std::fs;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::io::{Cursor, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug)]
enum JavaError {
    NotInstalled,
}

impl fmt::Display for JavaError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JavaError::NotInstalled => fmt.write_str("Java not installed."),
        }
    }
}

impl Context for JavaError {}

#[derive(Debug)]
enum EulaAcceptError {
    EulaNotAccepted,
    UserPromptError(String),
}

impl fmt::Display for EulaAcceptError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EulaAcceptError::EulaNotAccepted => fmt.write_str("EULA not accepted."),
            EulaAcceptError::UserPromptError(_) => fmt.write_str("User prompt failed."),
        }
    }
}

impl Context for EulaAcceptError {}

#[derive(Debug)]
struct InstallError;

impl fmt::Display for InstallError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Installation failed.")
    }
}

impl Context for InstallError {}

#[tokio::main]
async fn main() {
    env_logger::init();
    let res = mcserver().await;

    match res {
        Ok(_) => println!("Done."),
        Err(err) => {
            log::error!("\n{err:?}");
        }
    }
}

async fn check_for_java() -> Result<i32, JavaError> {
    let javatest = Command::new("java")
        .arg("--version")
        .output()
        .report()
        .change_context(JavaError::NotInstalled)
        .attach_printable("Java not found.".to_string())?;
    Ok(0)
}

async fn accept_mc_eula() -> Result<i32, EulaAcceptError> {
    let eula_accepted = Confirm::new("Please agree to the MINECRAFT END USER LICENSE AGREEMENT at https://account.mojang.com/documents/minecraft_eula before continuing.")
        .with_default(false)
        .prompt();

    match eula_accepted {
        Ok(true) => Ok(0),
        Ok(false) => {
            Err(Report::new(EulaAcceptError::EulaNotAccepted)
                .attach_printable("EULA not accepted."))
        }
        Err(err) => Err(
            Report::new(EulaAcceptError::UserPromptError(format!("{err:?}")))
                .attach_printable(format!("{err:?}")),
        ),
    }
}

async fn mcserver() -> Result<i32, InstallError> {
    check_for_java()
        .await
        .change_context(InstallError)
        .attach_printable("Java is not installed, please install it.".to_string())?;

    let versions = vec![
        "1.12.2", "1.13", "1.13.1", "1.13.2", "1.14", "1.14.1", "1.14.2", "1.14.3", "1.14.4",
        "1.15", "1.15.1", "1.15.2", "1.16", "1.16.1", "1.16.2", "1.16.3", "1.16.4", "1.16.5",
        "1.17", "1.17.1", "1.18", "1.18.1", "1.18.2", "1.19", "1.19.1", "1.19.2",
    ];

    accept_mc_eula()
        .await
        .change_context(InstallError)
        .attach_printable("You have to read and accept the Minecraft EULA before continuing.")?;

    let version = Select::new("What Minecraft version do you want?", versions).prompt();

    let version = match version {
        Ok(version) => version,
        Err(_) => {
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
        "1.19.2" => "https://piston-data.mojang.com/v1/objects/f69c284232d7c7580bd89a5a4931c3581eae1378/server.jar",
        _ => {
            println!("Invalid version");
            return;
        }

    };
    let directory = Text::new("Please specify your desired output directory.").prompt();
    let directory = match directory {
        Ok(directory) => directory,
        Err(_processing) => {
            println!("You have to select a directory before continuing.");
            return;
        }
    };
    let directory: &str = match directory.clone().to_str() {
        "" => {
            println!("You have to select a directory before continuing.");
            return;
        }
        _ => directory.to_str(),
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
    let mut content = Cursor::new(response.bytes().await.unwrap());
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
        .spawn()
        .unwrap()
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))
        .unwrap();

    let reader = BufReader::new(command);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));

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
            .args(["+x", file_name.to_str()])
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
