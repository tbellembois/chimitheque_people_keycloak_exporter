use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use log::debug;
use rusqlite::{Connection, OpenFlags};
use serde::Serialize;

#[derive(Serialize, Debug)]
struct Realm<'a> {
    realm: &'a str,
    users: Vec<Person<'a>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct Person<'a> {
    id: i32,
    username: String,
    email: String,
    emailVerified: bool,
    createdTimestamp: u128,
    enabled: bool,
    totp: bool,
    credentials: Vec<String>,
    disableableCredentialTypes: Vec<String>,
    requiredActions: Vec<String>,
    realmRoles: Vec<&'a str>,
    notBefore: u8,
    groups: Vec<String>,
}

fn main() {
    env_logger::init();

    let current_system_time = SystemTime::now();
    let duration_since_epoch = current_system_time.duration_since(UNIX_EPOCH).unwrap();
    let milliseconds_timestamp = duration_since_epoch.as_millis();

    let connection =
        Connection::open_with_flags("storage.db", OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();

    let mut stmt = connection
        .prepare("SELECT person_id, person_email FROM person WHERE person_email != 'admin@chimitheque.fr'")
        .unwrap();

    let mut extracted_emails = HashMap::new();

    let people: Vec<Person> = stmt
        .query_map([], |row| {
            let person_email: String = row.get(1)?;
            let username = person_email.clone().to_ascii_lowercase();
            let email = person_email.clone().to_ascii_lowercase();

            if extracted_emails.get(&email).is_some() {
                panic!("{} already present", email);
            }

            extracted_emails.insert(email.clone(), "not_used");

            Ok(Person {
                id: row.get(0)?,
                username,
                email,
                emailVerified: true,
                createdTimestamp: milliseconds_timestamp,
                enabled: true,
                totp: true,
                credentials: Vec::new(),
                disableableCredentialTypes: Vec::new(),
                requiredActions: Vec::new(),
                realmRoles: ["default-roles-chimitheque"].to_vec(),
                notBefore: 0,
                groups: Vec::new(),
            })
        })
        .unwrap()
        .map(|p| p.unwrap())
        .collect();

    let realm = Realm {
        realm: "chimitheque",
        users: people,
    };
    debug!("{:?}", realm);

    let file = File::create("keycloak.json").unwrap();
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &realm).unwrap();
    writer.flush().unwrap();
}
