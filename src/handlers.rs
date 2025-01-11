use fancy_regex::Regex;

use crate::handler_wrapper::{Handler, HandlerResult, Match, RegexHandlerOptions};
use crate::transforms;
use lazy_static::lazy_static;

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
        Regex::new(r"(?i)\[?\]?3840x\d{4}[\])?]?").unwrap(),
        transforms::value("2160p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::new(r"(?i)\[?\]?1920x\d{3,4}[\])?]?").unwrap(),
        transforms::value("1080p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::new(r"(?i)\[?\]?1280x\d{3}[\])?]?").unwrap(),
        transforms::value("720p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::new(r"(?i)\[?\]?(\d{3,4}x\d{3,4})[\])?]?p?").unwrap(),
        transforms::value("$1p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::new(r"(?i)(480|720|1080)0[pi]").unwrap(),
        transforms::value("$1p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::new(r"(?i)(?:QHD|QuadHD|WQHD|2560(\d+)?x(\d+)?1440p?)").unwrap(),
        transforms::value("1440p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::new(r"(?i)(?:Full HD|FHD|1920(\d+)?x(\d+)?1080p?)").unwrap(),
        transforms::value("1080p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::new(r"(?i)(?:BD|HD|M)(2160p?|4k)").unwrap(),
        transforms::value("2160p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::new(r"(?i)(?:BD|HD|M)1080p?").unwrap(),
        transforms::value("1080p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::new(r"(?i)(?:BD|HD|M)720p?").unwrap(),
        transforms::value("720p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::new(r"(?i)(?:BD|HD|M)480p?").unwrap(),
        transforms::value("480p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::new(r"(?i)\b(?:4k|2160p|1080p|720p|480p)(?!.*\b(?:4k|2160p|1080p|720p|480p)\b)").unwrap(),
        transforms::resolution_transform,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::new(r"(?i)\b4k|21600?[pi]\b").unwrap(),
        transforms::value("2160p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::new(r"(?i)(\d{3,4})[pi]").unwrap(),
        transforms::value("$1p"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "resolution",
        |t| &mut t.resolution,
        Regex::new(r"(?i)(240|360|480|576|720|1080|2160|3840)[pi]").unwrap(),
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
        Regex::new(r"(?i)\b(?:H[DQ][ .-]*)?CAM(?!.?(S|E|\()\d+)(?:H[DQ])?(?:[ .-]*Rip|Rp)?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::new(r"(?i)\b(?:H[DQ][ .-]*)?S[ \.\-]print\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::new(r"(?i)\b(?:HD[ .-]*)?T(?:ELE)?(C|S)(?:INE|YNC)?(?:Rip)?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::new(r"(?i)\bPre.?DVD(?:Rip)?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::new(r"(?i)\b(?:DVD?|BD|BR)?[ .-]*Scr(?:eener)?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::new(r"(?i)\bDVB[ .-]*(?:Rip)?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::new(r"(?i)\bSAT[ .-]*Rips?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::new(r"(?i)\bLeaked\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::new(r"(?i)threesixtyp").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::new(r"(?i)\bR5|R6\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::new(r"(?i)(?:Deleted[ .-]*)?Scene(?:s)?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "trash",
        |t| &mut t.trash,
        Regex::new(r"(?i)\bHQ.?(Clean)?.?(Aud(io)?)?\b").unwrap(),
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
        Regex::new(r"(?i)(?:\W|^)\[?(?:0?[1-9]|[12][0-9]|3[01])[. ]?(?:st|nd|rd|th)?[. \-/\\](?:feb(?:ruary)?|jan(?:uary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sept?(?:ember)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)[. \-/\\](?:19[7-9]|20[012])[0-9]\]?(?:\W|$)").unwrap(),
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
        Regex::new(r"(?i)(?:\W|^)(\[?\]?(?:0?[1-9]|[12][0-9]|3[01])[. ]?(?:st|nd|rd|th)?([. \-\/\\])(?:feb(?:ruary)?|jan(?:uary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sept?(?:ember)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)\2(?:0[1-9]|[0126789][0-9])[\])]?)(?:\W|$)").unwrap(),
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
        Regex::new(r"(?i)\b\d+[kmg]bps\b").unwrap(),
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
        Regex::new(r"(?i)[([]?(?!^)(?:^|[^0-9])((?:19\d|20[012])\d)(?!\d|kbps)[)\]]?").unwrap(),
        transforms::parse::<i32>,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));*/
    parser.add_handler(Handler::from_regex(
        "year",
        |t| &mut t.year,
        Regex::new(r"(?i)^[\[\(]?((?:19[0-9]|20[012])[0-9])(?![0-9]|kbps)[\]\)]?").unwrap(),
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
        Regex::new(r"(?i)\b\d{2,3}(th)?[\.\s\-\+_\/(),]Anniversary[\.\s\-\+_\/(),](Edition|Ed)?\b").unwrap(),
        transforms::value("Anniversary Edition"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::new(r"(?i)\bUltimate[\.\s\-\+_\/(),]Edition\b").unwrap(),
        transforms::value("Ultimate Edition"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::new(r#"(?i)\bExtended[\.\s\-\+_\/(),]Director\"?s\b"#).unwrap(),
        transforms::value("Directors Cut"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::new(r"(?i)\b(custom.?)?Extended\b").unwrap(),
        transforms::value("Extended Edition"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::new(r#"(?i)\bDirector\"?s[\.\s\-\+_\/(),]Cut\b"#).unwrap(),
        transforms::value("Directors Cut"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::new(r#"(?i)\bCollector\"?s\b"#).unwrap(),
        transforms::value("Collectors Edition"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::new(r"(?i)\bTheatrical\b").unwrap(),
        transforms::value("Theatrical"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::new(r"(?i)\bUncut\b").unwrap(),
        transforms::value("Uncut"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::new(r"(?i)\bIMAX\b").unwrap(),
        transforms::value("IMAX"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::new(r#"(?i)\b\.Diamond\.\b"#).unwrap(),
        transforms::value("Diamond Edition"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "edition",
        |t| &mut t.edition,
        Regex::new(r"(?i)\bRemaster(?:ed)?\b").unwrap(),
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
        Regex::new(r"(?i)\b(?:AI.?)?(Upscal(ed?|ing)|Enhanced?)\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "upscaled",
        |t| &mut t.upscaled,
        Regex::new(r"(?i)\b(?:iris2|regrade|ups(uhd|fhd|hd|4k))\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "upscaled",
        |t| &mut t.upscaled,
        Regex::new(r"(?i)\b\.AI\.\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    // Convert
    parser.add_handler(Handler::from_regex(
        "convert",
        |t| &mut t.convert,
        Regex::new(r"(?i)\bCONVERT\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    // Hardcoded
    parser.add_handler(Handler::from_regex(
        "hardcoded",
        |t| &mut t.hardcoded,
        Regex::new(r"(?i)\bHC|HARDCODED\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    // Proper
    parser.add_handler(Handler::from_regex(
        "proper",
        |t| &mut t.proper,
        Regex::new(r"(?i)\b(?:REAL.)?PROPER\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    // Repack
    parser.add_handler(Handler::from_regex(
        "repack",
        |t| &mut t.repack,
        Regex::new(r"(?i)\bREPACK|RERIP\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    // Retail
    parser.add_handler(Handler::from_regex(
        "retail",
        |t| &mut t.retail,
        Regex::new(r"(?i)\bRetail\b").unwrap(),
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
        Regex::new(r"(?i)\bRemaster(?:ed)?\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    // Documentary
    parser.add_handler(Handler::from_regex(
        "documentary",
        |t| &mut t.documentary,
        Regex::new(r"(?i)\bDOCU(?:menta?ry)?\b").unwrap(),
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
        Regex::new(r"(?i)\bunrated|uncensored\b").unwrap(),
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
        Regex::new(r"(?i)\b(?:HD[ .-]*)?T(?:ELE)?S(?:YNC)?(?:Rip)?\b").unwrap(),
        transforms::value("TeleSync"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\b(?:HD[ .-]*)?T(?:ELE)?C(?:INE)?(?:Rip)?\b").unwrap(),
        transforms::value("TeleCine"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\b(?:DVD?|BD|BR)?[ .-]*Scr(?:eener)?\b").unwrap(),
        transforms::value("SCR"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bP(?:RE)?-?(HD|DVD)(?:Rip)?\b").unwrap(),
        transforms::value("SCR"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bBlu[ .-]*Ray\b(?=.*remux)").unwrap(),
        transforms::value("BluRay REMUX"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)(?:BD|BR|UHD)[- ]?remux").unwrap(),
        transforms::value("BluRay REMUX"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)remux.*\bBlu[ .-]*Ray\b").unwrap(),
        transforms::value("BluRay REMUX"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bremux\b").unwrap(),
        transforms::value("REMUX"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bBlu[ .-]*Ray\b(?![ .-]*Rip)").unwrap(),
        transforms::value("BluRay"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bUHD[ .-]*Rip\b").unwrap(),
        transforms::value("UHDRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bHD[ .-]*Rip\b").unwrap(),
        transforms::value("HDRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bMicro[ .-]*HD\b").unwrap(),
        transforms::value("HDRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\b(?:BR|Blu[ .-]*Ray)[ .-]*Rip\b").unwrap(),
        transforms::value("BRRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bBD[ .-]*Rip\b|\bBDR\b|\bBD-RM\b|\[BD[\]\) .,-]|\(BD[\]\) .,-]").unwrap(),
        transforms::value("BDRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\b(?:HD[ .-]*)?DVD[ .-]*Rip\b").unwrap(),
        transforms::value("DVDRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bVHS[ .-]*Rip?\b").unwrap(),
        transforms::value("VHSRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bDVD(?:R\d?|.*Mux)?\b").unwrap(),
        transforms::value("DVD"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bVHS\b").unwrap(),
        transforms::value("VHS"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bPPVRip\b").unwrap(),
        transforms::value("PPVRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bHD.?TV.?Rip\b").unwrap(),
        transforms::value("HDTVRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bHD.?TV\b").unwrap(),
        transforms::value("HDTV"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bDVB[ .-]*(?:Rip)?\b").unwrap(),
        transforms::value("HDTV"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bSAT[ .-]*Rips?\b").unwrap(),
        transforms::value("SATRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bTVRips?\b").unwrap(),
        transforms::value("TVRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bR5\b").unwrap(),
        transforms::value("R5"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\b(?:DL|WEB|BD|BR)MUX\b").unwrap(),
        transforms::value("WEBMux"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bWEB[ .-]*Rip\b").unwrap(),
        transforms::value("WEBRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bWEB[ .-]?DL[ .-]?Rip\b").unwrap(),
        transforms::value("WEB-DLRip"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\bWEB[ .-]*(DL|.BDrip|.DLRIP)\b").unwrap(),
        transforms::value("WEB-DL"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "quality",
        |t| &mut t.quality,
        Regex::new(r"(?i)\b(?<!\w.)WEB\b|\bWEB(?!([ \.\-\(\],]+\d))\b").unwrap(),
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
        Regex::new(r"(?i)\b(?:H[DQ][ .-]*)?CAM(?!.?(S|E|\()\d+)(?:H[DQ])?(?:[ .-]*Rip|Rp)?\b").unwrap(),
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
        Regex::new(r"(?i)\b(?:H[DQ][ .-]*)?S[ \.\-]print").unwrap(),
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
        Regex::new(r"(?i)\bPDTV\b").unwrap(),
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
        Regex::new(r"(?i)\bhevc\s?10\b").unwrap(),
        transforms::value("10bit"),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "bit_depth",
        |t| &mut t.bit_depth,
        Regex::new(r"(?i)(?:8|10|12)[-\.]?(?=bit)").unwrap(),
        transforms::value("$1bit"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "bit_depth",
        |t| &mut t.bit_depth,
        Regex::new(r"(?i)\bhdr10\b").unwrap(),
        transforms::value("10bit"),
        RegexHandlerOptions::default(),
    ));
    parser.add_handler(Handler::from_regex(
        "bit_depth",
        |t| &mut t.bit_depth,
        Regex::new(r"(?i)\bhi10\b").unwrap(),
        transforms::value("10bit"),
        RegexHandlerOptions::default(),
    ));

    parser.add_handler(Handler::new("bit_dpeth", |context| {
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
        Regex::new(r"(?i)\bDV\b|dolby.?vision|\bDoVi\b").unwrap(),
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
        Regex::new(r"(?i)HDR10(?:\+|[-\.\s]?plus)").unwrap(),
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
        Regex::new(r"(?i)\bHDR(?:10)?\b").unwrap(),
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
        Regex::new(r"(?i)\bSDR\b").unwrap(),
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
        Regex::new(r"(?i)\b[hx][\. \-]?264\b").unwrap(),
        transforms::value("avc"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "codec",
        |t| &mut t.codec,
        Regex::new(r"(?i)\b[hx][\. \-]?265\b").unwrap(),
        transforms::value("hevc"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "codec",
        |t| &mut t.codec,
        Regex::new(r"(?i)HEVC10(bit)?\b|\b[xh][\. \-]?265\b").unwrap(),
        transforms::value("hevc"),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "codec",
        |t| &mut t.codec,
        Regex::new(r"(?i)\bhevc(?:\s?10)?\b").unwrap(),
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
        Regex::new(r"(?i)\bdivx|xvid\b").unwrap(),
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
        Regex::new(r"(?i)\bavc\b").unwrap(),
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
        Regex::new(r"(?i)\bav1\b").unwrap(),
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
        Regex::new(r"(?i)\b(?:mpe?g\d*)\b").unwrap(),
        transforms::value("mpeg"),
        RegexHandlerOptions {
            remove: true,
            skip_if_already_found: false,
            ..Default::default()
        },
    ));

    lazy_static! {
        static ref REMOVE_SPACE_AND_DASH: Regex = Regex::new(r"[ .-]+").unwrap();
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
        Regex::new(r"(?i)\bDDP?5[ \.\_]1\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("5.1"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "channels",
        |t| &mut t.channels,
        Regex::new(r"(?i)\b5\.1(?:ch)?\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("5.1"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "channels",
        |t| &mut t.channels,
        Regex::new(r"(?i)\b7[\.\- ]1(?:\.?ch(?:annel)?)?\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("7.1"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "channels",
        |t| &mut t.channels,
        Regex::new(r"(?i)\b2\.0\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("2.0"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "channels",
        |t| &mut t.channels,
        Regex::new(r"(?i)\bstereo\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("stereo"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "channels",
        |t| &mut t.channels,
        Regex::new(r"(?i)\bmono\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("mono"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: false,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "channels",
        |t| &mut t.channels,
        Regex::new(r"(?i)\b(?:x[2-4]|5[\W]1(?:x[2-4])?)\b").unwrap(),
        transforms::chain_transforms(transforms::replace_value("5.1"), transforms::uniq_concat),
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));
    parser.add_handler(Handler::from_regex(
        "channels",
        |t| &mut t.channels,
        Regex::new(r"(?i)\b2\.0(?:x[2-4])\b").unwrap(),
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
        Regex::new(r"(?i)\bDDP5[ \.\_]1\b").unwrap(),
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
        Regex::new(r"(?i)\b(?!.+HR)(DTS.?HD.?Ma(ster)?|DTS.?X)\b").unwrap(),
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
        Regex::new(r"(?i)\bDTS(?!(.?HD.?Ma(ster)?|.X)).?(HD.?HR|HD)?\b").unwrap(),
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
        Regex::new(r"(?i)\b(Dolby.?)?Atmos\b").unwrap(),
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
        Regex::new(r"(?i)\b(TrueHD|\.True\.)\b").unwrap(),
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
        Regex::new(r"(?i)\bFLAC(?:\+?2\.0)?(x[2-4])?\b").unwrap(),
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
        Regex::new(r"(?i)\bEAC-?3(?:[. -]?[256]\.[01])?\b").unwrap(),
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
        Regex::new(r"(?i)\bAC-?3(x2)?(?:[ .-](5\.1)?[x+]2\.?0?x?3?)?\b").unwrap(),
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
        Regex::new(r"(?i)\b5\.1(ch)?\b").unwrap(),
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
        Regex::new(r"(?i)\b(DD2?[\+p]2?(.?5.1)?|DD Plus|Dolby Digital Plus)\b").unwrap(),
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
        Regex::new(r"(?i)\b(DD|Dolby.?Digital.?)2?(5.?1)?(?!.?(Plus|P|\+))\b").unwrap(),
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
        Regex::new(r"(?i)\bDolbyD\b").unwrap(),
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
        Regex::new(r"(?i)\bQ?Q?AAC(x?2)?\b").unwrap(),
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
        Regex::new(r"(?i)\b(H[DQ])?.?(Clean.?Aud(io)?)\b").unwrap(),
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
        Regex::new(
            r"(?i)- ?(?!\d+$|S\d+|\d+x|ep?\d+|[^\[]+\]$)([^\-\. \[]+[^\-\. \[\)\]\d][^\-\. \[\)\]]*)(?:\[[\w.-]+])?(?=\.\w{2,4}$|$)",
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
        Regex::new(r"(?i)\.?[\[(]?\b(MKV|AVI|MP4|WMV|MPG|MPEG)\b[\])]?").unwrap(),
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
        Regex::new(r"(?i)\bvol(?:s|umes?)?[. -]*(?:\d{1,2}[., +/\\&-]+)+\d{1,2}\b").unwrap(),
        transforms::range_func,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));

    lazy_static! {
        static ref VOLUME_REGEX: Regex = Regex::new(r"(?i)\bvol(?:ume)?[. -]*(\d{1,2})").unwrap();
    }

    parser.add_handler(Handler::new("volumes", |context| {
        let title = &context.title;
        let start_index = context.matched.get("year").map(|y| y.match_index).unwrap_or(0);

        if let Some(cap) = VOLUME_REGEX.captures(&title[start_index..]).ok().flatten() {
            let m = cap.get(0).unwrap();
            let vol = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();

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
        Regex::new(r"(?i)\b(temporadas?|completa)\b").unwrap(),
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
        Regex::new(r"(?i)(?:\bthe\W)?(?:\bcomplete|collection|dvd)?\b[ .]?\bbox[ .-]?set\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::new(r"(?i)(?:\bthe\W)?(?:\bcomplete|collection|dvd)?\b[ .]?\bmini[ .-]?series\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::new(r"(?i)(?:\bthe\W)?(?:\bcomplete|full|all)\b.*\b(?:series|seasons|collection|episodes|set|pack|movies)\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::new(r"(?i)\b(?:series|seasons|movies?)\b.*\b(?:complete|collection)\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::new(r"(?i)(?:\bthe\W)?\bultimate\b[ .]\bcollection\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            skip_if_already_found: false,
            ..Default::default()
        },
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::new(r"(?i)\bcollection\b.*\b(?:set|pack|movies)\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions::default(),
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::new(r"(?i)\bcollection\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            skip_from_title: true,
            ..Default::default()
        },
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::new(r"(?i)duology|trilogy|quadr[oi]logy|tetralogy|pentalogy|hexalogy|heptalogy|anthology").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            skip_if_already_found: false,
            ..Default::default()
        },
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::new(r"(?i)\bcompleta\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            remove: true,
            ..Default::default()
        },
    ));

    parser.add_handler(Handler::from_regex(
        "complete",
        |t| &mut t.complete,
        Regex::new(r"(?i)\bsaga\b").unwrap(),
        transforms::true_if_found,
        RegexHandlerOptions {
            skip_from_title: true,
            skip_if_already_found: true,
            ..Default::default()
        },
    ));
}
