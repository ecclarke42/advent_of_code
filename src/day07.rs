use std::{
    collections::{
        hash_map::{Entry, HashMap},
        HashSet,
    },
    str::FromStr,
};

use daggy::{Dag, NodeIndex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color<'a>(pub &'a str);

pub struct Rule<'a> {
    color: Color<'a>,
    contents: Vec<(u16, Color<'a>)>,
}

impl<'a> TryFrom<&'a str> for Rule<'a> {
    type Error = RuleParseError;
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let (extra, (color, contents)) =
            parser::bag_rule(input).map_err(|e| RuleParseError::Parsing(e.to_string()))?;
        if !extra.is_empty() {
            Err(RuleParseError::TooMuch)
        } else {
            Ok(Rule {
                color: Color(color),
                contents: contents
                    .into_iter()
                    .map(|(n, color)| (n, Color(color)))
                    .collect(),
            })
        }
    }
}

#[derive(Debug)]
pub enum RuleParseError {
    TooMuch,
    Parsing(String),
}

#[derive(Debug, Default)]
pub struct Tree<'a> {
    dag: Dag<Color<'a>, u16>,
    colors: HashMap<Color<'a>, NodeIndex>,
}

impl<'a> Tree<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an iterator over the colors of bags that at some point contain
    /// (i.e. are an anscestor at any point) a give color bag
    pub fn bags_that_can_contain(
        &self,
        color: &Color<'a>,
    ) -> Option<impl Iterator<Item = &Color<'a>>> {
        let mut parents = HashSet::new();
        populate_ancestor_set(&self.dag, *self.colors.get(color)?, &mut parents);
        Some(
            parents
                .into_iter()
                .filter_map(|idx| self.dag.node_weight(idx)),
        )
    }

    /// Count the total number of bags that a bag of a given color must contain
    pub fn count_contents_of(&self, color: &Color<'a>) -> Option<u16> {
        Some(sum_child_edges(&self.dag, *self.colors.get(color)?, 1))
    }
}

fn populate_ancestor_set<N, E>(dag: &Dag<N, E>, node: NodeIndex, set: &mut HashSet<NodeIndex>) {
    use daggy::Walker;
    dag.parents(node).iter(dag).for_each(|(_, node)| {
        set.insert(node);
        populate_ancestor_set(dag, node, set);
    });
}

// TODO: could probably use E with some type bounds...
fn sum_child_edges<N>(dag: &Dag<N, u16>, node: NodeIndex, multiplier: u16) -> u16 {
    use daggy::Walker;
    dag.children(node)
        .iter(dag)
        .map(|(edge, node)| {
            // Unwrap is fine, since we wouldn't have the index if it didn't exist
            let edge = *dag.edge_weight(edge).unwrap();
            multiplier * (edge + sum_child_edges(dag, node, edge))
        })
        .sum()
}

impl<'a> FromIterator<Rule<'a>> for Tree<'a> {
    fn from_iter<T: IntoIterator<Item = Rule<'a>>>(iter: T) -> Self {
        let mut dag = Dag::new();
        let mut colors = HashMap::new();
        for rule in iter {
            let idx = *colors
                .entry(rule.color)
                .or_insert_with(|| dag.add_node(rule.color));

            for (count, color) in rule.contents {
                match colors.entry(color) {
                    Entry::Occupied(entry) => {
                        // We've already seen this color
                        dag.add_edge(idx, *entry.get(), count)
                            .expect("Failed to add Node"); // TODO: handle this
                    }
                    Entry::Vacant(entry) => {
                        // We haven't seen this color yet
                        let (_, child_idx) = dag.add_child(idx, count, color);
                        entry.insert(child_idx);
                    }
                };
            }
        }

        Self { dag, colors }
    }
}

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take, take_until, take_while},
        character::complete::space0,
        combinator::{map_res, opt},
        multi::separated_list0,
        sequence::{delimited, terminated, tuple},
        IResult, InputLength, Parser,
    };

    use super::*;

    #[allow(clippy::type_complexity)]
    pub fn bag_rule(input: &str) -> IResult<&str, (&str, Vec<(u16, &str)>)> {
        let take_until_and_consume =
            |tag: &'static str| terminated(take_until(tag), take(tag.input_len()));

        let inner_bag = {
            let take_number = map_res(take_while(|c: char| c.is_ascii_digit()), u16::from_str);
            let take_bag_color = delimited(
                space0,
                take_until_and_consume(" bag"),
                tuple((opt(tag("s")), space0)),
            );

            tuple((take_number, take_bag_color))
        };

        let take_bag_color = take_until_and_consume(" bags contain ");
        let take_rules = {
            let rules = terminated(separated_list0(tag(", "), inner_bag), tag("."));
            let no_rules = tag("no other bags.").map(|_| Vec::new());
            alt((rules, no_rules))
        };

        tuple((take_bag_color, take_rules))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_RULES1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const TEST_RULES2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    fn tree(input: &str) -> Tree<'_> {
        input
            .lines()
            .map(|input| Rule::try_from(input).expect("Failed to parse"))
            .collect()
    }

    #[test]
    pub fn test_parents() {
        let parents = tree(TEST_RULES1)
            .bags_that_can_contain(&Color("shiny gold"))
            .expect("Shiny gold not found!")
            .count();

        assert_eq!(parents, 4);
    }

    #[test]
    fn test_contains() {
        let my_bag = Color("shiny gold");

        assert_eq!(
            tree(TEST_RULES1)
                .count_contents_of(&my_bag)
                .expect("Shiny gold not found!"),
            32
        );

        assert_eq!(
            tree(TEST_RULES2)
                .count_contents_of(&my_bag)
                .expect("Shiny gold not found!"),
            126
        );
    }
}
