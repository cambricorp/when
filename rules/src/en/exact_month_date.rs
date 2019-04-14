use crate::common_matchers::match_ordinal;
use crate::errors::DateTimeError;
use crate::tokens::{
    Adverbs, Articles, Month, Ordinals, Prepositions, Priority, TimeInterval, Token, When,
};
use crate::{consts, rules::RuleResult, stub, Dist, TokenDesc};
use chrono::prelude::*;

use nom::{alt, apply, call, many_till, named_args, take, tuple, types::CompleteStr};

define_num!(day_num: (Token::Number, Priority(0)));

define!(numeric_ord:
    [(Token::Ordinals(Ordinals::First), Priority(1)), "1st", Dist(0)] |
    [(Token::Ordinals(Ordinals::Second), Priority(1)), "2nd", Dist(0)] |
    [(Token::Ordinals(Ordinals::Third), Priority(1)), "3rd", Dist(0)] |
    [(Token::Ordinals(Ordinals::Fourth), Priority(1)), "4th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Fifth), Priority(1)), "5th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Sixth), Priority(1)), "6th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Seventh), Priority(1)), "7th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Eighth), Priority(1)), "8th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Ninth), Priority(1)), "9th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Tenth), Priority(1)), "10th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Eleventh), Priority(1)), "11th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Twelfth), Priority(1)), "12th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Thirteenth), Priority(1)), "13th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Fourteenth), Priority(1)), "14th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Fifteenth), Priority(1)), "15th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Sixteenth), Priority(1)), "16th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Seventeenth), Priority(1)), "17th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Eighteenth), Priority(1)), "18th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Nineteenth), Priority(1)), "19th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Twentieth), Priority(1)), "20th", Dist(0)] |
    [(Token::Ordinals(Ordinals::TwentyFirst), Priority(1)), "21st", Dist(0)] |
    [(Token::Ordinals(Ordinals::TwentySecond), Priority(1)), "22nd", Dist(0)] |
    [(Token::Ordinals(Ordinals::TwentyThird), Priority(1)), "23rt", Dist(0)] |
    [(Token::Ordinals(Ordinals::TwentyFourth), Priority(1)), "24th", Dist(0)] |
    [(Token::Ordinals(Ordinals::TwentyFifth), Priority(1)), "25th", Dist(0)] |
    [(Token::Ordinals(Ordinals::TwentySixth), Priority(1)), "26th", Dist(0)] |
    [(Token::Ordinals(Ordinals::TwentySeventh), Priority(1)), "27th", Dist(0)] |
    [(Token::Ordinals(Ordinals::TwentyEighth), Priority(1)), "28th", Dist(0)] |
    [(Token::Ordinals(Ordinals::TwentyNinth), Priority(1)), "29th", Dist(0)] |
    [(Token::Ordinals(Ordinals::Thirtieth), Priority(1)), "30nt", Dist(0)] |
    [(Token::Ordinals(Ordinals::ThirtiethFirst), Priority(1)), "31st", Dist(0)]
);

define!(twentieth:
    [(Token::Ordinals(Ordinals::Twentieth), Priority(2)), "twentieth", Dist(3)] |
    [(Token::Ordinals(Ordinals::Twentieth), Priority(2)), "twenty", Dist(2)]
);
define!(thirtieth:
    [(Token::Ordinals(Ordinals::Thirtieth), Priority(2)), "thirtieth", Dist(3)] |
    [(Token::Ordinals(Ordinals::Thirtieth), Priority(2)), "thirty", Dist(2)]
);

combine!(tens => twentieth | thirtieth);

define!(first:
    [(Token::Ordinals(Ordinals::First), Priority(3)), "first", Dist(2)] |
    [(Token::Ordinals(Ordinals::First), Priority(3)), "one", Dist(0)]
);
define!(second:
    [(Token::Ordinals(Ordinals::Second), Priority(3)), "second", Dist(2)] |
    [(Token::Ordinals(Ordinals::Second), Priority(3)), "two", Dist(0)]
);
define!(third:
    [(Token::Ordinals(Ordinals::Third), Priority(3)), "third", Dist(2)] |
    [(Token::Ordinals(Ordinals::Third), Priority(3)), "three", Dist(2)]
);
define!(fourth:
    [(Token::Ordinals(Ordinals::Fourth), Priority(3)), "fourth", Dist(2)] |
    [(Token::Ordinals(Ordinals::Fourth), Priority(3)), "four", Dist(1)]
);
define!(fifth:
    [(Token::Ordinals(Ordinals::Fifth), Priority(3)), "fifth", Dist(2)] |
    [(Token::Ordinals(Ordinals::Fifth), Priority(3)), "five", Dist(1)]
);
define!(sixth:
    [(Token::Ordinals(Ordinals::Sixth), Priority(3)), "sixth", Dist(2)] |
    [(Token::Ordinals(Ordinals::Sixth), Priority(3)), "six", Dist(0)]
);
define!(seventh:
    [(Token::Ordinals(Ordinals::Seventh), Priority(3)), "seventh", Dist(3)] |
    [(Token::Ordinals(Ordinals::Seventh), Priority(3)), "seven", Dist(2)]
);
define!(eighth:
    [(Token::Ordinals(Ordinals::Eighth), Priority(3)), "eighth", Dist(2)] |
    [(Token::Ordinals(Ordinals::Eighth), Priority(3)), "eight", Dist(1)]
);
define!(ninth:
    [(Token::Ordinals(Ordinals::Ninth), Priority(3)), "ninth", Dist(2)] |
    [(Token::Ordinals(Ordinals::Ninth), Priority(3)), "nine", Dist(1)]
);
define!(tenth:
    [(Token::Ordinals(Ordinals::Tenth), Priority(3)), "tenth", Dist(2)] |
    [(Token::Ordinals(Ordinals::Tenth), Priority(3)), "ten", Dist(0)]
);
define!(eleventh:
    [(Token::Ordinals(Ordinals::Eleventh), Priority(3)), "eleventh", Dist(3)] |
    [(Token::Ordinals(Ordinals::Eleventh), Priority(3)), "eleven", Dist(2)]
);
define!(twelfth:
    [(Token::Ordinals(Ordinals::Twelfth), Priority(3)), "twelfth", Dist(2)] |
    [(Token::Ordinals(Ordinals::Twelfth), Priority(3)), "twelve", Dist(2)]
);
define!(thirteenth:
    [(Token::Ordinals(Ordinals::Thirteenth), Priority(3)), "thirteenth", Dist(4)] |
    [(Token::Ordinals(Ordinals::Thirteenth), Priority(3)), "thirteen", Dist(2)]
);
define!(fourteenth:
    [(Token::Ordinals(Ordinals::Fourteenth), Priority(3)), "fourteenth", Dist(4)] |
    [(Token::Ordinals(Ordinals::Fourteenth), Priority(3)), "fourteen", Dist(2)]
);
define!(fifteenth:
    [(Token::Ordinals(Ordinals::Fifteenth), Priority(3)), "fifteenth", Dist(3)] |
    [(Token::Ordinals(Ordinals::Fifteenth), Priority(3)), "fifteen", Dist(2)]
);
define!(sixteenth:
    [(Token::Ordinals(Ordinals::Sixteenth), Priority(3)), "sixteenth", Dist(3)] |
    [(Token::Ordinals(Ordinals::Sixteenth), Priority(3)), "sixteen", Dist(2)]
);
define!(seventeenth:
    [(Token::Ordinals(Ordinals::Seventeenth), Priority(3)), "seventeenth", Dist(4)] |
    [(Token::Ordinals(Ordinals::Seventeenth), Priority(3)), "seventeen", Dist(3)]
);
define!(eighteenth:
    [(Token::Ordinals(Ordinals::Eighteenth), Priority(3)), "eighteenth", Dist(3)] |
    [(Token::Ordinals(Ordinals::Eighteenth), Priority(3)), "eighteen", Dist(2)]
);
define!(nineteenth:
    [(Token::Ordinals(Ordinals::Nineteenth), Priority(3)), "nineteenth", Dist(3)] |
    [(Token::Ordinals(Ordinals::Nineteenth), Priority(3)), "nineteen", Dist(2)]
);

combine!(ordinal => first | second | third | fourth | fifth | sixth | seventh | eighth | ninth |
                    tenth | eleventh | twelfth | thirteenth | fourteenth | fifteenth |
                    sixteenth | seventeenth | eighteenth | nineteenth | twentieth | thirtieth);

define!(of: (Token::Prepositions(Prepositions::Of), Priority(4)), "of", Dist(0));

define!(
    january:
    [(Token::Month(Month::January), Priority(5)), "january", Dist(3)] |
    [(Token::Month(Month::January), Priority(5)), "jan", Dist(0)]
);
define!(
    february:
    [(Token::Month(Month::February), Priority(5)), "february", Dist(3)] |
    [(Token::Month(Month::February), Priority(5)), "feb", Dist(0)]
);
define!(march: (Token::Month(Month::March), Priority(5)), "march", Dist(2));
define!(
    april:
    [(Token::Month(Month::April), Priority(5)), "april", Dist(2)] |
    [(Token::Month(Month::April), Priority(5)), "apr", Dist(0)]
);
define!(may: (Token::Month(Month::May), Priority(5)), "may", Dist(1));
define!(june:
    [(Token::Month(Month::June), Priority(5)), "june", Dist(2)] |
    [(Token::Month(Month::June), Priority(5)), "jun", Dist(0)]
);
define!(july:
    [(Token::Month(Month::July), Priority(5)), "july", Dist(2)] |
    [(Token::Month(Month::July), Priority(5)), "jul", Dist(0)]
);
define!(august:
    [(Token::Month(Month::August), Priority(5)), "august", Dist(2)] |
    [(Token::Month(Month::August), Priority(5)), "aug", Dist(0)]
);
define!(september:
    [(Token::Month(Month::September), Priority(5)), "september", Dist(3)] |
    [(Token::Month(Month::September), Priority(5)), "sept", Dist(1)]
);
define!(october:
    [(Token::Month(Month::October), Priority(5)), "october", Dist(3)] |
    [(Token::Month(Month::October), Priority(5)), "oct", Dist(0)]
);
define!(november:
    [(Token::Month(Month::November), Priority(5)), "november", Dist(3)] |
    [(Token::Month(Month::November), Priority(5)), "nov", Dist(0)]
);
define!(december:
    [(Token::Month(Month::December), Priority(5)), "december", Dist(3)] |
    [(Token::Month(Month::December), Priority(5)), "dec", Dist(0)]
);

combine!(month => january | february | march | april | may | june | july | august | september |
                  october | november | december);

named_args!(parse<'a>(exact_match: bool)<CompleteStr<'a>, (Vec<CompleteStr<'a>>,
                             ( TokenDesc, TokenDesc, TokenDesc, TokenDesc ) )>,

    many_till!(take!(1),
        alt!(
            // 31th of february, 1st of january
            tuple!(apply!(numeric_ord, exact_match), apply!(of, exact_match), apply!(month, exact_match),
                   stub) |
            // 31th february, 1st january
            tuple!(apply!(numeric_ord, exact_match), apply!(month, exact_match), stub, stub) |
            // twentieth first of february (from 20 to 31 inclusive)
            tuple!(apply!(tens, exact_match), apply!(ordinal, exact_match), apply!(of, exact_match),
                   apply!(month, exact_match)) |
            // eighteenth of february (from 1 to 19 inclusive)
            tuple!(apply!(ordinal, exact_match), apply!(of, exact_match), apply!(month, exact_match),
                   stub) |
            // twentieth first of february (from 20 to 31 inclusive)
            tuple!(apply!(tens, exact_match), apply!(ordinal, exact_match), apply!(month, exact_match),
                   stub) |
            // eighteenth of february (from 1 to 19 inclusive)
            tuple!(apply!(ordinal, exact_match), apply!(month, exact_match), stub, stub) |
            // 4 march
            tuple!(day_num, apply!(month, exact_match), stub, stub) |
            // january, december 
            tuple!(apply!(month, exact_match), stub, stub, stub)

            // TODO: Add "march 4th"
        )
    )
);

make_interpreter!(positions = 4);

fn make_time(res: &mut RuleResult, local: DateTime<Local>, input: &str) {
    let mut tens = None;

    // day as a plain number
    let mut day = res
        .token_by_priority(Priority(0))
        .map_or(None, |t| match t {
            Token::Number(n) => Some(n),
            _ => unreachable!(),
        });

    // day as ordinal 1st, 2nd, 3rd, etc.
    if day.is_none() {
        day = match_ordinal(res.token_by_priority(Priority(1)));
    }

    // human readable form, i.e. twenty two, etc.
    res.token_by_priority(Priority(2)).map_or((), |t| match t {
        Token::Ordinals(Ordinals::Twentieth) => tens = Some(20),
        Token::Ordinals(Ordinals::Thirtieth) => tens = Some(30),
        _ => unreachable!(),
    });

    // TODO: Simplify code somehow
    if day.is_none() {
        let ones = match_ordinal(res.token_by_priority(Priority(3)));
        if let Some(t) = tens {
            match ones {
                // for numbers less than 10 - sum tens and ones
                Some(x) if x < 10 => day = Some(x + t),
                Some(x) => {
                    res.set_error(DateTimeError::InvalidTime {
                        msg: input.to_string(),
                        what: "day".to_string(),
                        value: x + t,
                    });
                    return;
                }
                None => day = tens,
            }
        } else {
            day = ones;
        }
    }

    let day_no = day.unwrap_or(1);
    if day_no <= 0 {
        res.set_error(DateTimeError::InvalidTime {
            msg: input.to_string(),
            what: "day".to_string(),
            value: day_no,
        });
        return;
    }

    res.set_day(day_no);

    let token = res.token_by_priority(Priority(5));
    let month = token.map_or(1, |t| match t {
        Token::Month(Month::January) => 1,
        Token::Month(Month::February) => 2,
        Token::Month(Month::March) => 3,
        Token::Month(Month::April) => 4,
        Token::Month(Month::May) => 5,
        Token::Month(Month::June) => 6,
        Token::Month(Month::July) => 7,
        Token::Month(Month::August) => 8,
        Token::Month(Month::September) => 9,
        Token::Month(Month::October) => 10,
        Token::Month(Month::November) => 11,
        Token::Month(Month::December) => 12,
        _ => unreachable!(),
    });

    // TODO: Add days in month check

    res.set_month(month);
}

#[cfg(test)]
mod tests {
    use super::interpret;
    use crate::errors::DateTimeError::{AmbiguousTime, InvalidTime};
    use crate::tokens::{Priority, Pronouns, TimeOfDay, Token, When};
    use crate::{consts, MatchBounds};
    use chrono::prelude::*;

    fn fixed_time() -> DateTime<Local> {
        Local.ymd(2019, 1, 1).and_hms(0, 0, 0)
    }

    #[test]
    fn test_exact_month() {
        let result = interpret("3rd march", false, fixed_time());
        assert_eq!(result.get_day(), 3);
        assert_eq!(result.get_month(), 3);

        let result = interpret("3rd of march", false, fixed_time());
        assert_eq!(result.get_day(), 3);
        assert_eq!(result.get_month(), 3);

        let result = interpret("3 march", false, fixed_time());
        assert_eq!(result.get_day(), 3);
        assert_eq!(result.get_month(), 3);

        let result = interpret("twenty seventh of april", false, fixed_time());
        assert_eq!(result.get_day(), 27);
        assert_eq!(result.get_month(), 4);

        let result = interpret("thirtieth of december", false, fixed_time());
        assert_eq!(result.get_day(), 30);
        assert_eq!(result.get_month(), 12);

        let result = interpret("december", false, fixed_time());
        assert_eq!(result.get_month(), 12);

        let result = interpret("twenty fourteen of april", false, fixed_time());
        assert_eq!(
            result.context,
            Err(InvalidTime {
                msg: "twenty fourteen of april".to_owned(),
                what: "day".to_owned(),
                value: 34,
            })
        );

        let result = interpret("-3 march", false, fixed_time());
        assert_eq!(
            result.context,
            Err(InvalidTime {
                msg: "-3 march".to_owned(),
                what: "day".to_owned(),
                value: -3,
            })
        );
    }

}
