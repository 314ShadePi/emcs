mod error_types;
use c314_utils::prelude::ToStr;
use error_stack::{IntoReport, Report, Result, ResultExt};
use inquire::{self, Confirm, Select, Text};
use std::fs;
use std::io::{BufRead, BufReader};
use std::io::{Cursor, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use crate::error_types::*;

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
    Command::new("java")
        .arg("--version")
        .output()
        .report()
        .change_context(JavaError::NotInstalledError)
        .attach_printable("Java not found.".to_string())?;
    Ok(0)
}

async fn accept_mc_eula() -> Result<i32, EulaAcceptError> {
    let eula_accepted = Confirm::new("Please agree to the Minecraft EULA at https://account.mojang.com/documents/minecraft_eula before continuing.")
        .with_default(false)
        .prompt()
        .report()
        .change_context(EulaAcceptError::UserPromptError)
        .attach_printable("Couldn't get confirmation from user.")?;

    match eula_accepted {
        true => Ok(0),
        false => Err(Report::new(EulaAcceptError::EulaNotAcceptedError)
            .attach_printable("EULA not accepted.")),
    }
}

async fn get_version() -> Result<(i32, String), VersionError> {
    let versions = vec![
        "1.12.2", "1.13", "1.13.1", "1.13.2", "1.14", "1.14.1", "1.14.2", "1.14.3", "1.14.4",
        "1.15", "1.15.1", "1.15.2", "1.16", "1.16.1", "1.16.2", "1.16.3", "1.16.4", "1.16.5",
        "1.17", "1.17.1", "1.18", "1.18.1", "1.18.2", "1.19", "1.19.1", "1.19.2",
    ];

    let version = Select::new("What Minecraft version do you want?", versions)
        .prompt()
        .report()
        .change_context(VersionError::UserPromptError)
        .attach_printable("Couldn't get version name from user.")?;

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
            return Err(Report::new(VersionError::InvalidVersionError).attach_printable("Invalid version detected."));
        }

    };

    Ok((0, url.to_string()))
}

async fn get_directory() -> Result<(i32, String), DirectoryError> {
    let directory = Text::new("Please specify your desired output directory.")
        .prompt()
        .report()
        .change_context(DirectoryError::UserPromptError)
        .attach_printable("Couldn't get directory name from user.")?;

    let directory = match directory.clone().to_str() {
        "" => {
            println!("You have to select a directory before continuing.");
            return Err(Report::new(DirectoryError::EmptyNameError)
                .attach_printable("Directory name must not be empty."));
        }
        _ => directory.to_str(),
    };

    fs::create_dir_all(&directory)
        .report()
        .change_context(DirectoryError::CreationError)
        .attach_printable(format!("Failed to create directory '{}'", &directory))?;

    Ok((0, directory.to_string()))
}

async fn download_server(url: String, directory: String) -> Result<i32, ServerDownloadError> {
    println!("Downloading server...");
    let response = reqwest::get(url)
        .await
        .report()
        .change_context(ServerDownloadError::RequestError)
        .attach_printable("Could not fetch server.")?;

    let file_name: String = directory.clone().to_string() + "/server.jar";

    let mut file = std::fs::File::create(&file_name)
        .report()
        .change_context(ServerDownloadError::FileCreationError)
        .attach_printable("Could not create server.jar.")?;

    let response_bytes = response
        .bytes()
        .await
        .report()
        .change_context(ServerDownloadError::ResponseBytesError)
        .attach_printable("Failed to get response bytes.")?;

    let mut content = Cursor::new(response_bytes);

    std::io::copy(&mut content, &mut file)
        .report()
        .change_context(ServerDownloadError::ContentCopyError)
        .attach_printable("Could not copy content to server.jar.")?;

    println!("Server downloaded to {}", &file_name);

    Ok(0)
}

async fn run_server_create_files(directory: String) -> Result<i32, ServerInitError> {
    println!("Starting server to create files...");

    let output = Command::new("java")
        .args(["-Xmx1024M", "-Xms1024M", "-jar", "./server.jar", "nogui"])
        .current_dir(directory)
        .output()
        .report()
        .change_context(ServerInitError)
        .attach_printable("Could not run server.")?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
    Ok(0)
}

async fn modify_eula(directory: String) -> Result<i32, EulaModError> {
    let mut eula = fs::File::open(Path::new(&directory).join("eula.txt"))
        .report()
        .change_context(EulaModError::FileOpenError)
        .attach_printable("Failed to open eula.txt.")?;

    let mut eula_file = String::new();

    eula.read_to_string(&mut eula_file)
        .report()
        .change_context(EulaModError::FileReadError)
        .attach_printable("Failed to read from eula.txt.")?;
    let eulafile = eula_file.replace("eula=false", "eula=true");

    let mut eula = fs::File::create(Path::new(&directory).join("eula.txt"))
        .report()
        .change_context(EulaModError::FileCreateError)
        .attach_printable("Failed to create eula.txt.")?;

    eula.write_all(eulafile.as_bytes())
        .report()
        .change_context(EulaModError::FileWríteError)
        .attach_printable("Failed to write to eula.txt.")?;

    Ok(0)
}

async fn create_server() -> Result<(i32, String), ServerCreationError> {
    let url = get_version()
        .await
        .change_context(ServerCreationError::VersionError)
        .attach_printable("Couldn't get version.")?;

    let directory = get_directory()
        .await
        .change_context(ServerCreationError::DirectoryError)
        .attach_printable("Failed to create directory.")?;

    download_server(url.1, directory.1.clone())
        .await
        .change_context(ServerCreationError::ServerDownloadError)
        .attach_printable("Failed to download server.")?;

    run_server_create_files(directory.1.clone())
        .await
        .change_context(ServerCreationError::ServerInitError)
        .attach_printable("Could not run server.")?;

    modify_eula(directory.1.clone())
        .await
        .change_context(ServerCreationError::EulaModError)
        .attach_printable("Failed to modify eula.txt.")?;
    Ok((0, directory.1))
}

async fn run_server(directory: String) -> Result<i32, ServerRunError> {
    println!("Starting server...");
    println!("To stop the server, type \"stop\"");
    let command = Command::new("java")
        .args(["-Xmx1024M", "-Xms1024M", "-jar", "./server.jar", "nogui"])
        .current_dir(directory)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .report()
        .change_context(ServerRunError::SpawnError)
        .attach_printable("Could not spawn server instance.")?
        .stdout
        .ok_or_else(|| {
            Report::new(ServerRunError::StdoutError)
                .attach_printable("Could not capture standard output from server instance.")
        })?;

    let reader = BufReader::new(command);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));
    Ok(0)
}

async fn create_start_file(directory: String) -> Result<i32, StartfileCreationError> {
    if cfg!(windows) {
        let file_name = directory.clone().to_string() + "/run.cmd";
        let mut file = fs::File::create(file_name.clone())
            .report()
            .change_context(StartfileCreationError::FileCreateError)
            .attach_printable(format!("Could not create file {file_name}."))?;
        let content = format!("java -Xmx1024M -Xms1024M -jar ./server.jar nogui");
        file.write_all(content.as_bytes())
            .report()
            .change_context(StartfileCreationError::FileWríteError)
            .attach_printable(format!("Could not write to file {file_name}"))?;
    } else {
        let file_name = directory.clone().to_string() + "/run.sh";
        let mut file = fs::File::create(file_name.clone())
            .report()
            .change_context(StartfileCreationError::FileCreateError)
            .attach_printable(format!("Could not create file {file_name}."))?;
        let content = format!("java -Xmx1024M -Xms1024M -jar ./server.jar nogui");
        file.write_all(content.as_bytes())
            .report()
            .change_context(StartfileCreationError::FileWríteError)
            .attach_printable(format!("Could not write to file {file_name}"))?;
    }
    Ok(0)
}

async fn mcserver() -> Result<i32, InstallError> {
    check_for_java()
        .await
        .change_context(InstallError)
        .attach_printable("Java is not installed, please install it.".to_string())?;

    accept_mc_eula()
        .await
        .change_context(InstallError)
        .attach_printable("You have to read and accept the Minecraft EULA before continuing.")?;

    let directory = create_server()
        .await
        .change_context(InstallError)
        .attach_printable("Failed to create server.")?
        .1;

    create_start_file(directory.clone())
        .await
        .change_context(InstallError)
        .attach_printable("Failed to create start file.")?;

    run_server(directory)
        .await
        .change_context(InstallError)
        .attach_printable("Could not run server")?;

    Ok(0)
}
