use crate::StreamBuf;

pub trait StreamCodec<T> {
    fn write(&self, value: &T, buf: &mut StreamBuf);
    fn read(&self, buf: &mut StreamBuf) -> Option<T>;
}

pub trait DefaultStreamCodec
where
    Self: Sized,
{
    fn stream_codec() -> impl StreamCodec<Self>;
}
