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
}
