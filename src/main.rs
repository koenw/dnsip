use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::TokioAsyncResolver;
use std::time::Duration;
use structopt::StructOpt;

fn parse_lookup_ip_strategy(
    src: &str,
) -> Result<hickory_resolver::config::LookupIpStrategy, String> {
    match src.to_ascii_lowercase().as_str() {
        "ipv4only" | "ipv4" | "ip4" | "4" => {
            Ok(hickory_resolver::config::LookupIpStrategy::Ipv4Only)
        }
        "ipv6only" | "ipv6" | "ip6" | "6" => {
            Ok(hickory_resolver::config::LookupIpStrategy::Ipv6Only)
        }
        "ipv4andipv6" | "ipv6andipv4" | "both" => {
            Ok(hickory_resolver::config::LookupIpStrategy::Ipv4AndIpv6)
        }
        "ipv6thenipv4" => Ok(hickory_resolver::config::LookupIpStrategy::Ipv6thenIpv4),
        "ipv4thenipv6" => Ok(hickory_resolver::config::LookupIpStrategy::Ipv4thenIpv6),
        e => Err(format!(
            "Failed to parse IP Strategy: don't know how what to make of '{e}'"
        )),
    }
}

/// Resolve DNS names to IP addresses
///
/// dnsip resolves the given hostname and prints it's addresses to stdout, one per line.
#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
#[structopt(max_term_width = 80)]
#[structopt(name = "dnsip", about = "Resolve DNS names to IP addresses")]
struct Opt {
    #[structopt(long, default_value = "ipv4andipv6", case_insensitive = true, parse(try_from_str = parse_lookup_ip_strategy), possible_values = &["ipv4", "ipv6", "ipv4thenipv6", "ipv6thenipv4", "ipv4andipv6"])]
    ip_strategy: hickory_resolver::config::LookupIpStrategy,

    /// Timeout for the DNS request
    #[structopt(long, parse(try_from_str = parse_duration::parse), default_value = "2s")]
    timeout: Duration,

    /// Use DNSSEC to validate the request
    #[structopt(long)]
    validate: bool,

    /// Show intermediate responses
    #[structopt(long)]
    preserve_intermediates: bool,

    /// Use edns for larger records
    #[structopt(long)]
    edns0: bool,

    /// Be more verbose
    #[structopt(short = "v", parse(from_occurrences), default_value = "0")]
    verbose: usize,

    /// DNS name to resolve
    #[structopt()]
    host: String,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let mut resolver_opts = ResolverOpts::default();
    resolver_opts.timeout = opt.timeout;
    resolver_opts.validate = opt.validate;
    resolver_opts.ip_strategy = opt.ip_strategy;
    resolver_opts.preserve_intermediates = opt.preserve_intermediates;
    resolver_opts.edns0 = opt.edns0;

    let rt = tokio::runtime::Runtime::new()?;
    let resolver =
        rt.block_on(async { TokioAsyncResolver::tokio(ResolverConfig::default(), resolver_opts) });

    match rt.block_on(resolver.lookup_ip(&opt.host)) {
        Ok(results) => {
            for ip in results.iter() {
                println!("{:?}", ip);
            }
        }
        Err(e) => {
            if opt.verbose > 0 {
                eprintln!("No address found for &opt.host: {e}");
            }
        }
    };

    Ok(())
}
