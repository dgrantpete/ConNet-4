import React from 'react';
import './style.css';

function GameBoard({ board, placePieceCallback, currentPlayer }) {
    const getLastEmptyRowIndex = (column_index) => {
        const column = board[column_index];
        let row_index = column.lastIndexOf(null);
        return row_index;
    }

    const createPieceClass = (playerPiece, isLastEmptyRow) => {
        let classes = "game-piece";

        if (isLastEmptyRow) {
            playerPiece = currentPlayer;
            classes += " last-empty-row";
        }

        if (playerPiece === null) {
            return classes;
        }

        return classes + ` player-${playerPiece}-piece`;
    }

    return (
        <div className="game-board">
            {board.map(
                (column, columnIndex) => {
                    const lastEmptyRow = getLastEmptyRowIndex(columnIndex);
                    return (
                        <div
                            className="game-column"
                            key={columnIndex}
                            onClick={() => placePieceCallback(columnIndex)}>
                            {column.map(
                                (piece, rowIndex) =>
                                    <div className="piece-slot" key={rowIndex}>
                                        <div className={createPieceClass(piece, lastEmptyRow === rowIndex)} />
                                    </div>
                            )}
                        </div>
                    );
                }
            )}
        </div>
    );
}

export default GameBoard;