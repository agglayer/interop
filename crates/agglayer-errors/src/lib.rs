pub mod prelude {
    pub use crate::ResultExt;
}

pub trait ResultExt {
    fn log_err(self, msg: &str) -> Self;
    fn log_err_with<F>(self, f: F) -> Self
    where
        F: FnOnce() -> String;

    fn log_warn(self, msg: &str) -> Self;
    fn log_warn_with<F>(self, f: F) -> Self
    where
        F: FnOnce() -> String;
}

impl<T, E> ResultExt for Result<T, E>
where
    E: std::fmt::Debug,
{
    fn log_err(self, msg: &str) -> Self {
        if let Err(error) = &self {
            tracing::error!(?error, "{msg}");
        }
        self
    }

    fn log_err_with<F>(self, msg: F) -> Self
    where
        F: FnOnce() -> String,
    {
        if let Err(error) = &self {
            tracing::error!(?error, "{}", msg());
        }
        self
    }

    fn log_warn(self, msg: &str) -> Self {
        if let Err(error) = &self {
            tracing::warn!(?error, "{msg}");
        }
        self
    }

    fn log_warn_with<F>(self, msg: F) -> Self
    where
        F: FnOnce() -> String,
    {
        if let Err(error) = &self {
            tracing::warn!(?error, "{}", msg());
        }
        self
    }
}

#[macro_export]
macro_rules! match_err {
    ($expr:expr $(, $ty:ty : $pat:pat => $handler:expr)* $(, @default : $default_pat:pat => $default_handler:expr)? $(,)?) => {
        'result: {
            let expr = $expr;
            $(
                #[allow(unused_variables)] // Pattern variables are unused from this downcast_ref
                if let Some($pat) = expr.downcast_ref::<$ty>() {
                    let Ok($pat) = expr.downcast::<$ty>() else {
                        // This should never happen because we just checked downcast_ref
                        unreachable!("downcast_ref succeeded but downcast failed");
                    };
                    let res = $handler;
                    break 'result res;
                }
            )*
            $(
                let $default_pat = expr;
                $default_handler
            )?
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::InternalError;

    #[derive(Debug, thiserror::Error)]
    #[error("Foo")]
    struct Foo;

    #[derive(Debug, thiserror::Error)]
    #[error("Bar")]
    struct Bar(&'static str);

    #[derive(Debug, thiserror::Error)]
    #[error("Quux")]
    struct Quux;

    fn use_match_err_to_make_a_value<E: Into<eyre::Report>>(err: E) -> String {
        let err: eyre::Report = err.into();
        match_err!(err,
            Foo: _ => "foo".to_string(),
            Bar: Bar("foo") => "bar/foo".to_string(),
            Bar: Bar(s) => format!("bar({s})"),
            InternalError: _ => "internal".to_string(),
            @default: _ => "other".to_string(),
        )
    }

    #[test]
    fn match_err_to_make_a_value() {
        // Basic tests
        assert_eq!(use_match_err_to_make_a_value(Foo), "foo");
        assert_eq!(use_match_err_to_make_a_value(Bar("baz")), "bar(baz)");
        assert_eq!(use_match_err_to_make_a_value(Bar("foo")), "bar/foo");
        assert_eq!(
            use_match_err_to_make_a_value(InternalError::new()),
            "internal"
        );

        // Default works
        assert_eq!(use_match_err_to_make_a_value(Quux), "other");
        assert_eq!(use_match_err_to_make_a_value(eyre::eyre!("baz")), "other");

        // Wrapping with context has no impact
        assert_eq!(
            use_match_err_to_make_a_value(eyre::Report::from(Foo).wrap_err("with some context")),
            "foo"
        );

        // Wrapping with another match-able error shows the first match among the
        // match_err branches.
        // Not necessarily the best choice, but it should be good enough for now.
        assert_eq!(
            use_match_err_to_make_a_value(eyre::Report::from(Foo).wrap_err(Bar("baz"))),
            "foo"
        );
        assert_eq!(
            use_match_err_to_make_a_value(eyre::Report::from(Bar("baz")).wrap_err(Foo)),
            "foo"
        );

        // Wrapping an unknown error with a match-able error does show the match-able
        // error
        assert_eq!(
            use_match_err_to_make_a_value(eyre::Report::from(Quux).wrap_err(Foo)),
            "foo"
        );
        assert_eq!(
            use_match_err_to_make_a_value(eyre::Report::from(Quux).wrap_err(Foo).wrap_err(Quux)),
            "foo"
        );
    }

    fn use_match_err_to_run_code<E: Into<eyre::Report>>(err: E) -> String {
        let err: eyre::Report = err.into();
        let mut res = "other".to_string();
        match_err!(err,
            Foo: _ => {
                res = "foo".to_string();
            },
            Bar: Bar("foo") => {
                res = "bar/foo".to_string();
            },
            Bar: Bar(s) => {
                res = format!("bar({s})");
            },
            InternalError: _ => {
                res = "internal".to_string();
            },
            // No @default branch, the result is () everywhere
        );
        res
    }

    #[test]
    fn match_err_to_run_code() {
        // Basic tests
        assert_eq!(use_match_err_to_run_code(Foo), "foo");
        assert_eq!(use_match_err_to_run_code(Bar("baz")), "bar(baz)");
        assert_eq!(use_match_err_to_run_code(Bar("foo")), "bar/foo");
        assert_eq!(use_match_err_to_run_code(InternalError::new()), "internal");

        // Default works
        assert_eq!(use_match_err_to_run_code(Quux), "other");
        assert_eq!(use_match_err_to_run_code(eyre::eyre!("baz")), "other");

        // Wrapping with context has no impact
        assert_eq!(
            use_match_err_to_run_code(eyre::Report::from(Foo).wrap_err("with some context")),
            "foo"
        );

        // Wrapping with another match-able error shows the first match among the
        // match_err branches.
        // Not necessarily the best choice, but it should be good enough for now.
        assert_eq!(
            use_match_err_to_run_code(eyre::Report::from(Foo).wrap_err(Bar("baz"))),
            "foo"
        );
        assert_eq!(
            use_match_err_to_run_code(eyre::Report::from(Bar("baz")).wrap_err(Foo)),
            "foo"
        );

        // Wrapping an unknown error with a match-able error does show the match-able
        // error
        assert_eq!(
            use_match_err_to_run_code(eyre::Report::from(Quux).wrap_err(Foo)),
            "foo"
        );
        assert_eq!(
            use_match_err_to_run_code(eyre::Report::from(Quux).wrap_err(Foo).wrap_err(Quux)),
            "foo"
        );
    }
}
