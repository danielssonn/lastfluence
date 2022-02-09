/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
 use marine_rs_sdk::marine;

 use marine_rs_sdk::MountedBinaryResult;


const OPENSEA_URL: &str = "https://testnets-api.opensea.io/api/v1/assets?order_direction=desc&offset=0&limit=3";
const RARIBLE_URL: &str = "https://ethereum-api.rarible.org/v0.1/nft/items/all?size=5";


pub fn main() {}

#[marine]
pub struct NFTEcho {
    pub msg:String,
    pub reply: String,
}

#[marine]
pub fn opensea_fetch_len() -> NFTEcho {
    log::info!("fetching from OpenSea");

    let curl_says = curl(vec![OPENSEA_URL.to_owned()]) ;
    let result = String::from_utf8(curl_says.stdout).unwrap();

    // String::from_utf8(result.stdout).unwrap();

    NFTEcho{
        msg: format!("NFTs Size from OpenSea"),
        reply: result.to_string().len().to_string()

    }
}

#[marine]
pub fn rarible_fetch_len() -> NFTEcho {
    log::info!("fetching from Rarible");

    let curl_says = curl(vec![RARIBLE_URL.to_owned()]) ;
    let result = String::from_utf8(curl_says.stdout).unwrap();

    NFTEcho{
        msg: format!("NFTs Size on Rarible"),
        reply: result.to_string().len().to_string()

    }
    
}

#[marine]
pub fn opensea_fetch() -> NFTEcho {
    log::info!("fetching from OpenSea");

    let curl_says = curl(vec![OPENSEA_URL.to_owned()]) ;
    let result = String::from_utf8(curl_says.stdout).unwrap();

    // String::from_utf8(result.stdout).unwrap();

    NFTEcho{
        msg: format!("NFTs from OpenSea"),
        reply: result.to_string()

    }
}

#[marine]
pub fn rarible_fetch() -> NFTEcho {
    log::info!("fetching from Rarible");

    let curl_says = curl(vec![RARIBLE_URL.to_owned()]) ;
    let result = String::from_utf8(curl_says.stdout).unwrap();

    NFTEcho{
        msg: format!("NFTs from Rarible"),
        reply: result.to_string()

    }
    
}

#[marine]
pub fn hello(from: String) -> NFTEcho {
    log::info!("echo from NFT module");

    NFTEcho{
        msg: format!("NFT Hello from: \n{}", from),
        reply: format!("NFT Hello back to you \n{}", from),

    }
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    fn curl(cmd: Vec<String>) -> MountedBinaryResult;
}