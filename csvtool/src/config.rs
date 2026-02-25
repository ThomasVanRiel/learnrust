use crate::filter::FilterOp;

pub struct Config {
    pub filename: String,
    pub filters: Vec<(String, FilterOp, String)>,
    pub sort: Option<String>,
    pub limit: Option<usize>,
    pub stats: bool,
}

impl Config {
    pub fn build_filter(filter_string: &String) -> Option<(String, FilterOp, String)> {
        let ops = [
            ("==", FilterOp::Eq),
            ("!=", FilterOp::Ne),
            (">=", FilterOp::Ge),
            ("<=", FilterOp::Se),
            (">", FilterOp::Gt),
            ("<", FilterOp::St),
        ];
        for (op_str, op_obj) in ops {
            if let Some(op_pos) = filter_string.find(op_str) {
                // Check if two sides are present around the operator
                if filter_string.len() > op_pos + op_str.len() {
                    return Some((
                        filter_string[..op_pos].to_string().to_lowercase(),
                        op_obj,
                        filter_string[op_pos + op_str.len()..]
                            .to_string()
                            .to_lowercase(),
                    ));
                } else {
                    println!("Filter syntax is col{op_str}query, not {filter_string}");
                    return None;
                }
            }
        }

        // If no filter string was found
        println!("Filter syntax is col<Op>query, not {filter_string}");
        None
    }

    pub fn new(args: &[String]) -> Result<Config, String> {
        let filename = args.get(1);

        let has_stats_flag = args.iter().any(|a| a.eq("--stats"));

        let filters: Vec<_> = args
            .iter()
            // Loop over the args in tuples (position, arg)
            .enumerate()
            // Filter items on arg == "--filter"
            .filter(|(_, a)| a.as_str() == "--filter")
            // Recreate the iterator by replacing the item by the arg at the next position
            .filter_map(|(i, _)| args.get(i + 1))
            // Replace the items by the filter item (String, FilterOp, String)
            // Closure `|s| Config::build_filter(s)` is shortened to the function because the
            // signature matches. No closure middleman required.
            .filter_map(Config::build_filter)
            // Collect the iterator to Vec<(String, FilterOp, String)>
            .collect();

        // Check if sort flag is in args and get key
        let sort = if let Some(pos) = args.iter().position(|a| a.eq("--sort")) {
            if let Some(sort_key) = args.get(pos + 1) {
                Some(sort_key.to_string())
            } else {
                println!("--sort flag specified but no key provided.");
                None
            }
        } else {
            None
        };

        // Check if limit flag is in args and get key
        let limit = if let Some(pos) = args.iter().position(|a| a.eq("--limit")) {
            if let Some(limit_string) = args.get(pos + 1) {
                Some(limit_string.parse::<usize>().map_err(|e| {
                    format!("Error: {e} while parsing string \"{limit_string}\" to usize")
                })?)
            } else {
                println!("--limit flag specified but no amount specified.");
                None
            }
        } else {
            None
        };

        match (filename, filters, sort, limit, has_stats_flag) {
            (Some(filename), filters, sort, limit, stats) => Ok(Config {
                filename: filename.to_string(),
                filters,
                sort,
                limit,
                stats,
            }),
            _ => Err(String::from(
                "Usage: csvtool <file> [--filter heading=query]",
            )),
        }
    }
}
