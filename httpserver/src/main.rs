mod router;
mod handler;
mod server;

fn main() {
    let server = server::Server::new("localhost:3000");
    server.run();
}


#[cfg(test)]
pub mod test {
    use std::ops::Add;

    pub struct Tst {
        tstr: String
    }

    #[test]
    fn test() {
        let ta = Tst {
            tstr: "ttta".to_string()
        };

        let mut tb = Tst {
            tstr: "tttb".to_string()
        };

        println!("{}", ta.tstr);
    }
}