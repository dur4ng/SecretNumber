const express = require('express')
const app = express()
const port = 3000
const server = app.listen(port)
const io = require('socket.io')(server)

// dictionary
let players = {};

//Create a random number for the game
const SecretNumber = Math.random();
console.log(SecretNumber);
//Hello World line taken from the express website
app.get('/', (req, res) => {
  res.send('Hello World!')
})

//The 'connection' is a reserved event name in socket.io
//For whenever a connection is established between the server and a client
io.on('connection', (socket) => {

    socket.on('newPlayer', data => {
        if (Object.keys(players).length <= 2){
            console.log("New client connected, with id: "+socket.id);
            players[socket.id] = data;
            console.log("Player: "+players[socket.id]);
            console.log("Current number of players: "+Object.keys(players).length);
            console.log("players dictionary: ", players);
        }
        
    })
    socket.on('disconnect', function(){
        delete players[socket.id];
        console.log("Goodbye client with id "+socket.id);
        console.log("Current number of players: "+Object.keys(players).length);
        socket.broadcast.emit('PlayerWon', "Your challenger disconnected!!!")
        io.emit('updatePlayers', players);
    })

    socket.on('checkSecretNumber', data => {
        console.log(data);
        if (data == SecretNumber) {
            socket.broadcast.emit('PlayerWon', "One Player won!!!")
        } else {
            socket.broadcast.emit('PlayerFail', "One Player fail!!!")
        }
    })

});