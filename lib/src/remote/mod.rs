pub mod origin;

#[derive(Debug)]
pub struct RemoteAddr {
    pub protocol: String,
    pub url: String,
    pub path: String,
}

pub fn parse_local_config_url(url: &str) -> RemoteAddr {
    let protocol: Vec<&str> = url.split("://").collect();
    let url: Vec<&str> = protocol[1].split("/").collect();
    let repository_path: String = url[1..].join("/");
    RemoteAddr { protocol: protocol[0].to_owned(), url: url[0].to_owned(), path: repository_path }
}
