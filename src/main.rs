use ::ringbuf::traits::*;
use ::futures::prelude::*;

// This will be annotated with proc macro that generates topology
// Which will generate necessary methods
struct Registry;

// For now the behaviour of an actor is defined by started() & receive()
// (or respond()?, dig in later).
// The actor do not know about other actors,
// but their interfaces i.e. Send/Recv
pub trait Actor {
    fn started(&mut self) -> impl ::futures::Future<Output = ()>;
}

// Will have to change later
pub trait Handle {
    fn handle(&mut self) -> impl ::futures::Future<Output = ()>;
}

// Abstractions for barebone stuff


// The following example demonstrates a simple topology consisting of
// Alice (producer), Bob (transformer/processor), Charlie (consumer)
// Alice produces Foo and sends to Bob,
// Bob transforms into Bar and sends to Charlie,
// Charlie consumes Bar.
struct Alice {
    // bob: 
}

struct Bob;
struct Charlie;

struct Foo(::core::primitive::i32);
struct Bar(::std::borrow::Cow<'static, ::core::primitive::str>);

#[::tokio::main]
async fn main() {
    use ::ringbuf::traits::*;

    const N: ::core::primitive::usize = 16;

    let mut buf = ::ringbuf::StaticRb::<::core::primitive::i32, N>::default();
    let (mut tx, mut rx) = buf.split_ref();
}
