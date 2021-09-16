//Establishing a connection with the server on port 5500y
const socket = io('http://localhost:3000');

//Grabbing the button element by the ID
const HelloBtn = document.getElementById('helloButton');
const SenderBtn = document.getElementById('Sender');

let clientPlayers = {};

//Callback function fires on the event called 'serverToClient'
socket.on('serverToClient', (data) => {
    alert(data);
})

//Client sends a message at the moment it got connected with the server
socket.emit('clientToServer', "Hello, server!");

//Event listener on the button element: sends a message to the server when clicked
HelloBtn.addEventListener('click', () => {
    socket.emit('clientToClient', "Hello to the fellow clients!");
})


socket.emit('newPlayer', "hi!");

//Event sends the number to the server
SenderBtn.addEventListener('click', () => {
    SenderBtn.disabled = true;
    socket.emit('checkSecretNumber', document.getElementById("Text").value)
})

//Callback function of checkNumber from the server
socket.on('PlayerWon', (data) => {
    SenderBtn.disabled = false;
    alert(data);
})
socket.on('PlayerFail', (data) => {
    SenderBtn.disabled = false;
    alert(data);
})

socket.on('RoomIsFull', (data) => {
    alert(data);
})