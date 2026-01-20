// This will be annotated with proc macro that generates topology
// Which will generate necessary methods
pub struct Registry;

// For now the behaviour of an actor is defined by started() & receive()
// (or respond()?, dig in later).
// The actor do not know about other actors,
// but their interfaces i.e. Send/Recv
pub trait Actor {
    fn started(&mut self) -> impl ::futures::Future<Output = ()>;
}

// Will have to change later
pub trait Receive<T> {
    fn recv(&mut self, _: T) -> impl ::futures::Future<Output = ()>;
}

// Abstractions for barebone stuff


pub mod sync {
    pub trait Send<T> {
        type Fut<'fut>: ::futures::Future<Output = ()>
        where
            Self: 'fut;

        fn send(&mut self, _: T) -> Self::Fut<'_>;
    }

    pub trait Recv<T> {
        type Fut<'fut>: ::futures::Future<Output = T>
        where
            Self: 'fut;

        fn recv(&mut self) -> Self::Fut<'_>;
    }

    mod util {
        #[repr(transparent)]
        pub struct InfallibleFut<Fut>(pub(crate) Fut);

        impl<Fut, T, Err> ::futures::Future for InfallibleFut<Fut>
        where 
            Fut: ::futures::Future<Output = ::core::result::Result<T, Err>>,
        {
            type Output = T;

            fn poll(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>) -> ::core::task::Poll<Self::Output> {
                let fut = unsafe { self.map_unchecked_mut(|s| &mut s.0) };

                match fut.poll(cx) {
                    ::core::task::Poll::Pending => ::core::task::Poll::Pending,
                    ::core::task::Poll::Ready(::core::result::Result::Ok(val)) => ::core::task::Poll::Ready(val),
                    ::core::task::Poll::Ready(::core::result::Result::Err(_)) => ::core::unreachable!(),
                }
            }
        }
    }

    mod spsc {
        #[cfg(feature = "ringbuf")]
        mod ringbuf {}

        #[cfg(feature = "flume")]
        mod flume {
            impl<T> crate::sync::Send<T> for ::flume::Sender<T> {
                type Fut<'fut> = crate::sync::util::InfallibleFut<::flume::r#async::SendFut<'fut, T>>
                where
                    Self: 'fut;
            
                fn send(&mut self, val: T) -> Self::Fut<'_> {
                    crate::sync::util::InfallibleFut(self.send_async(val))
                }
            }

            impl<T> crate::sync::Recv<T> for ::flume::Receiver<T> {
                type Fut<'fut> = crate::sync::util::InfallibleFut<::flume::r#async::RecvFut<'fut, T>>
                where
                    Self: 'fut;
            
                fn recv(&mut self) -> Self::Fut<'_> {
                    crate::sync::util::InfallibleFut(self.recv_async())
                }
            }
        }
    }
}
