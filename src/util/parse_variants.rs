pub fn parse_variants(variants_str: &str) -> Vec<(String, u32, u32)> {
    variants_str
        .split(',')
        .filter_map(|s| {
            let parts: Vec<&str> = s.split('_').collect();
            if parts.len() == 3 {
                let name = parts[0].to_string();
                let width = parts[1].parse::<u32>().ok()?;
                let height = parts[2].parse::<u32>().ok()?;
                Some((name, width, height))
            } else {
                None
            }
        })
        .collect()
}
