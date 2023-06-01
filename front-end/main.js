// Get all the columns
let columns = document.querySelectorAll('.game-column');

// Initialize the first player
let currentPlayer = 'player-1-piece';

// Add an event listener to each column
columns.forEach(function (column) {
    // Add a mouseover event to show the semi-transparent version of the current player's piece
    column.addEventListener('mouseover', function () {
        // Find the last empty piece
        let lastEmptyPiece = getLastEmptyPiece(column);

        // If there's an empty piece, show a semi-transparent version of the current player's piece
        if (lastEmptyPiece) {
            lastEmptyPiece.classList.remove('empty-piece');
            lastEmptyPiece.classList.add(currentPlayer, 'semi-transparent');
        }
    });

    // Add a mouseout event to remove the semi-transparent version of the current player's piece
    column.addEventListener('mouseout', function () {
        // Find the last empty piece
        let lastEmptyPiece = getLastEmptyPiece(column);

        // If there's an empty piece, remove the semi-transparent version of the current player's piece
        if (lastEmptyPiece) {
            lastEmptyPiece.classList.add('empty-piece');
            lastEmptyPiece.classList.remove(currentPlayer, 'semi-transparent');
        }
    });

    // Add a click event to place the current player's piece
    column.addEventListener('click', function () {
        // Find the last empty piece
        let lastEmptyPiece = getLastEmptyPiece(column);

        // If there's an empty piece, place the current player's piece
        if (lastEmptyPiece) {
            lastEmptyPiece.classList.remove('empty-piece', 'semi-transparent');
            lastEmptyPiece.classList.add(currentPlayer);

            // Switch the current player
            currentPlayer = (currentPlayer === 'player-1-piece') ? 'player-2-piece' : 'player-1-piece';
        }
    });
});

// Function to find the last empty piece in a column
function getLastEmptyPiece(column) {
    // Get all the pieces in the column
    let pieces = column.querySelectorAll('.game-piece');

    // Initialize the last empty piece
    let lastEmptyPiece = null;

    // Go through the pieces from bottom to top
    for (let i = pieces.length - 1; i >= 0; i--) {
        if (pieces[i].classList.contains('empty-piece') || pieces[i].classList.contains('semi-transparent')) {
            lastEmptyPiece = pieces[i];
            break;
        }
    }

    return lastEmptyPiece;
}
