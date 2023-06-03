import React, { Component } from 'react'

class GameBoard extends Component {
    constructor(props) {
        super(props);

        this.state = {
            board: new Array(this.props.width).fill(null).map(() => new Array(this.props.height).fill(null)),
            currentPlayer: 1
        }

        console.log(this.state);
    }

    render() {
        return <div className="game-board">
            {this.state.board.map(
                (column, column_index) => 
                {
                    const last_empty_row = this.get_last_empty_row_index(column_index);
                    return <div 
                    className="game-column" 
                    key={column_index}
                    onClick={() => this.place_piece(column_index)}>
                        {column.map(
                            (piece, row_index) =>
                            <div className="piece-slot" key={row_index}>
                                <div className={this.create_piece_class(piece, last_empty_row === row_index)} />
                            </div>
                        )}
                    </div>
                }
            )}
        </div>;
    }

    create_piece_class(player_piece, is_last_empty_row) {
        let classes = "game-piece";

        if (is_last_empty_row) {
            player_piece = this.state.currentPlayer;
            classes += " last-empty-row";
        }

        if (player_piece === null) {
            return classes;
        }

        return classes + ` player-${player_piece}-piece`;
    }

    place_piece(column_index) {
        const row_index = this.get_last_empty_row_index(column_index);

        if (row_index === -1) {
            return;
        }

        const new_board = this.state.board.map(innerArray => [...innerArray]);
        new_board[column_index][row_index] = this.state.currentPlayer;

        this.setState({
            board: new_board,
            currentPlayer: this.state.currentPlayer === 1 ? 2 : 1
        });
    }

    get_last_empty_row_index(column_index) {
        const column = this.state.board[column_index];
        let row_index = column.lastIndexOf(null);

        return row_index;
    }
}

export default GameBoard;