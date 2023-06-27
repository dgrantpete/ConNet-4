using System.Runtime.InteropServices;
using Game.Core.State;

namespace Game.Core.Logic
{
    public partial class RustGameLogic : IGameLogic
    {
        private readonly GameCellState[,] _gameBoard;

        public GameBoardView GameBoardView { get; private init; }

        public RustGameLogic(int rows, int cols)
        {
            _gameBoard = new GameCellState[rows, cols];

            foreach (int row in Enumerable.Range(0, rows))
            {
                foreach (int col in Enumerable.Range(0, cols))
                {
                    _gameBoard[row, col] = GameCellState.Empty;
                }
            }

            GameBoardView = new GameBoardView(_gameBoard);
        }

        public PlacePieceResult PlacePiece(int columnIndex, Player player)
        {
            unsafe
            {
                fixed (GameCellState* boardPtr = _gameBoard)
                {
                    return drop_in_piece((IntPtr)boardPtr, _gameBoard.GetLength(0), _gameBoard.GetLength(1), columnIndex, player);
                }
            }
        }

        [DllImport("game_logic_rust.dll", CallingConvention = CallingConvention.Cdecl)]
        private static extern PlacePieceResult drop_in_piece(IntPtr board, int rowCount, int columnCount, int columnIndex, Player player);

    }
}
