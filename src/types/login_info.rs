use crate::utils::random::{random_hex, random_mac_address};

#[derive(Debug, Default)]
pub struct LoginInfo {
    pub uuid: String,
    pub protocol: String,
    pub fhash: String,
    pub mac: String,
    pub requested_name: String,
    pub hash2: String,
    pub fz: String,
    pub f: String,
    pub player_age: String,
    pub game_version: String,
    pub lmode: String,
    pub cbits: String,
    pub rid: String,
    pub gdpr: String,
    pub hash: String,
    pub category: String,
    pub token: String,
    pub total_playtime: String,
    pub door_id: String,
    pub klv: String,
    pub meta: String,
    pub platform_id: String,
    pub device_version: String,
    pub zf: String,
    pub country: String,
    pub user: String,
    pub wk: String,
}

impl LoginInfo {
    pub fn new() -> LoginInfo {
        LoginInfo {
            uuid: String::new(),
            protocol: "209".to_string(),
            fhash: "-716928004".to_string(),
            mac: random_mac_address(),
            requested_name: "BraveDuck".to_string(),
            hash2: String::new(),
            fz: "47142936".to_string(),
            f: "1".to_string(),
            player_age: "20".to_string(),
            game_version: "4.63".to_string(),
            lmode: "1".to_string(),
            cbits: "1040".to_string(),
            rid: random_hex(32, true),
            gdpr: "3".to_string(),
            hash: "0".to_string(),
            category: "_-5100".to_string(),
            token: String::new(),
            total_playtime: "0".to_string(),
            door_id: String::new(),
            klv: String::new(),
            meta: String::new(),
            platform_id: "0,1,1".to_string(),
            device_version: "0".to_string(),
            zf: "-821693372".to_string(),
            country: "us".to_string(),
            user: String::new(),
            wk: random_hex(32, true),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "uuid|{}\nprotocol|{}\nfhash|{}\nmac|{}\nrequested_name|{}\nhash2|{}\nfz|{}\nf|{}\nplayer_age|{}\ngame_version|{}\nlmode|{}\ncbits|{}\nrid|{}\ngdpr|{}\nhash|{}\ncategory|{}\ntoken|{}\ntotal_playtime|{}\ndoor_id|{}\nklv|{}\nmeta|{}\nplatform_id|{}\ndevice_version|{}\nzf|{}\ncountry|{}\nuser|{}\nwk|{}",
            self.uuid, self.protocol, self.fhash, self.mac, self.requested_name, self.hash2, self.fz, self.f, self.player_age, self.game_version, self.lmode, self.cbits, self.rid, self.gdpr, self.hash, self.category, self.token, self.total_playtime, self.door_id, self.klv, self.meta, self.platform_id, self.device_version, self.zf, self.country, self.user, self.wk
        )
    }
}
