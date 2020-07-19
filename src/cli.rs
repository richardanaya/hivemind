use clap::Clap;

#[derive(Clap)]
#[clap(
    version = "0.0.0",
    author = "Richard Anaya <https://github.com/richardanaya/>"
)]
pub struct Opts {
    /// Show verbose logs
    #[clap(short)]
    pub verbose: bool,

    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Join(JoinCluster),
    Run(RunNode),
    Get(GetKey),
    Set(SetKey),
}

/// Get the value of a key
#[derive(Clap)]
pub struct GetKey {
    /// Key
    pub key: String,

    /// Address of cluster node
    #[clap(short)]
    pub cluster_node_address: String,
}

/// Set the value of a key
#[derive(Clap)]
pub struct SetKey {
    /// Key
    pub key: String,

    /// Value
    pub value: String,

    /// Type of value
    #[clap(short)]
    pub value_type: String,

    /// Address of cluster node
    #[clap(short)]
    pub cluster_node_address: String,
}

/// Run a hivemind node
#[derive(Clap)]
pub struct RunNode {
    /// Host of node
    #[clap(short, default_value = "127.0.0.1")]
    pub host: String,

    /// Port of local node
    #[clap(short, default_value = "9900")]
    pub port: u16,
}

/// Join a cluster of hivemind nodes
#[derive(Clap)]
pub struct JoinCluster {
    /// Address of cluster node
    pub cluster_node_address: String,

    /// Host of local node
    #[clap(short, default_value = "127.0.0.1")]
    pub host: String,

    /// Port of local node
    #[clap(short, default_value = "9900")]
    pub port: u16,
}
