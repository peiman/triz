//! TRIZ engineering parameters: search + contradiction formulation.
//!
//! Pure domain logic — serde only, no framework imports. The 39 TRIZ
//! parameters are embedded at compile time from `data/parameters.json`
//! (repo-root) and parsed once on first use.
//!
//! Honest scope: these parameters are ENGINEERING-DOMAIN only. For
//! software/UX/organizational problems they do not apply — go straight
//! to separation principles + function analysis.

use serde::Serialize;
use std::fmt;
use std::sync::OnceLock;

/// The TRIZ parameter data file, embedded at compile time.
/// Path is relative to this source file: crates/domain/src/ → repo root.
const PARAMS_JSON: &str = include_str!("../../../data/parameters.json");

/// Maximum number of ranked matches returned by `parameter_search`.
const MAX_MATCHES: usize = 5;

/// Score for an exact (case-insensitive) name/alias match.
const SCORE_EXACT: u32 = 100;
/// Score when one string is a substring of the other.
const SCORE_SUBSTRING: u32 = 60;
/// Base score for token-overlap matches (plus the overlap count).
const SCORE_TOKEN_BASE: u32 = 20;

/// Honesty caveat shared by both commands.
const DOMAIN_CAVEAT: &str = "These 39 parameters are engineering-domain only. \
For software/UX/organizational problems, skip parameters and go straight to \
separation principles + function analysis.";

/// One TRIZ engineering parameter as loaded from the data file.
#[derive(Debug, Clone, Serialize, serde::Deserialize, PartialEq)]
pub struct Parameter {
    pub number: u8,
    pub name: String,
    pub gloss: String,
    pub aliases: Vec<String>,
}

/// Shape of the `parameters.json` file (we only need the `parameters` array).
#[derive(serde::Deserialize)]
struct ParametersFile {
    parameters: Vec<Parameter>,
}

/// Parse the embedded parameter data exactly once.
fn parameters() -> &'static [Parameter] {
    static PARAMS: OnceLock<Vec<Parameter>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        let file: ParametersFile =
            serde_json::from_str(PARAMS_JSON).expect("embedded parameters.json is valid");
        file.parameters
    })
}

/// Split a string into lowercase alphanumeric tokens.
fn tokens(s: &str) -> Vec<String> {
    s.split(|c: char| !c.is_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(str::to_lowercase)
        .collect()
}

/// Score one candidate string against an already-normalized query.
fn score_candidate(candidate: &str, query: &str) -> u32 {
    let candidate = candidate.to_lowercase();
    if candidate == query {
        return SCORE_EXACT;
    }
    if candidate.contains(query) || query.contains(&candidate) {
        return SCORE_SUBSTRING;
    }
    let cand_tokens = tokens(&candidate);
    let query_tokens = tokens(query);
    let overlap = query_tokens
        .iter()
        .filter(|t| cand_tokens.contains(t))
        .count() as u32;
    if overlap > 0 {
        SCORE_TOKEN_BASE + overlap
    } else {
        0
    }
}

/// Best score for a parameter across its name + aliases, plus which
/// candidate produced it.
fn score_parameter<'a>(param: &'a Parameter, query: &str) -> (u32, &'a str) {
    let mut best = 0;
    let mut matched_on = param.name.as_str();
    for candidate in std::iter::once(&param.name).chain(param.aliases.iter()) {
        let score = score_candidate(candidate, query);
        if score > best {
            best = score;
            matched_on = candidate.as_str();
        }
    }
    (best, matched_on)
}

/// One ranked parameter match.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ParamMatch {
    pub number: u8,
    pub name: String,
    pub gloss: String,
    pub matched_on: String,
    pub score: u32,
}

/// Result of a parameter search: the query and its ranked matches.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ParameterSearchResult {
    pub query: String,
    pub matches: Vec<ParamMatch>,
}

impl fmt::Display for ParameterSearchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.matches.is_empty() {
            return write!(
                f,
                "No match for \"{}\" — these parameters are engineering-domain \
only; try browsing, or this may be a software/UX problem where parameters \
don't apply.",
                self.query
            );
        }
        writeln!(f, "Parameters matching \"{}\":", self.query)?;
        for (i, m) in self.matches.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{} — {}", m.number, m.name)?;
        }
        Ok(())
    }
}

/// Rank all 39 parameters against `query` and return the top matches.
///
/// Scoring (blind-tested algorithm — do not change): the query is
/// lowercased + trimmed; each parameter is scored as the best over its
/// `[name] + aliases`: exact-equal = 100; substring (either way) = 60;
/// otherwise token overlap = 20 + overlap_count. Parameters with score 0
/// are dropped; the rest are sorted descending and capped at 5.
pub fn parameter_search(query: &str) -> ParameterSearchResult {
    let normalized = query.trim().to_lowercase();
    let mut scored: Vec<ParamMatch> = parameters()
        .iter()
        .filter_map(|p| {
            let (score, matched_on) = score_parameter(p, &normalized);
            if score > 0 {
                Some(ParamMatch {
                    number: p.number,
                    name: p.name.clone(),
                    gloss: p.gloss.clone(),
                    matched_on: matched_on.to_string(),
                    score,
                })
            } else {
                None
            }
        })
        .collect();
    // Stable sort by descending score keeps the data-file order for ties.
    scored.sort_by(|a, b| b.score.cmp(&a.score));
    scored.truncate(MAX_MATCHES);
    ParameterSearchResult {
        query: query.trim().to_string(),
        matches: scored,
    }
}

/// A parameter side of a contradiction, resolved from a free-text term.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ResolvedParam {
    pub query: String,
    /// `None` when the term matched no parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ResolvedParam {
    fn resolve(query: &str) -> Self {
        let top = parameter_search(query).matches.into_iter().next();
        match top {
            Some(m) => ResolvedParam {
                query: query.trim().to_string(),
                number: Some(m.number),
                name: Some(m.name),
            },
            None => ResolvedParam {
                query: query.trim().to_string(),
                number: None,
                name: None,
            },
        }
    }
}

/// Whether a contradiction is between two parameters (Technical) or one
/// parameter that must take opposite values (Physical).
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ContradictionKind {
    Technical,
    Physical,
}

const ROUTE_PHYSICAL: &str = "Physical contradiction (one parameter must take \
opposite values). Resolve with the separation principles: in time / in space / \
upon condition / between system levels.";

const ROUTE_TECHNICAL: &str = "Technical contradiction (two parameters \
conflict). Surface the physical contradiction underneath, then resolve with \
separation principles. The 39x39 contradiction matrix is optional/legacy and \
is not consulted here.";

/// Result of formulating a contradiction from two free-text terms.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ContradictionResult {
    pub improving: ResolvedParam,
    pub worsening: ResolvedParam,
    pub kind: ContradictionKind,
    pub route: String,
    pub note: String,
}

impl fmt::Display for ContradictionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn side(r: &ResolvedParam) -> String {
            match (r.number, &r.name) {
                (Some(n), Some(name)) => format!("{n} — {name}"),
                _ => format!("(no parameter matched \"{}\")", r.query),
            }
        }
        let kind = match self.kind {
            ContradictionKind::Technical => "Technical",
            ContradictionKind::Physical => "Physical",
        };
        writeln!(f, "Improving: {}", side(&self.improving))?;
        writeln!(f, "Worsening: {}", side(&self.worsening))?;
        writeln!(f, "Kind: {kind}")?;
        writeln!(f, "{}", self.route)?;
        write!(f, "Note: {}", self.note)
    }
}

/// Formulate a contradiction from two free-text terms.
///
/// Each side is resolved to its top parameter match via
/// `parameter_search`. Classification: if both sides resolve to the same
/// parameter number it is a Physical contradiction; otherwise Technical.
/// (If a side fails to resolve, it surfaces as a no-match side and the
/// contradiction is treated as Technical.)
pub fn formulate_contradiction(improving: &str, worsening: &str) -> ContradictionResult {
    let improving = ResolvedParam::resolve(improving);
    let worsening = ResolvedParam::resolve(worsening);

    let kind = match (improving.number, worsening.number) {
        (Some(a), Some(b)) if a == b => ContradictionKind::Physical,
        _ => ContradictionKind::Technical,
    };
    let route = match kind {
        ContradictionKind::Physical => ROUTE_PHYSICAL,
        ContradictionKind::Technical => ROUTE_TECHNICAL,
    };

    ContradictionResult {
        improving,
        worsening,
        kind,
        route: route.to_string(),
        note: DOMAIN_CAVEAT.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_file_has_39_parameters() {
        assert_eq!(parameters().len(), 39);
    }

    #[test]
    fn durability_returns_multiple_params_including_14_15_27() {
        let result = parameter_search("durability");
        let numbers: Vec<u8> = result.matches.iter().map(|m| m.number).collect();
        assert!(result.matches.len() > 1, "expected multiple matches");
        for n in [14, 15, 27] {
            assert!(numbers.contains(&n), "expected param {n} in {numbers:?}");
        }
    }

    #[test]
    fn throughput_top_match_is_39() {
        let result = parameter_search("throughput");
        assert_eq!(result.matches.first().unwrap().number, 39);
    }

    #[test]
    fn exact_name_match_scores_100() {
        let result = parameter_search("Strength");
        let top = result.matches.first().unwrap();
        assert_eq!(top.number, 14);
        assert_eq!(top.score, SCORE_EXACT);
    }

    #[test]
    fn results_are_sorted_descending_and_capped() {
        let result = parameter_search("durability");
        assert!(result.matches.len() <= MAX_MATCHES);
        for pair in result.matches.windows(2) {
            assert!(pair[0].score >= pair[1].score, "not sorted descending");
        }
    }

    #[test]
    fn no_match_query_returns_empty() {
        let result = parameter_search("asdfqwerzzz");
        assert!(result.matches.is_empty());
    }

    #[test]
    fn no_match_display_explains_engineering_domain() {
        let result = parameter_search("asdfqwerzzz");
        let text = format!("{result}");
        assert!(text.contains("engineering-domain"));
        assert!(text.contains("software/UX"));
    }

    #[test]
    fn search_result_serializes_with_matches_array() {
        let result = parameter_search("durability");
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["query"], "durability");
        assert!(json["matches"].is_array());
        assert!(json["matches"][0]["number"].is_u64());
    }

    #[test]
    fn weight_vs_strength_is_technical_with_params_1_and_14() {
        let result = formulate_contradiction("weight", "strength");
        assert_eq!(result.kind, ContradictionKind::Technical);
        assert_eq!(result.improving.number, Some(1));
        assert_eq!(result.worsening.number, Some(14));
    }

    #[test]
    fn same_parameter_is_physical() {
        let result = formulate_contradiction("reliability", "reliability");
        assert_eq!(result.kind, ContradictionKind::Physical);
        assert_eq!(result.improving.number, result.worsening.number);
        assert_eq!(result.improving.number, Some(27));
    }

    #[test]
    fn technical_route_mentions_matrix_is_legacy() {
        let result = formulate_contradiction("weight", "strength");
        assert!(result.route.contains("Technical contradiction"));
        assert!(result.route.contains("optional/legacy"));
    }

    #[test]
    fn physical_route_mentions_separation_principles() {
        let result = formulate_contradiction("reliability", "reliability");
        assert!(result.route.contains("separation principles"));
        assert!(result.route.contains("in time / in space"));
    }

    #[test]
    fn no_match_side_is_surfaced() {
        let result = formulate_contradiction("asdfqwerzzz", "strength");
        assert_eq!(result.improving.number, None);
        assert_eq!(result.worsening.number, Some(14));
        // A no-match side is treated as Technical (sides differ).
        assert_eq!(result.kind, ContradictionKind::Technical);
        let text = format!("{result}");
        assert!(text.contains("no parameter matched"));
    }

    #[test]
    fn contradiction_serializes_with_kind_and_note() {
        let result = formulate_contradiction("weight", "strength");
        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["kind"], "technical");
        assert_eq!(json["improving"]["number"], 1);
        assert_eq!(json["worsening"]["number"], 14);
        assert!(json["note"]
            .as_str()
            .unwrap()
            .contains("engineering-domain"));
    }
}
