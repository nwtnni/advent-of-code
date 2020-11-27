use reqwest::blocking;

pub struct Client {
    _token: String,
    _inner: blocking::Client,
}
