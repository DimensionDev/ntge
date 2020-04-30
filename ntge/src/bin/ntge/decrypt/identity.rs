use crate::{util::load_identity_at_path, util::load_local_identities, util::Identity};
use clap::value_t;
use std::path::Path;

pub(crate) fn fetch_identity(arg_matches: &clap::ArgMatches) -> Option<Identity> {
    let request_identity_name =
        value_t!(arg_matches, "identity", String).unwrap_or_else(|e| e.exit());
    let local_identities = load_local_identities();

    if let Some(position) = local_identities
        .iter()
        .position(|i| i.name == request_identity_name)
    {
        Some(local_identities[position].clone())
    } else {
        // try to fetch name as path
        let path = Path::new(&request_identity_name);

        load_identity_at_path(&path.into())
    }
}

#[test]
fn it_loads_local_identities() {
    let identities = load_local_identities();
    println!("{:?}", identities);
}
