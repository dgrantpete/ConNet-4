import React, { useState } from 'react';

import GameBoard from './game-board';

function Game({ height, width }) {
    const [board, setBoard] = useState(generateEmptyBoard(width, height));
    const [currentPlayer, setCurrentPlayer] = useState(1);

    const getLastEmptyRowIndex = (column_index) => {
        const column = board[column_index];
        let row_index = column.lastIndexOf(null);
        return row_index;
    }

    const placePiece = (column_index) => {
        const row_index = getLastEmptyRowIndex(column_index);

        if (row_index === -1) {
            return;
        }

        const new_board = board.map(innerArray => [...innerArray]);
        new_board[column_index][row_index] = currentPlayer;

        setBoard(new_board);
        setCurrentPlayer(currentPlayer === 1 ? 2 : 1);
    }
    
    return (
        <main className="game-container">
            <GameBoard board={board} placePieceCallback={placePiece} currentPlayer={currentPlayer} />
            <button onClick={() => {
                setBoard(generateEmptyBoard(width, height));
                setCurrentPlayer(1);
                }}>Reset</button>
        </main>
    );
}

function generateEmptyBoard(width, height) {
    return new Array(width).fill(null).map(() => new Array(height).fill(null));
}

export default Game;