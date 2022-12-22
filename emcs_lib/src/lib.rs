pub mod error;
pub mod mc_versions;
pub mod options;

use std::{
    fs::{self, File},
    io::{self, Cursor, Read, Write},
    process::Command,
};

use error::InstallServerError;

pub fn install_mc_server(options: options::Options) -> Result<(), InstallServerError> {
    check_for_java()?;

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            match create_server_files(options).await {
                Ok(_) => (),
                Err(e) => {
                    log::error!("{:?}", e);
                    std::process::exit(1);
                }
            }
        });

    Ok(())
}

pub fn check_for_java() -> Result<(), InstallServerError> {
    Command::new("java").arg("--version").spawn()?;

    Ok(())
}

async fn create_server_files(options: options::Options) -> Result<(), InstallServerError> {
    let res = reqwest::get(options.mcver.to_link()).await?;

    fs::create_dir_all(options.dir())?;

    let mut server_file = File::create(options.dir().join("server.jar"))?;

    let mut content = Cursor::new(res.bytes().await?);

    io::copy(&mut content, &mut server_file)?;

    Command::new("java")
        .args(["-Xmx1024M", "-Xms1024M", "-jar", "./server.jar", "nogui"])
        .current_dir(options.dir())
        .spawn()?;

    let mut eula = File::open(options.dir().join("eula.txt"))?;
    let mut eula_content = String::new();
    eula.read_to_string(&mut eula_content)?;
    let eula_content = eula_content.replace("eula=false", "eula=true");
    let mut eula = File::create(options.dir().join("eula.txt"))?;
    eula.write_all(eula_content.as_bytes())?;

    todo!()
}
