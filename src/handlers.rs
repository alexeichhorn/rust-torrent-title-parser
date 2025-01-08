use fancy_regex::Regex;

use crate::handler_wrapper::{Handler, RegexHandlerOptions};
use crate::transforms;

pub fn add_default_handlers(parser: &mut super::Parser) {
    // Adult
    parser.add_handler(Handler::from_regex(
        "adult",
        |t| &mut t.adult,
        Regex::new(r"(?i)\b(?:xxx|xx)\b").unwrap(), // (?i) = case insensitive
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: true,
            skip_from_title: true,
            ..Default::default()
        },
    ));
    // TODO: add adult keyword pattern here

    // Scene
    parser.add_handler(Handler::from_regex(
        "scene",
        |t| &mut t.scene,
        Regex::new(r"(\b\d{3,4}p\b.*[_. ]WEB[_. ][^D][^L]\b|\b-(?:CAKES|GGEZ|GGWP|GLHF|GOSSIP|NAISU|KOGI|PECULATE|SLOT|EDITH|ETHEL|ELEANOR|B2B|SPAMnEGGS|FTP|DiRT|SYNCOPY|BAE|SuccessfulCrab|NHTFS|SURCODE|B0MBARDIERS)\b)").unwrap(), // removed positive/negative lookahead (compated to Python version)
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));

    // Extras (this stuff can be trashed)
    parser.add_handler(Handler::from_regex(
        "extras",
        |t| &mut t.extras,
        Regex::new(r"(?i)\bNCED\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("NCED"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "extras",
        |t| &mut t.extras,
        Regex::new(r"(?i)\bNCOP\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("NCOP"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "extras",
        |t| &mut t.extras,
        Regex::new(r"(?i)\b(?:Deleted[ .-]*)?Scene(?:s)?\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("Deleted Scene"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "extras",
        |t| &mut t.extras,
        Regex::new(r"(?i)\b(?:19\d{2}|20\d{2})\b.*?\bFeaturettes?\b|\bFeaturettes?\b(?!.*?\b(?:19\d{2}|20\d{2})\b)").unwrap(),
        transforms::chain_transforms(transforms::replace_value("Featurette"), transforms::uniq_concat),
        RegexHandlerOptions {
            skip_from_title: true,
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "extras",
        |t| &mut t.extras,
        Regex::new(r"(?i)\b(?:19\d{2}|20\d{2})\b.*?\bSample\b|\bSample\b(?!.*?\b(?:19\d{2}|20\d{2})\b)").unwrap(),
        transforms::chain_transforms(transforms::replace_value("Sample"), transforms::uniq_concat),
        RegexHandlerOptions {
            skip_from_title: true,
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "extras",
        |t| &mut t.extras,
        Regex::new(r"(?i)\b(?:19\d{2}|20\d{2})\b.*?\bTrailers?\b|\bTrailers?\b(?!.*?\b(?:19\d{2}|20\d{2}|\.(?:Park|And))\b)").unwrap(),
        transforms::chain_transforms(transforms::replace_value("Trailer"), transforms::uniq_concat),
        RegexHandlerOptions {
            skip_from_title: true,
            remove: false,
            ..Default::default()
        },
    ));

    // PPV
    parser.add_handler(Handler::from_regex(
        "ppv",
        |t| &mut t.ppv,
        Regex::new(r"(?i)\bPPV\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            skip_from_title: true,
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "ppv",
        |t| &mut t.ppv,
        Regex::new(r"(?i)\b\W?Fight.?Nights?\W?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            skip_from_title: true,
            remove: false,
            ..Default::default()
        },
    ));

    // First batch of site (before languages to get rid of domain name with country code)
    parser.add_handler(Handler::from_regex(
        "site",
        |t| &mut t.site,
        Regex::new(r"(?i)^(www?[\.,][\w-]+\.[\w-]+(?:\.[\w-]+)?)\s+-\s*").unwrap(),
        transforms::identity,
        RegexHandlerOptions {
            skip_from_title: true,
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "site",
        |t| &mut t.site,
        Regex::new(r"(?i)^((?:www?[\.,])?[\w-]+\.[\w-]+(?:\.[\w-]+)*?)\s+-\s*").unwrap(),
        transforms::identity,
        RegexHandlerOptions {
            skip_if_already_found: false,
            ..Default::default()
        },
    ));

    /*

        # Episode code
    parser.add_handler("episode_code", regex.compile(r"[[(]([a-zA-Z0-9]{8})[\])](?=\.[a-zA-Z0-9]{1,5}$|$)"), uppercase, {"remove": True})
    parser.add_handler("episode_code", regex.compile(r"\[([A-Z0-9]{8})]"), uppercase, {"remove": True}) */

    // Episode code
    parser.add_handler(Handler::from_regex(
        "episode_code",
        |t| &mut t.episode_code,
        Regex::new(r"[\[\(]([0-9A-Za-z]{8})[\]\)](?=\.[0-9A-Za-z]{1,5}$|$)").unwrap(),
        transforms::uppercase,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "episode_code",
        |t| &mut t.episode_code,
        Regex::new(r"\[([A-Z0-9]{8})]").unwrap(),
        transforms::uppercase,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
}
