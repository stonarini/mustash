pub trait Handler<C, R> {
    fn handle_command(cmd: C) -> R;
}
