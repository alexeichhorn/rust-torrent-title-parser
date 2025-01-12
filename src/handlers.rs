use regex::Regex as LiteRegex;
use regress::Regex;

use crate::extensions::regex::RegexStringExt;
use crate::handler_wrapper::{Handler, HandlerResult, Match, RegexHandlerOptions};
use crate::transforms;
use lazy_static::lazy_static;

pub fn add_default_handlers(parser: &mut super::Parser) {
    // Adult
    parser.add_handler(Handler::from_regex(
        "adult",
        |t| &mut t.adult,
        Regex::case_insensitive(r"\b(?:xxx|xx)\b").unwrap(),
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
        Regex::case_insensitive(r"\bNCED\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("NCED"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "extras",
        |t| &mut t.extras,
        Regex::case_insensitive(r"\bNCOP\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("NCOP"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "extras",
        |t| &mut t.extras,
        Regex::case_insensitive(r"\b(?:Deleted[ .-]*)?Scene(?:s)?\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("Deleted Scene"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "extras",
        |t| &mut t.extras,
        Regex::case_insensitive(r"\b(?:19\d{2}|20\d{2})\b.*?\bFeaturettes?\b|\bFeaturettes?\b(?!.*?\b(?:19\d{2}|20\d{2})\b)").unwrap(),
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
        Regex::case_insensitive(r"\b(?:19\d{2}|20\d{2})\b.*?\bSample\b|\bSample\b(?!.*?\b(?:19\d{2}|20\d{2})\b)").unwrap(),
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
        Regex::case_insensitive(r"\b(?:19\d{2}|20\d{2})\b.*?\bTrailers?\b|\bTrailers?\b(?!.*?\b(?:19\d{2}|20\d{2}|\.(?:Park|And))\b)")
            .unwrap(),
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
        Regex::case_insensitive(r"\bPPV\b").unwrap(),
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
        Regex::case_insensitive(r"\b\W?Fight.?Nights?\W?\b").unwrap(),
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
        Regex::case_insensitive(r"^(www?[\.,][\w-]+\.[\w-]+(?:\.[\w-]+)?)\s+-\s*").unwrap(),
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
        Regex::case_insensitive(r"^((?:www?[\.,])?[\w-]+\.[\w-]+(?:\.[\w-]+)*?)\s+-\s*").unwrap(),
        transforms::identity,
        RegexHandlerOptions {
            skip_if_already_found: false,
            ..Default::default()
        },
    ));

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

    // Resolution
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::case_insensitive(r"\[?\]?3840x\d{4}[\])?]?").unwrap(),
        transforms::value("2160p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::case_insensitive(r"\[?\]?1920x\d{3,4}[\])?]?").unwrap(),
        transforms::value("1080p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::case_insensitive(r"\[?\]?1280x\d{3}[\])?]?").unwrap(),
        transforms::value("720p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::case_insensitive(r"\[?\]?(\d{3,4}x\d{3,4})[\])?]?p?").unwrap(),
        transforms::value("$1p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::case_insensitive(r"(480|720|1080)0[pi]").unwrap(),
        transforms::value("$1p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::case_insensitive(r"(?:QHD|QuadHD|WQHD|2560(\d+)?x(\d+)?1440p?)").unwrap(),
        transforms::value("1440p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::case_insensitive(r"(?:Full HD|FHD|1920(\d+)?x(\d+)?1080p?)").unwrap(),
        transforms::value("1080p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::case_insensitive(r"(?:BD|HD|M)(2160p?|4k)").unwrap(),
        transforms::value("2160p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::case_insensitive(r"(?:BD|HD|M)1080p?").unwrap(),
        transforms::value("1080p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::case_insensitive(r"(?:BD|HD|M)720p?").unwrap(),
        transforms::value("720p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::case_insensitive(r"(?:BD|HD|M)480p?").unwrap(),
        transforms::value("480p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::case_insensitive(r"\b(?:4k|2160p|1080p|720p|480p)(?!.*\b(?:4k|2160p|1080p|720p|480p)\b)").unwrap(),
        transforms::resolution_transform,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::case_insensitive(r"\b4k|21600?[pi]\b").unwrap(),
        transforms::value("2160p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::case_insensitive(r"(\d{3,4})[pi]").unwrap(),
        transforms::value("$1p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::case_insensitive(r"(240|360|480|576|720|1080|2160|3840)[pi]").unwrap(),
        transforms::lowercase,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));

    // Trash
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::case_insensitive(r"\b(?:H[DQ][ .-]*)?CAM(?!.?(S|E|\()\d+)(?:H[DQ])?(?:[ .-]*Rip|Rp)?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::case_insensitive(r"\b(?:H[DQ][ .-]*)?S[ \.\-]print\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::case_insensitive(r"\b(?:HD[ .-]*)?T(?:ELE)?(C|S)(?:INE|YNC)?(?:Rip)?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::case_insensitive(r"\bPre.?DVD(?:Rip)?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::case_insensitive(r"\b(?:DVD?|BD|BR)?[ .-]*Scr(?:eener)?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::case_insensitive(r"\bDVB[ .-]*(?:Rip)?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::case_insensitive(r"\bSAT[ .-]*Rips?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::case_insensitive(r"\bLeaked\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::case_insensitive(r"threesixtyp").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::case_insensitive(r"\bR5|R6\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::case_insensitive(r"(?:Deleted[ .-]*)?Scene(?:s)?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::case_insensitive(r"\bHQ.?(Clean)?.?(Aud(io)?)?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));

    // Date
    parser.add_handler(Handler::from_regex(
        "date",
        |t| &mut t.date,
        Regex::new(r"(?:\W|^)\[?(?:19[6-9]|20[012])[0-9][. \-/\\](?:0[1-9]|1[012])[. \-/\\](?:0[1-9]|[12][0-9]|3[01])\]?(?:\W|$)").unwrap(),
        transforms::date_from_format("%Y %m %d"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "date",
        |t| &mut t.date,
        Regex::new(r"(?:\W|^)(\[?\]?(?:0[1-9]|[12][0-9]|3[01])([. \-/\\])(?:0[1-9]|1[012])\2(?:19[6-9]|20[01])[0-9][\])]?)(?:\W|$)")
            .unwrap(),
        transforms::date_from_format("%d %m %Y"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "date",
        |t| &mut t.date,
        Regex::new(r"(?:\W)(\[?\]?(?:0[1-9]|1[012])([. \-/\\])(?:0[1-9]|[12][0-9]|3[01])\2(?:[0][1-9]|[0126789][0-9])[\])]?)(?:\W|$)")
            .unwrap(),
        transforms::date_from_format("%m %d %y"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "date",
        |t| &mut t.date,
        Regex::new(r"(?:\W)(\[?\]?(?:0[1-9]|[12][0-9]|3[01])([. \-/\\])(?:0[1-9]|1[012])\2(?:[0][1-9]|[0126789][0-9])[\])]?)(?:\W|$)")
            .unwrap(),
        transforms::date_from_format("%d %m %y"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "date",
        |t| &mut t.date,
        Regex::case_insensitive(r"(?:\W|^)\[?(?:0?[1-9]|[12][0-9]|3[01])[. ]?(?:st|nd|rd|th)?[. \-/\\](?:feb(?:ruary)?|jan(?:uary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sept?(?:ember)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)[. \-/\\](?:19[7-9]|20[012])[0-9]\]?(?:\W|$)").unwrap(),
        transforms::date_from_formats(&[
            "%d %b %Y",      // regular format (9 Dec 2019)
            "%dst %b %Y",    // for 1st, 21st, 31st
            "%dnd %b %Y",    // for 2nd, 22nd
            "%drd %b %Y",    // for 3rd, 23rd
            "%dth %b %Y",    // for 4th, 5th, etc
            "%d %B %Y",      // full month name without ordinal
            "%dst %B %Y",    // full month name versions
            "%dnd %B %Y",
            "%drd %B %Y",
            "%dth %B %Y",
        ]),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "date",
        |t| &mut t.date,
        Regex::case_insensitive(r"(?:\W|^)(\[?\]?(?:0?[1-9]|[12][0-9]|3[01])[. ]?(?:st|nd|rd|th)?([. \-\/\\])(?:feb(?:ruary)?|jan(?:uary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sept?(?:ember)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)\2(?:0[1-9]|[0126789][0-9])[\])]?)(?:\W|$)").unwrap(),
        transforms::date_from_format("%d %b %y"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "date",
        |t| &mut t.date,
        Regex::new(r"(?:\W|^)(\[?\]?20[012][0-9](?:0[1-9]|1[012])(?:0[1-9]|[12][0-9]|3[01])[\])]?)(?:\W|$)").unwrap(),
        transforms::date_from_format("%Y%m%d"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));

    // Complete
    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::new(r"\b((?:19\d|20[012])\d[ .]?-[ .]?(?:19\d|20[012])\d)\b").unwrap(), // year range
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::new(r"[\[\(][ .]?((19\d|20[012])\d[ .]?-[ .]?\d{2})[ .]?[\]\)]").unwrap(), // year range
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));

    // Bit Rate
    parser.add_handler(Handler::from_regex(
        "bitrate",
        |t| &mut t.bitrate,
        Regex::case_insensitive(r"\b\d+[kmg]bps\b").unwrap(),
        transforms::lowercase,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));

    // Year
    parser.add_handler(Handler::from_regex(
        "year",
        |t| &mut t.year,
        Regex::new(r"\b(20[0-9]{2}|2100)(?!\D*\d{4}\b)").unwrap(),
        transforms::parse::<i32>,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    /*parser.add_handler(Handler::from_regex( // TODO: temporarily disabled as regex doesn't really work with fancy_regex
        "year",
        |t| &mut t.year,
        Regex::case_insensitive(r"[([]?(?!^)(?:^|[^0-9])((?:19\d|20[012])\d)(?!\d|kbps)[)\]]?").unwrap(),
        transforms::parse::<i32>,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));*/
    parser.add_handler(Handler::from_regex(
        "year",
        |t| &mut t.year,
        Regex::case_insensitive(r"^[\[\(]?((?:19[0-9]|20[012])[0-9])(?![0-9]|kbps)[\]\)]?").unwrap(),
        transforms::parse::<i32>,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));

    // Edition
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::case_insensitive(r"\b\d{2,3}(th)?[\.\s\-\+_\/(),]Anniversary[\.\s\-\+_\/(),](Edition|Ed)?\b").unwrap(),
        transforms::value("Anniversary Edition"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::case_insensitive(r"\bUltimate[\.\s\-\+_\/(),]Edition\b").unwrap(),
        transforms::value("Ultimate Edition"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::case_insensitive(r#"\bExtended[\.\s\-\+_\/(),]Director\"?s\b"#).unwrap(),
        transforms::value("Directors Cut"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::case_insensitive(r"\b(custom.?)?Extended\b").unwrap(),
        transforms::value("Extended Edition"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::case_insensitive(r#"\bDirector\"?s[\.\s\-\+_\/(),]Cut\b"#).unwrap(),
        transforms::value("Directors Cut"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::case_insensitive(r#"\bCollector\"?s\b"#).unwrap(),
        transforms::value("Collectors Edition"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::case_insensitive(r"\bTheatrical\b").unwrap(),
        transforms::value("Theatrical"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::case_insensitive(r"\bUncut\b").unwrap(),
        transforms::value("Uncut"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::case_insensitive(r"\bIMAX\b").unwrap(),
        transforms::value("IMAX"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::case_insensitive(r#"\b\.Diamond\.\b"#).unwrap(),
        transforms::value("Diamond Edition"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::case_insensitive(r"\bRemaster(?:ed)?\b").unwrap(),
        transforms::value("Remastered"),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: true,
            ..Default::default()
        },
    ));

    // Upscaled
    parser.add_handler(Handler::from_regex(
        "upscaled",
        |t| &mut t.upscaled,
        Regex::case_insensitive(r"\b(?:AI.?)?(Upscal(ed?|ing)|Enhanced?)\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "upscaled",
        |t| &mut t.upscaled,
        Regex::case_insensitive(r"\b(?:iris2|regrade|ups(uhd|fhd|hd|4k))\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "upscaled",
        |t| &mut t.upscaled,
        Regex::case_insensitive(r"\b\.AI\.\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    // Convert
    parser.add_handler(Handler::from_regex(
        "convert",
        |t| &mut t.convert,
        Regex::case_insensitive(r"\bCONVERT\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    // Hardcoded
    parser.add_handler(Handler::from_regex(
        "hardcoded",
        |t| &mut t.hardcoded,
        Regex::case_insensitive(r"\bHC|HARDCODED\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    // Proper
    parser.add_handler(Handler::from_regex(
        "proper",
        |t| &mut t.proper,
        Regex::case_insensitive(r"\b(?:REAL.)?PROPER\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    // Repack
    parser.add_handler(Handler::from_regex(
        "repack",
        |t| &mut t.repack,
        Regex::case_insensitive(r"\bREPACK|RERIP\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    // Retail
    parser.add_handler(Handler::from_regex(
        "retail",
        |t| &mut t.retail,
        Regex::case_insensitive(r"\bRetail\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));

    // Remastered
    parser.add_handler(Handler::from_regex(
        "remastered",
        |t| &mut t.remastered,
        Regex::case_insensitive(r"\bRemaster(?:ed)?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    // Documentary
    parser.add_handler(Handler::from_regex(
        "documentary",
        |t| &mut t.documentary,
        Regex::case_insensitive(r"\bDOCU(?:menta?ry)?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            skip_from_title: true,
            ..Default::default()
        },
    ));

    // Unrated
    parser.add_handler(Handler::from_regex(
        "unrated",
        |t| &mut t.unrated,
        Regex::case_insensitive(r"\bunrated|uncensored\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    // Region
    parser.add_handler(Handler::from_regex(
        "region",
        |t| &mut t.region,
        Regex::new(r"R\d\b").unwrap(),
        transforms::identity,
        RegexHandlerOptions {
            skip_if_first: true,
            ..Default::default()
        },
    ));

    // Quality
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\b(?:HD[ .-]*)?T(?:ELE)?S(?:YNC)?(?:Rip)?\b").unwrap(),
        transforms::value("TeleSync"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\b(?:HD[ .-]*)?T(?:ELE)?C(?:INE)?(?:Rip)?\b").unwrap(),
        transforms::value("TeleCine"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\b(?:DVD?|BD|BR)?[ .-]*Scr(?:eener)?\b").unwrap(),
        transforms::value("SCR"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bP(?:RE)?-?(HD|DVD)(?:Rip)?\b").unwrap(),
        transforms::value("SCR"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bBlu[ .-]*Ray\b(?=.*remux)").unwrap(),
        transforms::value("BluRay REMUX"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"(?:BD|BR|UHD)[- ]?remux").unwrap(),
        transforms::value("BluRay REMUX"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"remux.*\bBlu[ .-]*Ray\b").unwrap(),
        transforms::value("BluRay REMUX"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bremux\b").unwrap(),
        transforms::value("REMUX"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bBlu[ .-]*Ray\b(?![ .-]*Rip)").unwrap(),
        transforms::value("BluRay"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bUHD[ .-]*Rip\b").unwrap(),
        transforms::value("UHDRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bHD[ .-]*Rip\b").unwrap(),
        transforms::value("HDRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bMicro[ .-]*HD\b").unwrap(),
        transforms::value("HDRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\b(?:BR|Blu[ .-]*Ray)[ .-]*Rip\b").unwrap(),
        transforms::value("BRRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bBD[ .-]*Rip\b|\bBDR\b|\bBD-RM\b|\[BD[\]\) .,-]|\(BD[\]\) .,-]").unwrap(),
        transforms::value("BDRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\b(?:HD[ .-]*)?DVD[ .-]*Rip\b").unwrap(),
        transforms::value("DVDRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bVHS[ .-]*Rip?\b").unwrap(),
        transforms::value("VHSRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bDVD(?:R\d?|.*Mux)?\b").unwrap(),
        transforms::value("DVD"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bVHS\b").unwrap(),
        transforms::value("VHS"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bPPVRip\b").unwrap(),
        transforms::value("PPVRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bHD.?TV.?Rip\b").unwrap(),
        transforms::value("HDTVRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bHD.?TV\b").unwrap(),
        transforms::value("HDTV"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bDVB[ .-]*(?:Rip)?\b").unwrap(),
        transforms::value("HDTV"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bSAT[ .-]*Rips?\b").unwrap(),
        transforms::value("SATRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bTVRips?\b").unwrap(),
        transforms::value("TVRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bR5\b").unwrap(),
        transforms::value("R5"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\b(?:DL|WEB|BD|BR)MUX\b").unwrap(),
        transforms::value("WEBMux"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bWEB[ .-]*Rip\b").unwrap(),
        transforms::value("WEBRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bWEB[ .-]?DL[ .-]?Rip\b").unwrap(),
        transforms::value("WEB-DLRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bWEB[ .-]*(DL|.BDrip|.DLRIP)\b").unwrap(),
        transforms::value("WEB-DL"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\b(?<!\w.)WEB\b|\bWEB(?!([ \.\-\(\],]+\d))\b").unwrap(),
        transforms::value("WEB"),
        RegexHandlerOptions {
            remove: true,
            skip_from_title: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\b(?:H[DQ][ .-]*)?CAM(?!.?(S|E|\()\d+)(?:H[DQ])?(?:[ .-]*Rip|Rp)?\b").unwrap(),
        transforms::value("CAM"),
        RegexHandlerOptions {
            remove: true,
            skip_from_title: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\b(?:H[DQ][ .-]*)?S[ \.\-]print").unwrap(),
        transforms::value("CAM"),
        RegexHandlerOptions {
            remove: true,
            skip_from_title: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::case_insensitive(r"\bPDTV\b").unwrap(),
        transforms::value("PDTV"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));

    /*

    # Video depth
    parser.add_handler("bit_depth", regex.compile(r"\bhevc\s?10\b", regex.IGNORECASE), value("10bit"))
    parser.add_handler("bit_depth", regex.compile(r"(?:8|10|12)[-\.]?(?=bit)", regex.IGNORECASE), value("$1bit"), {"remove": True})
    parser.add_handler("bit_depth", regex.compile(r"\bhdr10\b", regex.IGNORECASE), value("10bit"))
    parser.add_handler("bit_depth", regex.compile(r"\bhi10\b", regex.IGNORECASE), value("10bit"))



    def handle_bit_depth(context):
        result = context["result"]
        if "bit_depth" in result:
            # Replace hyphens and spaces with nothing (effectively removing them)
            result["bit_depth"] = result["bit_depth"].replace(" ", "").replace("-", "")

    parser.add_handler("bit_depth", handle_bit_depth)
     */

    // Video depth
    parser.add_handler(Handler::from_regex(
        "bit_depth",
        |t| &mut t.bit_depth,
        Regex::case_insensitive(r"\bhevc\s?10\b").unwrap(),
        transforms::value("10bit"),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "bit_depth",
        |t| &mut t.bit_depth,
        Regex::case_insensitive(r"(?:8|10|12)[-\.]?(?=bit)").unwrap(),
        transforms::value("$1bit"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "bit_depth",
        |t| &mut t.bit_depth,
        Regex::case_insensitive(r"\bhdr10\b").unwrap(),
        transforms::value("10bit"),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "bit_depth",
        |t| &mut t.bit_depth,
        Regex::case_insensitive(r"\bhi10\b").unwrap(),
        transforms::value("10bit"),
        RegexHandlerOptions::default(),
    ));

    parser.add_handler(Handler::new("bit_depth", |context| {
        if let Some(bit_depth) = context.result.bit_depth.clone() {
            // Remove hypens and spaces
            context.result.bit_depth = Some(bit_depth.replace("-", "").replace(" ", ""));
        }
        None
    }));

    /*
    # HDR
    parser.add_handler("hdr", regex.compile(r"\bDV\b|dolby.?vision|\bDoVi\b", regex.IGNORECASE), uniq_concat(value("DV")), {"remove": True, "skipIfAlreadyFound": False})
    parser.add_handler("hdr", regex.compile(r"HDR10(?:\+|[-\.\s]?plus)", regex.IGNORECASE), uniq_concat(value("HDR10+")), {"remove": True, "skipIfAlreadyFound": False})
    parser.add_handler("hdr", regex.compile(r"\bHDR(?:10)?\b", regex.IGNORECASE), uniq_concat(value("HDR")), {"remove": True, "skipIfAlreadyFound": False})
    parser.add_handler("hdr", regex.compile(r"\bSDR\b", regex.IGNORECASE), uniq_concat(value("SDR")), {"remove": True, "skipIfAlreadyFound": False})
    */

    // HDR
    parser.add_handler(Handler::from_regex(
        "hdr",
        |t| &mut t.hdr,
        Regex::case_insensitive(r"\bDV\b|dolby.?vision|\bDoVi\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("DV"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "hdr",
        |t| &mut t.hdr,
        Regex::case_insensitive(r"HDR10(?:\+|[-\.\s]?plus)").unwrap(),
        transforms::chain_transforms(transforms::replace_value("HDR10+"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "hdr",
        |t| &mut t.hdr,
        Regex::case_insensitive(r"\bHDR(?:10)?\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("HDR"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "hdr",
        |t| &mut t.hdr,
        Regex::case_insensitive(r"\bSDR\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("SDR"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));

    /*
    # Codec
    parser.add_handler("codec", regex.compile(r"\b[hx][\. \-]?264\b", regex.IGNORECASE), value("avc"), {"remove": True})
    parser.add_handler("codec", regex.compile(r"\b[hx][\. \-]?265\b", regex.IGNORECASE), value("hevc"), {"remove": True})
    parser.add_handler("codec", regex.compile(r"\bHEVC10(bit)?\b|\b[xh][\. \-]?265\b", regex.IGNORECASE), value("hevc"), {"remove": True})
    parser.add_handler("codec", regex.compile(r"\bhevc(?:\s?10)?\b", regex.IGNORECASE), value("hevc"), {"remove": True, "skipIfAlreadyFound": False})
    parser.add_handler("codec", regex.compile(r"\bdivx|xvid\b", regex.IGNORECASE), value("xvid"), {"remove": True, "skipIfAlreadyFound": False})
    parser.add_handler("codec", regex.compile(r"\bavc\b", regex.IGNORECASE), value("avc"), {"remove": True, "skipIfAlreadyFound": False})
    parser.add_handler("codec", regex.compile(r"\bav1\b", regex.IGNORECASE), value("av1"), {"remove": True, "skipIfAlreadyFound": False})
    parser.add_handler("codec", regex.compile(r"\b(?:mpe?g\d*)\b", regex.IGNORECASE), value("mpeg"), {"remove": True, "skipIfAlreadyFound": False})

    def handle_space_in_codec(context):
        if context["result"].get("codec"):
            context["result"]["codec"] = regex.sub("[ .-]", "", context["result"]["codec"])

    parser.add_handler("codec", handle_space_in_codec)
    */

    // Codec
    parser.add_handler(Handler::from_regex(
        "codec",
        |t| &mut t.codec,
        Regex::case_insensitive(r"\b[hx][\. \-]?264\b").unwrap(),
        transforms::value("avc"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "codec",
        |t| &mut t.codec,
        Regex::case_insensitive(r"\b[hx][\. \-]?265\b").unwrap(),
        transforms::value("hevc"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "codec",
        |t| &mut t.codec,
        Regex::case_insensitive(r"HEVC10(bit)?\b|\b[xh][\. \-]?265\b").unwrap(),
        transforms::value("hevc"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "codec",
        |t| &mut t.codec,
        Regex::case_insensitive(r"\bhevc(?:\s?10)?\b").unwrap(),
        transforms::value("hevc"),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "codec",
        |t| &mut t.codec,
        Regex::case_insensitive(r"\bdivx|xvid\b").unwrap(),
        transforms::value("xvid"),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "codec",
        |t| &mut t.codec,
        Regex::case_insensitive(r"\bavc\b").unwrap(),
        transforms::value("avc"),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "codec",
        |t| &mut t.codec,
        Regex::case_insensitive(r"\bav1\b").unwrap(),
        transforms::value("av1"),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "codec",
        |t| &mut t.codec,
        Regex::case_insensitive(r"\b(?:mpe?g\d*)\b").unwrap(),
        transforms::value("mpeg"),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));

    lazy_static! {
        static ref REMOVE_SPACE_AND_DASH: LiteRegex = LiteRegex::new(r"[ .-]+").unwrap();
    }
    parser.add_handler(Handler::new("codec", |context| {
        if let Some(codec) = context.result.codec.clone() {
            context.result.codec = Some(REMOVE_SPACE_AND_DASH.replace_all(&codec, "").to_string());
        }
        None
    }));

    /*
    # Channels
    parser.add_handler("channels", regex.compile(r"\bDDP?5[ \.\_]1\b", regex.IGNORECASE), uniq_concat(value("5.1")), {"remove": False})
    parser.add_handler("channels", regex.compile(r"\b5\.1(ch)?\b", regex.IGNORECASE), uniq_concat(value("5.1")), {"remove": False})
    parser.add_handler("channels", regex.compile(r"\b7[\.\- ]1(.?ch(annel)?)?\b", regex.IGNORECASE), uniq_concat(value("7.1")), {"remove": False})
    parser.add_handler("channels", regex.compile(r"\b2\.0\b", regex.IGNORECASE), uniq_concat(value("2.0")), {"remove": False})
    parser.add_handler("channels", regex.compile(r"\bstereo\b", regex.IGNORECASE), uniq_concat(value("stereo")), {"remove": False})
    parser.add_handler("channels", regex.compile(r"\bmono\b", regex.IGNORECASE), uniq_concat(value("mono")), {"remove": False})
    parser.add_handler("channels", regex.compile(r"\b(?:x[2-4]|5[\W]1(?:x[2-4])?)\b", regex.IGNORECASE), uniq_concat(value("5.1")), {"remove": True})
    parser.add_handler("channels", regex.compile(r"\b2\.0(?:x[2-4])\b", regex.IGNORECASE), uniq_concat(value("2.0")), {"remove": True})
     */

    // Channels
    parser.add_handler(Handler::from_regex(
        "channels",
        |t| &mut t.channels,
        Regex::case_insensitive(r"\bDDP?5[ \.\_]1\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("5.1"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "channels",
        |t| &mut t.channels,
        Regex::case_insensitive(r"\b5\.1(?:ch)?\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("5.1"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "channels",
        |t| &mut t.channels,
        Regex::case_insensitive(r"\b7[\.\- ]1(?:\.?ch(?:annel)?)?\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("7.1"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "channels",
        |t| &mut t.channels,
        Regex::case_insensitive(r"\b2\.0\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("2.0"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "channels",
        |t| &mut t.channels,
        Regex::case_insensitive(r"\bstereo\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("stereo"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "channels",
        |t| &mut t.channels,
        Regex::case_insensitive(r"\bmono\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("mono"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "channels",
        |t| &mut t.channels,
        Regex::case_insensitive(r"\b(?:x[2-4]|5[\W]1(?:x[2-4])?)\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("5.1"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "channels",
        |t| &mut t.channels,
        Regex::case_insensitive(r"\b2\.0(?:x[2-4])\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("2.0"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));

    /*
    # Audio
    parser.add_handler("audio", regex.compile(r"\bDDP5[ \.\_]1\b", regex.IGNORECASE), uniq_concat(value("Dolby Digital Plus")), {"remove": True, "skipIfFirst": True})
    parser.add_handler("audio", regex.compile(r"\b(?!.+HR)(DTS.?HD.?Ma(ster)?|DTS.?X)\b", regex.IGNORECASE), uniq_concat(value("DTS Lossless")), {"remove": True, "skipIfAlreadyFound": False})
    parser.add_handler("audio", regex.compile(r"\bDTS(?!(.?HD.?Ma(ster)?|.X)).?(HD.?HR|HD)?\b", regex.IGNORECASE), uniq_concat(value("DTS Lossy")), {"remove": True, "skipIfAlreadyFound": False})
    parser.add_handler("audio", regex.compile(r"\b(Dolby.?)?Atmos\b", regex.IGNORECASE), uniq_concat(value("Atmos")), {"remove": True, "skipIfAlreadyFound": False})
    parser.add_handler("audio", regex.compile(r"\b(TrueHD|\.True\.)\b", regex.IGNORECASE), uniq_concat(value("TrueHD")), {"remove": True, "skipIfAlreadyFound": False, "skipFromTitle": True})
    parser.add_handler("audio", regex.compile(r"\bTRUE\b"), uniq_concat(value("TrueHD")), {"remove": True, "skipIfAlreadyFound": False, "skipFromTitle": True})
    parser.add_handler("audio", regex.compile(r"\bFLAC(?:\+?2\.0)?(x[2-4])?\b", regex.IGNORECASE), uniq_concat(value("FLAC")), {"remove": True, "skipIfAlreadyFound": False})
    parser.add_handler("audio", regex.compile(r"\bEAC-?3(?:[. -]?[256]\.[01])?\b", regex.IGNORECASE), uniq_concat(value("EAC3")), {"remove": True, "skipIfAlreadyFound": False})
    parser.add_handler("audio", regex.compile(r"\bAC-?3(x2)?(?:[ .-](5\.1)?[x+]2\.?0?x?3?)?\b", regex.IGNORECASE), uniq_concat(value("AC3")), {"remove": True, "skipIfAlreadyFound": False})
    parser.add_handler("audio", regex.compile(r"\b5\.1(ch)?\b", regex.IGNORECASE), uniq_concat(value("AC3")), {"remove": True, "skipIfAlreadyFound": True})
    parser.add_handler("audio", regex.compile(r"\b(DD2?[\+p]2?(.?5.1)?|DD Plus|Dolby Digital Plus)\b", regex.IGNORECASE), uniq_concat(value("Dolby Digital Plus")), {"remove": True, "skipIfAlreadyFound": False})
    parser.add_handler("audio", regex.compile(r"\b(DD|Dolby.?Digital.?)2?(5.?1)?(?!.?(Plus|P|\+))\b", regex.IGNORECASE), uniq_concat(value("Dolby Digital")), {"remove": True, "skipIfAlreadyFound": False})
    parser.add_handler("audio", regex.compile(r"\bDolbyD\b", regex.IGNORECASE), uniq_concat(value("Dolby Digital")), {"remove": True, "skipIfFirst": True})
    parser.add_handler("audio", regex.compile(r"\bQ?Q?AAC(x?2)?\b", regex.IGNORECASE), uniq_concat(value("AAC")), {"remove": True, "skipIfAlreadyFound": False})
    parser.add_handler("audio", regex.compile(r"\b(H[DQ])?.?(Clean.?Aud(io)?)\b", regex.IGNORECASE), uniq_concat(value("HQ Clean Audio")), {"remove": True, "skipIfAlreadyFound": False})
    */

    // Audio
    parser.add_handler(Handler::from_regex(
        "audio",
        |t| &mut t.audio,
        Regex::case_insensitive(r"\bDDP5[ \.\_]1\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("Dolby Digital Plus"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_first: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "audio",
        |t| &mut t.audio,
        Regex::case_insensitive(r"\b(?!.+HR)(DTS.?HD.?Ma(ster)?|DTS.?X)\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("DTS Lossless"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "audio",
        |t| &mut t.audio,
        Regex::case_insensitive(r"\bDTS(?!(.?HD.?Ma(ster)?|.X)).?(HD.?HR|HD)?\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("DTS Lossy"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "audio",
        |t| &mut t.audio,
        Regex::case_insensitive(r"\b(Dolby.?)?Atmos\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("Atmos"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "audio",
        |t| &mut t.audio,
        Regex::case_insensitive(r"\b(TrueHD|\.True\.)\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("TrueHD"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            skip_from_title: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "audio",
        |t| &mut t.audio,
        Regex::new(r"\bTRUE\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("TrueHD"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            skip_from_title: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "audio",
        |t| &mut t.audio,
        Regex::case_insensitive(r"\bFLAC(?:\+?2\.0)?(x[2-4])?\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("FLAC"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "audio",
        |t| &mut t.audio,
        Regex::case_insensitive(r"\bEAC-?3(?:[. -]?[256]\.[01])?\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("EAC3"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "audio",
        |t| &mut t.audio,
        Regex::case_insensitive(r"\bAC-?3(x2)?(?:[ .-](5\.1)?[x+]2\.?0?x?3?)?\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("AC3"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "audio",
        |t| &mut t.audio,
        Regex::case_insensitive(r"\b5\.1(ch)?\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("AC3"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "audio",
        |t| &mut t.audio,
        Regex::case_insensitive(r"\b(DD2?[\+p]2?(.?5.1)?|DD Plus|Dolby Digital Plus)\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("Dolby Digital Plus"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "audio",
        |t| &mut t.audio,
        Regex::case_insensitive(r"\b(DD|Dolby.?Digital.?)2?(5.?1)?(?!.?(Plus|P|\+))\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("Dolby Digital"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "audio",
        |t| &mut t.audio,
        Regex::case_insensitive(r"\bDolbyD\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("Dolby Digital"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_first: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "audio",
        |t| &mut t.audio,
        Regex::case_insensitive(r"\bQ?Q?AAC(x?2)?\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("AAC"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "audio",
        |t| &mut t.audio,
        Regex::case_insensitive(r"\b(H[DQ])?.?(Clean.?Aud(io)?)\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("HQ Clean Audio"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));

    /*
    # Group
    parser.add_handler("group", regex.compile(r"- ?(?!\d+$|S\d+|\d+x|ep?\d+|[^[]+]$)([^\-. []+[^\-. [)\]\d][^\-. [)\]]*)(?:\[[\w.-]+])?(?=\.\w{2,4}$|$)", regex.IGNORECASE), none, {"remove": False})

    # Container
    parser.add_handler("container", regex.compile(r"\.?[\[(]?\b(MKV|AVI|MP4|WMV|MPG|MPEG)\b[\])]?", regex.IGNORECASE), lowercase)
    */

    // Group
    parser.add_handler(Handler::from_regex(
        "group",
        |t| &mut t.group,
        Regex::case_insensitive(
            r"- ?(?!\d+$|S\d+|\d+x|ep?\d+|[^\[]+\]$)([^\-\. \[]+[^\-\. \[\)\]\d][^\-\. \[\)\]]*)(?:\[[\w.-]+])?(?=\.\w{2,4}$|$)",
        )
        .unwrap(),
        transforms::identity,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));

    // Container
    parser.add_handler(Handler::from_regex(
        "container",
        |t| &mut t.container,
        Regex::case_insensitive(r"\.?[\[(]?\b(MKV|AVI|MP4|WMV|MPG|MPEG)\b[\])]?").unwrap(),
        transforms::lowercase,
        RegexHandlerOptions::default(),
    ));

    /*
    # Volume
    parser.add_handler("volumes", regex.compile(r"\bvol(?:s|umes?)?[. -]*(?:\d{1,2}[., +/\\&-]+)+\d{1,2}\b", regex.IGNORECASE), range_func, {"remove": True})

    def handle_volumes(context):
        title = context["title"]
        result = context["result"]
        matched = context["matched"]

        start_index = matched.get("year", {}).get("match_index", 0)
        match = regex.search(r"\bvol(?:ume)?[. -]*(\d{1,2})", title[start_index:], regex.IGNORECASE)

        if match:
            matched["volumes"] = {"match": match.group(0), "match_index": match.start()}
            result["volumes"] = [int(match.group(1))]
            return {"raw_match": match.group(0), "match_index": match.start() + start_index, "remove": True}
        return None

    parser.add_handler("volumes", handle_volumes)
    */

    // Volume
    parser.add_handler(Handler::from_regex(
        "volumes",
        |t| &mut t.volumes,
        Regex::case_insensitive(r"\bvol(?:s|umes?)?[. -]*(?:\d{1,2}[., +/\\&-]+)+\d{1,2}\b").unwrap(),
        transforms::range_func,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));

    lazy_static! {
        static ref VOLUME_REGEX: Regex = Regex::case_insensitive(r"\bvol(?:ume)?[. -]*(\d{1,2})").unwrap();
    }

    parser.add_handler(Handler::new("volumes", |context| {
        let title = &context.title;
        let start_index = context.matched.get("year").map(|y| y.match_index).unwrap_or(0);

        if let Some(m) = VOLUME_REGEX.find_str(&title[start_index..]) {
            let vol = m.group(1).unwrap().as_str().parse::<i32>().unwrap();

            context.matched.insert(
                "volumes".to_string(),
                Match {
                    raw_match: m.as_str().to_string(),
                    match_index: m.start(),
                },
            );

            context.result.volumes = vec![vol];

            return Some(HandlerResult {
                raw_match: m.as_str().to_string(),
                match_index: m.start() + start_index,
                remove: true,
                skip_from_title: false,
            });
        }
        None
    }));

    /*
    # Pre-Language
    parser.add_handler("languages", regex.compile(r"\b(temporadas?|completa)\b", regex.IGNORECASE), uniq_concat(value("es")), {"skipIfAlreadyFound": False})
    */

    // Pre-Language
    parser.add_handler(Handler::from_regex(
        "languages",
        |t| &mut t.languages,
        Regex::case_insensitive(r"\b(temporadas?|completa)\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("es"), transforms::uniq_concat),
        RegexHandlerOptions {
            skip_if_already_found: false,
            ..Default::default()
        },
    ));

    /*
    # Complete
    parser.add_handler("complete", regex.compile(r"(?:\bthe\W)?(?:\bcomplete|collection|dvd)?\b[ .]?\bbox[ .-]?set\b", regex.IGNORECASE), boolean)
    parser.add_handler("complete", regex.compile(r"(?:\bthe\W)?(?:\bcomplete|collection|dvd)?\b[ .]?\bmini[ .-]?series\b", regex.IGNORECASE), boolean)
    parser.add_handler("complete", regex.compile(r"(?:\bthe\W)?(?:\bcomplete|full|all)\b.*\b(?:series|seasons|collection|episodes|set|pack|movies)\b", regex.IGNORECASE), boolean)
    parser.add_handler("complete", regex.compile(r"\b(?:series|seasons|movies?)\b.*\b(?:complete|collection)\b", regex.IGNORECASE), boolean)
    parser.add_handler("complete", regex.compile(r"(?:\bthe\W)?\bultimate\b[ .]\bcollection\b", regex.IGNORECASE), boolean, {"skipIfAlreadyFound": False})
    parser.add_handler("complete", regex.compile(r"\bcollection\b.*\b(?:set|pack|movies)\b", regex.IGNORECASE), boolean)
    parser.add_handler("complete", regex.compile(r"\bcollection\b", regex.IGNORECASE), boolean, {"skipFromTitle": True})
    parser.add_handler("complete", regex.compile(r"duology|trilogy|quadr[oi]logy|tetralogy|pentalogy|hexalogy|heptalogy|anthology", regex.IGNORECASE), boolean, {"skipIfAlreadyFound": False})
    parser.add_handler("complete", regex.compile(r"\bcompleta\b", regex.IGNORECASE), boolean, {"remove": True})
    parser.add_handler("complete", regex.compile(r"\bsaga\b", regex.IGNORECASE), boolean, {"skipFromTitle": True, "skipIfAlreadyFound": True})
     */

    // Complete
    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::case_insensitive(r"(?:\bthe\W)?(?:\bcomplete|collection|dvd)?\b[ .]?\bbox[ .-]?set\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::case_insensitive(r"(?:\bthe\W)?(?:\bcomplete|collection|dvd)?\b[ .]?\bmini[ .-]?series\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::case_insensitive(r"(?:\bthe\W)?(?:\bcomplete|full|all)\b.*\b(?:series|seasons|collection|episodes|set|pack|movies)\b")
            .unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::case_insensitive(r"\b(?:series|seasons|movies?)\b.*\b(?:complete|collection)\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::case_insensitive(r"(?:\bthe\W)?\bultimate\b[ .]\bcollection\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            skip_if_already_found: false,
            ..Default::default()
        },
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::case_insensitive(r"\bcollection\b.*\b(?:set|pack|movies)\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::case_insensitive(r"\bcollection\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            skip_from_title: true,
            ..Default::default()
        },
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::case_insensitive(r"duology|trilogy|quadr[oi]logy|tetralogy|pentalogy|hexalogy|heptalogy|anthology").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            skip_if_already_found: false,
            ..Default::default()
        },
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::case_insensitive(r"\bcompleta\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::case_insensitive(r"\bsaga\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            skip_from_title: true,
            skip_if_already_found: true,
            ..Default::default()
        },
    ));

    /*
     Seasons
    parser.add_handler("seasons", regex.compile(r"(?:complete\W|seasons?\W|\W|^)((?:s\d{1,2}[., +/\\&-]+)+s\d{1,2}\b)", regex.IGNORECASE), range_func, {"remove": True})
    parser.add_handler("seasons", regex.compile(r"(?:complete\W|seasons?\W|\W|^)[([]?(s\d{2,}-\d{2,}\b)[)\]]?", regex.IGNORECASE), range_func, {"remove": True})
    parser.add_handler("seasons", regex.compile(r"(?:complete\W|seasons?\W|\W|^)[([]?(s[1-9]-[2-9])[)\]]?", regex.IGNORECASE), range_func, {"remove": True})
    parser.add_handler("seasons", regex.compile(r"\d+ª(?:.+)?(?:a.?)?\d+ª(?:(?:.+)?(?:temporadas?))", regex.IGNORECASE), range_func, {"remove": True})
    parser.add_handler("seasons", regex.compile(r"(?:(?:\bthe\W)?\bcomplete\W)?(?:seasons?|[Сс]езони?|temporadas?)[. ]?[-:]?[. ]?[([]?((?:\d{1,2}[., /\\&]+)+\d{1,2}\b)[)\]]?", regex.IGNORECASE), range_func, {"remove": True})
    parser.add_handler("seasons", regex.compile(r"(?:(?:\bthe\W)?\bcomplete\W)?(?:seasons?|[Сс]езони?|temporadas?)[. ]?[-:]?[. ]?[([]?((?:\d{1,2}[.-]+)+[1-9]\d?\b)[)\]]?", regex.IGNORECASE), range_func, {"remove": True})
    parser.add_handler("seasons", regex.compile(r"(?:(?:\bthe\W)?\bcomplete\W)?season[. ]?[([]?((?:\d{1,2}[. -]+)+[1-9]\d?\b)[)\]]?(?!.*\.\w{2,4}$)", regex.IGNORECASE), range_func, {"remove": True})
    parser.add_handler("seasons", regex.compile(r"(?:(?:\bthe\W)?\bcomplete\W)?\bseasons?\b[. -]?(\d{1,2}[. -]?(?:to|thru|and|\+|:)[. -]?\d{1,2})\b", regex.IGNORECASE), range_func, {"remove": True})
    parser.add_handler("seasons", regex.compile(r"(?:(?:\bthe\W)?\bcomplete\W)?(?:saison|seizoen|season|series|temp(?:orada)?):?[. ]?(\d{1,2})\b", regex.IGNORECASE), array(integer))
    parser.add_handler("seasons", regex.compile(r"(\d{1,2})(?:-?й)?[. _]?(?:[Сс]езон|sez(?:on)?)(?:\W?\D|$)", regex.IGNORECASE), array(integer))
    parser.add_handler("seasons", regex.compile(r"[Сс]езон:?[. _]?№?(\d{1,2})(?!\d)", regex.IGNORECASE), array(integer))
    parser.add_handler("seasons", regex.compile(r"(?:\D|^)(\d{1,2})Â?[°ºªa]?[. ]*temporada", regex.IGNORECASE), array(integer), {"remove": True})
    parser.add_handler("seasons", regex.compile(r"t(\d{1,3})(?:[ex]+|$)", regex.IGNORECASE), array(integer), {"remove": True})
    parser.add_handler("seasons", regex.compile(r"(?:(?:\bthe\W)?\bcomplete)?s(\d{1,3})(?:[\Wex]|\d{2}\b|$)", regex.IGNORECASE), array(integer), {"remove": False, "skipIfAlreadyFound": False})
    parser.add_handler("seasons", regex.compile(r"(?:(?:\bthe\W)?\bcomplete\W)?(?:\W|^)(\d{1,2})[. ]?(?:st|nd|rd|th)[. ]*season", regex.IGNORECASE), array(integer))
    parser.add_handler("seasons", regex.compile(r"(?<=S)\d{2}(?=E\d+)"), array(integer))
    parser.add_handler("seasons", regex.compile(r"(?:\D|^)(\d{1,2})[xх]\d{1,3}(?:\D|$)"), array(integer))
    parser.add_handler("seasons", regex.compile(r"\bSn([1-9])(?:\D|$)"), array(integer))
    parser.add_handler("seasons", regex.compile(r"[[(](\d{1,2})\.\d{1,3}[)\]]"), array(integer))
    parser.add_handler("seasons", regex.compile(r"-\s?(\d{1,2})\.\d{2,3}\s?-"), array(integer))
    parser.add_handler("seasons", regex.compile(r"(?:^|\/)(\d{1,2})-\d{2}\b(?!-\d)"), array(integer))
    parser.add_handler("seasons", regex.compile(r"[^\w-](\d{1,2})-\d{2}(?=\.\w{2,4}$)"), array(integer))
    parser.add_handler("seasons", regex.compile(r"(?<!\bEp?(?:isode)? ?\d+\b.*)\b(\d{2})[ ._]\d{2}(?:.F)?\.\w{2,4}$"), array(integer))
    parser.add_handler("seasons", regex.compile(r"\bEp(?:isode)?\W+(\d{1,2})\.\d{1,3}\b", regex.IGNORECASE), array(integer))
    parser.add_handler("seasons", regex.compile(r"\bSeasons?\b.*\b(\d{1,2}-\d{1,2})\b", regex.IGNORECASE), range_func)
    parser.add_handler("seasons", regex.compile(r"(?:\W|^)(\d{1,2})(?:e|ep)\d{1,3}(?:\W|$)", regex.IGNORECASE), array(integer))
     */

    // Seasons
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"(?:complete\W|seasons?\W|\W|^)((?:s\d{1,2}[., +/\\&-]+)+s\d{1,2}\b)").unwrap(),
        transforms::range_func,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"(?:complete\W|seasons?\W|\W|^)[\(\[]?(s\d{2,}-\d{2,}\b)[\)\]]?").unwrap(),
        transforms::range_func,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"(?:complete\W|seasons?\W|\W|^)[\(\[]?(s[1-9]-[2-9])[\)\]]?").unwrap(),
        transforms::range_func,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"\d+ª(?:.+)?(?:a.?)?\d+ª(?:(?:.+)?(?:temporadas?))").unwrap(),
        transforms::range_func,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"(?:(?:\bthe\W)?\bcomplete\W)?(?:seasons?|[Сс]езони?|temporadas?)[. ]?[-:]?[. ]?[\(\[]?((?:\d{1,2}[., /\\&]+)+\d{1,2}\b)[\)\]]?").unwrap(),
        transforms::range_func,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(
            r"(?:(?:\bthe\W)?\bcomplete\W)?(?:seasons?|[Сс]езони?|temporadas?)[. ]?[-:]?[. ]?[\(\[]?((?:\d{1,2}[.-]+)+[1-9]\d?\b)[\)\]]?",
        )
        .unwrap(),
        transforms::range_func,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"(?:(?:\bthe\W)?\bcomplete\W)?season[. ]?[\(\[]?((?:\d{1,2}[. -]+)+[1-9]\d?\b)[\)\]]?(?!.*\.\w{2,4}$)")
            .unwrap(),
        transforms::range_func,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"(?:(?:\bthe\W)?\bcomplete\W)?\bseasons?\b[. -]?(\d{1,2}[. -]?(?:to|thru|and|\+|:)[. -]?\d{1,2})\b")
            .unwrap(),
        transforms::range_func,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"(?:(?:\bthe\W)?\bcomplete\W)?(?:saison|seizoen|season|series|temp(?:orada)?):?[. ]?(\d{1,2})\b").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"(\d{1,2})(?:-?й)?[. _]?(?:[Сс]езон|sez(?:on)?)(?:\W?\D|$)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"[Сс]езон:?[. _]?№?(\d{1,2})(?!\d)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"(?:\D|^)(\d{1,2})Â?[°ºªa]?[. ]*temporada").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"t(\d{1,3})(?:[ex]+|$)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"(?:(?:\bthe\W)?\bcomplete)?s(\d{1,3})(?:[\Wex]|\d{2}\b|$)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions {
            remove: false,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"(?:(?:\bthe\W)?\bcomplete\W)?(?:\W|^)(\d{1,2})[. ]?(?:st|nd|rd|th)[. ]*season").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"(?<=S)\d{2}(?=E\d+)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"(?:\D|^)(\d{1,2})[xх]\d{1,3}(?:\D|$)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"\bSn([1-9])(?:\D|$)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"[\[\(](\d{1,2})\.\d{1,3}[\)\]]").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"-\s?(\d{1,2})\.\d{2,3}\s?-").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"(?:^|\/)(\d{1,2})-\d{2}\b(?!-\d)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"[^\w-](\d{1,2})-\d{2}(?=\.\w{2,4}$)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"^(?!.*\bEp?(?:isode)? ?\d+\b.*\b\d{2}[ ._]\d{2}).*\b(\d{2})[ ._]\d{2}(?:\.F)?\.\w{2,4}$").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"\bEp(?:isode)?\W+(\d{1,2})\.\d{1,3}\b").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"\bSeasons?\b.*\b(\d{1,2}-\d{1,2})\b").unwrap(),
        transforms::range_func,
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "seasons",
        |t| &mut t.seasons,
        Regex::case_insensitive(r"(?:\W|^)(\d{1,2})(?:e|ep)\d{1,3}(?:\W|$)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));

    /*
     Episodes
    parser.add_handler("episodes", regex.compile(r"(?:[\W\d]|^)e[ .]?[([]?(\d{1,3}(?:[ .-]*(?:[&+]|e){1,2}[ .]?\d{1,3})+)(?:\W|$)", regex.IGNORECASE), range_func)
    parser.add_handler("episodes", regex.compile(r"(?:[\W\d]|^)ep[ .]?[([]?(\d{1,3}(?:[ .-]*(?:[&+]|ep){1,2}[ .]?\d{1,3})+)(?:\W|$)", regex.IGNORECASE), range_func)
    parser.add_handler("episodes", regex.compile(r"(?:[\W\d]|^)\d+[xх][ .]?[([]?(\d{1,3}(?:[ .]?[xх][ .]?\d{1,3})+)(?:\W|$)", regex.IGNORECASE), range_func)
    parser.add_handler("episodes", regex.compile(r"(?:[\W\d]|^)(?:episodes?|[Сс]ерии:?)[ .]?[([]?(\d{1,3}(?:[ .+]*[&+][ .]?\d{1,3})+)(?:\W|$)", regex.IGNORECASE), range_func)
    parser.add_handler("episodes", regex.compile(r"[([]?(?:\D|^)(\d{1,3}[ .]?ao[ .]?\d{1,3})[)\]]?(?:\W|$)", regex.IGNORECASE), range_func)
    parser.add_handler("episodes", regex.compile(r"(?:[\W\d]|^)(?:e|eps?|episodes?|[Сс]ерии:?|\d+[xх])[ .]*[([]?(\d{1,3}(?:-\d{1,3})+)(?:\W|$)", regex.IGNORECASE), range_func)
    parser.add_handler("episodes", regex.compile(r"[st]\d{1,2}[. ]?[xх-]?[. ]?(?:e|x|х|ep|-|\.)[. ]?(\d{1,4})(?:[abc]|v0?[1-4]|\D|$)", regex.IGNORECASE), array(integer), {"remove": True})
    parser.add_handler("episodes", regex.compile(r"\b[st]\d{2}(\d{2})\b", regex.IGNORECASE), array(integer))
    parser.add_handler("episodes", regex.compile(r"(?:\W|^)(\d{1,3}(?:[ .]*~[ .]*\d{1,3})+)(?:\W|$)", regex.IGNORECASE), range_func)
    parser.add_handler("episodes", regex.compile(r"-\s(\d{1,3}[ .]*-[ .]*\d{1,3})(?!-\d)(?:\W|$)", regex.IGNORECASE), range_func)
    parser.add_handler("episodes", regex.compile(r"s\d{1,2}\s?\((\d{1,3}[ .]*-[ .]*\d{1,3})\)", regex.IGNORECASE), range_func)
    parser.add_handler("episodes", regex.compile(r"(?:^|\/)\d{1,2}-(\d{2})\b(?!-\d)"), array(integer))
    parser.add_handler("episodes", regex.compile(r"(?<!\d-)\b\d{1,2}-(\d{2})(?=\.\w{2,4}$)"), array(integer))
    parser.add_handler("episodes", regex.compile(r"(?<=^\[.+].+)[. ]+-[. ]+(\d{1,4})[. ]+(?=\W)", regex.IGNORECASE), array(integer))
    parser.add_handler("episodes", regex.compile(r"(?<!(?:seasons?|[Сс]езони?)\W*)(?:[ .([-]|^)(\d{1,3}(?:[ .]?[,&+~][ .]?\d{1,3})+)(?:[ .)\]-]|$)", regex.IGNORECASE), range_func)
    parser.add_handler("episodes", regex.compile(r"(?<!(?:seasons?|[Сс]езони?)\W*)(?:[ .([-]|^)(\d{1,3}(?:-\d{1,3})+)(?:[ .)(\]]|-\D|$)", regex.IGNORECASE), range_func)
    parser.add_handler("episodes", regex.compile(r"\bEp(?:isode)?\W+\d{1,2}\.(\d{1,3})\b", regex.IGNORECASE), array(integer))
    parser.add_handler("episodes", regex.compile(r"(?:\b[ée]p?(?:isode)?|[Ээ]пизод|[Сс]ер(?:ии|ия|\.)?|cap(?:itulo)?|epis[oó]dio)[. ]?[-:#№]?[. ]?(\d{1,4})(?:[abc]|v0?[1-4]|\W|$)", regex.IGNORECASE), array(integer))
    parser.add_handler("episodes", regex.compile(r"\b(\d{1,3})(?:-?я)?[ ._-]*(?:ser(?:i?[iyj]a|\b)|[Сс]ер(?:ии|ия|\.)?)", regex.IGNORECASE), array(integer))
    parser.add_handler("episodes", regex.compile(r"(?:\D|^)\d{1,2}[. ]?[xх][. ]?(\d{1,3})(?:[abc]|v0?[1-4]|\D|$)"), array(integer))  # Fixed: Was catching `1.x265` as episode.
    parser.add_handler("episodes", regex.compile(r"(?<=S\d{2}E)\d+", regex.IGNORECASE), array(integer))
    parser.add_handler("episodes", regex.compile(r"[[(]\d{1,2}\.(\d{1,3})[)\]]"), array(integer))
    parser.add_handler("episodes", regex.compile(r"\b[Ss]\d{1,2}[ .](\d{1,2})\b"), array(integer))
    parser.add_handler("episodes", regex.compile(r"-\s?\d{1,2}\.(\d{2,3})\s?-"), array(integer))
    parser.add_handler("episodes", regex.compile(r"(?<=\D|^)(\d{1,3})[. ]?(?:of|из|iz)[. ]?\d{1,3}(?=\D|$)", regex.IGNORECASE), array(integer))
    parser.add_handler("episodes", regex.compile(r"\b\d{2}[ ._-](\d{2})(?:.F)?\.\w{2,4}$"), array(integer))
    parser.add_handler("episodes", regex.compile(r"(?<!^)\[(\d{2,3})](?!(?:\.\w{2,4})?$)"), array(integer))
    parser.add_handler("episodes", regex.compile(r"(\d+)(?=.?\[([A-Z0-9]{8})])", regex.IGNORECASE), array(integer))
    parser.add_handler("episodes", regex.compile(r"(?<![xh])\b264\b|\b265\b", regex.IGNORECASE), array(integer), {"remove": True})
    parser.add_handler("episodes", regex.compile(r"(?<!\bMovie\s-\s)(?<=\s-\s)\d+(?=\s[-(\s])"), array(integer), {"remove": True, "skipIfAlreadyFound": True})
    parser.add_handler("episodes", regex.compile(r"(?:\W|^)(?:\d+)?(?:e|ep)(\d{1,3})(?:\W|$)", regex.IGNORECASE), array(integer))

    def handle_episodes(context):
        title = context["title"]
        result = context.get("result", {})
        matched = context.get("matched", {})

        if "episodes" not in result:
            start_indexes = [comp.get("match_index") for comp in [matched.get("year"), matched.get("seasons")] if comp and comp.get("match_index", None)]
            end_indexes = [comp["match_index"] for comp in [matched.get("resolution"), matched.get("quality"), matched.get("codec"), matched.get("audio")] if comp and comp.get("match_index", None)]

            start_index = min(start_indexes) if start_indexes else 0
            end_index = min(end_indexes + [len(title)])

            beginning_title = title[:end_index]
            middle_title = title[start_index:end_index]

            matches = regex.search(r"(?<!movie\W*|film\W*|^)(?:[ .]+-[ .]+|[([][ .]*)(\d{1,4})(?:a|b|v\d|\.\d)?(?:\W|$)(?!movie|film|\d+)", beginning_title, regex.IGNORECASE) or regex.search(r"^(?:[([-][ .]?)?(\d{1,4})(?:a|b|v\d)?(?:\W|$)(?!movie|film)", middle_title, regex.IGNORECASE)

            if matches:
                episode_numbers = [int(num) for num in regex.findall(r"\d+", matches.group(1))]
                result["episodes"] = episode_numbers
                return {"match_index": title.index(matches.group(0))}

        return None

    parser.add_handler("episodes", handle_episodes)
     */

    // Episodes
    parser.add_handler(Handler::from_regex(
        "episodes",
        |t| &mut t.episodes,
        Regex::case_insensitive(r"(?:[\W\d]|^)e[ .]?[([]?(\d{1,3}(?:[ .-]*(?:[&+]|e){1,2}[ .]?\d{1,3})+)(?:\W|$)").unwrap(),
        transforms::range_func,
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |t| &mut t.episodes,
        Regex::case_insensitive(r"(?:[\W\d]|^)ep[ .]?[([]?(\d{1,3}(?:[ .-]*(?:[&+]|ep){1,2}[ .]?\d{1,3})+)(?:\W|$)").unwrap(),
        transforms::range_func,
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |t| &mut t.episodes,
        Regex::case_insensitive(r"(?:[\W\d]|^)\d+[xх][ .]?[([]?(\d{1,3}(?:[ .]?[xх][ .]?\d{1,3})+)(?:\W|$)").unwrap(),
        transforms::range_func,
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |t| &mut t.episodes,
        Regex::case_insensitive(r"(?:[\W\d]|^)(?:episodes?|[Сс]ерии:?)[ .]?[([]?(\d{1,3}(?:[ .+]*[&+][ .]?\d{1,3})+)(?:\W|$)").unwrap(),
        transforms::range_func,
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |t| &mut t.episodes,
        Regex::case_insensitive(r"[([]?(?:\D|^)(\d{1,3}[ .]?ao[ .]?\d{1,3})[\)\]]?(?:\W|$)").unwrap(),
        transforms::range_func,
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |t| &mut t.episodes,
        Regex::case_insensitive(r"(?:[\W\d]|^)(?:e|eps?|episodes?|[Сс]ерии:?|\d+[xх])[ .]*[([]?(\d{1,3}(?:-\d{1,3})+)(?:\W|$)").unwrap(),
        transforms::range_func,
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |t| &mut t.episodes,
        Regex::case_insensitive(r"[st]\d{1,2}[. ]?[xх-]?[. ]?(?:e|x|х|ep|-|\.)[. ]?(\d{1,4})(?:[abc]|v0?[1-4]|\D|$)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |t| &mut t.episodes,
        Regex::case_insensitive(r"\b[st]\d{2}(\d{2})\b").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |t| &mut t.episodes,
        Regex::case_insensitive(r"(?:\W|^)(\d{1,3}(?:[ .]*~[ .]*\d{1,3})+)(?:\W|$)").unwrap(),
        transforms::range_func,
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::case_insensitive(r"-\s(\d{1,3}[ .]*-[ .]*\d{1,3})(?!-\d)(?:\W|$)").unwrap(),
        transforms::range_func,
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::case_insensitive(r"s\d{1,2}\s?\((\d{1,3}[ .]*-[ .]*\d{1,3})\)").unwrap(),
        transforms::range_func,
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::new(r"(?:^|\/)\d{1,2}-(\d{2})\b(?!-\d)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::new(r"(?<!\d-)\b\d{1,2}-(\d{2})(?=\.\w{2,4}$)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |t| &mut t.episodes,
        Regex::case_insensitive(r"^\[.+].+[. ]+-[. ]+(\d{1,4})[. ]+(?=\W)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |t| &mut t.episodes,
        Regex::case_insensitive(r"(?:^|[ .\(\[-])(?!(?:seasons?|[Сс]езони?)\W)(\d{1,3}(?:[ .]?[,&+~][ .]?\d{1,3})+)(?:[ .\)\]-]|$)")
            .unwrap(),
        transforms::range_func,
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |t| &mut t.episodes,
        Regex::case_insensitive(r"(?:^|[ .\(\[-])(?!(?:seasons?|[Сс]езони?)\W)(\d{1,3}(?:-\d{1,3})+)(?:[ .\)\]]|-\D|$)").unwrap(),
        transforms::range_func,
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::case_insensitive(r"\bEp(?:isode)?\W+\d{1,2}\.(\d{1,3})\b").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::case_insensitive(r"(?:\b[ée]p?(?:isode)?|[Ээ]пизод|[Сс]ер(?:ии|ия|\.)?|cap(?:itulo)?|epis[oó]dio)[. ]?[-:#№]?[. ]?(\d{1,4})(?:[abc]|v0?[1-4]|\W|$)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::case_insensitive(r"\b(\d{1,3})(?:-?я)?[ ._-]*(?:ser(?:i?[iyj]a|\b)|[Сс]ер(?:ии|ия|\.)?)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::new(r"(?:\D|^)\d{1,2}[. ]?[xх][. ]?(\d{1,3})(?:[abc]|v0?[1-4]|\D|$)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::case_insensitive(r"(?<=S\d{2}E)\d+").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |t| &mut t.episodes,
        Regex::new(r"[\[\(]\d{1,2}\.(\d{1,3})[\)\]]").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::new(r"\b[Ss]\d{1,2}[ .](\d{1,2})\b").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::new(r"-\s?\d{1,2}\.(\d{2,3})\s?-").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::case_insensitive(r"(?<=\D|^)(\d{1,3})[. ]?(?:of|из|iz)[. ]?\d{1,3}(?=\D|$)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::new(r"\b\d{2}[ ._-](\d{2})(?:.F)?\.\w{2,4}$").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::new(r"(?<!^)\[(\d{2,3})](?!(?:\.\w{2,4})?$)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::case_insensitive(r"(\d+)(?=.?\[([A-Z0-9]{8})])").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::case_insensitive(r"(?<![xh])\b264\b|\b265\b").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::new(r"(?<!\bMovie\s-\s)(?<=\s-\s)\d+(?=\s[-(\s])").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "episodes",
        |r| &mut r.episodes,
        Regex::case_insensitive(r"(?:\W|^)(?:\d+)?(?:e|ep)(\d{1,3})(?:\W|$)").unwrap(),
        |v, _| Some(vec![v.parse().ok()?]),
        RegexHandlerOptions::default(),
    ));

    lazy_static! {
        static ref EPISODE_RE1: Regex = Regex::case_insensitive(
            r"(?<!movie\W*|film\W*|^)(?:[ .]+-[ .]+|[([][ .]*)(\d{1,4})(?:a|b|v\d|\.\d)?(?:\W|$)(?!movie|film|\d+)"
        )
        .unwrap();
        static ref EPISODE_RE2: Regex = Regex::case_insensitive(r"^(?:[\[\(-][ .]?)?(\d{1,4})(?:a|b|v\d)?(?:\W|$)(?!movie|film)").unwrap();
        static ref EPISODE_RE3: Regex = Regex::new(r"\d+").unwrap();
    }
    parser.add_handler(Handler::new("episodes", |context| {
        if context.result.episodes.is_empty() {
            let start_indexes = [
                context.matched.get("year").map(|m| m.match_index),
                context.matched.get("seasons").map(|m| m.match_index),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

            let end_indexes = [
                context.matched.get("resolution").map(|m| m.match_index),
                context.matched.get("quality").map(|m| m.match_index),
                context.matched.get("codec").map(|m| m.match_index),
                context.matched.get("audio").map(|m| m.match_index),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

            let start_index = start_indexes.iter().min().copied().unwrap_or(0);
            let end_index = end_indexes.iter().min().copied().unwrap_or(context.title.len());

            let beginning_title = &context.title[..end_index];
            let middle_title = &context.title[start_index..end_index];

            if let Some(m) = EPISODE_RE1.find_str(beginning_title).or_else(|| EPISODE_RE2.find_str(middle_title)) {
                let episode_str = m.group(1).unwrap().as_str();
                let episode_numbers: Vec<i32> = EPISODE_RE3
                    .find_iter_str(episode_str)
                    .filter_map(|m| m.as_str().parse().ok())
                    .collect();

                if !episode_numbers.is_empty() {
                    context.result.episodes = episode_numbers;
                    return Some(HandlerResult {
                        raw_match: m.as_str().to_string(),
                        match_index: context.title.find(m.as_str()).unwrap(),
                        remove: false,
                        skip_from_title: false,
                    });
                }
            }
        }
        None
    }));
}
