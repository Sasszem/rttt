/// Game result enum
/// Constructed by [Board.get_result(Symbol)](../../board/struct.Board.html#method.result)
#[derive(PartialEq)]
pub enum GameResult {
    Won,
    Lost,
    Draw,
    Running,
}
