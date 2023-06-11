import './App.css';
import Game from './components/game-board/game.jsx';
import Header from './components/header/header.jsx';
import React, { Component } from 'react';

class App extends Component {
  render() {
    console.log('render');

    return (
      <div className="App">
        <Header />
        <Game height={6} width={7} />
      </div>
    );
  }
}

export default App;
