import logo from './logo.svg';
import './App.css';
import GameBoard from './GameBoard';
import React, { Component } from 'react'

class App extends Component {
  render() {
    console.log('render');

    return (
      <main>
        <div className="game-container">
          <GameBoard height={6} width={7} />
        </div>
      </main>
    );
  }
}

export default App;
