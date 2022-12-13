use std::{error::Error, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum MCVersions {
    V1_12_2,
    V1_13,
    V1_13_1,
    V1_13_2,
    V1_14,
    V1_14_1,
    V1_14_2,
    V1_14_3,
    V1_14_4,
    V1_15,
    V1_15_1,
    V1_15_2,
    V1_16,
    V1_16_1,
    V1_16_2,
    V1_16_3,
    V1_16_4,
    V1_16_5,
    V1_17,
    V1_17_1,
    V1_18,
    V1_18_1,
    V1_18_2,
    V1_19,
    V1_19_1,
    V1_19_2,
    V1_19_3,
}

impl MCVersions {
    pub fn to_link(&self) -> &str {
        match self {
            Self::V1_12_2 => "https://launcher.mojang.com/v1/objects/886945bfb2b978778c3a0288fd7fab09d315b25f/server.jar",
            Self::V1_13 => "https://launcher.mojang.com/v1/objects/d0caafb8438ebd206f99930cfaecfa6c9a13dca0/server.jar",
            Self::V1_13_1 => "https://launcher.mojang.com/v1/objects/fe123682e9cb30031eae351764f653500b7396c9/server.jar",
            Self::V1_13_2 => "https://launcher.mojang.com/v1/objects/3737db93722a9e39eeada7c27e7aca28b144ffa7/server.jar",
            Self::V1_14 => "https://launcher.mojang.com/v1/objects/f1a0073671057f01aa843443fef34330281333ce/server.jar",
            Self::V1_14_1 => "https://launcher.mojang.com/v1/objects/ed76d597a44c5266be2a7fcd77a8270f1f0bc118/server.jar",
            Self::V1_14_2 => "https://launcher.mojang.com/v1/objects/808be3869e2ca6b62378f9f4b33c946621620019/server.jar",
            Self::V1_14_3 => "https://launcher.mojang.com/v1/objects/d0d0fe2b1dc6ab4c65554cb734270872b72dadd6/server.jar",
            Self::V1_14_4 => "https://launcher.mojang.com/v1/objects/3dc3d84a581f14691199cf6831b71ed1296a9fdf/server.jar",
            Self::V1_15 => "https://launcher.mojang.com/v1/objects/e9f105b3c5c7e85c7b445249a93362a22f62442d/server.jar",
            Self::V1_15_1 => "https://launcher.mojang.com/v1/objects/4d1826eebac84847c71a77f9349cc22afd0cf0a1/server.jar",
            Self::V1_15_2 => "https://launcher.mojang.com/v1/objects/bb2b6b1aefcd70dfd1892149ac3a215f6c636b07/server.jar",
            Self::V1_16 => "https://launcher.mojang.com/v1/objects/a0d03225615ba897619220e256a266cb33a44b6b/server.jar",
            Self::V1_16_1 => "https://launcher.mojang.com/v1/objects/a412fd69db1f81db3f511c1463fd304675244077/server.jar",
            Self::V1_16_2 => "https://launcher.mojang.com/v1/objects/c5f6fb23c3876461d46ec380421e42b289789530/server.jar",
            Self::V1_16_3 => "https://launcher.mojang.com/v1/objects/f02f4473dbf152c23d7d484952121db0b36698cb/server.jar",
            Self::V1_16_4 => "https://launcher.mojang.com/v1/objects/35139deedbd5182953cf1caa23835da59ca3d7cd/server.jar",
            Self::V1_16_5 => "https://launcher.mojang.com/v1/objects/1b557e7b033b583cd9f66746b7a9ab1ec1673ced/server.jar",
            Self::V1_17 => "https://launcher.mojang.com/v1/objects/0a269b5f2c5b93b1712d0f5dc43b6182b9ab254e/server.jar",
            Self::V1_17_1 => "https://launcher.mojang.com/v1/objects/a16d67e5807f57fc4e550299cf20226194497dc2/server.jar",
            Self::V1_18 => "https://launcher.mojang.com/v1/objects/3cf24a8694aca6267883b17d934efacc5e44440d/server.jar",
            Self::V1_18_1 => "https://launcher.mojang.com/v1/objects/125e5adf40c659fd3bce3e66e67a16bb49ecc1b9/server.jar",
            Self::V1_18_2 => "https://launcher.mojang.com/v1/objects/c8f83c5655308435b3dcf03c06d9fe8740a77469/server.jar",
            Self::V1_19 => "https://launcher.mojang.com/v1/objects/e00c4052dac1d59a1188b2aa9d5a87113aaf1122/server.jar",
            Self::V1_19_1 => "https://piston-data.mojang.com/v1/objects/8399e1211e95faa421c1507b322dbeae86d604df/server.jar",
            Self::V1_19_2 => "https://piston-data.mojang.com/v1/objects/f69c284232d7c7580bd89a5a4931c3581eae1378/server.jar",
            Self::V1_19_3 => "https://piston-data.mojang.com/v1/objects/c9df48efed58511cdd0213c56b9013a7b5c9ac1f/server.jar",
        }
    }
}

impl std::fmt::Display for MCVersions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MCVersions::V1_12_2 => write!(f, "1.12.2"),
            MCVersions::V1_13 => write!(f, "1.13"),
            MCVersions::V1_13_1 => write!(f, "1.13.1"),
            MCVersions::V1_13_2 => write!(f, "1.13.2"),
            MCVersions::V1_14 => write!(f, "1.14"),
            MCVersions::V1_14_1 => write!(f, "1.14.1"),
            MCVersions::V1_14_2 => write!(f, "1.14.2"),
            MCVersions::V1_14_3 => write!(f, "1.14.3"),
            MCVersions::V1_14_4 => write!(f, "1.14.4"),
            MCVersions::V1_15 => write!(f, "1.15"),
            MCVersions::V1_15_1 => write!(f, "1.15.1"),
            MCVersions::V1_15_2 => write!(f, "1.15.2"),
            MCVersions::V1_16 => write!(f, "1.16"),
            MCVersions::V1_16_1 => write!(f, "1.16.1"),
            MCVersions::V1_16_2 => write!(f, "1.16.2"),
            MCVersions::V1_16_3 => write!(f, "1.16.3"),
            MCVersions::V1_16_4 => write!(f, "1.16.4"),
            MCVersions::V1_16_5 => write!(f, "1.16.5"),
            MCVersions::V1_17 => write!(f, "1.17"),
            MCVersions::V1_17_1 => write!(f, "1.17.1"),
            MCVersions::V1_18 => write!(f, "1.18"),
            MCVersions::V1_18_1 => write!(f, "1.18.1"),
            MCVersions::V1_18_2 => write!(f, "1.18.2"),
            MCVersions::V1_19 => write!(f, "1.19"),
            MCVersions::V1_19_1 => write!(f, "1.19.1"),
            MCVersions::V1_19_2 => write!(f, "1.19.2"),
            MCVersions::V1_19_3 => write!(f, "1.19.3"),
        }
    }
}

impl FromStr for MCVersions {
    type Err = MCVerParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1.12.2" => Ok(Self::V1_12_2),
            "1.13" => Ok(Self::V1_13),
            "1.13.1" => Ok(Self::V1_13_1),
            "1.13.2" => Ok(Self::V1_13_2),
            "1.14" => Ok(Self::V1_14),
            "1.14.1" => Ok(Self::V1_14_1),
            "1.14.2" => Ok(Self::V1_14_2),
            "1.14.3" => Ok(Self::V1_14_3),
            "1.14.4" => Ok(Self::V1_14_4),
            "1.15" => Ok(Self::V1_15),
            "1.15.1" => Ok(Self::V1_15_1),
            "1.15.2" => Ok(Self::V1_15_2),
            "1.16" => Ok(Self::V1_16),
            "1.16.1" => Ok(Self::V1_16_1),
            "1.16.2" => Ok(Self::V1_16_2),
            "1.16.3" => Ok(Self::V1_16_3),
            "1.16.4" => Ok(Self::V1_16_4),
            "1.16.5" => Ok(Self::V1_16_5),
            "1.17" => Ok(Self::V1_17),
            "1.17.1" => Ok(Self::V1_17_1),
            "1.18" => Ok(Self::V1_18),
            "1.18.1" => Ok(Self::V1_18_1),
            "1.18.2" => Ok(Self::V1_18_2),
            "1.19" => Ok(Self::V1_19),
            "1.19.1" => Ok(Self::V1_19_1),
            "1.19.2" => Ok(Self::V1_19_2),
            "1.19.3" => Ok(Self::V1_19_3),
            _ => Err(MCVerParseErr {}),
        }
    }
}

#[derive(Debug)]
pub struct MCVerParseErr;

impl Error for MCVerParseErr {}

impl std::fmt::Display for MCVerParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Couldn't parse mc version.")
    }
}
