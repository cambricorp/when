use super::combinators::{stub, tokenize_count_symbols, Dist};
use super::consts;
use super::errors::SemanticError;
use super::rules::{Context, RuleResult, TokenDesc};
use super::tokens::{Priority, Pronouns, TimeOfDay, Token, When};
use chrono::prelude::*;

use nom::{alt, apply, call, many_till, named_args, tuple, types::CompleteStr};

define!(now: (Token::When(When::Now), Priority(0)), "now", Dist(0));

define!(last: (Token::When(When::Last), Priority(1)), "last", Dist(1));
define!(next: (Token::When(When::Next), Priority(1)), "next", Dist(1));
define!(past: (Token::When(When::Past), Priority(1)), "past", Dist(1));
define!(this: (Token::Pronouns(Pronouns::This), Priority(1)), "this", Dist(1));

combine!(adj => last | this | next | past);

define!(today: (Token::When(When::Today), Priority(1)), "today", Dist(1));
define!(tonight: (Token::When(When::Tonight), Priority(1)), "tonight", Dist(2));
define!(
    tomorrow:
    [(Token::When(When::Tomorrow), Priority(1)), "tomorrow", Dist(2)] |
    [(Token::When(When::Tomorrow), Priority(1)), "tmr", Dist(0)]
);
define!(yesterday: (Token::When(When::Yesterday), Priority(1)), "yesterday", Dist(2));

combine!(when => today | tonight | yesterday | tomorrow);

define!(night: (Token::TimeOfDay(TimeOfDay::Night), Priority(2)), "night", Dist(1));
define!(morning: (Token::TimeOfDay(TimeOfDay::Morning), Priority(2)), "morning", Dist(2));
define!(evening: (Token::TimeOfDay(TimeOfDay::Evening), Priority(2)), "evening", Dist(2));
define!(noon: (Token::TimeOfDay(TimeOfDay::Noon), Priority(2)), "noon", Dist(1));
define!(afternoon: (Token::TimeOfDay(TimeOfDay::Afternoon), Priority(2)), "afternoon", Dist(2));

combine!(time_of_day => night | morning | evening | noon | afternoon);

named_args!(parse<'a>(exact_match: bool)<CompleteStr<'a>, (Vec<usize>,
                             ( TokenDesc, TokenDesc, ) )>,
    many_till!(tokenize_count_symbols,
        alt!(
            // last night, this morning, etc.
            tuple!(apply!(adj, exact_match), apply!(time_of_day, exact_match)) |
            // tomorrow evening, today morning, etc.
            tuple!(apply!(when, exact_match), apply!(time_of_day, exact_match)) |
            // today, tomorrow, yesterday, etc.
            tuple!(apply!(when, exact_match), stub) |
            // now
            tuple!(apply!(now, exact_match), stub) |
            // night, morning, evening, etc
            tuple!(apply!(time_of_day, exact_match), stub)
        )
    )
);

make_interpreter!(positions = 2);

fn make_time<'a, 'b, Tz: TimeZone>(
    res: &'a RuleResult,
    _tz_aware: DateTime<Tz>,
    _input: &'b str,
) -> Result<Context, SemanticError<'b>> {
    let mut ctx = Context::default();

    let token = res.token_by_priority(Priority(1));
    if token.is_some() {
        match token.unwrap() {
            Token::When(When::Last) | Token::When(When::Past) => {
                ctx.set_duration(-24 * i64::from(consts::HOUR));
            }
            Token::When(When::Next) => {
                ctx.set_duration(24 * i64::from(consts::HOUR));
            }
            Token::When(When::Tomorrow) => {
                ctx.set_duration(24 * i64::from(consts::HOUR));
            }
            Token::When(When::Yesterday) => {
                ctx.set_duration(-24 * i64::from(consts::HOUR));
            }
            Token::When(When::Tonight) => {
                ctx.hour = Some(23);
                ctx.minute = Some(0);
            }
            _ => (),
        }
    }

    let token = res.token_by_priority(Priority(2));

    if token.is_some() {
        match token.unwrap() {
            Token::TimeOfDay(TimeOfDay::Morning) => {
                ctx.hour = Some(8);
                ctx.minute = Some(0);
            }
            Token::TimeOfDay(TimeOfDay::Noon) => {
                ctx.hour = Some(12);
                ctx.minute = Some(0);
            }
            Token::TimeOfDay(TimeOfDay::Afternoon) => {
                ctx.hour = Some(15);
                ctx.minute = Some(0);
            }
            Token::TimeOfDay(TimeOfDay::Evening) => {
                ctx.hour = Some(18);
                ctx.minute = Some(0);
            }
            Token::TimeOfDay(TimeOfDay::Night) => {
                ctx.hour = Some(23);
                ctx.minute = Some(0);
            }
            _ => (),
        }
    }

    Ok(ctx)
}

#[cfg(test)]
mod tests {
    use super::interpret;
    use crate::rules::consts;
    use crate::rules::rules::MatchBounds;
    use chrono::prelude::*;

    fn fixed_time() -> DateTime<Local> {
        Local.ymd(2019, 1, 1).and_hms(0, 0, 0)
    }

    #[test]
    fn test_casual_date() {
        let result = interpret("The deadline is now, ok", false, fixed_time()).unwrap();
        assert_eq!(
            result.bounds,
            Some(MatchBounds {
                start_idx: 16,
                end_idx: 19
            })
        );
        assert_eq!(result.get_duration_sec(), 0);

        let result = interpret("The deadline is today", false, fixed_time()).unwrap();
        assert_eq!(
            result.bounds,
            Some(MatchBounds {
                start_idx: 16,
                end_idx: 21
            })
        );
        assert_eq!(result.get_duration_sec(), 0);

        let result = interpret("The deadline is tonight", false, fixed_time()).unwrap();
        assert_eq!(
            result.bounds,
            Some(MatchBounds {
                start_idx: 16,
                end_idx: 23
            })
        );
        assert_eq!(result.get_hours(), 23);
        assert_eq!(result.get_minutes(), 0);

        let result = interpret("The deadline is tomorrow", false, fixed_time()).unwrap();
        assert_eq!(
            result.bounds,
            Some(MatchBounds {
                start_idx: 16,
                end_idx: 24
            })
        );
        assert_eq!(result.get_duration_sec(), 24 * consts::HOUR as i64);

        let result = interpret("The deadline was yesterday", false, fixed_time()).unwrap();
        assert_eq!(
            result.bounds,
            Some(MatchBounds {
                start_idx: 17,
                end_idx: 26
            })
        );
        assert_eq!(result.get_duration_sec(), -24 * consts::HOUR as i64);

        let result = interpret("Please call me tomorrow evening", false, fixed_time()).unwrap();
        assert_eq!(
            result.bounds,
            Some(MatchBounds {
                start_idx: 15,
                end_idx: 31
            })
        );
        assert_eq!(result.get_duration_sec(), 24 * consts::HOUR as i64);
        assert_eq!(result.get_hours(), 18);

        let result = interpret("He told me that yesterday morning", false, fixed_time()).unwrap();
        assert_eq!(
            result.bounds,
            Some(MatchBounds {
                start_idx: 16,
                end_idx: 33
            })
        );
        assert_eq!(result.get_duration_sec(), -24 * consts::HOUR as i64);
        assert_eq!(result.get_hours(), 8);

        let result = interpret("last night I fell asleep", false, fixed_time()).unwrap();
        assert_eq!(
            result.bounds,
            Some(MatchBounds {
                start_idx: 0,
                end_idx: 10
            })
        );
        assert_eq!(result.get_duration_sec(), -24 * consts::HOUR as i64);
        assert_eq!(result.get_hours(), 23);

        let result = interpret("come next evening please", false, fixed_time()).unwrap();
        assert_eq!(
            result.bounds,
            Some(MatchBounds {
                start_idx: 5,
                end_idx: 17
            })
        );
        assert_eq!(result.get_duration_sec(), 24 * consts::HOUR as i64);
        assert_eq!(result.get_hours(), 18);
    }
}
