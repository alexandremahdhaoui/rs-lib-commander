pub trait Command<S>: Sized {
    fn execute(strategy: &S, manifest: &str) -> Result<Self, String>;
}

// Add support for UNDO() operation.
pub trait Commander<S> {
    fn execute<T: Command<S>>(&self, manifest: &str) -> Result<T, String> {
        T::execute(self.strategy(), manifest)
    }
    fn new(strategy: S) -> Self;
    fn strategy(&self) -> &S;
}

pub struct Client<S> {
    strategy: S
}

impl<S> Commander<S> for Client<S> {
    fn new(strategy: S) -> Self {
        Self{ strategy }
    }

    fn strategy(&self) -> &S {
        return &self.strategy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ExampleStrategy {
        pattern: String
    }

    #[derive(Debug, PartialEq)]
    struct ExampleCommand {}

    impl Command<ExampleStrategy> for ExampleCommand {
        fn execute(strategy: &ExampleStrategy, manifest: &str) -> Result<Self, String> {
            if manifest == strategy.pattern {
                Ok(Self {})
            } else {
                error()
            }
        }
    }

    fn command() -> ExampleCommand {
        ExampleCommand {}
    }

    fn error() -> Result<ExampleCommand,String> {
        Err("failed".to_string())
    }

    fn client() -> Client<ExampleStrategy> {
        Client::new(ExampleStrategy {pattern: String::from("works") })
    }

    #[test]
    fn positive() {
        let i = client();
        let e = command();
        let o = i.execute::<ExampleCommand>("works").unwrap();
        assert_eq!(e, o)
    }

    #[test]
    fn negative() {
        let i = client();
        let e = error();
        let o = i.execute::<ExampleCommand>("");
        assert_eq!(e, o)
    }
}
