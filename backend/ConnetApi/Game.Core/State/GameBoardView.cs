namespace Game.Core.State
{
    public class GameBoardView
    {
        private readonly GameCellState[,] _board;

        public GameBoardView(GameCellState[,] board)
        {
            _board = board;
        }

        public GameCellState this[int row, int col]
        {
            get
            {
                return _board[row, col];
            }
        }

        public int RowCount
        {
            get
            {
                return _board.GetLength(0);
            }
        }

        public int ColumnCount
        {
            get
            {
                return _board.GetLength(1);
            }
        }
    }
}
