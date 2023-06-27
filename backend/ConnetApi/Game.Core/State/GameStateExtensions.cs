namespace Game.Core.State
{
    internal static class GameStateExtensions
    {
        public static Player? ToPlayer(this GameCellState gameCellState)
        {
            return gameCellState switch
            {
                GameCellState.Player1 => Player.Player1,
                GameCellState.Player2 => Player.Player2,
                _ => null
            };
        }

        public static GameCellState ToCellState(this Player player)
        {
            return player switch
            {
                Player.Player1 => GameCellState.Player1,
                Player.Player2 => GameCellState.Player2,
                _ => throw new ArgumentException("Invalid player value.", nameof(player))
            };
        }
    }
}
