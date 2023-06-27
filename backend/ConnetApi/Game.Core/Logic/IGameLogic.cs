using Game.Core.State;

namespace Game.Core.Logic
{
    public interface IGameLogic
    {
        public GameBoardView GameBoardView { get; }

        public PlacePieceResult PlacePiece(int columnIndex, Player player);

        public bool IsColumnFull(int columnIndex) => GameBoardView[GameBoardView.RowCount - 1, columnIndex] != GameCellState.Empty;
    }
}
