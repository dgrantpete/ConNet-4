using System.Runtime.InteropServices;

namespace Game.Core.State
{
    [StructLayout(LayoutKind.Sequential)]
    public struct PlacePieceResult
    {
        public bool IsWinningMove;
        public int Points;
    }
}
