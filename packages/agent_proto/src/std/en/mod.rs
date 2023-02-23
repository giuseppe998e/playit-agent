mod agent;
mod control;
mod generics;
mod hmac;
mod socket;

pub trait MessageEncode: Sized {
    fn write_into<W: ::std::io::Write>(self, buf: &mut W) -> ::std::io::Result<()>;
}
