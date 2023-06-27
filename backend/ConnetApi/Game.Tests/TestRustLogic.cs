using Game.Core.Logic;
using Game.Core.State;

namespace Game.Tests
{
    [TestClass]
    public class TestRustLogic
    {
        [TestMethod]
        public void CanInstantiate()
        {
            var game = new RustGameLogic(6, 7);
            Assert.IsNotNull(game);
        }

        [TestMethod]
        public void CanViewBoard()
        {
            var game = new RustGameLogic(6, 7);

            Assert.IsTrue(game.GameBoardView[0, 0] == GameCellState.Empty);
        }

        [TestMethod]
        public void CanPlacePiece()
        {
            var game = new RustGameLogic(6, 7);

            game.PlacePiece(0, Player.Player1);

            Assert.IsTrue(game.GameBoardView[0, 0] == GameCellState.Player1);
        }
    }
}