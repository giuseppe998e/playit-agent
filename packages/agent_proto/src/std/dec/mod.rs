mod agent;
mod control;
mod generics;
mod hmac;
mod socket;

pub trait MessageDecode: Sized {
    fn read_from<R: ::std::io::Read>(input: &mut R) -> ::std::io::Result<Self>;
}
