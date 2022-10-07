pub trait Command<B, S>: Sized {
    fn execute(bridge: &B, strategy: &S, manifest: &str) -> Result<Self, String>;
}

// Add support for UNDO() operation.
pub trait Commander<B, S> {
    fn execute<T: Command<B, S>>(&self, manifest: &str) -> Result<T, String> {
        T::execute(self.get_bridge(),self.get_strategy(), manifest)
    }
    fn new(bridge: B, strategy: S) -> Self;
    fn get_bridge(&self) -> &B;
    fn get_strategy(&self) -> &S;
}

pub struct Client<B, S> {
    bridge: B,
    strategy: S
}

impl<B, S> Commander<B, S> for Client<B, S> {
    fn new(bridge: B, strategy: S) -> Self {
        Self{ bridge, strategy }
    }

    fn get_bridge(&self) -> &B {
        return &self.bridge
    }
    fn get_strategy(&self) -> &S {
        return &self.strategy
    }
}

pub struct ClientBuilder<B, S> {
    client: Client<Option<B>, Option<S>>
}
impl<B, S> ClientBuilder<B, S> {
    fn new() -> Self {
        Self{ client: Client { bridge: None, strategy: None }}
    }
    fn set_bridge(&mut self, bridge: B) -> &mut ClientBuilder<B, S> {
        self.client.bridge = Some(bridge);
        self
    }
    fn set_strategy(&mut self, strategy: S) -> &mut ClientBuilder<B, S> {
        self.client.strategy = Some(strategy);
        self
    }
    fn build(&mut self) -> Client<B, S> {
        Client::new(self.client.bridge.take().unwrap(), self.client.strategy.take().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ExampleStrategy;

    struct ExampleBridge {
        pattern: String
    }

    #[derive(Debug, PartialEq)]
    struct ExampleCommand {}

    impl Command<ExampleBridge, ExampleStrategy> for ExampleCommand {
        fn execute(bridge: &ExampleBridge, _strategy: &ExampleStrategy, manifest: &str) -> Result<Self, String> {
            if manifest == bridge.pattern {
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

    fn client() -> Client<ExampleBridge, ExampleStrategy> {
        Client::new(ExampleBridge {pattern: String::from("works")}, ExampleStrategy{})
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
