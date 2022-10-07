# rs-lib-commander

The commander approach uses *Bridge*,*Strategy*,*Command* pattern to decouple the client usage from its implementation.

The user of `struct Client` can specify a `strategy` & a `bridge` when building the `Client`.
- Bridge abstracts any implementation interface that should be used during execution.
- Strategy abstracts a particular algorithm that should be used during execution.
  Strategy is especially useful because it helps the user of the Client to e.g. deserialize a struct from any arbitrary
  `rs-lib-strategy-serde` implementation.
  This example lets us easily understand we should let the user specify how to deserialize the potential raw data the 
  Bridge will intercept.

Their usage is on the responsibility of the implementer, but these fields are named explicitly to ensure best practices.

## TODO:

- [x] implement `Bridge` in the `Commander` & `Command` traits.
- [x] create the `ClientBuilder` (very trivial but why not having a builder for this type).
- [] add tests for `ClientBuilder`.
- [] implement `undo` operation for `trait Command`,`trait Commander`.
- [] Implement `memento` to allow an easier implementation of `undo`.
  Especially because we don't want to explecitely declare & manage `state`,`history` neither in `Command` nor in 
  `Commander`.