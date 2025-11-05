use anyhow::anyhow;
use math_expression_parser::*;
use pest::Parser;

fn main() -> anyhow::Result<()> {
    let pair = Grammar::parse(Rule::input, "(12+34)")?
        .next()
        .ok_or_else(|| anyhow!("Failed to parse input"))?;
    dbg!(pair);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_sum() -> anyhow::Result<()> {
        let pair = Grammar::parse(Rule::input, "(12+34)")?
            .next()
            .ok_or_else(|| anyhow!("No pair"))?;
        assert_eq!(pair.as_rule(), Rule::input);
        assert_eq!(pair.as_span().start(), 0);
        Ok(())
    }
}
