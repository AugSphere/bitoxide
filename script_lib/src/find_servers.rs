use std::collections::BTreeSet;

use bitburner_api::NS;

use crate::max_money_rate;

pub fn find_hackable_servers(ns: &NS) -> Vec<String> {
    let hacking_level = ns.get_hacking_level();
    let mut hackable_servers: Vec<String> = find_all_server_names(ns, "home", false)
        .into_iter()
        .filter(|host| {
            ns.has_root_access(host) && ns.get_server_required_hacking_level(host) <= hacking_level
        })
        .collect();
    hackable_servers.sort_by(|a, b| {
        max_money_rate(ns, a)
            .total_cmp(&max_money_rate(ns, b))
            .reverse()
    });
    hackable_servers
}

pub fn find_all_server_names(ns: &NS, first_host: &str, with_home: bool) -> Vec<String> {
    let mut to_visit: Vec<String> = vec![first_host.to_owned()];
    let mut visited = BTreeSet::<String>::new();

    loop {
        let Some(host) = to_visit.pop() else {
            break;
        };
        if visited.contains(&host) {
            continue;
        }
        let Some(neighbors) = ns.scan(Some(&host)) else {
            continue;
        };

        to_visit.extend(neighbors.into_iter().filter(|n| !visited.contains(n)));
        visited.insert(host);
    }
    if !with_home {
        visited.remove("home");
    }
    visited.into_iter().collect()
}
