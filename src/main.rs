fn main() {
    njvm::NinjaVM::<std::io::StdinLock<'_>, std::io::StdoutLock<'_>>::start();
}
