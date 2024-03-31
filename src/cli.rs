use clap::{Arg, Command};

pub fn generate() -> Command {
    Command::new("load-balancer")
        .name("loadbalancer")
        .author("Limerio")
        .about("Load balancer for learning. Not for production use")
        .version("v0.0.2")
        .arg(
            Arg::new("port")
                .short('p')
                .default_value("8000")
                .help("Port for the load balancer"),
        )
        .arg(
            Arg::new("servers")
                .short('s')
                .required(true)
                .value_delimiter(',')
                .help("List of servers separate by commas"),
        )
}
