use std::time::Instant;

use tracing::{
    event,
    span::{Attributes, Id},
    Subscriber,
};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

pub struct InstantLayer;

impl<S> Layer<S> for InstantLayer
where
    S: Subscriber,
    S: for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, _attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).unwrap();

        span.extensions_mut().insert(Instant::now());
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        let span = ctx.span(&id).unwrap();
        let extension = span.extensions();

        let started_at = extension.get::<Instant>().unwrap();

        event!(
            tracing::Level::DEBUG,
            target = span.metadata().target(),
            "Span \"{}\" took {}μs",
            span.metadata().name(),
            (Instant::now() - *started_at).as_micros(),
        );
    }
}
