fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    njvm::NinjaVM::<
        std::io::StdinLock<'_>,
        std::io::StdoutLock<'_>,
        std::io::StderrLock<'_>,
    >::start(args);
}
